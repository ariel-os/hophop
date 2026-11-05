// SPDX-FileCopyrightText: Copyright Christian Amsüss <chrysn@fsfe.org>, Silano Systems
// SPDX-License-Identifier: MIT OR Apache-2.0
//! High-level wrappers around the Nordic's DECT MAC.

pub mod error;

mod callbacks;
mod debug_helpers;
mod shared_queues;

use nrf_modem::ErrorSource;
use nrfxlib_sys_pretty::{dect_control, dect_dlc, dect_mac};

use ts_103_636_utils::identifiers::{AbsoluteChannel, LongRdId, NetworkId32, ShortRdId};

use error::MacError;
use shared_queues::*;

/// Singleton represnting control over the DECT operation of libmodem and that the callbacks are
/// set up.
///
/// This is built to be a good async representation of the nrfxlib API, and should not include any
/// operations logic.
///
/// On the long run, while the design will stay nrfxlib specific in its API calls, all uses of
/// `nrfxlib-sys` should be removed.
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
/// As long as there is no state that needs to be held at runtime, this is a ZST, and exclusive
/// access to it ensures that the static queues set up in the private `shared_queues` module
/// (except for those split off at startup) are not accessed simultaneously (which would make them
/// unreliable in that one party might expect an event that was taken by another party, or
/// simultaneous polling might result in a busy wait).
pub struct DectMac(());

pub struct DlcTx(());

pub struct DlcRx(());

impl DectMac {
    /// See hophop::nrfxlib_phy::DectPhy::init_after_modem_init() for `_modem_is_set_up` context
    ///
    /// This immediately starts setting up the MAC system mode. If we later find out we don't want
    /// that, there'll probably be some kind of "desiccated" form of this object where a user can
    /// first initialize the desiccated form and then do a "set MAC system mode" function, both
    /// which steps this combines.
    pub fn create(_modem_is_set_up: ()) -> (Self, DlcTx, DlcRx) {
        // SAFETY: used as per C API (and it'd even check for NULLs here and inside)
        unsafe { dect_mac::callback_set(&callbacks::OP_CALLBACKS, &callbacks::MAC_NTF_CALLBACKS) }
            .into_result()
            .expect(
                "Unsuccessful operation would mean we passed NULL or had a None in the callbacks.",
            );

        (Self(()), DlcTx(()), DlcRx(()))
    }

    /// Waits for an event on the SINGLETON channel, and reports its success.
    ///
    /// To detect misunderstanding of the authors' about the callback machinery, this takes an
    /// expectation of the event that triggered the singleton event; if need be, that could be
    /// turned into a debug_assert, and with corresponding changes in the `SingletonEvent` type to
    /// not use actual memory for the variants.
    async fn receive_singleton_event(
        &mut self,
        source: SingletonEventSource,
    ) -> Result<(), MacError> {
        // Not micro-optimizing into non-async -> impl Future -- for readability, and because this
        // is about to be fixed in https://github.com/rust-lang/goals/issues/623
        let event = SINGLETON_EVENTS.receive().await;
        assert_eq!(event.source, source);
        event.result
    }

    pub async fn systemmode_set_mac(&mut self) {
        unsafe { dect_control::systemmode_set(dect_control::systemmode_values::MAC) }
            .into_result()
            .expect("Failed to set system mode");
        self.receive_singleton_event(SingletonEventSource::SystemMode)
            .await
            .expect("Failed to set system mode");
    }

    // &mut is probably not needed. FIXME: ask if C API can be enhanced
    pub async fn control_configure(&mut self, params: &mut dect_control::configure_params) {
        unsafe { dect_control::configure(params) }
            .into_result()
            .expect("Failed to set configuration params");
        self.receive_singleton_event(SingletonEventSource::Configure)
            .await
            .expect("Failed to set configuration params");
    }

    async fn control_functional_mode_set(&mut self, mode: u8) {
        unsafe { dect_control::functional_mode_set(mode) }
            .into_result()
            .expect("Failed to set functional mode");
        self.receive_singleton_event(SingletonEventSource::FunctionalMode)
            .await
            .expect("Failed to set functional mode");
    }

    pub async fn control_functional_mode_set_deactivate(&mut self) {
        self.control_functional_mode_set(dect_control::functional_mode_values::DEACTIVATE)
            .await
    }

    pub async fn control_functional_mode_set_activate(&mut self) {
        self.control_functional_mode_set(dect_control::functional_mode_values::ACTIVATE)
            .await
    }

    /// Starts a network scan.
    ///
    /// Scan results are available inside the async closure. If the scan times out, the closure is
    /// dropped. If the closure completes, the scan is stopped before returning from this function.
    // &mut is probably not needed. FIXME: ask if C API can be enhanced
    pub async fn mac_network_scan<R>(
        &mut self,
        params: &mut dect_mac::network_scan_params,
        callback: impl AsyncFnOnce(ScanReceiver<'_>) -> R,
    ) -> Option<R> {
        unsafe { dect_mac::network_scan(params) }
            .into_result()
            .expect("Failed to start the scan");

        use embassy_futures::select::{Either::*, select};

        match select(
            self.receive_singleton_event(SingletonEventSource::NetworkScan),
            callback(ScanReceiver(Default::default())),
        )
        .await
        {
            First(e) => {
                e.unwrap();
                None
            }
            Second(r) => {
                unsafe { dect_mac::network_scan_stop() }
                    .into_result()
                    .expect("Failed to stop the scan");

                // One of those is the First that never happened / was cancelled -- so we don't
                // really care about their sequence or success
                let ev1 = SINGLETON_EVENTS.receive().await;
                let ev2 = SINGLETON_EVENTS.receive().await;
                assert!(
                    (ev1.source == SingletonEventSource::NetworkScan
                        && ev2.source == SingletonEventSource::NetworkScanStop)
                        || (ev1.source == SingletonEventSource::NetworkScanStop
                            && ev2.source == SingletonEventSource::NetworkScan)
                );

                // FIXME: cancel and await two end events
                Some(r)
            }
        }
    }

    /// Starts association as a PT.
    ///
    /// The function completes successfully when an association is made; the association may be
    /// lost at any time; at that time, the returned future also completes.
    ///
    /// # Caveats
    ///
    /// The RelaseReceiver is not lifetimed, as it does not rule out other operations on `self`;
    /// nonetheless, it must not be polled (ideally: be dropped) as soon as a new association is
    /// started. (FiXME: Should `self` be split somehow?)
    // FIXME allow configuring flows
    pub async fn mac_association(
        &mut self,
        long_rd_id: LongRdId,
        network_id: NetworkId32,
    ) -> Result<
        (
            AssociationEvent,
            impl core::future::Future<Output = AssociationEndEvent> + use<>,
        ),
        MacError,
    > {
        let mut tx_flow_configs = [
            dect_mac::tx_flow_config {
                flow_id: 6, // "User plane data -- flow 4"
                priority: 4,
                dlc_service_type: dect_dlc::service_type_values::_3,
                dlc_sdu_lifetime: dect_dlc::sdu_lifetime_values::_8_S,
            },
            dect_mac::tx_flow_config {
                //flow_id: 0b11, // Table 6.3.4-2: IE type field encoding for MAC Extension field encoding 00, 01, 10 -- do they really want this for "User Data Plane -- flow 1"?
                flow_id: 1, // "Higher layer signalling - flow 1"
                priority: 0,
                dlc_service_type:
                    dect_dlc::service_type_values::_3,
                dlc_sdu_lifetime:
                    // picking something long, I think right now we can only TX right after a beacon
                    dect_dlc::sdu_lifetime_values::_8_S,
            },
        ];
        unsafe {
            dect_mac::association(&mut dect_mac::association_params {
                // FIXME where do we use the channel information? Did the MAC remember? (Probably:
                // After all, it's not just channel but the whole info stuff from the beacon).
                long_rd_id: long_rd_id.into(),
                network_id: network_id.into(),
                info_triggers: dect_mac::parent_info_triggers {
                    // This value is practically undocumented, but the dect_shell UI has notes
                    // on it:
                    //
                    // "Set maximum number of consecutive missed cluster beacons. Note: set to
                    // modem when creating association and has impact when FT device is
                    // considered as out of range or FT is turned off, resulting the automatic
                    // disassociation of the FT device. Value 0 means that no limit and no
                    // automatic disassociation is done."
                    //
                    // Observation is that no release event happens beyond the
                    // cluster_beacon_rx_failure_ntf, so it stands to reason that that event is
                    // to be treated just like a release event. (They do, however, get sent
                    // repreatedly).
                    num_beacon_rx_failures: 3,
                },
                num_flows: tx_flow_configs
                    .len()
                    .try_into()
                    .expect("Absurd number of configs"),
                tx_flow_configs: &mut tx_flow_configs as _,
            })
        }
        .into_result()
        .expect("Failed to start association attempt");

        let association_event = ASSOCIATION_EVENTS.receive().await?;

        Ok((association_event, RELEASE_EVENTS.receive()))
    }
}

impl DlcTx {
    /// Transmits data in one of the flows.
    ///
    /// This is a primitive wrapper in that it only returns after transmission; a better version
    /// would return once successfully enqueued and signal completion later.
    pub async fn dlc_data_tx(
        &mut self,
        flow_id: u8,
        destination: LongRdId,
        data: &[u8],
    ) -> Result<(), MacError> {
        unsafe {
            dect_dlc::data_tx(&mut dect_dlc::data_tx_params {
                // FIXME use when managing those; right now any value is good
                transaction_id: 0,
                flow_id,
                // send to our neighbor
                long_rd_id: destination.into(),
                // FIXME: verify that the C API really doesn't want to write there
                data: data as *const _ as *mut _,
                data_len: data.len(),
            })
        }
        .into_result()
        // FIXME: Can this fail just because the slice is bad?
        .expect("Failed to start TX attempt");

        let done = shared_queues::DLC_TX_DONE.receive().await;
        done.status
    }
}

impl DlcRx {
    pub async fn dlc_data_rx(&mut self) -> DlcDataRx {
        if let Some(d) = PACKETS.lock().await.pop_front() {
            return d;
        }
        loop {
            PACKET_READY.receive().await;
            if let Some(d) = PACKETS.lock().await.pop_front() {
                return d;
            }
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
enum SingletonEventSource {
    Configure,
    FunctionalMode,
    NetworkScan,
    NetworkScanStop,
    SystemMode,
}

/// An event that goes through the single big event queue because the application generally expects
/// one of them to return.
///
/// This carries the source practically needlessly, as the application already awaits the queue
/// from a place where it knows what comes in. The source is strictly debug information, that is
/// asserted on -- future versions could make the asserts debug_asserts, and the source field
/// optional.
#[derive(Debug)]
struct SingletonEvent {
    source: SingletonEventSource,
    result: Result<(), MacError>,
}

trait ResultMacErrorExt {
    fn originating_from(self, source: SingletonEventSource) -> SingletonEvent;
}

impl ResultMacErrorExt for Result<(), MacError> {
    fn originating_from(self, source: SingletonEventSource) -> SingletonEvent {
        SingletonEvent {
            source,
            result: self,
        }
    }
}

// Almost nrf_modem_dect_mac_cluster_beacon_ntf_cb_params, but dropping the ies -- not because we
// really want to (I'd rather memcpy than create a new struct), but because that pointer makes the
// whole type not Send, and it comes from the ISR. We wouldn't touch it, but are in no position to
// impl Send on it.
#[derive(defmt::Format)]
pub struct ClusterBeacon {
    pub channel: AbsoluteChannel,
    pub transmitter_short_rd_id: ShortRdId,
    pub transmitter_long_rd_id: LongRdId,
    pub network_id: NetworkId32,
}

#[derive(Copy, Clone)]
pub struct ScanReceiver<'brand>(core::marker::PhantomData<&'brand ()>);

impl<'brand> ScanReceiver<'brand> {
    // FIXME dress up in non _sys dependent API
    pub async fn next(self) -> ClusterBeacon {
        BEACON_EVENTS.receive().await
    }
}

#[derive(defmt::Format)]
struct DlcDataTxDone {
    pub transaction_id: u32,
    pub status: Result<(), MacError>,
    pub flow_id: u8,
    pub peer: LongRdId,
}

pub struct DlcDataRx {
    pub(crate) long_rd_id: u32,
    pub(crate) flow_id: u8,
    pub(crate) data: heapless::vec::Vec<u8, 1501>,
}

impl DlcDataRx {
    pub fn sender(&self) -> u32 {
        self.long_rd_id
    }

    pub fn flow_id(&self) -> u8 {
        self.flow_id
    }

    pub fn data(&self) -> &[u8] {
        self.data.as_slice()
    }
}

#[derive(defmt::Format)]
#[allow(
    clippy::enum_variant_names,
    reason = "the enum is about the abstract effect of a relase; some of them involve a concrete release message"
)]
pub(crate) enum Release {
    /// Received a Release notification from the peer.
    PeerRelease(ts_103_636_numbers::mac_details::release_message::ReleaseCause),
    /// Received a Beacon RX failure, assuming the association is released.
    BeaconRxFailure,
    /// Release due to the asynchronous response to an own [`dect_mac::assoc_release()`] call.
    OurRelease,
}

#[derive(defmt::Format)]
pub struct AssociationEndEvent {
    pub(crate) long_rd_id: LongRdId,
    pub(crate) release_cause: Release,
}

#[derive(defmt::Format)]
pub struct AssociationEvent {
    pub(crate) long_rd_id: LongRdId,
    pub(crate) ipv6_config: Ipv6Config,
    // Deliberately not in here: RSSI, MCS etc -- those might change, right?
}

#[derive(defmt::Format, Debug, Copy, Clone, minicbor::Encode)]
// FIXME: Why can't this be cbor(transparent)?
#[cbor(flat)]
pub enum Ipv6Config {
    #[n(2)]
    Full(#[n(0)] [u8; 16]),
    #[n(1)]
    Prefix(#[n(0)] [u8; 8]),
    #[n(0)]
    None,
}
