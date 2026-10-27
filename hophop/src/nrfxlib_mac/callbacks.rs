// SPDX-FileCopyrightText: Copyright Christian Amsüss <chrysn@fsfe.org>, Silano Systems
// SPDX-License-Identifier: MIT OR Apache-2.0
//! Global callbacks used with JNordic's DECT MAC.
//!
//! All their output goes into global channels set up in [`super::shared_queues`] -- the underlying
//! `libmodem` doesn't cater for a `*void` context, and while that'd good general practice, it'd
//! only make sense if the whole library carried some "which DECT device" pointers, and for their
//! hardware there just are no multiples.
#![allow(
    unused_variables,
    reason = "stop complaining while there are todo!() items"
)]

use defmt::{info, warn};
use nrfxlib_sys_pretty::{dect_dlc, dect_mac};

use ts_103_636_utils::identifiers::{AbsoluteChannel, LongRdId, NetworkId32, ShortRdId};

use super::debug_helpers::debug_ies;
use super::error::MacErrorExt;
use super::shared_queues::*;
use super::{AssociationEndEvent, ClusterBeacon, DlcDataRx};

pub(super) static OP_CALLBACKS: dect_mac::op_callbacks = dect_mac::op_callbacks {
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

pub(super) static MAC_NTF_CALLBACKS: dect_mac::ntf_callbacks = dect_mac::ntf_callbacks {
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
    params: *mut dect_mac::control_functional_mode_cb_params,
) {
    // SAFETY: implied in C API
    let params = unsafe { &*params };
    info!(
        "Functional mode set completed: {:?}",
        params.status.into_mac_status(),
    );
    SINGLETON_EVENTS
        .try_send(params.status.into_mac_status())
        .unwrap();
}
unsafe extern "C" fn control_configure(params: *mut dect_mac::control_configure_cb_params) {
    // SAFETY: implied in C API
    let params = unsafe { &*params };
    info!(
        "Control configure was accepted: {:?}",
        params.status.into_mac_status(),
    );
    SINGLETON_EVENTS
        .try_send(params.status.into_mac_status())
        .unwrap();
}
unsafe extern "C" fn control_systemmode(params: *mut dect_mac::control_systemmode_cb_params) {
    // SAFETY: implied in C API
    let params = unsafe { &*params };
    info!(
        "System mode set completed: {:?}",
        params.status.into_mac_status()
    );
    SINGLETON_EVENTS
        .try_send(params.status.into_mac_status())
        .unwrap();
}
unsafe extern "C" fn association(params: *mut dect_mac::association_cb_params) {
    // SAFETY: implied in C API
    let params = unsafe { &*params };
    info!(
        "Association callback, status {:?}, long_rd_id 0x{:x}",
        params.status.into_mac_status(),
        params.long_rd_id
    );
    if params.status == 0 {
        info!(
            "  RX signal info: MCS {}, TX power {}, RSSI2 {}, SNR {}",
            params.rx_signal_info.mcs,
            params.rx_signal_info.transmit_power,
            params.rx_signal_info.rssi_2,
            params.rx_signal_info.snr
        );
        info!(
            "  IPv6 config: type {} address 0x{:x}",
            match params.ipv6_config.type_ {
                dect_mac::ipv6_address_type_values::FULL => "full",
                dect_mac::ipv6_address_type_values::NONE => "none",
                dect_mac::ipv6_address_type_values::PREFIX => "prefix",
                _ => "?",
            },
            params.ipv6_config.address
        );
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

    SINGLETON_EVENTS
        .try_send(params.status.into_mac_status())
        .unwrap();
}
unsafe extern "C" fn association_release(params: *mut dect_mac::association_release_cb_params) {
    todo!()
}
unsafe extern "C" fn cluster_beacon_receive(
    params: *mut dect_mac::cluster_beacon_receive_cb_params,
) {
    todo!()
}
unsafe extern "C" fn cluster_beacon_receive_stop(
    params: *mut dect_mac::cluster_beacon_receive_stop_cb_params,
) {
    todo!()
}
unsafe extern "C" fn cluster_configure(params: *mut dect_mac::cluster_configure_cb_params) {
    let &dect_mac::cluster_configure_cb_params { status } = unsafe { &*params };
    SINGLETON_EVENTS.try_send(status.into_mac_status()).unwrap();
}
unsafe extern "C" fn cluster_info(params: *mut dect_mac::cluster_info_cb_params) {
    todo!()
}
unsafe extern "C" fn neighbor_info(params: *mut dect_mac::neighbor_info_cb_params) {
    todo!()
}
unsafe extern "C" fn neighbor_list(params: *mut dect_mac::neighbor_list_cb_params) {
    todo!()
}
unsafe extern "C" fn dlc_data_tx(params: *mut dect_dlc::data_tx_cb_params) {
    // SAFETY: implied in C API
    let params = unsafe { &*params };
    // FIXME: use a more elaborate channel once we support more than the current dlc_data_tx
    SINGLETON_EVENTS
        .try_send(params.status.into_mac_status())
        .unwrap();
}
unsafe extern "C" fn dlc_data_discard(params: *mut dect_dlc::data_discard_cb_params) {
    todo!()
}
unsafe extern "C" fn network_beacon_configure(
    params: *mut dect_mac::network_beacon_configure_cb_params,
) {
    todo!()
}
unsafe extern "C" fn network_scan(params: *mut dect_mac::network_scan_cb_params) {
    // SAFETY: implied in C API
    let params = unsafe { &*params };
    // Ignoring num_channels; not sure why how that'd be new information
    SINGLETON_EVENTS
        .try_send(params.status.into_mac_status())
        .unwrap();
}
unsafe extern "C" fn network_scan_stop(params: *mut dect_mac::network_scan_stop_cb_params) {
    // SAFETY: implied in C API
    let params = unsafe { &*params };
    SINGLETON_EVENTS
        .try_send(params.status.into_mac_status())
        .unwrap();
}
unsafe extern "C" fn rssi_scan(params: *mut dect_mac::rssi_scan_cb_params) {
    // SAFETY: implied in C API
    let &dect_mac::rssi_scan_cb_params { status } = unsafe { &*params };
    SINGLETON_EVENTS.try_send(status.into_mac_status()).unwrap();
}
unsafe extern "C" fn rssi_scan_stop(params: *mut dect_mac::rssi_scan_stop_cb_params) {
    todo!()
}

unsafe extern "C" fn association_ntf(params: *mut dect_mac::association_ntf_cb_params) {
    // SAFETY: implied in C API
    let &dect_mac::association_ntf_cb_params {
        status,
        tx_method,
        rx_signal_info:
            dect_mac::rx_signal_info_t {
                mcs,
                transmit_power,
                rssi_2,
                snr,
            },
        short_rd_id,
        long_rd_id,
        number_of_ies,
        association_request,
        ies,
    } = unsafe { &*params };
    let status = status.into_mac_status();
    let (Some(short_rd_id), Some(long_rd_id)) =
        (ShortRdId::new(short_rd_id), LongRdId::new(long_rd_id))
    else {
        warn!("Ignoring association NTF with reserved values.");
        return;
    };
    info!(
        "Association: status {} from {} / {}",
        status, short_rd_id, long_rd_id
    );
    info!(
        "TX method {}, RX signal info MCS={} TXpwr={} RSSI2={} SNR={}, association request FIXME",
        tx_method, mcs, transmit_power, rssi_2, snr
    );
    // SAFETY: implied by C ABI
    debug_ies(unsafe { core::slice::from_raw_parts(ies, number_of_ies as _) });
}
unsafe extern "C" fn association_release_ntf(
    params: *mut dect_mac::association_release_ntf_cb_params,
) {
    // SAFETY: implied in C API
    let &dect_mac::association_release_ntf_cb_params {
        release_cause,
        long_rd_id,
    } = unsafe { &*params };

    let long_rd_id = LongRdId::new(long_rd_id).unwrap();
    let release_cause =
        ts_103_636_numbers::mac_details::release_message::ReleaseCause::new(release_cause);

    let _ = RELEASE_EVENTS.try_send(AssociationEndEvent {
        long_rd_id,
        release_cause: super::ReleaseOrBeaconFailure::Release(release_cause),
    });
}
unsafe extern "C" fn cluster_ch_load_change_ntf(
    params: *mut dect_mac::cluster_ch_load_change_ntf_cb_params,
) {
    todo!("Only relevant for FTs")
}
unsafe extern "C" fn neighbor_inactivity_ntf(
    params: *mut dect_mac::neighbor_inactivity_ntf_cb_params,
) {
    // SAFETY: implied in C API
    let &nrfxlib_sys::nrf_modem_dect_mac_neighbor_inactivity_ntf_cb_params { long_rd_id } =
        unsafe { &*params };
    let (Some(long_rd_id),) = (LongRdId::new(long_rd_id),) else {
        warn!("Ignoring inactivity NTF with reserved value.");
        return;
    };

    // This will be tricky to use: We only see the long RD ID. If the device reboots and keeps the
    // long RD ID and sends a different short RD ID (as hophop PTs currently do), the inactivity
    // notification only comes after the next association. So in a sense we might need to keep a
    // counter of how often a device has been registered per RD ID, if we want to act on inactivity
    // notifications at all.
    info!("Inactivity notification from {}", long_rd_id);
}
unsafe extern "C" fn neighbor_paging_failure_ntf(
    params: *mut dect_mac::neighbor_paging_failure_ntf_cb_params,
) {
    todo!("Only relevant for FTs")
}
unsafe extern "C" fn rssi_scan_ntf(params: *mut dect_mac::rssi_scan_ntf_cb_params) {
    // SAFETY: implied in C API
    let &dect_mac::rssi_scan_ntf_cb_params {
        channel,
        busy_percentage,
        rssi_meas_array_size,
        busy,
        possible,
        free,
    } = unsafe { &*params };
    let Some(channel) = AbsoluteChannel::new(channel) else {
        warn!("Ignoring rssi_scan_ntf with reserved values.");
        return;
    };
    // SAFETY: explicit in C API
    // FIXME: semantics are bit strings
    let busy = unsafe { core::slice::from_raw_parts(busy, rssi_meas_array_size) };
    let possible = unsafe { core::slice::from_raw_parts(possible, rssi_meas_array_size) };
    let free = unsafe { core::slice::from_raw_parts(free, rssi_meas_array_size) };

    info!(
        "RSSI report: channel {} busy {}%: {:x} / {:x} / {:x}",
        channel, busy_percentage, busy, possible, free
    );
}
unsafe extern "C" fn cluster_beacon_ntf(params: *mut dect_mac::cluster_beacon_ntf_cb_params) {
    // SAFETY: implied in C API
    let params = unsafe { &*params };
    let (
        Some(channel),
        Some(transmitter_short_rd_id),
        Some(transmitter_long_rd_id),
        Some(network_id),
    ) = (
        AbsoluteChannel::new(params.channel),
        ShortRdId::new(params.transmitter_short_rd_id),
        LongRdId::new(params.transmitter_long_rd_id),
        NetworkId32::new(params.network_id),
    )
    else {
        warn!("Ignoring beacon NTF with reserved values.");
        return;
    };
    // It's a bit unfortunate that we have to copy around here rather than just re-owning a
    // pool message, but we can still try to do better when we see what's in the actual IPC
    // API.
    if BEACON_EVENTS
        .try_send(ClusterBeacon {
            channel,
            transmitter_short_rd_id,
            transmitter_long_rd_id,
            network_id,
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
            "Got cluster beacon! Channel {}, TX short {}, TX long {}, network {}, {} IEs",
            channel,
            transmitter_short_rd_id,
            transmitter_long_rd_id,
            network_id,
            params.number_of_ies
        );
        debug_ies(unsafe { core::slice::from_raw_parts(params.ies, params.number_of_ies as _) });
    }
}
unsafe extern "C" fn cluster_beacon_rx_failure_ntf(
    params: *mut dect_mac::cluster_beacon_rx_failure_ntf_cb_params,
) {
    // SAFETY: implied in C API
    let &dect_mac::cluster_beacon_rx_failure_ntf_cb_params { long_rd_id } = unsafe { &*params };

    let long_rd_id = LongRdId::new(long_rd_id).unwrap();

    let _ = RELEASE_EVENTS.try_send(AssociationEndEvent {
        long_rd_id,
        release_cause: super::ReleaseOrBeaconFailure::BeaconRxFailure,
    });
}
unsafe extern "C" fn ipv6_config_update_ntf(
    params: *mut dect_mac::ipv6_config_update_ntf_cb_params,
) {
    todo!()
}
unsafe extern "C" fn network_beacon_ntf(params: *mut dect_mac::network_beacon_ntf_cb_params) {
    // SAFETY: implied in C API
    let params = unsafe { &*params };
    info!(
        "Got network beacon on channel {} from 0x{:x} on network 0x{:x}",
        params.channel, params.transmitter_long_rd_id, params.network_id,
    );
    // FIXME: do we *do* anything with those, esp. w/rt sending them to the scan process?
}
unsafe extern "C" fn capability_ntf(params: *mut dect_mac::capability_ntf_cb_params) {
    // SAFETY: implied in C API
    let params = unsafe { &*params };
    info!(
        "Got capability NTF: max_mcs = {}, band_info = [_; {}]",
        params.max_mcs, params.num_band_info_elems,
    );
}
unsafe extern "C" fn dlc_data_rx_ntf(params: *mut dect_dlc::data_rx_ntf_cb_params) {
    // SAFETY: implied in C API
    let params = unsafe { &*params };
    // SAFETY: implied in C API (and we're only using it in this callback)
    // FIXME: Can we get a `_mut`? (Not that I'd know what to use it for, given we have to copy
    // the data out in the ISR)
    let data = unsafe { core::slice::from_raw_parts(params.data as *mut u8, params.data_len) };
    info!(
        "Got DLC data RX: flow {}, peer 0x{:x}, data 0x{:02x}",
        params.flow_id, params.long_rd_id, data
    );
    if let Ok(vec) = data.try_into() {
        let Ok(mut packet_lock) = PACKETS.try_lock() else {
            warn!("Could not enqueue: packet is just being popped");
            return;
        };
        if packet_lock
            .push_back(DlcDataRx {
                long_rd_id: params.long_rd_id,
                flow_id: params.flow_id,
                data: vec,
            })
            .is_ok()
        {
            let _ = PACKET_READY.try_send(());
        } else {
            warn!("Could not enqueue: queue full");
        }
    } else {
        warn!("Could not enqueue: too big for heapless vec");
    };
}
unsafe extern "C" fn dlc_flow_control_ntf(params: *mut dect_dlc::flow_control_ntf_cb_params) {
    todo!()
}
