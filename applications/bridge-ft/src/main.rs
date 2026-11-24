// SPDX-FileCopyrightText: Copyright Christian Amsüss <chrysn@fsfe.org>, Silano Systems
// SPDX-License-Identifier: MIT OR Apache-2.0
//! Runs a DECT FT and forwards IP traffic to a UART.

#![no_std]
#![no_main]

use ariel_os::log::{debug, info, warn};

use ts_103_636_utils::identifiers::{AbsoluteChannel, LongRdId};

use hophop::nrfxlib_mac::{DectMac, Ipv6Config};
use nrfxlib_sys_pretty::{dect_control, dect_mac};

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

    info!(
        "Please set this IPv6 address on the host: {}",
        our_network.address_of(our_long_id, our_long_id)
    );

    ariel_os::asynch::spawner().spawn(coap_task().unwrap());

    // A very stupid ad-hoc buffer pool that works for sending around &'static mut items.
    //
    // (Doing heapless ObjectBlock or BoxBlock would not quite work because we really need to send
    // &'static mut, which is a shortcoming of the current SLIPMUX2IP mechanism, and also,
    // initializing it from ConstStaticCell has Send bound trouble).
    const EMPTY_BLOCK: ariel_os::coap::SingleBuffer = ariel_os::coap::SingleBuffer::new();
    static BUFFERS: static_cell::ConstStaticCell<[ariel_os::coap::SingleBuffer; 8]> =
        static_cell::ConstStaticCell::new([EMPTY_BLOCK; 8]);
    let buffers: &mut [_; 8] = BUFFERS.take();
    let buffer_pointers: [&mut _; 8] = buffers.each_mut();
    // Initially just all our owned buffers, but we gladly also take up other parties' buffers.
    // As long as they are stored in here, they are to be empty.
    let mut buffer_pool = heapless::Vec::<_, 8>::from(buffer_pointers);

    #[derive(PartialEq, Eq)]
    enum Source {
        Radio,
        Slipmux,
    }

    loop {
        // Very quick and very dirty non-border router -- if a peer doesn't address something in
        // such a way that DLC can forward it, we have to
        let (source, buffer) = match embassy_futures::select::select(
            dlc_rx.dlc_data_rx(),
            ariel_os::coap::SLIPMUX2IP.wait(),
        )
        .await
        {
            embassy_futures::select::Either::First(rx) => {
                let data = rx.data();

                let buffer = if let Some(buffer) = buffer_pool.pop() {
                    if let Ok(_) = buffer.extend_from_slice(data) {
                        buffer
                    } else {
                        warn!("Not forwarding packet through SLIPMUX: packet too big");
                        // This goes away once we just use xarxa pools everywhere
                        buffer_pool.push(buffer).unwrap();
                        continue;
                    }
                } else {
                    warn!("Not forwarding packet through SLIPMUX: buffer not available");
                    continue;
                };

                (Source::Radio, buffer)
            }
            embassy_futures::select::Either::Second(inbuf) => {
                if inbuf.is_empty() {
                    buffer_pool
                        .push(inbuf)
                        .expect("why else would slip send us one");
                    continue;
                }
                (Source::Slipmux, inbuf)
            }
        };

        let buffer = core::mem::DropGuard::new(buffer, |buffer| {
            buffer.clear();
            if let Err(overflown) = buffer_pool.push(buffer) {
                ariel_os::coap::IP2SLIPMUX.signal(overflown);
            }
        });

        // Dissect IP, and decrement hop limit early (before we forget)

        if *buffer == &hophop::ni6w::KEEPALIVE_MESSAGE {
            debug!("Received keepalive message");
            continue;
        }

        let Some(dest_addr) = buffer.get(24..).and_then(|s| s.first_chunk()) else {
            warn!("DLC data too short to be IP");
            continue;
        };
        let dest_addr = core::net::Ipv6Addr::from_octets(*dest_addr);

        /* Decrementing the hop limit would be a nice touch, but that'd kill at least ff02::1
         * pings, and probably also causes troule if any connected peer sets a hop limit of 1,
         * which would be perfectly reasonable.
        let new_hoplimit = buffer[7].saturating_sub(1);
        if new_hoplimit == 0 {
            warn!("Discarding packet that underflew hop limit");
            continue;
        }
        buffer[7] = new_hoplimit;
        */

        // Routing decisions:

        if dest_addr.is_multicast() {
            warn!("FIXME: Send to multicast on all but incoming interface");
            /* Tempting to do it this way, but doesn't work that way for two reasons:
             * - packets coming in multicast from the radio need to be distributed again
             * - DLC layer says "INVALID_PARAM" when attempting to send to broadcast
             *
             * possible workarounds include unicasting this to all peers.
            if source == Source::Slipmux {
                match dlc_tx.dlc_data_tx(1, LongRdId::BROADCAST, &buffer).await {
                    Ok(()) => debug!("Multicast packet forwarded successfully."),
                    Err(e) => warn!("Unhandled error forwarding multicast packet: {:?}", e),
                }
            } else {
                let buffer = core::mem::DropGuard::dismiss(buffer);
                ariel_os::coap::IP2SLIPMUX.signal(buffer);
            }
            */
        } else {
            let mac_dest = our_network.matches_and_extract(&our_long_id, &dest_addr);

            if let Some(dest) = mac_dest
                && dest != our_long_id
            {
                match dlc_tx.dlc_data_tx(1, dest, &buffer).await {
                    Ok(()) => debug!("Packet forwarded successfully to {:?}", dest,),
                    Err(e) if e.is_rd_not_found() => {
                        warn!(
                            "Failed to forward packet to {:?}; FIXME: Send ICMP (likely type 1 Destination Unreachable, Code 3 Address Unreachable), possibly with rate limiting",
                            dest
                        );
                    }
                    Err(e) => warn!("Unhandled error forwarding packet to {:?}: {:?}", dest, e),
                }
            } else {
                if source == Source::Slipmux {
                    warn!(
                        "Slipmux sent packet with destination outside our routing information, ignoring it."
                    );
                    continue;
                } else {
                    let buffer = core::mem::DropGuard::dismiss(buffer);

                    // Note that we're *not* implementing promisc mode here: Linux on tun appears to
                    // just return link-local packets.
                    ariel_os::coap::IP2SLIPMUX.signal(buffer);
                }
            };
        }
    }
}

#[ariel_os::task()]
async fn coap_task() {
    use coap_handler_implementations::*;
    let handler = new_dispatcher().with_wkc();
    ariel_os::coap::coap_run(handler).await;
}
