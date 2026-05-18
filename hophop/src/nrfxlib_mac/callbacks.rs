#![allow(
    unused_variables,
    reason = "stop complaining while there are todo!() items"
)]

use defmt::{info, warn};
use nrf_modem::nrfxlib_sys;

use super::ClusterBeacon;
use super::debug_helpers::debug_ies;
use super::error::MacErrorExt;
use super::shared_queues::*;

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
        "Functional mode set completed: {:?}",
        params.status.as_mac_status(),
    );
    SINGLETON_EVENTS
        .try_send(params.status.as_mac_status())
        .unwrap();
}
unsafe extern "C" fn control_configure(
    params: *mut nrfxlib_sys::nrf_modem_dect_mac_control_configure_cb_params,
) {
    // SAFETY: implied in C API
    let params = unsafe { &*params };
    info!(
        "Control configure was accepted: {:?}",
        params.status.as_mac_status(),
    );
    SINGLETON_EVENTS
        .try_send(params.status.as_mac_status())
        .unwrap();
}
unsafe extern "C" fn control_systemmode(
    params: *mut nrfxlib_sys::nrf_modem_dect_mac_control_systemmode_cb_params,
) {
    // SAFETY: implied in C API
    let params = unsafe { &*params };
    info!(
        "System mode set completed: {:?}",
        params.status.as_mac_status()
    );
    SINGLETON_EVENTS
        .try_send(params.status.as_mac_status())
        .unwrap();
}
unsafe extern "C" fn association(
    params: *mut nrfxlib_sys::nrf_modem_dect_mac_association_cb_params,
) {
    // SAFETY: implied in C API
    let params = unsafe { &*params };
    info!(
        "Association callback, status {:?}, long_rd_id 0x{:x}",
        params.status.as_mac_status(),
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
    info!("  IPv6 config: type {} address 0x{:x}", match params.ipv6_config.type_ {
        nrfxlib_sys::nrf_modem_dect_mac_ipv6_address_type_NRF_MODEM_DECT_MAC_IPV6_ADDRESS_TYPE_FULL => "full",
        nrfxlib_sys::nrf_modem_dect_mac_ipv6_address_type_NRF_MODEM_DECT_MAC_IPV6_ADDRESS_TYPE_NONE => "none",
        nrfxlib_sys::nrf_modem_dect_mac_ipv6_address_type_NRF_MODEM_DECT_MAC_IPV6_ADDRESS_TYPE_PREFIX => "prefix",
        _ => "?"
    }, params.ipv6_config.address);
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
unsafe extern "C" fn dlc_data_tx(params: *mut nrfxlib_sys::nrf_modem_dect_dlc_data_tx_cb_params) {
    // SAFETY: implied in C API
    let params = unsafe { &*params };
    // FIXME: use a more elaborate channel once we support more than the current dlc_data_tx
    SINGLETON_EVENTS
        .try_send(params.status.as_mac_status())
        .unwrap();
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
    SINGLETON_EVENTS
        .try_send(params.status.as_mac_status())
        .unwrap();
}
unsafe extern "C" fn network_scan_stop(
    params: *mut nrfxlib_sys::nrf_modem_dect_mac_network_scan_stop_cb_params,
) {
    // SAFETY: implied in C API
    let params = unsafe { &*params };
    SINGLETON_EVENTS
        .try_send(params.status.as_mac_status())
        .unwrap();
}
unsafe extern "C" fn rssi_scan(params: *mut nrfxlib_sys::nrf_modem_dect_mac_rssi_scan_cb_params) {
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
    if BEACON_EVENTS
        .try_send(ClusterBeacon {
            channel: params.channel,
            transmitter_short_rd_id: params.transmitter_short_rd_id,
            transmitter_long_rd_id: params.transmitter_long_rd_id,
            network_id: params.network_id,
        })
        .is_ok()
    {
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
        "Got DLC data RX: flow {}, peer 0x{:x}, data 0x{:02x}",
        params.flow_id, params.long_rd_id, data
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
