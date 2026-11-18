// SPDX-FileCopyrightText: Copyright Christian Amsüss <chrysn@fsfe.org>, Silano Systems
// SPDX-License-Identifier: MIT OR Apache-2.0
//! Runs a DECT FT and forwards IP traffic to a UART.

#![no_std]
#![no_main]

use ariel_os::log::{info, warn};

use ts_103_636_utils::identifiers::{AbsoluteChannel, LongRdId};

use hophop::nrfxlib_mac::{DectMac, Ipv6Config};
use nrfxlib_sys_pretty::{dect_control, dect_dlc, dect_mac};

#[ariel_os::task(autostart)]
async fn main() {
    /* FIXME: duplicate from ../embedded-pt/ */
    info!("Initializing DECT MAC, trusting that Ariel OS did the basic setup");
    let (mut dect, mut dlc_tx, mut dlc_rx) = DectMac::create(());

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

    /* end duplicate */

    dect.control_functional_mode_set_deactivate().await;

    let keys = Some(hophop::association::Mode1Keys {
        // 4A7573744164656661756C7421212121 is more readable as:
        cipher_key: *b"JustAdefault!!!!",
        integrity_key: *b"JustAdefault!!!!",
    });
    let security = if let Some(hophop::association::Mode1Keys {
        integrity_key,
        cipher_key,
    }) = keys
    {
        dect_control::configure_params_security {
            mode: dect_mac::security_mode_values::_1,
            integrity_key: integrity_key,
            cipher_key: cipher_key,
        }
    } else {
        dect_control::configure_params_security {
            mode: dect_mac::security_mode_values::NONE,
            integrity_key: [0; _],
            cipher_key: [0; _],
        }
    };

    dect.control_configure(&mut dect_control::configure_params {
        // The maximum; we'll leave it to the network to lower the limits.
        // FIXME: take from hardware (although here it's the the standard's limit, but maybe we
        // have to adjust to a lower one from the hardware?)
        // FIXME why does this fail when we put in 23_DB?
        max_tx_power: dect_mac::tx_power_values::_10_DB,
        // FIXME: take from hardware
        max_mcs: dect_mac::max_mcs_values::_4,
        // FIXME: Decide (this is what the vendor examples default to)
        expected_mcs1_rx_rssi_level: -68,
        // FIXME: could we decide that later? (probably not: we come in with som expectation of
        // the network already)
        long_rd_id: our_long_id.into(),
        // FIXME: configure
        phy_band_group_index: dect_mac::band_group_index_values::IDX0,
        power_save: false,
        security,
        // FIXME: Decide (this is what the vendor uses in their examples)
        stats_averaging_length: 2,
    })
    .await;

    dect.control_functional_mode_set_activate().await;

    info!("Starting pro forma RSSI scan");
    // -71dBm..-85dBm and 200ms: defaults from dects_hell
    dect.rssi_scan(200, -85, -71, &[AbsoluteChannel::new(1665).unwrap()])
        .await
        .unwrap();
    info!(
        "Scan complete. Please look at previous debug to assess channel quality; continuing with the selected channel anyway."
    );

    // Pick one by a process similar to how eMuhdra picks their secret keys
    let channel = AbsoluteChannel::new(1665).unwrap();
    let our_network = Ipv6Config::LinkLocal;
    info!("Continuing with channel {}", channel);

    dect.cluster_configure(channel, our_network).await.unwrap();
    info!("Running cluster.");

    ariel_os::asynch::spawner().spawn(coap_task().unwrap());

    loop {
        // Very quick and very dirty non-border router -- if a peer doesn't address something in
        // such a way that DLC can forward it, we have to
        let rx = &dlc_rx.dlc_data_rx().await;
        let data = rx.data();

        let Some(dest_addr) = data.get(24..).and_then(|s| s.first_chunk()) else {
            warn!("DLC data too short to be IP");
            continue;
        };
        let dest_addr = core::net::Ipv6Addr::from_octets(*dest_addr);
        let Some(dest) = our_network.matches_and_extract(&our_long_id, &dest_addr) else {
            warn!(
                "Received DLC from {} on {} but it's not to be forwarded here: dest addr is {}",
                rx.sender(),
                rx.flow_id(),
                dest_addr,
            );
            continue;
        };

        // This is needless copying, for we could also make DlcDataRx destructible into such a Vec,
        // but we're .
        let mut data_out = heapless::Vec::<u8, 1501>::from_slice(&data).expect("Sizes align");

        let new_hoplimit = data_out[7].saturating_sub(1);
        if new_hoplimit == 0 {
            warn!("Discarding packet that underflew hop limit");
            continue;
        }
        data_out[7] = new_hoplimit;

        match dlc_tx.dlc_data_tx(rx.flow_id(), dest, &data_out).await {
            Ok(()) => info!(
                "Forwarded packet to {:?} with hop limit {}",
                dest, new_hoplimit
            ),
            Err(e) if e.is_rd_not_found() => warn!(
                "Failed to forward packet to {:?}; FIXME: Send ICMP (likely type 1 Destination Unreachable, Code 3 Address Unreachable), possibly with rate limiting",
                dest
            ),
            Err(e) => warn!("Unhandled error forwarding packet to {:?}: {:?}", dest, e),
        }
    }
}

#[ariel_os::task()]
async fn coap_task() {
    use coap_handler_implementations::*;
    let handler = new_dispatcher().with_wkc();
    ariel_os::coap::coap_run(handler).await;
}
