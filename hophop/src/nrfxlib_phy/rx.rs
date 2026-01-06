// SPDX-FileCopyrightText: Copyright Christian Amsüss <chrysn@fsfe.org>, Silano Systems
// SPDX-License-Identifier: MIT OR Apache-2.0

use embassy_sync::{
    blocking_mutex::raw::CriticalSectionRawMutex,
    mutex::{Mutex, MutexGuard},
};
use nrf_modem::{ErrorSource, nrfxlib_sys};

use super::{DECT_EVENTS, DectEvent, DectPhy, MixedError};

/// Kind of a bump allocator for received data, as that doesn't fit in small events.
///
/// Might later be turned into a ring buffer if any methods support stream-processing multiple
/// events.
///
/// Sized 2400 somewhat arbitrarily because it could take 10 runs of RSSI data.
static RECVBUF: Mutex<CriticalSectionRawMutex, heapless::Vec<u8, 2400>> =
    Mutex::new(heapless::Vec::new());

#[derive(Debug, defmt::Format, Copy, Clone)]
pub struct Pcc {
    pub time: u64,
    pub data: PccData,
}

#[derive(Debug, defmt::Format, Copy, Clone)]
pub enum PccData {
    Type1([u8; 5]),
    Type2([u8; 10]),
}

impl PccData {
    pub fn as_slice(&self) -> &[u8] {
        match self {
            PccData::Type1(data) => &data[..],
            PccData::Type2(data) => &data[..],
        }
    }

    /// Creates an owned PCC data value from a PCC event
    ///
    /// # Safety
    ///
    /// This is safe to use if the struct adheres to the nrfxlib C API; in particular, if the
    /// event's hdr is initialized according to the phy_type.
    ///
    /// # Errors
    ///
    /// This returns successfully only on known phy_type values.
    unsafe fn from_c(event: &nrfxlib_sys::nrf_modem_dect_phy_pcc_event) -> Result<Self, PccError> {
        match event.phy_type {
            0 => Ok(PccData::Type1(unsafe { event.hdr.type_1 })),
            1 => Ok(PccData::Type2(unsafe { event.hdr.type_2 })),
            _ => Err(PccError::UnexpectedEventDetails),
        }
    }
}

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
    pub pcc: Pcc,
    pub pdc_len: Result<usize, PdcError>,
}

/// Result of a single receive operation.
///
/// This keeps a lock on the receive buffer, and must therefore be dropped before the next attempt
/// to perform any other operation.
pub struct RecvResult<'a> {
    data: MutexGuard<'static, CriticalSectionRawMutex, heapless::Vec<u8, 2400>>,
    // FIXME: rename misnmomer (but in a separate commit)
    indices: Result<RecvOk, PccError>,
    // This ensures that a .recv() result is used before the next attempt to receive something (as
    // that would panic around locking RECV_BUF).
    _phantom: core::marker::PhantomData<&'a mut ()>,
}

impl RecvResult<'_> {
    pub fn pcc_time(&self) -> Result<u64, PccError> {
        Ok(self.indices?.pcc.time)
    }
    pub fn pcc(&self) -> Result<&[u8], PccError> {
        Ok(self
            .indices
            .as_ref()
            .map_err(Clone::clone)?
            .pcc
            .data
            .as_slice())
    }
    pub fn pdc(&self) -> Result<&[u8], PdcError> {
        let pcc_and_rest = self.indices.map_err(PdcError::PccError)?;
        let len = pcc_and_rest.pdc_len?;
        self.data.get(..len).ok_or(PdcError::OutOfSpace)
    }
}

#[derive(Default)]
pub(crate) enum RecvResultBuilder {
    #[default]
    Waiting,
    GotPcc(Result<Pcc, PccError>),
    GotBoth(RecvOk),
}

impl RecvResultBuilder {
    pub fn feed(&mut self, event: DectEvent) -> Result<core::ops::ControlFlow<()>, MixedError> {
        use RecvResultBuilder::*;
        match (&self, event) {
            (Waiting, DectEvent::Pcc(pcc)) => {
                *self = GotPcc(Ok(pcc));
            }
            (Waiting, DectEvent::PccError(e)) => {
                *self = GotPcc(Err(e));
            }
            (GotPcc(Ok(pcc)), DectEvent::Pdc(pdc_len)) => {
                *self = GotBoth(RecvOk {
                    pcc: *pcc,
                    pdc_len: Ok(pdc_len),
                });
            }
            (GotPcc(Ok(pcc)), DectEvent::PdcError) => {
                *self = GotBoth(RecvOk {
                    pcc: *pcc,
                    pdc_len: Err(PdcError::CrcError),
                });
            }
            (_, DectEvent::Completed(c)) => {
                c?;
                return Ok(core::ops::ControlFlow::Break(()));
            }
            _ => panic!("Sequence violation"),
        }
        Ok(core::ops::ControlFlow::Continue(()))
    }

    pub fn finish<'a>(self) -> Option<RecvResult<'a>> {
        use RecvResultBuilder::*;
        let result = match self {
            Waiting => return None,
            GotPcc(Err(e)) => Err(e),
            GotPcc(Ok(pcc)) => Ok(RecvOk {
                pcc,
                pdc_len: Err(PdcError::NotReceived),
            }),
            GotBoth(sll) => Ok(sll),
        };

        Some(RecvResult {
            data: RECVBUF
                .try_lock()
                .expect("Was checked before, and ISR users release this before returning"),
            indices: result,
            _phantom: core::marker::PhantomData,
        })
    }

    pub fn is_ready(&self) -> bool {
        match &self {
            RecvResultBuilder::GotBoth(_) => true,
            RecvResultBuilder::GotPcc(Err(_)) => true,
            _ => false,
        }
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

    // SAFETY: As per struct details.
    let header = match unsafe { PccData::from_c(pcc) } {
        Ok(h) => h,
        // FIXME: Returning a PccError is not *quite* the right thing, as it'll upset later Pdc
        // evengt processings.
        Err(e) => return DectEvent::PccError(e),
    };
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

    DectEvent::Pcc(Pcc {
        time: pcc.stf_start_time,
        data: header,
    })
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

pub(crate) fn clear_recvbuf() {
    let mut recvbuf = RECVBUF
        .try_lock()
        .expect("Buffer in use; unsafe construction of DectPhy, or pending future was dropped.");
    recvbuf.clear();
    drop(recvbuf);
}

impl DectPhy {
    // FIXME: heapless is not great for signature yet
    pub async fn rx(&mut self) -> Result<Option<RecvResult<'_>>, MixedError> {
        clear_recvbuf();

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

        let mut result_builder = RecvResultBuilder::default();

        while result_builder
            .feed(DECT_EVENTS.receive().await.event)?
            .is_continue()
        {}

        Ok(result_builder.finish())
    }
}
