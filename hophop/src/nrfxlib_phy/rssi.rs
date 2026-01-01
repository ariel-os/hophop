// SPDX-FileCopyrightText: Copyright Christian Amsüss <chrysn@fsfe.org>, Silano Systems
// SPDX-License-Identifier: MIT OR Apache-2.0

use heapless::{box_pool, pool::boxed::BoxBlock};
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
box_pool!(RssiPool: RssiEvent);

/// Initiates the RSSI pool.
#[inline]
pub(super) fn init() {
    static RSSI_BUFFER: StaticCell<[BoxBlock<RssiEvent>; 16]> = StaticCell::new();
    for b in RSSI_BUFFER.init_with(|| core::array::from_fn(|_| BoxBlock::new())) {
        RssiPool.manage(b);
    }
}

/// Resulting data slice of a single RSSI measurement.
#[derive(Debug)]
pub struct RssiEvent {
    handle: u32,
    start_time: u64,
    data: [u8; 240],
}

impl RssiEvent {
    pub fn start_time(&self) -> u64 {
        self.start_time
    }

    pub fn data(&self) -> &[u8] {
        &self.data
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

    if let Ok(result) = RssiPool.alloc(RssiEvent {
        start_time: rssi.meas_start_time,
        handle: rssi.handle,
        data: meas.try_into()
            // FIXME: As some point, we might also receive shorter RSSI data.
            .unwrap(),
    }) {
        DectEvent::Rssi(Some(result))
    } else {
        DectEvent::Rssi(None)
    }
}

impl DectPhy {
    /// Read a single RSSI series.
    ///
    /// The resulting data comes in an owned buffer. It is up to the caller to drop that in time
    /// for later RSSI measurements to be taken; otherwise, later RSSI invocations will err.
    pub async fn rssi(
        &mut self,
        carrier: u16,
    ) -> Result<impl core::ops::Deref<Target = RssiEvent>, MixedError> {
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
                DectEvent::Rssi(res) => {
                    result = Some(res);
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

        result
            .ok_or(MixedError::UsageError)
    }

    pub async fn rssi_bulk(
        &mut self,
        // channel, start time to be populated, outputs
        carriers: &mut [(u16, u64, impl AsMut<[[u8; 240]]>)],
        // FIXME: return times
    ) -> Result<(), MixedError> {
        let now = self.time_get().await?;

        // Rather than doing nothing until we start, we could also start recording now for some
        // slices, and use the first incoming data to calculate the remaining points. However, that
        // requires the first slice to be long (minimum somewhere between 2 and 8, and I think
        // above 4), so let's start with the generalized approach.
        let slack = 69120 * 1; // 1ms should suffice, given that no data needs to be copied around.

        let mut current_start_time = now + slack;

        for (handle, (carrier, _starttime, destbuffers)) in carriers.iter_mut().enumerate() {
            // We don't need them mut here, but it's easier than asking AsRef too.
            let destbuffers = destbuffers.as_mut();
            let params = nrfxlib_sys::nrf_modem_dect_phy_rssi_params {
                start_time: current_start_time,
                handle: handle.try_into().unwrap(),
                carrier: *carrier,
                duration: (48 * destbuffers.len()).try_into().map_err(|_| MixedError::UsageError)?, // in half slots
                reporting_interval: nrfxlib_sys::nrf_modem_dect_phy_rssi_interval_NRF_MODEM_DECT_PHY_RSSI_INTERVAL_24_SLOTS, // 24 slots = 10ms, because we request in that granularity anyway
            };
            unsafe { nrfxlib_sys::nrf_modem_dect_phy_rssi(&raw const params) }.into_result()?;
            let duration = u64::try_from(destbuffers.len()).unwrap() * 691_200 /* 10ms */;
            current_start_time = current_start_time.wrapping_add(duration);
            current_start_time = current_start_time.wrapping_add(
                u64::from(
                    super::latency::LATENCY_INFO
                        .operation
                        .receive
                        .active_to_idle_rx_rssi,
                ) - 138, // -138 works on this firmware (eg. when switching from 1665 to 1667), -139 does not (even when staying on 1665)
            );
        }

        for (handle, (_carrier, starttime, destbuffers)) in carriers.iter_mut().enumerate() {
            for (destbufferindex, destbuffer) in destbuffers.as_mut().iter_mut().enumerate() {
                match DECT_EVENTS.receive().await.event {
                    DectEvent::Rssi(Some(res)) => {
                        // Protect against AsMut impls that are not actually constant length.
                        assert_eq!(u32::try_from(handle).unwrap(), res.handle);

                        defmt::debug!("Got some RSSI at {}", res.start_time());
                        if destbufferindex == 0 {
                            *starttime = res.start_time();
                        }
                        destbuffer.copy_from_slice(res.data());
                    }
                    DectEvent::Rssi(None) => {
                        panic!("Async was not polled fast enough, could not empty RSSI buffers")
                    }
                    e => panic!("Sequence violation: {e:?}"),
                }
            }
            match DECT_EVENTS.receive().await.event {
                DectEvent::Completed(Ok(())) => (),
                DectEvent::Completed(e) => {
                    defmt::info!("Completed with error {:?}", e);
                    e?
                }
                _ => panic!("Sequence violation"),
            };
        }

        Ok(())
    }
}
