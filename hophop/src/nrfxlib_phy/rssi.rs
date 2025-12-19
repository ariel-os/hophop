// SPDX-FileCopyrightText: Copyright Christian Amsüss <chrysn@fsfe.org>, Silano Systems
// SPDX-License-Identifier: MIT OR Apache-2.0

use embassy_sync::{
    blocking_mutex::raw::CriticalSectionRawMutex,
    mutex::{Mutex, MutexGuard},
};
use nrf_modem::{ErrorSource, nrfxlib_sys};

use super::{DECT_EVENTS, DectEvent, DectPhy, MixedError};

/// Kind of a bump allocator for RSSI data, as that doesn't fit in small events.
///
/// Might later be turned into a ring buffer if any methods support stream-processing multiple
/// events.
///
/// Sized 2400 somewhat arbitrarily because it could take 10 runs of RSSI data.
static RECVBUF: Mutex<CriticalSectionRawMutex, heapless::Vec<u8, 2400>> =
    Mutex::new(heapless::Vec::new());

/// Resulting data slice of a single RSSI measurement.
///
/// This keeps a lock on the receive buffer, and must therefore be dropped before the next attempt
/// to perform any other operation.
pub struct RssiResult<'a>(
    MutexGuard<'static, CriticalSectionRawMutex, heapless::Vec<u8, 2400>>,
    core::ops::Range<usize>,
    // This ensures that a result is used before the next attempt to receive something (as
    // that would panic around locking RECV_BUF).
    core::marker::PhantomData<&'a mut ()>,
);

impl RssiResult<'_> {
    pub fn data(&self) -> &[u8] {
        &self.0[self.1.clone()]
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

    if let Ok(mut recvbuf) = RECVBUF.try_lock() {
        let start = recvbuf.len();
        if recvbuf.extend_from_slice(meas).is_ok() {
            DectEvent::Rssi(rssi.meas_start_time, Some(start..(start + meas.len())))
        } else {
            DectEvent::Rssi(rssi.meas_start_time, None)
        }
    } else {
        DectEvent::Rssi(rssi.meas_start_time, None)
    }
}

fn clear_recvbuf() {
    let mut recvbuf = RECVBUF
        .try_lock()
        .expect("Buffer in use; unsafe construction of DectPhy, or pending future was dropped.");
    recvbuf.clear();
    drop(recvbuf);
}

impl DectPhy {
    pub async fn rssi(&mut self, carrier: u16) -> Result<(u64, RssiResult<'_>), MixedError> {
        clear_recvbuf();

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

        Ok((
            result.0,
            RssiResult(
                RECVBUF
                    .try_lock()
                    .expect("Was checked before, and ISR users release this before returning"),
                result.1,
                core::marker::PhantomData,
            ),
        ))
    }
}
