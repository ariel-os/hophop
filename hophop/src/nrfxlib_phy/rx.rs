// SPDX-FileCopyrightText: Copyright Christian Amsüss <chrysn@fsfe.org>, Silano Systems
// SPDX-License-Identifier: MIT OR Apache-2.0

use embassy_sync::{blocking_mutex::raw::CriticalSectionRawMutex, mutex::MutexGuard};
use nrf_modem::{ErrorSource, nrfxlib_sys};

use super::{DECT_EVENTS, DectEvent, DectPhy, MixedError, RECVBUF};

#[derive(Debug, defmt::Format, Copy, Clone)]
#[non_exhaustive]
pub enum PccError {
    CrcError,
    UnexpectedEventDetails,
}

#[derive(Debug, defmt::Format, Copy, Clone)]
#[non_exhaustive]
pub enum PdcError {
    CrcError,
    OutOfSpace,
    // Maybe if it straddled the timeout? I did observe this when sender and recipient timeouts
    // could have lined up.
    NotReceived,
    PccError(PccError),
}

/// Details of a [`RecvResult`] that did result in data being received.
#[derive(Copy, Clone)]
pub struct RecvOk {
    pub pcc_time: u64,
    pub pcc_len: usize,
    pub pdc_len: Result<usize, PdcError>,
}

/// Result of a single receive operation.
///
/// This keeps a lock on the receive buffer, and must therefore be dropped before the next attempt
/// to perform any other operation.
pub struct RecvResult<'a> {
    data: MutexGuard<'static, CriticalSectionRawMutex, heapless::Vec<u8, 2400>>,
    indices: Result<RecvOk, PccError>,
    // This ensures that a .recv() result is used before the next attempt to receive something (as
    // that would panic around locking RECV_BUF).
    _phantom: core::marker::PhantomData<&'a mut ()>,
}

impl RecvResult<'_> {
    pub fn pcc_time(&self) -> Result<u64, PccError> {
        Ok(self.indices?.pcc_time)
    }
    pub fn pcc(&self) -> Result<&[u8], PccError> {
        Ok(&self.data[..self.indices?.pcc_len])
    }
    pub fn pdc(&self) -> Result<&[u8], PdcError> {
        let pcc_and_rest = self.indices.map_err(PdcError::PccError)?;
        let start = pcc_and_rest.pcc_len;
        let len = pcc_and_rest.pdc_len?;
        self.data
            .get(start..start + len)
            .ok_or(PdcError::OutOfSpace)
    }
}

/// # Safety
///
/// This function must only be called in the event handler, which is when libmodem implies that the
/// pointers inside the event struct are valid.
#[inline]
pub(super) unsafe fn event_pcc(pcc: *const nrfxlib_sys::nrf_modem_dect_phy_pcc_event) -> DectEvent {
    // SAFETY: Checked the discriminator
    let pcc = unsafe { &*pcc };

    let header_len = match pcc.phy_type {
        0 => 5,
        1 => 10,
        _ => return DectEvent::PccError(PccError::UnexpectedEventDetails),
    };
    // SAFETY: As per struct details.
    // (Easier to pass this on as bytes and do our own field access later)
    let header = &unsafe { pcc.hdr.type_2 }[..header_len];
    defmt::trace!(
        "PCC start {} handle {} phy_type {} rssi2 {} snr {} transaction {} hdr st {} hdr {:02x}",
        pcc.stf_start_time,
        pcc.handle,
        pcc.phy_type,
        pcc.rssi_2,
        pcc.snr,
        pcc.transaction_id,
        pcc.header_status,
        header
    );

    let mut recvbuf = RECVBUF
        .try_lock()
        .expect("Was checked when doing a request");

    assert_eq!(recvbuf.len(), 0);
    recvbuf
        .extend_from_slice(header)
        .expect("Length is small enough to always fit");
    DectEvent::Pcc(pcc.stf_start_time, header.len())
}

/// # Safety
///
/// This function must only be called in the event handler, which is when libmodem implies that the
/// pointers inside the event struct are valid.
#[inline]
pub(super) unsafe fn event_pdc(pdc: *const nrfxlib_sys::nrf_modem_dect_phy_pdc_event) -> DectEvent {
    // SAFETY: Checked the discriminator
    let pdc = unsafe { &*pdc };
    // SAFETY: Implied by the C API
    let data = unsafe { core::slice::from_raw_parts(pdc.data as *const u8, pdc.len) };
    defmt::trace!(
        "PDC handle {} trns {} data {:02x}",
        pdc.handle,
        pdc.transaction_id,
        data,
    );

    let mut recvbuf = RECVBUF
        .try_lock()
        .expect("Was checked when doing a request");

    // Either it fits or it doesn't; the user will see when trying to access the buffer up
    // to it.
    // FIXME: Does it makes ense to store it as far as possible?
    let _ = recvbuf.extend_from_slice(data);
    DectEvent::Pdc(data.len())
}

impl DectPhy {
    // FIXME: heapless is not great for signature yet
    pub async fn rx(&mut self) -> Result<Option<RecvResult<'_>>, MixedError> {
        self.clear_recvbuf();

        unsafe {
            // FIXME: everything
            nrfxlib_sys::nrf_modem_dect_phy_rx(&nrfxlib_sys::nrf_modem_dect_phy_rx_params {
                start_time: 0,
                handle: 54321,
                network_id: 0x12345678, // like dect_shell defaults
                mode: nrfxlib_sys::nrf_modem_dect_phy_rx_mode_NRF_MODEM_DECT_PHY_RX_MODE_SINGLE_SHOT,
                rssi_interval: nrfxlib_sys::nrf_modem_dect_phy_rssi_interval_NRF_MODEM_DECT_PHY_RSSI_INTERVAL_OFF,
                link_id: nrfxlib_sys::nrf_modem_dect_phy_link_id {
                    short_network_id: 0,
                    short_rd_id: 0,
                },
                rssi_level: 0,
                carrier: 1665, // like dect_shell ping default
                // ~ 1 second
                duration: 70000000,
                filter: nrfxlib_sys::nrf_modem_dect_phy_rx_filter {
                    short_network_id: 0,
                    is_short_network_id_used: 0,
                    receiver_identity: 0,
                },
            })
        }
        .into_result()?;

        let mut pcc = None;
        let mut pdc = None;

        loop {
            match DECT_EVENTS.receive().await.event {
                DectEvent::Pcc(start, pcc_len) => {
                    debug_assert!(pcc.is_none(), "Sequence violation");
                    pcc = Some(Ok((start, pcc_len)));
                }
                DectEvent::PccError(e) => {
                    debug_assert!(pcc.is_none(), "Sequence violation");
                    pcc = Some(Err(e));
                }
                DectEvent::Pdc(pcd_len) => {
                    debug_assert!(pdc.is_none(), "Sequence violation");
                    pdc = Some(Ok(pcd_len));
                }
                DectEvent::PdcError => {
                    debug_assert!(pdc.is_none(), "Sequence violation");
                    pdc = Some(Err(PdcError::CrcError));
                }
                DectEvent::Completed(Ok(())) => {
                    break;
                }
                DectEvent::Completed(e) => e?,
                _ => panic!("Sequence violation"),
            }
        }

        let result = match (pcc, pdc) {
            (None, None) => return Ok(None),
            (Some(Err(e)), None) => Err(e),
            (Some(Ok((pcc_time, pcc_len))), None) => Ok(RecvOk {
                pcc_time,
                pcc_len,
                pdc_len: Err(PdcError::NotReceived),
            }),
            (Some(Ok((pcc_time, pcc_len))), Some(pdc_len)) => Ok(RecvOk {
                pcc_time,
                pcc_len,
                pdc_len,
            }),
            _ => panic!("Sequence violation"),
        };

        Ok(Some(RecvResult {
            data: RECVBUF
                .try_lock()
                .expect("Was checked before, and ISR users release this before returning"),
            indices: result,
            _phantom: core::marker::PhantomData,
        }))
    }
}
