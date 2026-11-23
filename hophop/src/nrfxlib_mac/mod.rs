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
    ) -> Result<AssociationEvent, MacError> {
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

        Ok(association_event)
    }

    pub async fn next_association_end(&mut self) -> AssociationEndEvent {
        // Not optimizing into a fn -> impl future: The compiler is already learning to do this
        // itself.
        RELEASE_EVENTS.receive().await
    }

    /// Asks the MAC to end the association with a given peer.
    ///
    /// This returns immediately; the effect is that [`Self::next_association_end()`] .
    ///
    /// # Caveats
    ///
    /// It is unclear how this acts in race conditions between associations being terminated by the
    /// peer or by beacon timeouts and this function; possible outcomes that the application should
    /// handle are receiving an error here (in which case it is plausible to expect that
    /// [`Self::next_association_end()`] will produce something), and that two release events (one
    /// from the other source and one from this function) occur, or just one event.
    ///
    /// The working assumption used in the rest of the crate is that up to two events are produced,
    /// but the `OurRelease` variant comes last.
    pub fn initiate_release_association(
        &mut self,
        peer: LongRdId,
        cause: dect_mac::release_cause,
    ) -> Result<(), nrf_modem::Error> {
        unsafe {
            dect_mac::association_release(&mut dect_mac::association_release_params {
                release_cause: cause,
                long_rd_id: peer.into(),
            })
        }
        .into_result()
    }

    /// Starts an RSSI scan.
    ///
    /// FIXME:
    /// - Results are currently not processed but merely shown in debug output from the interrupt
    ///   (they arrive in a _ntfy event).
    pub async fn rssi_scan(
        &mut self,
        channel_scan_length_ms: u8,
        threshold_min_dbm: i8,
        threshold_max_dbm: i8,
        // FIXME: … or allow scanning a full band
        channels: &[AbsoluteChannel],
    ) -> Result<(), MacError> {
        let mut channels_array = [0; _];
        if channels.len() > channels_array.len() {
            return error::MacErrorExt::into_mac_status(
                nrfxlib_sys_pretty::dect_mac::err_values::NO_MEMORY,
            );
        };
        channels_array
            .iter_mut()
            .zip(channels)
            .for_each(|(output, input)| *output = (*input).into());

        unsafe {
            dect_mac::rssi_scan(&mut dect_mac::rssi_scan_params {
                channel_scan_length: channel_scan_length_ms,
                threshold_min: threshold_min_dbm,
                threshold_max: threshold_max_dbm,
                band: 0,
                num_channels: channels.len().try_into().expect(""),
                channel_list: channels_array,
            })
        }
        .into_result()
        .expect("we sanitized all sanitizable parameters");
        self.receive_singleton_event(SingletonEventSource::RssiScan)
            .await
    }

    /// Starts operation as an FT.
    ///
    /// FIXME: All parameters are currently hard-coded.
    pub async fn cluster_configure(
        &mut self,
        channel: AbsoluteChannel,
        our_network: Ipv6Config,
    ) -> Result<(), MacError> {
        // FIXME all parameters
        let mut association_config = dect_mac::association_config {
        // as used in samples/dect/dect_shell/prj.conf, for example
        max_num_neighbours: 50,
        // documented as not supported
        max_num_ft_neighbours: 0,
        neighbor_info_triggers:
            dect_mac::association_config_neighbor_info_triggers {
                inactivity_timer: 60_000, // 60s
            },
        default_tx_flow_config:
            // AIU those are for the 2 signalling flows and the 4 user flows
            [
            dect_mac::association_config__bindgen_ty_2 {
                priority: 0,
                dlc_service_type: dect_dlc::service_type_values::_3,
                dlc_sdu_lifetime: dect_dlc::sdu_lifetime_values::_8_S,
            },
            dect_mac::association_config__bindgen_ty_2 {
                priority: 0,
                dlc_service_type: dect_dlc::service_type_values::_3,
                dlc_sdu_lifetime: dect_dlc::sdu_lifetime_values::_8_S,
            },
            dect_mac::association_config__bindgen_ty_2 {
                priority: 3,
                dlc_service_type: dect_dlc::service_type_values::_3,
                dlc_sdu_lifetime: dect_dlc::sdu_lifetime_values::_8_S,
            },
            dect_mac::association_config__bindgen_ty_2 {
                priority: 4,
                dlc_service_type: dect_dlc::service_type_values::_3,
                dlc_sdu_lifetime: dect_dlc::sdu_lifetime_values::_8_S,
            },
            dect_mac::association_config__bindgen_ty_2 {
                priority: 5,
                dlc_service_type: dect_dlc::service_type_values::_3,
                dlc_sdu_lifetime: dect_dlc::sdu_lifetime_values::_8_S,
            },
            dect_mac::association_config__bindgen_ty_2 {
                priority: 6,
                dlc_service_type: dect_dlc::service_type_values::_3,
                dlc_sdu_lifetime: dect_dlc::sdu_lifetime_values::_8_S,
            },
            ],
    };

        // TX power and RACH seem to be mandatory
        let mut config_flags = dect_mac::cluster_config_flags {
            _bitfield_1: nrfxlib_sys::__BindgenBitfieldUnit::new([0]),
            _bitfield_align_1: [0; 0],
        };
        config_flags.set_has_max_tx_power(1);
        config_flags.set_has_rach_config(1);

        // FIXME all parameters
        let mut cluster_config = dect_mac::cluster_config {
        // flags
        __bindgen_anon_1: dect_mac::cluster_config___bindgen_anon_1 { flags: config_flags },
        // don't be fooled: that's a nibble value as output by dect_utils_lib_dbm_to_phy_tx_power
        // (undocumented, but clear from dect_shell code) -- just so happens that the value 10
        // (0x0a) happens to also encode for 10dBm
        beacon_tx_power: 10,
        // not sure, but there are limited sets of allowed values
        count_to_trigger: dect_mac::count_to_trigger_values::_2,
        relative_quality: 0,
        min_quality: 0,
        // see beacon_tx_power
        cluster_max_tx_power: 10,
        cluster_beacon_period: dect_mac::cluster_beacon_period_values::_500_MS,
        cluster_channel: channel.into(),
        network_id: 0x87654321,
        rach_configuration: dect_mac::cluster_config_nrf_modem_dect_mac_cluster_rach_config {
            // probably disregarded due to bit mask
            policy: dect_mac::rach_config_policy_values::FILL,
            common: dect_mac::cluster_config_nrf_modem_dect_mac_cluster_rach_config_nrf_modem_dect_mac_rach_common_config {
                response_window_length: 8,
                max_transmission_length: 8,
                cw_min_sig: 2,
                cw_max_sig: 7,
            },
            config: dect_mac::cluster_config_nrf_modem_dect_mac_cluster_rach_config_nrf_modem_dect_mac_rach_config {
                fill:  dect_mac::cluster_config_nrf_modem_dect_mac_cluster_rach_config_nrf_modem_dect_mac_rach_config_nrf_modem_dect_mac_rach_config_fill {
                    percentage: 100,
                },
                // probably not used because policy is set to FILL
                periodic: dect_mac::cluster_config_nrf_modem_dect_mac_cluster_rach_config_nrf_modem_dect_mac_rach_config_nrf_modem_dect_mac_rach_config_periodic {
                    period: 0,
                    length: 0,
                },
            }
        },
        triggers: dect_mac::cluster_config_cluster_info_triggers {
            busy_threshold: 20,
        },
        ipv6_config: (&our_network).into(),
    };

        let mut params = dect_mac::cluster_configure_params {
            // FIXME all parameters

            // as used anywhere in v3.4.0 code (but probably should be influenced by the RSSI
            // bit field)
            cluster_period_start_offset: 0,
            association_config: &mut association_config,
            cluster_config: &mut cluster_config,
        };
        unsafe { dect_mac::cluster_configure(&mut params) }
            .into_result()
            .expect("library sets up presumably valid configuration");
        self.receive_singleton_event(SingletonEventSource::ClusterConfigure)
            .await
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
    ClusterConfigure,
    Configure,
    FunctionalMode,
    NetworkScan,
    NetworkScanStop,
    RssiScan,
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

#[derive(defmt::Format, PartialEq, Eq)]
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

/// Selectable options for NI6W addressing.
///
/// The Full mode is not supported well because it is unclear how it would be used.
#[derive(defmt::Format, Debug, Copy, Clone, minicbor::Encode)]
// FIXME: Why can't this be cbor(transparent)?
#[cbor(flat)]
pub enum Ipv6Config {
    #[n(2)]
    Full(#[n(0)] [u8; 16]),
    #[n(1)]
    Prefix(#[n(0)] [u8; 8]),
    #[n(0)]
    LinkLocal,
}

impl Ipv6Config {
    /// Given a sink RD ID and a destination IP address, determine if the address is on the link
    /// and has a usable destination RD ID, which is returned in that case.
    pub fn matches_and_extract(
        &self,
        sink: &LongRdId,
        address: &core::net::Ipv6Addr,
    ) -> Option<LongRdId> {
        let expected_prefix = match self {
            Ipv6Config::LinkLocal => &[0xfe, 0x80, 0, 0, 0, 0, 0, 0],
            Ipv6Config::Prefix(p) => p,
            // FIXME: This is where it's unclear.
            Ipv6Config::Full(_addr) => return None,
        };
        if &address.octets()[..8] != expected_prefix {
            return None;
        }
        if address.octets()[8..12] != u32::from(*sink).to_be_bytes() {
            return None;
        }

        LongRdId::new(u32::from_be_bytes(
            address.octets()[12..].try_into().unwrap(),
        ))
    }

    pub fn address_of(&self, sink: LongRdId, mac: LongRdId) -> core::net::Ipv6Addr {
        let mut address = [0; 16];
        address[8..12].copy_from_slice(&u32::from(sink).to_be_bytes());
        address[12..].copy_from_slice(&u32::from(mac).to_be_bytes());

        use crate::nrfxlib_mac::Ipv6Config;
        match self {
            Ipv6Config::Full(full) => address = *full,
            Ipv6Config::Prefix(prefix) => address[..8].copy_from_slice(prefix),
            Ipv6Config::LinkLocal => address[..2].copy_from_slice(&[0xfe, 0x80]),
        }

        address.into()
    }
}

impl From<&Ipv6Config> for dect_mac::ipv6_address_config_t {
    fn from(input: &Ipv6Config) -> Self {
        dect_mac::ipv6_address_config_t {
            type_: match input {
                Ipv6Config::LinkLocal => dect_mac::ipv6_address_type_values::NONE,
                Ipv6Config::Prefix(_) => dect_mac::ipv6_address_type_values::PREFIX,
                Ipv6Config::Full(_) => dect_mac::ipv6_address_type_values::FULL,
            },
            address: match input {
                Ipv6Config::LinkLocal => [0; _],
                Ipv6Config::Prefix(prefix) => {
                    let mut full = [0; _];
                    full[..8].copy_from_slice(prefix);
                    full
                }
                Ipv6Config::Full(a) => *a,
            },
        }
    }
}
