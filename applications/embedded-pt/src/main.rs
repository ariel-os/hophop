// SPDX-FileCopyrightText: Copyright Christian Amsüss <chrysn@fsfe.org>, Silano Systems
// SPDX-License-Identifier: MIT OR Apache-2.0
//! Runs a DECT PT and an embedded network stack.

#![no_std]
#![no_main]

use ariel_os::log::{info, warn};
use ariel_os::time::Timer;

use ts_103_636_utils::identifiers::{LongRdId, NetworkId24, AbsoluteChannel};

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

    let config = hophop::association::PtConfiguration {
        power_save: true,
        networks: &[
            // The network pre-configured with Nordic DECT shell
            hophop::association::CandidateNetwork {
                beacon_interval_ms: 2_000,
                channels: &[AbsoluteChannel::new(1665).expect("is a channel")],
                network_id: Some(NetworkId24::new(0x876543).expect("is not 6 digits and not 0")),
                keys: Some(hophop::association::Mode1Keys {
                    // 4A7573744164656661756C7421212121 is more readable as:
                    cipher_key: b"JustAdefault!!!!",
                    integrity_key: b"JustAdefault!!!!",
                }),
            },
        ],
        rd_id: our_long_id,
    };

    hophop::nrfxlib_mac::embassy_net::run_ni6w(
        &config,
        ariel_os::net::user_net_runner().await,
        dect,
    )
    .await;
}

#[ariel_os::task(autostart)]
async fn coap_run() -> ! {
    use coap_handler_implementations::{HandlerBuilder, SimpleRendered, new_dispatcher};

    let handler = new_dispatcher().at(&["hello"], SimpleRendered("Hello from hophop"));

    ariel_os::coap::coap_run(handler).await;
}
