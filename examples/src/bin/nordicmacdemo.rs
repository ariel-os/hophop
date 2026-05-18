// SPDX-FileCopyrightText: Copyright Christian Amsüss <chrysn@fsfe.org>, Silano Systems
// SPDX-License-Identifier: MIT OR Apache-2.0
//! Initial demo for Rust using the high-level (MAC etc) nrfxlib features

#![no_std]
#![no_main]

use ariel_os::debug::log::{Hex, error, info, warn};

use nrf_modem::ErrorSource;
use nrfxlib_sys;

mod err {
    use super::*;

    pub fn display(err: nrfxlib_sys::nrf_modem_dect_mac_err) -> &'static str {
        // Obivous FIXME to do this better
        //
        // (Probably through an extension trait, even though it's just a type alias so in theory
        // available on every u8, but we just won't use the trait widely)
        match err {
            nrfxlib_sys::nrf_modem_dect_mac_err_NRF_MODEM_DECT_MAC_STATUS_OK => "OK",
            nrfxlib_sys::nrf_modem_dect_mac_err_NRF_MODEM_DECT_MAC_STATUS_FAIL => "FAIL",
            nrfxlib_sys::nrf_modem_dect_mac_err_NRF_MODEM_DECT_MAC_STATUS_INVALID_PARAM => "INVALID_PARAM",
            nrfxlib_sys::nrf_modem_dect_mac_err_NRF_MODEM_DECT_MAC_STATUS_NOT_ALLOWED => "NOT_ALLOWED",
            nrfxlib_sys::nrf_modem_dect_mac_err_NRF_MODEM_DECT_MAC_STATUS_NO_CONFIG => "NO_CONFIG",
            nrfxlib_sys::nrf_modem_dect_mac_err_NRF_MODEM_DECT_MAC_STATUS_RD_NOT_FOUND => "RD_NOT_FOUND",
            nrfxlib_sys::nrf_modem_dect_mac_err_NRF_MODEM_DECT_MAC_STATUS_TEMP_FAILURE => "TEMP_FAILURE",
            nrfxlib_sys::nrf_modem_dect_mac_err_NRF_MODEM_DECT_MAC_STATUS_NO_RESOURCES => "NO_RESOURCES",
            nrfxlib_sys::nrf_modem_dect_mac_err_NRF_MODEM_DECT_MAC_STATUS_NO_RESPONSE => "NO_RESPONSE",
            nrfxlib_sys::nrf_modem_dect_mac_err_NRF_MODEM_DECT_MAC_STATUS_NW_REJECT => "NW_REJECT",
            nrfxlib_sys::nrf_modem_dect_mac_err_NRF_MODEM_DECT_MAC_STATUS_NO_MEMORY => "NO_MEMORY",
            nrfxlib_sys::nrf_modem_dect_mac_err_NRF_MODEM_DECT_MAC_STATUS_NO_RSSI_RESULTS => "NO_RSSI_RESULTS",
            nrfxlib_sys::nrf_modem_dect_mac_err_NRF_MODEM_DECT_MAC_STATUS_DLC_DISCARD_TIMER_EXPIRED => {
                "DLC_DISCARD_TIMER_EXPIRED"
            }
            _ => "(unknown error)",
        }
    }
}

fn debug_ies(ies: &[nrfxlib_sys::nrf_modem_dect_mac_ie]) {
    // This'd be easier if we just got the slice of the data
    for ie in ies {
        // I'm not fully sold that this is accurate.
        info!("IE: ie_type {}", ie.ie_type);
        #[cfg(false)]
        info!(
            "data dump: {}",
            Hex(unsafe {
                core::slice::from_raw_parts(
                    &ie.ie as *const _ as *const u8,
                    core::mem::size_of::<nrfxlib_sys::nrf_modem_dect_mac_ie__bindgen_ty_1>(),
                )
            })
        );
        match ie.ie_type {
            nrfxlib_sys::nrf_modem_dect_mac_ie_type_NRF_MODEM_DECT_MAC_IE_TYPE_RD_CAPABILITY => {
                let details = unsafe { &ie.ie.rd_capability };
                info!("  RD Capability: group_assign {} paging {} mesh {} scheduled {} dect_delay {} half_dup {} release {} operating_mode {} mac_security {} dlc {}",
                    details.group_assign_supported,
                    details.paging_supported,
                    details.mesh_supported,
                    details.scheduled_access_supported,
                    details.dect_delay_supported,
                    details.half_dup_supported,
                    details.release,
                    details.operating_mode,
                    details.mac_security,
                    details.dlc_service_type,
                );
                info!("    phy cap: power {} max_nss {} ... max_mcs {} ...",
                    details.phy_capabilities.power_class,
                    details.phy_capabilities.max_nss,
                    details.phy_capabilities.max_mcs,
                );
            },
            nrfxlib_sys::nrf_modem_dect_mac_ie_type_NRF_MODEM_DECT_MAC_IE_TYPE_RANDOM_ACCESS_RESOURCE => {
                let details = unsafe { &ie.ie.random_access_resource };
                info!("  Random Access Resource: channel {}, response_channel {}, max_tx {}, repetition {}",
                    unsafe { (&raw const details.channel).read_unaligned() },
                    unsafe { (&raw const details.response_channel).read_unaligned() },
                    details.max_rach_tx_length,
                    details.repetition,
                );
                info!("    allocation start {} slots {} length {}",
                    details.allocation.start_subslot,
                    details.allocation.use_slots,
                    unsafe { (&raw const details.allocation.length).read_unaligned() },
                );
            },
            _ => (),
        }
    }
}

mod callbacks {
    #![allow(
        unused_variables,
        reason = "stop complaining while there are todo!() items"
    )]

    use super::*;
    pub(super) static OP_CALLBACKS: nrfxlib_sys::nrf_modem_dect_mac_op_callbacks =
        nrfxlib_sys::nrf_modem_dect_mac_op_callbacks {
            control_functional_mode: Some(control_functional_mode),
            control_configure: Some(control_configure),
            control_systemmode: Some(control_systemmode),
            association: Some(association),
            association_release: Some(association_release),
            cluster_beacon_receive: Some(cluster_beacon_receive),
            cluster_beacon_receive_stop: Some(cluster_beacon_receive_stop),
            cluster_configure: Some(cluster_configure),
            cluster_info: Some(cluster_info),
            neighbor_info: Some(neighbor_info),
            neighbor_list: Some(neighbor_list),
            dlc_data_tx: Some(dlc_data_tx),
            dlc_data_discard: Some(dlc_data_discard),
            network_beacon_configure: Some(network_beacon_configure),
            network_scan: Some(network_scan),
            network_scan_stop: Some(network_scan_stop),
            rssi_scan: Some(rssi_scan),
            rssi_scan_stop: Some(rssi_scan_stop),
        };

    pub(super) static MAC_NTF_CALLBACKS: nrfxlib_sys::nrf_modem_dect_mac_ntf_callbacks =
        nrfxlib_sys::nrf_modem_dect_mac_ntf_callbacks {
            association_ntf: Some(association_ntf),
            association_release_ntf: Some(association_release_ntf),
            cluster_ch_load_change_ntf: Some(cluster_ch_load_change_ntf),
            neighbor_inactivity_ntf: Some(neighbor_inactivity_ntf),
            neighbor_paging_failure_ntf: Some(neighbor_paging_failure_ntf),
            rssi_scan_ntf: Some(rssi_scan_ntf),
            cluster_beacon_ntf: Some(cluster_beacon_ntf),
            cluster_beacon_rx_failure_ntf: Some(cluster_beacon_rx_failure_ntf),
            ipv6_config_update_ntf: Some(ipv6_config_update_ntf),
            network_beacon_ntf: Some(network_beacon_ntf),
            capability_ntf: Some(capability_ntf),
            dlc_data_rx_ntf: Some(dlc_data_rx_ntf),
            dlc_flow_control_ntf: Some(dlc_flow_control_ntf),
        };

    unsafe extern "C" fn control_functional_mode(
        params: *mut nrfxlib_sys::nrf_modem_dect_mac_control_functional_mode_cb_params,
    ) {
        // SAFETY: implied in C API
        let params = unsafe { &*params };
        info!(
            "Functional mode set completed: {}",
            err::display(params.status)
        );
        if params.status == 0 {
            SINGLETON_EVENTS.try_send(()).unwrap();
        }
    }
    unsafe extern "C" fn control_configure(
        params: *mut nrfxlib_sys::nrf_modem_dect_mac_control_configure_cb_params,
    ) {
        // SAFETY: implied in C API
        let params = unsafe { &*params };
        info!(
            "Control configure was accepted: {}",
            err::display(params.status)
        );
        if params.status == 0 {
            SINGLETON_EVENTS.try_send(()).unwrap();
        }
    }
    unsafe extern "C" fn control_systemmode(
        params: *mut nrfxlib_sys::nrf_modem_dect_mac_control_systemmode_cb_params,
    ) {
        // SAFETY: implied in C API
        let params = unsafe { &*params };
        info!("System mode set completed: {}", err::display(params.status));
        if params.status == 0 {
            SINGLETON_EVENTS.try_send(()).unwrap();
        }
    }
    unsafe extern "C" fn association(
        params: *mut nrfxlib_sys::nrf_modem_dect_mac_association_cb_params,
    ) {
        // SAFETY: implied in C API
        let params = unsafe { &*params };
        info!(
            "Association callback, status {}, long_rd_id 0x{:x}",
            err::display(params.status),
            params.long_rd_id
        );
        if params.status != 0 {
            return;
        }
        info!(
            "  RX signal info: MCS {}, TX power {}, RSSI2 {}, SNR {}",
            params.rx_signal_info.mcs,
            params.rx_signal_info.transmit_power,
            params.rx_signal_info.rssi_2,
            params.rx_signal_info.snr
        );
        info!("  IPv6 config: type {} address {}", match params.ipv6_config.type_ {
            nrfxlib_sys::nrf_modem_dect_mac_ipv6_address_type_NRF_MODEM_DECT_MAC_IPV6_ADDRESS_TYPE_FULL => "full",
            nrfxlib_sys::nrf_modem_dect_mac_ipv6_address_type_NRF_MODEM_DECT_MAC_IPV6_ADDRESS_TYPE_NONE => "none",
            nrfxlib_sys::nrf_modem_dect_mac_ipv6_address_type_NRF_MODEM_DECT_MAC_IPV6_ADDRESS_TYPE_PREFIX => "prefix",
            _ => "?"
        }, Hex(params.ipv6_config.address));
        // FIXME: only valid if bit is set
        info!(
            "  Association response: ack {}, reject…, flows {}, harq…, group_id…, resource_tag…",
            params.association_response.ack_status,
            match params.association_response.number_of_flows {
                7 => "all that were",
                _ => "some…",
            }
        );
        debug_ies(unsafe { core::slice::from_raw_parts(params.ies, params.number_of_ies as _) });
        // Unclear: When is which kind of the flags relevant? Or is this just a type punning union?
    }
    unsafe extern "C" fn association_release(
        params: *mut nrfxlib_sys::nrf_modem_dect_mac_association_release_cb_params,
    ) {
        todo!()
    }
    unsafe extern "C" fn cluster_beacon_receive(
        params: *mut nrfxlib_sys::nrf_modem_dect_mac_cluster_beacon_receive_cb_params,
    ) {
        todo!()
    }
    unsafe extern "C" fn cluster_beacon_receive_stop(
        params: *mut nrfxlib_sys::nrf_modem_dect_mac_cluster_beacon_receive_stop_cb_params,
    ) {
        todo!()
    }
    unsafe extern "C" fn cluster_configure(
        params: *mut nrfxlib_sys::nrf_modem_dect_mac_cluster_configure_cb_params,
    ) {
        todo!()
    }
    unsafe extern "C" fn cluster_info(
        params: *mut nrfxlib_sys::nrf_modem_dect_mac_cluster_info_cb_params,
    ) {
        todo!()
    }
    unsafe extern "C" fn neighbor_info(
        params: *mut nrfxlib_sys::nrf_modem_dect_mac_neighbor_info_cb_params,
    ) {
        todo!()
    }
    unsafe extern "C" fn neighbor_list(
        params: *mut nrfxlib_sys::nrf_modem_dect_mac_neighbor_list_cb_params,
    ) {
        todo!()
    }
    unsafe extern "C" fn dlc_data_tx(
        params: *mut nrfxlib_sys::nrf_modem_dect_dlc_data_tx_cb_params,
    ) {
        // SAFETY: implied in C API
        let params = unsafe { &*params };
        // FIXME: use a more elaborate channel once we support more than the current dlc_data_tx
        if params.status == 0 {
            SINGLETON_EVENTS.try_send(()).unwrap();
        } else {
            warn!("Could not TX: {}", err::display(params.status));
            // FIXME: send that there was an error.
            SINGLETON_EVENTS.try_send(()).unwrap();
        }
    }
    unsafe extern "C" fn dlc_data_discard(
        params: *mut nrfxlib_sys::nrf_modem_dect_dlc_data_discard_cb_params,
    ) {
        todo!()
    }
    unsafe extern "C" fn network_beacon_configure(
        params: *mut nrfxlib_sys::nrf_modem_dect_mac_network_beacon_configure_cb_params,
    ) {
        todo!()
    }
    unsafe extern "C" fn network_scan(
        params: *mut nrfxlib_sys::nrf_modem_dect_mac_network_scan_cb_params,
    ) {
        // SAFETY: implied in C API
        let params = unsafe { &*params };
        // Ignoring num_channels; not sure why how that'd be new information
        // FIXME: should we ignore the status because cancellation could have legitimately happened?
        if params.status == 0 {
            SINGLETON_EVENTS.try_send(()).unwrap();
        }
    }
    unsafe extern "C" fn network_scan_stop(
        params: *mut nrfxlib_sys::nrf_modem_dect_mac_network_scan_stop_cb_params,
    ) {
        // SAFETY: implied in C API
        let params = unsafe { &*params };
        // FIXME: should *this one* ignore the status?
        if params.status == 0 {
            SINGLETON_EVENTS.try_send(()).unwrap();
        }
    }
    unsafe extern "C" fn rssi_scan(
        params: *mut nrfxlib_sys::nrf_modem_dect_mac_rssi_scan_cb_params,
    ) {
        todo!()
    }
    unsafe extern "C" fn rssi_scan_stop(
        params: *mut nrfxlib_sys::nrf_modem_dect_mac_rssi_scan_stop_cb_params,
    ) {
        todo!()
    }

    unsafe extern "C" fn association_ntf(
        params: *mut nrfxlib_sys::nrf_modem_dect_mac_association_ntf_cb_params,
    ) {
        todo!("Only relevant for FTs")
    }
    unsafe extern "C" fn association_release_ntf(
        params: *mut nrfxlib_sys::nrf_modem_dect_mac_association_release_ntf_cb_params,
    ) {
        // SAFETY: implied in C API
        let params = unsafe { &*params };
        warn!(
            "Association is gone: cause {}, peer 0x{:x}",
            params.release_cause, params.long_rd_id
        );
    }
    unsafe extern "C" fn cluster_ch_load_change_ntf(
        params: *mut nrfxlib_sys::nrf_modem_dect_mac_cluster_ch_load_change_ntf_cb_params,
    ) {
        todo!("Only relevant for FTs")
    }
    unsafe extern "C" fn neighbor_inactivity_ntf(
        params: *mut nrfxlib_sys::nrf_modem_dect_mac_neighbor_inactivity_ntf_cb_params,
    ) {
        todo!("Only relevant for FTs")
    }
    unsafe extern "C" fn neighbor_paging_failure_ntf(
        params: *mut nrfxlib_sys::nrf_modem_dect_mac_neighbor_paging_failure_ntf_cb_params,
    ) {
        todo!("Only relevant for FTs")
    }
    unsafe extern "C" fn rssi_scan_ntf(
        params: *mut nrfxlib_sys::nrf_modem_dect_mac_rssi_scan_ntf_cb_params,
    ) {
        todo!("Only relevant for FTs")
    }
    unsafe extern "C" fn cluster_beacon_ntf(
        params: *mut nrfxlib_sys::nrf_modem_dect_mac_cluster_beacon_ntf_cb_params,
    ) {
        // SAFETY: implied in C API
        let params = unsafe { &*params };
        // It's a bit unfortunate that we have to copy around here rather than just re-owning a
        // pool message, but we can still try to do better when we see what's in the actual IPC
        // API.
        if BEACON_EVENTS.try_send(ClusterBeacon {
            channel: params.channel,
            transmitter_short_rd_id: params.transmitter_short_rd_id,
            transmitter_long_rd_id: params.transmitter_long_rd_id,
            network_id: params.network_id,
        }).is_ok() {
            // Abusing the fill level of BEACON_EVENTS for sensible debugging: If someone is
            // listening, we're in scanning mode, and it makes sense to debug. Otherwise, we're in
            // regular operation just receiving the becon of the cell we're in, and that'd just
            // produce clutter.
            //
            // (We spill some when we fill up the queue, but that's fine).

            info!(
                "Got cluster beacon! Channel {}, TX short 0x{:x}, TX long 0x{:x}, network 0x{:x}, {} IEs",
                params.channel,
                params.transmitter_short_rd_id,
                params.transmitter_long_rd_id,
                params.network_id,
                params.number_of_ies
            );
            debug_ies(unsafe { core::slice::from_raw_parts(params.ies, params.number_of_ies as _) });
        }
    }
    unsafe extern "C" fn cluster_beacon_rx_failure_ntf(
        params: *mut nrfxlib_sys::nrf_modem_dect_mac_cluster_beacon_rx_failure_ntf_cb_params,
    ) {
        // SAFETY: implied in C API
        let params = unsafe { &*params };
        info!(
            "Got cluster beacon RX failure on long_rd_id {} (which channel??)",
            params.long_rd_id
        );
    }
    unsafe extern "C" fn ipv6_config_update_ntf(
        params: *mut nrfxlib_sys::nrf_modem_dect_mac_ipv6_config_update_ntf_cb_params,
    ) {
        todo!()
    }
    unsafe extern "C" fn network_beacon_ntf(
        params: *mut nrfxlib_sys::nrf_modem_dect_mac_network_beacon_ntf_cb_params,
    ) {
        // SAFETY: implied in C API
        let params = unsafe { &*params };
        info!(
            "Got network beacon on channel {} from 0x{:x} on network 0x{:x}",
            params.channel, params.transmitter_long_rd_id, params.network_id,
        );
        // FIXME: do we *do* anything with those, esp. w/rt sending them to the scan process?
    }
    unsafe extern "C" fn capability_ntf(
        params: *mut nrfxlib_sys::nrf_modem_dect_mac_capability_ntf_cb_params,
    ) {
        // SAFETY: implied in C API
        let params = unsafe { &*params };
        info!(
            "Got capability NTF: max_mcs = {}, band_info = [_; {}]",
            params.max_mcs, params.num_band_info_elems,
        );
    }
    unsafe extern "C" fn dlc_data_rx_ntf(
        params: *mut nrfxlib_sys::nrf_modem_dect_dlc_data_rx_ntf_cb_params,
    ) {
        // SAFETY: implied in C API
        let params = unsafe { &*params };
        // SAFETY: implied in C API (and we're only using it in this callback)
        // FIXME: Can we get a `_mut`? (Not that I'd know what to use it for, given we have to copy
        // the data out in the ISR)
        let data = core::slice::from_raw_parts(params.data as *mut u8, params.data_len);
        info!(
            "Got DLC data RX: flow {}, peer 0x{:x}, data {}",
            params.flow_id,
            params.long_rd_id,
            Hex(data)
        );
        if let Ok(vec) = data.try_into() {
            if PACKETS.try_send(vec).is_err() {
                warn!("Could not enqueue: queue full");
            }
        } else {
            warn!("Could not enqueue: too big for heapless vec");
        };
    }
    unsafe extern "C" fn dlc_flow_control_ntf(
        params: *mut nrfxlib_sys::nrf_modem_dect_dlc_flow_control_ntf_cb_params,
    ) {
        todo!()
    }
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
struct DectMac(());

// This should evolve a bit
//
// So far, this is only used for the lockstepping parts, when exactly one thing is possible and
// exactly one thing happens.
use embassy_sync::blocking_mutex::raw::CriticalSectionRawMutex;
use embassy_sync::channel::Channel;
/// Events which are only generated while the [`DectMac`] is in a `&mut self` operation that has no
/// other events. Typically, this is all kinds of global mode changes.
///
/// Length is 2 to allow use for scan and scan_stop: If a scan is stopped before time, confirmation
/// of the end of scan and of the stop command will both arrive. They will be indistinguishable,
/// and that doesn't matter.
static SINGLETON_EVENTS: Channel<CriticalSectionRawMutex, (), 2> = Channel::new();

/// Events during a scan (maybe also during associated operation).
///
/// FIXME: How should overflow be indicated?
static BEACON_EVENTS: Channel<
    CriticalSectionRawMutex,
    ClusterBeacon,
    // FIXME: What's a good number?
    2,
> = Channel::new();

// FIXME: We definitely want to use something smarter; ideally ownership of net pool entries when
// we better understand who allocates what.
static PACKETS: Channel<
    CriticalSectionRawMutex,
    // If we make it bigger, we might easily exceed the ISR stack
    heapless::vec::Vec<u8, 100>,
    1,
> = Channel::new();

impl DectMac {
    /// See hophop::nrfxlib_phy::DectPhy::init_after_modem_init() for `_modem_is_set_up` context
    ///
    /// This immediately starts setting up the MAC system mode. If we later find out we don't want
    /// that, there'll probably be some kind of "desiccated" form of this object where a user can
    /// first initialize the desiccated form and then do a "set MAC system mode" function, both
    /// which steps this combines.
    fn create(_modem_is_set_up: ()) -> Self {
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

    async fn systemmode_set_mac(&mut self) {
        unsafe {
            nrfxlib_sys::nrf_modem_dect_control_systemmode_set(
                nrfxlib_sys::nrf_modem_dect_control_systemmode_NRF_MODEM_DECT_MODE_MAC,
            )
        }
        .into_result()
        .expect("Failed to set system mode");
        SINGLETON_EVENTS.receive().await;
    }

    // &mut is probably not needed. FIXME: ask if C API can be enhanced
    async fn control_configure(
        &mut self,
        params: &mut nrfxlib_sys::nrf_modem_dect_control_configure_params,
    ) {
        unsafe { nrfxlib_sys::nrf_modem_dect_control_configure(params) }
            .into_result()
            .expect("Failed to set configuration params");
        SINGLETON_EVENTS.receive().await;
    }

    async fn control_functional_mode_set_activate(&mut self) {
        unsafe {
            nrfxlib_sys::nrf_modem_dect_control_functional_mode_set(
                nrfxlib_sys::nrf_modem_dect_control_functional_mode_NRF_MODEM_DECT_CONTROL_FUNCTIONAL_MODE_ACTIVATE
            )
        }.into_result().expect("Failed to set functional mode");
        SINGLETON_EVENTS.receive().await;
    }

    /// Starts a network scan.
    ///
    /// Scan results are available inside the async closure. If the scan times out, the closure is
    /// dropped. If the closure completes, the scan is stopped before returning from this function.
    // &mut is probably not needed. FIXME: ask if C API can be enhanced
    async fn mac_network_scan<R>(
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
            First(()) => None,
            Second(r) => {
                unsafe { nrfxlib_sys::nrf_modem_dect_mac_network_scan_stop() }
                    .into_result()
                    .expect("Failed to stop the scan");

                // One of those is the First that never happened / was cancelled.
                SINGLETON_EVENTS.receive().await;
                SINGLETON_EVENTS.receive().await;

                // FIXME: cancel and await two end events
                Some(r)
            }
        }
    }

    /// Transmits data in one of the flows.
    ///
    /// This is a primitive wrapper in that it only returns after transmission; a better version
    /// would return once successfully enqueued and signal completion later.
    async fn dlc_data_tx(&mut self, flow_id: u8, destination: u32, data: &[u8]) {
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
        SINGLETON_EVENTS.receive().await;
    }
}

// Almost nrf_modem_dect_mac_cluster_beacon_ntf_cb_params, but dropping the ies -- not because we
// really want to (I'd rather memcpy than create a new struct), but because that pointer makes the
// whole type not Send, and it comes from the ISR. We wouldn't touch it, but are in no position to
// impl Send on it.
struct ClusterBeacon {
    pub channel: u16,
    pub transmitter_short_rd_id: u16,
    pub transmitter_long_rd_id: u32,
    pub network_id: u32,
}

#[derive(Copy, Clone)]
struct ScanReceiver<'brand>(core::marker::PhantomData<&'brand ()>);

impl<'brand> ScanReceiver<'brand> {
    // FIXME dress up in non _sys dependent API
    async fn next(self) -> ClusterBeacon {
        BEACON_EVENTS.receive().await
    }
}

#[ariel_os::task(autostart)]
async fn main() {
    info!("Initializing DECT MAC, trusting that Ariel OS did the basic setup");
    let mut dect = DectMac::create(());

    dect.systemmode_set_mac().await;

    dect.control_configure(&mut nrfxlib_sys::nrf_modem_dect_control_configure_params {
        // FIXME: Decide (this is a "let's keep it civilized" guess)
        max_tx_power: nrfxlib_sys::nrf_modem_dect_mac_tx_power_NRF_MODEM_DECT_MAC_TX_POWER_10_DB,
        // FIXME: take from hardware
        max_mcs: nrfxlib_sys::nrf_modem_dect_mac_max_mcs_NRF_MODEM_DECT_MAC_MAX_MCS_4,
        // FIXME: Decide (this is what the vendor examples default to)
        expected_mcs1_rx_rssi_level: -68,
        // FIXME: make configurable
        long_rd_id: 0xf00feaae,
        // FIXME: configure
        phy_band_group_index:
            nrfxlib_sys::nrf_modem_dect_mac_band_group_index_NRF_MODEM_DECT_MAC_PHY_BAND_GROUP_IDX0,
        // FIXME: configure
        power_save: true,
        security: nrfxlib_sys::nrf_modem_dect_control_configure_params__bindgen_ty_1 {
            // FIXME: That we set this means we'll have to change the configuration params
            // late (when we have *actual* keys). Can we just change the params later w/o
            // leaving functional mode?
            mode:
                nrfxlib_sys::nrf_modem_dect_mac_security_mode_NRF_MODEM_DECT_MAC_SECURITY_MODE_NONE,
            integrity_key: [0; _],
            cipher_key: [0; _],
        },
        // FIXME: Decide (this is what the vendor uses in their examples)
        stats_averaging_length: 2,
    })
    .await;

    dect.control_functional_mode_set_activate().await;

    // Making this standalone should make it easier later to give strategies such as "scan our
    // preferred channel for a few seconds, then the whole band".
    let find_our_network = async |r: ScanReceiver| {
        loop {
            let params = r.next().await;

            if params.network_id == 0x87654321 {
                // That's the demo network we are looking for, mostly following the DECT shell defaults.
                //
                // (What is missing is that we'd use that to dig up keys and then reconfigure the
                // keying)
                break params;
            }
        }
    };

    let params = dect.mac_network_scan(&mut nrfxlib_sys::nrf_modem_dect_mac_network_scan_params {
        band: 0, //nrfxlib_sys::nrf_modem_dect_mac_band_NRF_MODEM_DECT_MAC_PHY_BAND1,
        num_channels: 1, //0
        channel_list: [1665; 20], // so it terminates fast
        scan_time: 3_000, // ms -- 60s is the maximum. I guess this is per band?
        network_id_filter_mode: nrfxlib_sys::nrf_modem_dect_mac_nw_id_filter_mode_NRF_MODEM_DECT_MAC_NW_ID_FILTER_MODE_NONE,
        network_id_filter: 0,
    }, find_our_network).await;

    let Some(params) = params else {
        error!("Scanning for 3s found no beacon of our network, exiting.");
        return;
    };

    info!("Now that scanning is complete, attempting association with the found FT");
    // Can we do 0? (We can't put NULL in it, that'd be rejected even at function call time)
    // Probably not and makes no sense. Funnily, peers can't even `dect tx` and we don't get pings
    // if we don't put the "High layer signalling - flow 1" in. (At least a user data flow alone
    // replacing it is insufficient.)
    let mut tx_flow_configs = [
        nrfxlib_sys::nrf_modem_dect_mac_tx_flow_config {
            flow_id: 6, // "User plane data -- flow 4"
            priority: 4,
            dlc_service_type:
                nrfxlib_sys::nrf_modem_dect_dlc_service_type_NRF_MODEM_DECT_DLC_SERVICE_TYPE_3,
            dlc_sdu_lifetime:
                nrfxlib_sys::nrf_modem_dect_dlc_sdu_lifetime_NRF_MODEM_DECT_DLC_SDU_LIFETIME_8_S
        },
        nrfxlib_sys::nrf_modem_dect_mac_tx_flow_config {
            //flow_id: 0b11, // Table 6.3.4-2: IE type field encoding for MAC Extension field encoding 00, 01, 10 -- do they really want this for "User Data Plane -- flow 1"?
            flow_id: 1, // "Higher layer signalling - flow 1"
            priority: 0,
            dlc_service_type:
                nrfxlib_sys::nrf_modem_dect_dlc_service_type_NRF_MODEM_DECT_DLC_SERVICE_TYPE_3,
            dlc_sdu_lifetime:
                // picking something long, I think right now we can only TX right after a beacon
                nrfxlib_sys::nrf_modem_dect_dlc_sdu_lifetime_NRF_MODEM_DECT_DLC_SDU_LIFETIME_8_S,
        },
    ];
    unsafe {
        nrfxlib_sys::nrf_modem_dect_mac_association(
            &mut nrfxlib_sys::nrf_modem_dect_mac_association_params {
                // FIXME where do we use the channel information? Did the MAC remember? (Probably:
                // After all, it's not just channel but the whole info stuff from the beacon).
                long_rd_id: params.transmitter_long_rd_id,
                network_id: params.network_id,
                info_triggers: nrfxlib_sys::nrf_modem_dect_mac_parent_info_triggers {
                    // FIXME: this is a guess
                    num_beacon_rx_failures: 1,
                },
                num_flows: tx_flow_configs
                    .len()
                    .try_into()
                    .expect("Absurd number of configs"),
                tx_flow_configs: &mut tx_flow_configs as _,
            },
        )
    }
    .into_result()
    .expect("Failed to start association attempt");

    ariel_os::time::Timer::after_millis(4_000).await;

    // OK thisis funny: Data from all channels arrives at the dect_shell's IP stack, as evidenced
    // by both counts here going into the bytes received of `net stats`.
    //
    // So all channels are dumped into the IP stack indiscriminately? Can't be right, but OTOH, we
    // can work with that for something minimal, and set it right later.

    dect.dlc_data_tx(6, params.transmitter_long_rd_id, b"hello hello");
    info!("Sent on flow 6");

    dect.dlc_data_tx(
        1,
        params.transmitter_long_rd_id,
        b"00112233445566778899aabbccddeeff",
    );
    info!("Sent on flow 1");

    // FIXME: *don't* implement an IP stack, either pass data on to embassy-net or UART via slipmux.
    // (and not going into any FIXME worthy items down here, this is clearly a quick and stupid
    // hack)
    loop {
        info!("Idling with our own very primitive ping responder");
        let mut packet = PACKETS.receive().await;
        info!("Got packet {}", Hex(packet.as_slice()));
        if packet[6] != 0x3a {
            info!("Received packet is not ICMPv6, ignoring");
            continue;
        }
        packet[8..40].rotate_right(16); // or left :-)
        if packet[40] != 128 {
            info!("Received packet is not Echo Request, ignoring");
            continue;
        }
        packet[40] = 129; // Echo Reply
        // Not bothering to fix the ICMP checksum: the dect_shell is happy enough to report the
        // error well enough
        info!("Sending response");
        dect.dlc_data_tx(
            1,
            // Actually could be from someone else as well
            params.transmitter_long_rd_id,
            packet.as_slice(),
        ).await;
    }
}
