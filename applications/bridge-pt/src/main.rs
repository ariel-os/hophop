// SPDX-FileCopyrightText: Copyright Christian Amsüss <chrysn@fsfe.org>, Silano Systems
// SPDX-License-Identifier: MIT OR Apache-2.0
//! Runs a DECT PT and forwards IP traffic to a UART.

#![no_std]
#![no_main]

use ariel_os::log::{info, warn};

use ts_103_636_utils::identifiers::{AbsoluteChannel, LongRdId, NetworkId24};

#[ariel_os::task(autostart)]
async fn main() {
    /* FIXME: duplicate from ../embedded-pt/ */
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
        networks: const {
            &[
                // The network pre-configured with Nordic DECT shell
                hophop::association::CandidateNetwork {
                    beacon_interval_ms: 2_000,
                    channels: &[AbsoluteChannel::new(1665).expect("is a channel")],
                    network_id: Some(
                        NetworkId24::new(0x876543).expect("is not 6 digits and not 0"),
                    ),
                    keys: Some(hophop::association::Mode1Keys {
                        // 4A7573744164656661756C7421212121 is more readable as:
                        cipher_key: b"JustAdefault!!!!",
                        integrity_key: b"JustAdefault!!!!",
                    }),
                },
            ]
        },
        rd_id: our_long_id,
    };

    /* end duplicate */

    let control_hub = DirectToSlipmuxController::new(config);

    use coap_handler_implementations::*;
    let handler = new_dispatcher()
        .at(&["hello"], SimpleRendered("Hello from hophop"))
        // FIXME generalize so it works here as well
        .below(&["d"], control_hub.coap_server())
        // FIXME: This, as well as security, would be handled better by the Ariel OS CoAP server,
        // but right now we're sidestepping that; see laze-project.yml comment
        .with_wkc();

    embassy_futures::join::join(
        hophop::runner::Stack::new(dect).run(ariel_os::time::Delay, &mut &control_hub),
        ariel_os::coap::coap_run(handler),
    )
    .await
    .0
}

// This has parallels with the Ni6wNetworkController, but goes into the SLIPMUX2IP/IP2SLIPMUX
// controls.
struct DirectToSlipmuxController {
    static_config: hophop::association::PtConfiguration<'static>,
    association: core::cell::Cell<Option<hophop::association::Association>>,
    // FIXME: Currently, nothing sends into that
    poke: embassy_sync::channel::Channel<
        embassy_sync::blocking_mutex::raw::CriticalSectionRawMutex,
        hophop::runner::Poke,
        // Senders can just wait, or use fallible pushing
        1,
    >,
    buffer: core::cell::Cell<Option<&'static mut ariel_os::coap::SingleBuffer>>,
}

impl DirectToSlipmuxController {
    fn new(static_config: hophop::association::PtConfiguration<'static>) -> Self {
        use static_cell::ConstStaticCell;
        static BUFFER: ConstStaticCell<ariel_os::coap::SingleBuffer> =
            ConstStaticCell::new(ariel_os::coap::SingleBuffer::new());
        let buffer = Some(BUFFER.take()).into();

        Self {
            static_config,
            association: Default::default(),
            poke: embassy_sync::channel::Channel::new(),
            buffer,
        }
    }

    // FIXME copied and adjusted from hophop::embassy_net
    /// Builds a CoAP sub-tree (suitable for use with `.below()`)
    pub fn coap_server(&self) -> impl coap_handler::Handler + coap_handler::Reporting {
        use coap_handler_implementations::*;
        use coap_message_utils::Error;

        struct ServeStatus<'a>(&'a DirectToSlipmuxController);

        impl GetRenderable for ServeStatus<'_> {
            type Get = Option<hophop::association::Association>;

            fn get(&mut self) -> Result<Self::Get, Error> {
                Ok(self.0.association.get())
            }
        }

        wkc::ConstantSingleRecordReport::new(
            TypeHandler::new_minicbor_2(with_get(ServeStatus(self))),
            &[coap_handler::Attribute::ResourceType(
                "tag:ariel-os.org,2026:experimental-hophop",
            )],
        )
    }
}

impl<'a> hophop::runner::MacController for &'a DirectToSlipmuxController {
    fn on_dlc_rx(&mut self, data: hophop::nrfxlib_mac::DlcDataRx) {
        let Some(buffer) = self.buffer.take() else {
            warn!("Dropping packet -- overflow");
            return;
        };
        buffer.clear();
        if buffer.extend_from_slice(data.data()).is_ok() {
            ariel_os::coap::SLIPMUX2IP.signal(buffer);
        } else {
            warn!("Packet too long");
            self.buffer.set(Some(buffer));
        }
    }

    fn on_assoc_change(&mut self, assoc: Option<hophop::association::Association>) {
        info!("Learned about association: {:?}", assoc);

        // FIXME can we signal that over SLIP, or do we rely on the CoAP server to pick it up?
        if let Some(assoc) = assoc {
            self.association.set(Some(assoc));
        } else {
            self.association.set(None);
        }
    }

    fn next_dlc_tx(
        &mut self,
    ) -> impl core::future::Future<Output = hophop::runner::DlcTxItem> + use<'a> {
        async {
            let mut received;
            loop {
                received = ariel_os::coap::SLIPMUX2IP.wait().await;
                if received.is_empty() {
                    // just coming back from on_dlc_rx
                    assert!(self.buffer.take().is_none());
                    self.buffer.set(Some(received));
                    continue;
                }
                let to_send = if let Some(association) = self.association.get() {
                    let copied = heapless::vec::Vec::try_from(received.as_slice());
                    if let Ok(copied) = copied {
                        Some((1, association.parent, copied))
                    } else {
                        warn!("Dropping overly long packet");
                        None
                    }
                } else {
                    // FIXME counting
                    warn!("Dropping packet because currently not associated");
                    None
                };
                received.clear();
                ariel_os::coap::IP2SLIPMUX.signal(received);
                if let Some(to_send) = to_send {
                    return to_send;
                }
            }
        }
    }

    // FIXME duplicate from hophop::embassy_net
    fn next_config(
        &mut self,
    ) -> impl core::future::Future<Output = hophop::association::PtConfiguration<'static>> + use<'a>
    {
        // FIXME: This relies on the consumer just querying it once; on the long run, we must be
        // sure to always return Pending after the first emission, or already forward changing
        // configurations.

        let static_config = self.static_config.clone();

        async move { static_config }
    }

    fn next_action(
        &mut self,
    ) -> impl core::future::Future<Output = hophop::runner::Poke> + use<'a> {
        let receiver = self.poke.receiver();
        async move { receiver.receive().await }
    }
}
