// SPDX-FileCopyrightText: Copyright Christian Amsüss <chrysn@fsfe.org>, Silano Systems
// SPDX-License-Identifier: MIT OR Apache-2.0
//! Initial demo for Rust using the high-level (MAC etc) nrfxlib features

#![no_std]
#![no_main]

use ariel_os::log::{Hex, error, info, warn};

use ts_103_636_utils::identifiers::{LongRdId, NetworkId32};

use nrfxlib_sys;

#[ariel_os::task(autostart)]
async fn main() {
    info!("Initializing DECT MAC, trusting that Ariel OS did the basic setup");
    let mut dect = hophop::nrfxlib_mac::DectMac::create(());

    dect.systemmode_set_mac().await;

    let our_long_id = LongRdId::new(u32::from_be_bytes(
        *ariel_os::identity::device_id_bytes()
            .expect("we know this platform to have one")
            .as_ref()
            .first_chunk()
            .expect("we know this platform has sufficiently long IDs"),
    ))
    .expect("serial numbers used with examples are not so unlucky as to start with 4 byte zeros");
    info!("Our Long RD ID is {:?}", our_long_id);

    dect.control_configure(&mut nrfxlib_sys::nrf_modem_dect_control_configure_params {
        // FIXME: Decide (this is a "let's keep it civilized" guess)
        max_tx_power: nrfxlib_sys::nrf_modem_dect_mac_tx_power_NRF_MODEM_DECT_MAC_TX_POWER_10_DB,
        // FIXME: take from hardware
        max_mcs: nrfxlib_sys::nrf_modem_dect_mac_max_mcs_NRF_MODEM_DECT_MAC_MAX_MCS_4,
        // FIXME: Decide (this is what the vendor examples default to)
        expected_mcs1_rx_rssi_level: -68,
        long_rd_id: our_long_id.into(),
        // FIXME: configure
        phy_band_group_index:
            nrfxlib_sys::nrf_modem_dect_mac_band_group_index_NRF_MODEM_DECT_MAC_PHY_BAND_GROUP_IDX0,
        // FIXME: configure
        power_save: true,
        security: nrfxlib_sys::nrf_modem_dect_control_configure_params__bindgen_ty_1 {
            // Apparently we have to decode the beacon's ciphered parts immediately rather than
            // doing that later when having read the network ID. Not great; see
            // <https://devzone.nordicsemi.com/f/nordic-q-a/128223/dect-mac-security-key-by-network>
            // for pending clarification.
            mode: nrfxlib_sys::nrf_modem_dect_mac_security_mode_NRF_MODEM_DECT_MAC_SECURITY_MODE_1,
            // Keys from the dect shell defaults ('JustAdefault!!!!')
            integrity_key: [
                0x4A, 0x75, 0x73, 0x74, 0x41, 0x64, 0x65, 0x66, 0x61, 0x75, 0x6C, 0x74, 0x21, 0x21,
                0x21, 0x21,
            ],
            cipher_key: [
                0x4A, 0x75, 0x73, 0x74, 0x41, 0x64, 0x65, 0x66, 0x61, 0x75, 0x6C, 0x74, 0x21, 0x21,
                0x21, 0x21,
            ],
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

            if params.network_id == const { NetworkId32::new(0x87654321).unwrap() } {
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
    match dect
        .mac_association(params.transmitter_long_rd_id, params.network_id)
        .await
    {
        Ok(()) => info!("Associated; continuing demo"),
        Err(_) => {
            warn!("Association didn't work, stopping program.");
            return;
        }
    }

    hophop::nrfxlib_mac::embassy_net::run_ni6w(
        &mut ariel_os::net::user_net_runner().await,
        &mut dect,
        params.transmitter_long_rd_id,
    )
    .await;
}

#[ariel_os::task(autostart)]
async fn coap_run() -> ! {
    use coap_handler_implementations::{HandlerBuilder, new_dispatcher, SimpleRendered};

    let handler = new_dispatcher()
        .at(&["hello"], SimpleRendered("Hello from hophop"));

    ariel_os::coap::coap_run(handler).await;
}
