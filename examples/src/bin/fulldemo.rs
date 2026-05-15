// SPDX-FileCopyrightText: Copyright Christian Amsüss <chrysn@fsfe.org>, Silano Systems
// SPDX-License-Identifier: MIT OR Apache-2.0
//! Initial demo for Rust using the high-level (MAC etc) nrfxlib features

#![no_std]
#![no_main]

use ariel_os::debug::log::{Hex, info, warn};

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
            EVENTS.try_send(()).unwrap();
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
            EVENTS.try_send(()).unwrap();
        }
    }
    unsafe extern "C" fn control_systemmode(
        params: *mut nrfxlib_sys::nrf_modem_dect_mac_control_systemmode_cb_params,
    ) {
        // SAFETY: implied in C API
        let params = unsafe { &*params };
        info!("System mode set completed: {}", err::display(params.status));
        if params.status == 0 {
            EVENTS.try_send(()).unwrap();
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
        info!(
            "  Association response: ack {}, reject…, flows…, harq…, group_id…, resource_tag…",
            params.association_response.ack_status,
            // Not showing flows here: that typically gives 0x07 which means "all" … but I hoped to
            // learn theirs.
        );
        info!("  IEs: […; {}]", params.number_of_ies);
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
        todo!()
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
        info!(
            "Scan came back with status {} channels {}",
            err::display(params.status),
            params.num_scanned_channels
        );
    }
    unsafe extern "C" fn network_scan_stop(
        params: *mut nrfxlib_sys::nrf_modem_dect_mac_network_scan_stop_cb_params,
    ) {
        todo!()
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
        todo!()
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
        todo!()
    }
    unsafe extern "C" fn neighbor_inactivity_ntf(
        params: *mut nrfxlib_sys::nrf_modem_dect_mac_neighbor_inactivity_ntf_cb_params,
    ) {
        todo!()
    }
    unsafe extern "C" fn neighbor_paging_failure_ntf(
        params: *mut nrfxlib_sys::nrf_modem_dect_mac_neighbor_paging_failure_ntf_cb_params,
    ) {
        todo!()
    }
    unsafe extern "C" fn rssi_scan_ntf(
        params: *mut nrfxlib_sys::nrf_modem_dect_mac_rssi_scan_ntf_cb_params,
    ) {
        todo!()
    }
    unsafe extern "C" fn cluster_beacon_ntf(
        params: *mut nrfxlib_sys::nrf_modem_dect_mac_cluster_beacon_ntf_cb_params,
    ) {
        // SAFETY: implied in C API
        let params = unsafe { &*params };
        // Note that this happens both during scanning and when associated
        info!(
            "Got cluster beacon! Channel {}, TX short 0x{:x}, TX long 0x{:x}, network 0x{:x}, {} IEs",
            params.channel,
            params.transmitter_short_rd_id,
            params.transmitter_long_rd_id,
            params.network_id,
            params.number_of_ies
        );
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
        todo!()
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
/// Does it make sense for this to be smart and do higher level stuff, or better just have an
/// idiomatic async wrapper around the lib?
///
/// It is expected that as earlier with the PHY functions, this moves into hophop. But not now.
struct DectMac(());

// This should evolve a bit
//
// So far, this is only used for the lockstepping parts, when exactly one thing is possible and
// exactly one thing happens.
use embassy_sync::blocking_mutex::raw::CriticalSectionRawMutex;
use embassy_sync::channel::Channel;
static EVENTS: Channel<CriticalSectionRawMutex, (), 1> = Channel::new();

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
        EVENTS.receive().await;
    }

    // &mut is probably not needed. FIXME: ask if C API can be enhanced
    async fn control_configure(
        &mut self,
        params: &mut nrfxlib_sys::nrf_modem_dect_control_configure_params,
    ) {
        unsafe { nrfxlib_sys::nrf_modem_dect_control_configure(params) }
            .into_result()
            .expect("Failed to set configuration params");
        EVENTS.receive().await;
    }

    async fn control_functional_mode_set_activate(&mut self) {
        unsafe {
            nrfxlib_sys::nrf_modem_dect_control_functional_mode_set(
                nrfxlib_sys::nrf_modem_dect_control_functional_mode_NRF_MODEM_DECT_CONTROL_FUNCTIONAL_MODE_ACTIVATE
            )
        }.into_result().expect("Failed to set functional mode");
        EVENTS.receive().await;
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

    info!("Attempting a scan");
    unsafe {
        nrfxlib_sys::nrf_modem_dect_mac_network_scan(&mut nrfxlib_sys::nrf_modem_dect_mac_network_scan_params {
        band: 0, //nrfxlib_sys::nrf_modem_dect_mac_band_NRF_MODEM_DECT_MAC_PHY_BAND1,
        num_channels: 1, //0
        channel_list: [1665; 20], // so it terminates fast
        scan_time: 3_000, // ms -- 60s is the maximum. I guess this is per band?
        network_id_filter_mode: nrfxlib_sys::nrf_modem_dect_mac_nw_id_filter_mode_NRF_MODEM_DECT_MAC_NW_ID_FILTER_MODE_NONE,
        network_id_filter: 0,
    })
    }.into_result().expect("Failed to start the scan");

    // We'd have to wait way longer actually -- scan time is, like, 3s * 21 bands
    //
    // And we have to wait for the scan to complete, or the association would be "not allowed"
    ariel_os::time::Timer::after_millis(4_000).await;

    info!(
        "While scanning, attempting association with PT at programmer ..2646 (TX long ID 0x70d1776d)"
    );
    // Can we do 0? (We can't put NULL in it, that'd be rejected even at function call time)
    // Probably not and makes no sense. Funnily, peers can't even `dect tx` and we don't get pings
    // if we don't put the "High layer signalling - flow 1" in. (At least a user data flow alone
    // replacing it is insufficient.)
    let mut tx_flow_configs: [nrfxlib_sys::nrf_modem_dect_mac_tx_flow_config; 1] =
        [nrfxlib_sys::nrf_modem_dect_mac_tx_flow_config {
            //flow_id: 0b11, // Table 6.3.4-2: IE type field encoding for MAC Extension field encoding 00, 01, 10 -- do they really want this for "User Data Plane -- flow 1"?
            flow_id: 1, // "Higher layer signalling - flow 1"
            priority: 0,
            dlc_service_type:
                nrfxlib_sys::nrf_modem_dect_dlc_service_type_NRF_MODEM_DECT_DLC_SERVICE_TYPE_3,
            dlc_sdu_lifetime:
                nrfxlib_sys::nrf_modem_dect_dlc_sdu_lifetime_NRF_MODEM_DECT_DLC_SDU_LIFETIME_1_MS,
        }];
    unsafe {
        nrfxlib_sys::nrf_modem_dect_mac_association(
            &mut nrfxlib_sys::nrf_modem_dect_mac_association_params {
                // FIXME: get from scan
                long_rd_id: 0x70d1776d,
                // FIXME: get from scan
                network_id: 0x87654321,
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
}
