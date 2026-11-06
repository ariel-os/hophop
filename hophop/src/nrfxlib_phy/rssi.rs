// SPDX-FileCopyrightText: Copyright Christian Amsüss <chrysn@fsfe.org>, Silano Systems
// SPDX-License-Identifier: MIT OR Apache-2.0

use heapless::{
    box_pool,
    pool::boxed::{Box, BoxBlock},
};
use itertools::Itertools as _;
use nrf_modem::ErrorSource;
use nrfxlib_sys_pretty::dect_phy;
use static_cell::StaticCell;

use ts_103_636_utils::identifiers::{AbsoluteChannel, NetworkId32};

use super::{DECT_EVENTS, DectEvent, DectPhy, Handle, MixedError};

// Storage for RSSI results.
//
// These are passed to the caller owned; consequently, reception is fallible if the user does not
// drop their items in time.
//
// This can probably be optimized, eg. by being an atomic ring buffer with the sender living in the
// IRQ and the receiver in the consuming task.
box_pool!(RssiPool: RssiEvent);

const RSSI_POOL_SIZE: usize = 16;

/// Initiates the RSSI pool.
#[inline]
pub(super) fn init() {
    static RSSI_BUFFER: StaticCell<[BoxBlock<RssiEvent>; RSSI_POOL_SIZE]> = StaticCell::new();
    for b in RSSI_BUFFER.init_with(|| core::array::from_fn(|_| BoxBlock::new())) {
        RssiPool.manage(b);
    }
}

/// Resulting data slice of a single RSSI measurement.
#[derive(Debug)]
pub struct RssiEvent {
    start_time: u64,
    carrier: AbsoluteChannel,
    data: [u8; 240],
}

impl RssiEvent {
    pub fn start_time(&self) -> u64 {
        self.start_time
    }

    /// The channel on which the measurement was taken.
    pub fn carrier(&self) -> AbsoluteChannel {
        self.carrier
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
pub(super) unsafe fn event(rssi: *const dect_phy::rssi_event) -> DectEvent {
    // SAFETY: Checked the discriminator
    let rssi = unsafe { &*rssi };
    // SAFETY: It is valid now, which is as long as we use it
    // Casting because it's not precisely a signed integer anyuway (and our buffer is just
    // bytes).
    let meas = unsafe { core::slice::from_raw_parts(rssi.meas as *const u8, rssi.meas_len as _) };
    let carrier =
        AbsoluteChannel::new(rssi.carrier).expect("Modem measures on channels it was asked for");
    let handle = Handle(rssi.handle);
    defmt::trace!(
        "RSSI handle {} start {} carrier {}; {} measurements",
        handle,
        rssi.meas_start_time,
        carrier,
        meas.len(),
    );

    let mut data = [0; _];
    data[..meas.len()].copy_from_slice(meas);

    let owned = RssiPool
        .alloc(RssiEvent {
            start_time: rssi.meas_start_time,
            carrier,
            data,
        })
        .ok();

    DectEvent::Rssi(handle, owned)
}

impl DectPhy {
    /// Read a single RSSI series.
    ///
    /// The resulting data comes in an owned buffer. It is up to the caller to drop that in time
    /// for later RSSI measurements to be taken; otherwise, later RSSI invocations will err.
    pub async fn rssi(
        &mut self,
        carrier: AbsoluteChannel,
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

        let configured_handle = Handle(1234);
        let params = dect_phy::rssi_params {
            start_time: 0,
            handle: configured_handle.0,
            carrier: carrier.into(),
            duration: 48, // in subslots; 1 full report
            reporting_interval: dect_phy::rssi_interval_values::_24_SLOTS, // 24 slots = 10ms
        };
        unsafe { dect_phy::rssi(&raw const params) }.into_result()?;

        let mut result = None;

        loop {
            match DECT_EVENTS.receive().await.event {
                DectEvent::Rssi(handle, res) => {
                    assert_eq!(handle, configured_handle);
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

        result.ok_or(MixedError::UsageError)
    }

    /// Reads multiple RSSI slots in sequence.
    ///
    /// If a carrier shows up multiple times, the readings are taken in a single go; this interface
    /// is chosen as it enables pasing in a generic const value that can then be used to return the
    /// result values.
    ///
    /// The on_event function may keep or spool the `RssiEvent` events (eg. put them into a
    /// queue), but needs to drop them reasonably quickly so that the pool does not run out;
    /// otherwise it may receive None events that indicate the overflow (but are still emitted so
    /// that the receiver knows which events were empty).
    ///
    /// If an operation fails (eg. because it could not be started in time), the others still run.
    /// The failed one does not produce an `on_event` call, and the first error is returned once all
    /// are done. [`RssiEvent::carrier()`] tells which channel a measurement belongs to.
    ///
    /// See [`Self::rx()`] for the relevance of `network_id`.
    pub async fn rssi_bulk(
        &mut self,
        carriers: &[AbsoluteChannel],
        network_id: Option<NetworkId32>,
        mut on_event: impl AsyncFnMut(Option<Box<RssiPool>>),
        mut on_receive: impl AsyncFnMut(super::rx::RecvResult),
    ) -> Result<(), MixedError> {
        let now = self.time_get().await?;

        // Rather than doing nothing until we start, we could also start recording now for some
        // slices, and use the first incoming data to calculate the remaining points. However, that
        // requires the first slice to be long (minimum somewhere between 2 and 8, and I think
        // above 4), so let's start with the generalized approach.
        let slack_ms = 1; // 1ms should suffice, given that no data needs to be copied around.

        let mut current_start_time = now + slack_ms * super::TICKS_PER_MILLISECOND;

        fn handle_from_index(handle: usize) -> Handle {
            handle.try_into().ok().map(Handle).expect(
                "Carriers can't be so many this doesn't fit in the unmanaged portio of handles",
            )
        }

        for (handle, (multiples, carrier)) in carriers.iter().dedup_with_count().enumerate() {
            let handle = handle_from_index(handle);
            // We don't need them mut here, but it's easier than asking AsRef too.
            let params = dect_phy::rx_params {
                start_time: current_start_time,
                handle: handle.0,
                carrier: (*carrier).into(),
                // duration: (48 * destbuffers.len()).try_into().map_err(|_| MixedError::UsageError)?, // in half slots
                duration: u32::try_from(multiples).unwrap() * 691_200, // in time units
                rssi_interval: dect_phy::rssi_interval_values::_24_SLOTS, // 24 slots = 10ms, because we request in that granularity anyway
                // from here on it's RX params
                filter: dect_phy::rx_filter {
                    short_network_id: 0,
                    is_short_network_id_used: 0,
                    receiver_identity: 0,
                },
                mode: dect_phy::rx_mode_values::CONTINUOUS,
                link_id: dect_phy::link_id {
                    short_network_id: 0,
                    short_rd_id: 0,
                },
                network_id: network_id.map(u32::from).unwrap_or_default(),
                rssi_level: 0,
            };
            unsafe { dect_phy::rx(&raw const params) }.into_result()?;
            let duration = u64::try_from(multiples).unwrap() * 691_200 /* 10ms */;
            current_start_time = current_start_time.wrapping_add(duration);

            // Nordic: https://github.com/nrfconnect/sdk-nrfxlib/blob/aa61ca35788bcdea3b27b3a70e788dbe666d63e5/nrf_modem/include/nrf_modem_dect_phy.h#L930
            // Min scheduled operation start time = previous scheduled operation start time + its
            // duration + the scheduled operation transition time for the current radio mode.
            current_start_time = current_start_time.wrapping_add(u64::from(
                super::latency::LATENCY_INFO.radio_mode[super::RADIO_MODE as usize]
                    .scheduled_operation_transition,
            ));
        }

        // Every operation ends in exactly one completion → also one that fails (eg. could not be started in time). 
        // And reports no RSSI. DectEvent::Completed does not carry the handle, 
        // so completions are counted. Rather than returning at the first error,
        // this waits for all of them: otherwise, the remaining operations events would reach the
        // next user of the modem.
        let operations = carriers.iter().dedup_with_count().count();
        let mut completed = 0;
        let mut first_error = None;
        let mut recv_result = super::rx::RecvResultBuilder::default();
        while completed < operations {
            match DECT_EVENTS.receive().await.event {
                DectEvent::Rssi(received_handle, res) => {
                    assert!(
                        (received_handle.0 as usize) < operations,
                        "Sequence violation"
                    );

                    if let Some(res) = &res {
                        defmt::debug!("Got some RSSI at {}", res.start_time());
                    } else {
                        defmt::warn!("Got RSSI but data could not be allocate");
                    }
                    on_event(res).await;
                }
                e @ (DectEvent::Pcc(..)
                | DectEvent::Pdc(..)
                | DectEvent::PccError(..)
                | DectEvent::PdcError) => {
                    // Both the Break and the Error case only occur when feeding a Completed,
                    // which we don't do.
                    let _ = recv_result.feed(e)?;
                    if recv_result.is_ready() {
                        let result = core::mem::take(&mut recv_result);
                        let result = result.finish().expect("Done states have Some content");
                        on_receive(result).await;
                    }
                }
                DectEvent::Completed(completion) => {
                    completed += 1;

                    // a packet whose PDC did not arrive by the end of the operation will not get one
                    // any more.
                    // this assumes that no other operation completes between a packet's
                    // PCC and PDC → telling them apart needs the handles, which DectEvent drops
                    recv_result = Default::default();
                    if let Err(e) = completion {
                        defmt::warn!("RSSI operation failed: {}", e);
                        // only keep the first error, but still wait for all operations to complete
                        first_error.get_or_insert(e); 
                    }
                }
                _ => panic!("Sequence violation"),
            }
        }
        match first_error {
            Some(e) => Err(e.into()),
            None => Ok(()),
        }
    }
}
