// SPDX-FileCopyrightText: Copyright Christian Amsüss <chrysn@fsfe.org>, Silano Systems
// SPDX-License-Identifier: MIT OR Apache-2.0
//! Initial demo for Rust using the high-level (MAC etc) nrfxlib features

#![no_std]
#![no_main]

use ariel_os::debug::log::{Hex, error, info, warn};

use nrf_modem::ErrorSource;
use nrfxlib_sys;

#[ariel_os::task(autostart)]
async fn main() {
    info!("Initializing DECT MAC, trusting that Ariel OS did the basic setup");
    let mut dect = hophop::nrfxlib_mac::DectMac::create(());

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
    let find_our_network = async |r: hophop::nrfxlib_mac::ScanReceiver| {
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
                nrfxlib_sys::nrf_modem_dect_dlc_sdu_lifetime_NRF_MODEM_DECT_DLC_SDU_LIFETIME_8_S,
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

    dect.dlc_data_tx(6, params.transmitter_long_rd_id, b"hello hello")
        .await
        .unwrap();
    info!("Sent on flow 6");

    dect.dlc_data_tx(
        1,
        params.transmitter_long_rd_id,
        b"00112233445566778899aabbccddeeff",
    )
    .await
    .unwrap();
    info!("Sent on flow 1");

    // FIXME: *don't* implement an IP stack, either pass data on to embassy-net or UART via slipmux.
    // (and not going into any FIXME worthy items down here, this is clearly a quick and stupid
    // hack)
    loop {
        info!("Idling with our own very primitive ping responder");
        let mut packet = hophop::nrfxlib_mac::hack::PACKETS.receive().await;
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
        )
        .await
        .unwrap();
    }
}
