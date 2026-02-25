// SPDX-FileCopyrightText: Copyright Christian Amsüss <chrysn@fsfe.org>, Silano Systems
// SPDX-License-Identifier: MIT OR Apache-2.0

use heapless::{
    box_pool,
    pool::boxed::{Box, BoxBlock},
};
use nrf_modem::{ErrorSource, nrfxlib_sys};
use static_cell::StaticCell;

use super::{DECT_EVENTS, DectEvent, DectPhy, MixedError};

// Packet pool.
//
// FIXME: A better structure would be kind of a ring buffer that tracks droppings, or a dual bump
// allocator (because most of the time the buffers are dropped in order), but right now, this will
// do.
box_pool!(RecvPool: heapless::Vec<u8, RECV_POOL_ITEM_SIZE>);

const RECV_POOL_SIZE: usize = 16;
const RECV_POOL_ITEM_SIZE: usize = 1024;

/// Initiates the receive data pool.
#[inline]
pub(super) fn init() {
    static RECV_BUFFER: StaticCell<
        [BoxBlock<heapless::Vec<u8, RECV_POOL_ITEM_SIZE>>; RECV_POOL_SIZE],
    > = StaticCell::new();
    for b in RECV_BUFFER.init_with(|| core::array::from_fn(|_| BoxBlock::new())) {
        RecvPool.manage(b);
    }
}

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

// For error propagation out of an .as_ref() result
impl From<&PccError> for PccError {
    fn from(value: &PccError) -> Self {
        value.clone()
    }
}

#[derive(Debug, defmt::Format, Copy, Clone)]
#[non_exhaustive]
pub enum PdcError {
    CrcError,
    /// Not enough storage available right now.
    ///
    /// In the current slab pool implementation, this can mean either "all pool items are used" or
    /// "this is too big for our slabs". Left as one error because of the expectation that the slab
    /// allocator might be replaced with something that is more in the style of slices in a ring
    /// buffer (or two bump allocators).
    OutOfSpace,
    // Maybe if it straddled the timeout? I did observe this when sender and recipient timeouts
    // could have lined up.
    NotReceived,
    PccError(PccError),
}

// For error propagation out of an .as_ref() result
impl From<&PdcError> for PdcError {
    fn from(value: &PdcError) -> Self {
        value.clone()
    }
}

/// Details of a [`RecvResult`] that did result in data being received.
#[derive(Clone)]
pub struct RecvOk {
    pub pcc: Pcc,
    pub pdc: Result<Box<RecvPool>, PdcError>,
}

/// Result of a single receive operation.
///
/// This keeps a lock on the receive buffer, and must therefore be dropped before the next attempt
/// to perform any other operation.
pub type RecvResult = Result<RecvOk, PccError>;

impl RecvOk {
    pub fn pcc(&self) -> &[u8] {
        self
            .pcc
            .data
            .as_slice()
    }
    pub fn pdc(&self) -> Result<&[u8], PdcError> {
        Ok(self
            .pdc
            .as_ref()?
            .as_slice())
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
            (GotPcc(Ok(pcc)), DectEvent::Pdc(pdc)) => {
                *self = GotBoth(RecvOk {
                    pcc: *pcc,
                    pdc: pdc.ok_or(PdcError::OutOfSpace),
                });
            }
            (GotPcc(Ok(pcc)), DectEvent::PdcError) => {
                *self = GotBoth(RecvOk {
                    pcc: *pcc,
                    pdc: Err(PdcError::CrcError),
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

    pub fn finish(self) -> Option<RecvResult> {
        use RecvResultBuilder::*;
        let result = match self {
            Waiting => return None,
            GotPcc(Err(e)) => Err(e),
            GotPcc(Ok(pcc)) => Ok(RecvOk {
                pcc,
                pdc: Err(PdcError::NotReceived),
            }),
            GotBoth(sll) => Ok(sll),
        };

        Some(result)
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

    DectEvent::Pdc(
        heapless::Vec::try_from(data)
            .ok()
            .and_then(|v| RecvPool.alloc(v).ok()),
    )
}

impl DectPhy {
    pub async fn rx(&mut self) -> Result<Option<RecvResult>, MixedError> {
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
