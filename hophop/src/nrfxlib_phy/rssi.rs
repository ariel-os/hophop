// SPDX-FileCopyrightText: Copyright Christian Amsüss <chrysn@fsfe.org>, Silano Systems
// SPDX-License-Identifier: MIT OR Apache-2.0

use heapless::{
    box_pool,
    pool::boxed::{Box, BoxBlock},
};
use nrf_modem::{ErrorSource, nrfxlib_sys};
use static_cell::StaticCell;

use super::{DECT_EVENTS, DectEvent, DectPhy, MixedError};

// Storage for RSSI results.
//
// These are passed to the caller owned; consequently, reception is fallible if the user does not
// drop their items in time.
//
// This can probably be optimized, eg. by being an atomic ring buffer with the sender living in the
// IRQ and the receiver in the consuming task.
box_pool!(RssiPool: [u8; 240]);

/// Initiates the RSSI pool.
#[inline]
pub(super) fn init() {
    static RSSI_BUFFER: StaticCell<[BoxBlock<[u8; 240]>; 16]> = StaticCell::new();
    for b in RSSI_BUFFER.init_with(|| core::array::from_fn(|_| BoxBlock::new())) {
        RssiPool.manage(b);
    }
}

/// Resulting data slice of a single RSSI measurement.
pub struct RssiResult(Box<RssiPool>);

impl RssiResult {
    pub fn data(&self) -> &[u8] {
        &*self.0
    }
}

/// # Safety
///
/// This function must only be called in the event handler, which is when libmodem implies that the
/// pointers inside the event struct are valid.
#[inline]
pub(super) unsafe fn event(rssi: *const nrfxlib_sys::nrf_modem_dect_phy_rssi_event) -> DectEvent {
    // SAFETY: Checked the discriminator
    let rssi = unsafe { &*rssi };
    // SAFETY: It is valid now, which is as long as we use it
    // Casting because it's not precisely a signed integer anyuway (and our buffer is just
    // bytes).
    let meas = unsafe { core::slice::from_raw_parts(rssi.meas as *const u8, rssi.meas_len as _) };
    defmt::trace!(
        "RSSI handle {} start {} carrier {}; {} measurements",
        rssi.handle,
        rssi.meas_start_time,
        rssi.carrier,
        meas.len(),
    );

    if let Ok(buf) = RssiPool.alloc(
        meas.try_into()
            // FIXME: As some point, we might also receive shorter RSSI data.
            .unwrap(),
    ) {
        DectEvent::Rssi(rssi.meas_start_time, Some(buf))
    } else {
        DectEvent::Rssi(rssi.meas_start_time, None)
    }
}

impl DectPhy {
    /// Read a single RSSI series.
    ///
    /// The resulting data comes in an owned buffer. It is up to the caller to drop that in time
    /// for later RSSI measurements to be taken; otherwise, later RSSI invocations will err.
    pub async fn rssi(&mut self, carrier: u16) -> Result<(u64, RssiResult), MixedError> {
        // Relevant DECT constant timing parameters are 1 frame = 10ms, each 10ms frame is composed
        // of 24 slots,

        // - Reporting interval is every 12 or 24 slots. This is consistent with the delta of
        //   starting times being precisely 691200 (24 slots = 10ms, on a 69.120MHz clock), or
        //   345600 (12 slots = 5ms).
        //
        // - Depending on the reporting interval there are 240 or 120 values, so single reading
        //   takes 2880 clock ticks, or 10 readings per slot, which corresponds to lowest number of
        //   ODFM symbols (for µ=1).
        //
        // - Requesting a duration of N gives 5*N readings. This is given in subslots, which for
        //   µ=1 is 2 subslots per slot, and thus matches 10 readings per slot, 5 per subslot.

        let params = nrfxlib_sys::nrf_modem_dect_phy_rssi_params {
            start_time: 0,
            handle: 1234567,
            carrier,
            duration: 48, // in subslots; 1 full report
            reporting_interval: nrfxlib_sys::nrf_modem_dect_phy_rssi_interval_NRF_MODEM_DECT_PHY_RSSI_INTERVAL_24_SLOTS, // 24 slots = 10ms
        };
        unsafe { nrfxlib_sys::nrf_modem_dect_phy_rssi(&raw const params) }.into_result()?;

        let mut result = None;

        loop {
            match DECT_EVENTS.receive().await.event {
                DectEvent::Rssi(start, range) => {
                    debug_assert!(result.is_none(), "Sequence violation");
                    result = Some((
                        start,
                        range.expect("We requested just one run, that fits in the receive buffer"),
                    ));
                }
                DectEvent::Completed(Ok(())) => {
                    break;
                }
                DectEvent::Completed(e) => e?,
                _ => panic!("Sequence violation"),
            }
        }

        let Some(result) = result else {
            // FIXME: Verify that it's an actual completion error that happens when requesting an
            // unsupported channel.
            panic!("Sequence violation");
        };

        Ok((result.0, RssiResult(result.1)))
    }
}
