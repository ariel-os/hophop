pub mod error;

mod callbacks;
mod debug_helpers;
mod shared_queues;

use defmt::info;
use nrf_modem::{nrfxlib_sys, ErrorSource};

use error::MacError;
use shared_queues::*;

/// Public re-exports that should go away soon-ish and are only used for the duration of migrating
/// parts of the nordicmacdemo.
pub mod hack {
    // when removing, make pub(crate) again
    pub use super::shared_queues::PACKETS;
}

/// Singleton represnting control over the DECT operation of libmodem and that the callbacks are
/// set up.
///
/// This is built to be a good async representation of the nrfxlib API, and should not include any
/// operations logic.
///
/// ## Error handling
///
/// Error handling is rather on the panicky side: The caller is expected to adhere to the
/// underlying API's (currently implicit) requirements on sequences, such as not starting an
/// association while a scan is running. Only API violations (FIXME: currently: should) cause
/// panics, everything else is handled through errors.
///
/// ## Drop safety
///
/// None of this type's async methods are drop safe; dropping them gets the channels out of
/// sync, causing panics. FIXME: Find out a good alternative; options are:
///
/// * just document it,
/// * panic on drop,
/// * taint the object on drop (likely panicing other functions again),
/// * handle dropping for abortable functions such as dlc_data_tx,
/// * do that for others too, which for many means that every next user of self will wait
///   before its action is initialized. (In that case, the only reason to await completion after a
///   first poll, or if we move the action to before polling which we might do, is to get the error
///   out at the right place).
///
/// ## Further development
///
/// It is expected that as earlier with the PHY functions, this moves into hophop. But not now.
pub struct DectMac(());


impl DectMac {
    /// See hophop::nrfxlib_phy::DectPhy::init_after_modem_init() for `_modem_is_set_up` context
    ///
    /// This immediately starts setting up the MAC system mode. If we later find out we don't want
    /// that, there'll probably be some kind of "desiccated" form of this object where a user can
    /// first initialize the desiccated form and then do a "set MAC system mode" function, both
    /// which steps this combines.
    pub fn create(_modem_is_set_up: ()) -> Self {
        // SAFETY: used as per C API (and it'd even check for NULLs here and inside)
        unsafe {
            nrfxlib_sys::nrf_modem_dect_mac_callback_set(
                &callbacks::OP_CALLBACKS,
                &callbacks::MAC_NTF_CALLBACKS,
            )
        }
        .into_result()
        .expect("Unsuccessful operation would mean we passed NULL or had a None in the callbacks.");

        Self(())
    }

    pub async fn systemmode_set_mac(&mut self) {
        unsafe {
            nrfxlib_sys::nrf_modem_dect_control_systemmode_set(
                nrfxlib_sys::nrf_modem_dect_control_systemmode_NRF_MODEM_DECT_MODE_MAC,
            )
        }
        .into_result()
        .expect("Failed to set system mode");
        SINGLETON_EVENTS
            .receive()
            .await
            .expect("Failed to set system mode");
    }

    // &mut is probably not needed. FIXME: ask if C API can be enhanced
    pub async fn control_configure(
        &mut self,
        params: &mut nrfxlib_sys::nrf_modem_dect_control_configure_params,
    ) {
        unsafe { nrfxlib_sys::nrf_modem_dect_control_configure(params) }
            .into_result()
            .expect("Failed to set configuration params");
        SINGLETON_EVENTS
            .receive()
            .await
            .expect("Failed to set configuration params");
    }

    pub async fn control_functional_mode_set_activate(&mut self) {
        unsafe {
            nrfxlib_sys::nrf_modem_dect_control_functional_mode_set(
                nrfxlib_sys::nrf_modem_dect_control_functional_mode_NRF_MODEM_DECT_CONTROL_FUNCTIONAL_MODE_ACTIVATE
            )
        }.into_result().expect("Failed to set functional mode");
        SINGLETON_EVENTS
            .receive()
            .await
            .expect("Failed to set functional mode");
    }

    /// Starts a network scan.
    ///
    /// Scan results are available inside the async closure. If the scan times out, the closure is
    /// dropped. If the closure completes, the scan is stopped before returning from this function.
    // &mut is probably not needed. FIXME: ask if C API can be enhanced
    pub async fn mac_network_scan<R>(
        &mut self,
        params: &mut nrfxlib_sys::nrf_modem_dect_mac_network_scan_params,
        callback: impl AsyncFnOnce(ScanReceiver<'_>) -> R,
    ) -> Option<R> {
        unsafe { nrfxlib_sys::nrf_modem_dect_mac_network_scan(params) }
            .into_result()
            .expect("Failed to start the scan");

        use embassy_futures::select::{Either::*, select};

        match select(
            SINGLETON_EVENTS.receive(),
            callback(ScanReceiver(Default::default())),
        )
        .await
        {
            First(e) => {
                e.unwrap();
                None
            }
            Second(r) => {
                unsafe { nrfxlib_sys::nrf_modem_dect_mac_network_scan_stop() }
                    .into_result()
                    .expect("Failed to stop the scan");

                // One of those is the First that never happened / was cancelled.
                let _ = SINGLETON_EVENTS.receive().await;
                let _ = SINGLETON_EVENTS.receive().await;

                // FIXME: cancel and await two end events
                Some(r)
            }
        }
    }

    /// Transmits data in one of the flows.
    ///
    /// This is a primitive wrapper in that it only returns after transmission; a better version
    /// would return once successfully enqueued and signal completion later.
    pub async fn dlc_data_tx(
        &mut self,
        flow_id: u8,
        destination: u32,
        data: &[u8],
    ) -> Result<(), MacError> {
        unsafe {
            nrfxlib_sys::nrf_modem_dect_dlc_data_tx(
                &mut nrfxlib_sys::nrf_modem_dect_dlc_data_tx_params {
                    // FIXME use when managing those; right now any value is good
                    transaction_id: 0,
                    flow_id: flow_id,
                    // send to our neighbor
                    long_rd_id: destination,
                    // FIXME: verify that the C API really doesn't want to write there
                    data: data as *const _ as *mut _,
                    data_len: data.len(),
                },
            )
        }
        .into_result()
        // FIXME: Can this fail just because the slice is bad?
        .expect("Failed to start TX attempt");
        SINGLETON_EVENTS.receive().await
    }
}

// Almost nrf_modem_dect_mac_cluster_beacon_ntf_cb_params, but dropping the ies -- not because we
// really want to (I'd rather memcpy than create a new struct), but because that pointer makes the
// whole type not Send, and it comes from the ISR. We wouldn't touch it, but are in no position to
// impl Send on it.
pub struct ClusterBeacon {
    pub channel: u16,
    pub transmitter_short_rd_id: u16,
    pub transmitter_long_rd_id: u32,
    pub network_id: u32,
}

#[derive(Copy, Clone)]
pub struct ScanReceiver<'brand>(core::marker::PhantomData<&'brand ()>);

impl<'brand> ScanReceiver<'brand> {
    // FIXME dress up in non _sys dependent API
    pub async fn next(self) -> ClusterBeacon {
        BEACON_EVENTS.receive().await
    }
}
