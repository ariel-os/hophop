// SPDX-FileCopyrightText: Copyright Christian Amsüss <chrysn@fsfe.org>, Silano Systems
// SPDX-License-Identifier: MIT OR Apache-2.0
//! Runs a DECT PT and forwards IP traffic to a UART.

#![no_std]
#![no_main]

use ariel_os::log::{info, warn};

use embedded_io_async::{Read, Write};
use ts_103_636_utils::identifiers::{AbsoluteChannel, LongRdId, NetworkId24};

type UartPeripherals = ariel_os_boards::pins::HOST_FACING_UART;

#[ariel_os::task(autostart, peripherals)]
async fn main(peripherals: UartPeripherals) {
    let mut config = ariel_os::hal::uart::Config::default();
    config.baudrate = ariel_os::uart::Baudrate::_115200;
    info!("Selected UART configuration: {:?}", config);

    let mut uart_rx_buf = [0u8; 32];
    let mut uart_tx_buf = [0u8; 32];

    let uart = peripherals
        .build_with_config(&mut uart_rx_buf, &mut uart_tx_buf, config)
        .expect("Invalid UART configuration");

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

    // FIXME: will control_hub need a serving function that's also part of the parallel join?
    // Likely. But if not, implement MacController on it rather than &'a it.
    let slipmux_subtask = run_slipmux(uart, &control_hub);

    /* FIXME mostly duplicate again */

    embassy_futures::join::join3(
        hophop::runner::Stack::new(dect).run(ariel_os::time::Delay, &mut &control_hub),
        slipmux_subtask,
        // FIXME: Should this get its dedicated task? That'd give it its own waker, but then we
        // can't just use the reference.
        async {
            /*
            use coap_handler_implementations::{HandlerBuilder, SimpleRendered, new_dispatcher};

            let handler = new_dispatcher()
                .at(&["hello"], SimpleRendered("Hello from hophop"))
                .below(&["d"], control_hub.coap_server());
            ariel_os::coap::coap_run(handler).await;
            */
        },
    )
    .await
    .0
}

// This has parallels with the Ni6wNetworkController
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
    dect_to_slip_channel: DectToSlipChannel,
    slip_to_dect_channel: SlipToDectChannel,
}

impl DirectToSlipmuxController {
    fn new(static_config: hophop::association::PtConfiguration<'static>) -> Self {
        Self {
            static_config,
            association: Default::default(),
            poke: embassy_sync::channel::Channel::new(),
            dect_to_slip_channel: DectToSlipChannel::new(),
            slip_to_dect_channel: SlipToDectChannel::new(),
        }
    }
}

type DectToSlipChannel = embassy_sync::channel::Channel<
    embassy_sync::blocking_mutex::raw::CriticalSectionRawMutex,
    hophop::nrfxlib_mac::DlcDataRx,
    // Senders won't wait, let's not starve on UART slowness
    // FIXME: reuse xarxa buffers
    3,
>;
type SlipToDectChannel = embassy_sync::channel::Channel<
    embassy_sync::blocking_mutex::raw::CriticalSectionRawMutex,
    hophop::runner::DlcTxItem,
    // Senders can wait
    // FIXME: reuse xarxa buffers
    1,
>;

impl<'a> hophop::runner::MacController for &'a DirectToSlipmuxController {
    fn on_dlc_rx(&mut self, data: hophop::nrfxlib_mac::DlcDataRx) {
        if self.dect_to_slip_channel.try_send(data).is_err() {
            warn!("Dropping packet -- overflow");
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
        self.slip_to_dect_channel.receive()
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

async fn run_slipmux(
    mut uart: ariel_os::hal::uart::Uart<'_>,
    controller: &DirectToSlipmuxController,
) {
    let mut slipmux = slipmux::SingleBufferHandler::<heapless::Vec::<u8, 1024>>::default();
    let mut decoder = slipmux::Decoder::new();

    // This is heavily inspired by hophop::embassy_net

    loop {
        use embassy_futures::select::{Either, select};
        use slipmux::DecodeStatus;

        // FIXME: This is terribly inefficient;
        // https://github.com/ariel-os/ariel-os/pull/1613/changes#diff-855557691a40b0f2e1a49f8392c9267cee2cd341108ed30a8ea024aae77dd646
        // has remarks on how to use BufRead for more efficient reading, but until then, we just
        // eat the performance penalty.
        let mut uart_app_buffer = [0];

        match select(
            // of all the dect functions, this one fortunately is already cancel safe
            controller.dect_to_slip_channel.receive(),
            uart.read(&mut uart_app_buffer),
        )
        .await
        {
            Either::First(from_network) => {
                let mut encoder =
                    slipmux::ChunkedEncoder::new(slipmux::FrameType::Ip, from_network.data());
                loop {
                    let mut outbuf = [0; 32];
                    let size = encoder.encode_chunk(&mut outbuf);
                    if size == 0 {
                        break;
                    }
                    uart.write_all(&outbuf[..size]).await.unwrap();
                }
            }
            Either::Second(from_uart) => {
                let Ok(1) = from_uart else {
                    warn!("Error reading from UART");
                    continue;
                };
                let byte = uart_app_buffer[0];
                match decoder.decode(byte, &mut slipmux) {
                    Err(_) => {
                        warn!("Decoding error; trying at the next byte.");
                    }
                    Ok(DecodeStatus::Incomplete) => {
                        // no action needed
                    }
                    Ok(DecodeStatus::FrameCompleteDiagnostic) => {
                        // Use up to the cursor, and silently ignore overflows.
                        let (buffer, error) = slipmux.take_diagnostic_and_error();
                        let text = core::str::from_utf8(&buffer);
                        warn!(
                            "Peer sent diagnostic data. This will no be forwarded; content was {:?}{}",
                            text.map_err(|_| &buffer),
                            if error.is_some() { "..." } else { "" },
                        );
                    }
                    Ok(DecodeStatus::FrameCompleteIp) => {
                        let (buffer, error) = slipmux.take_ip_and_error();
                        if error.is_some() {
                            warn!("Frame overflew slipmux buffer, won't relay it.");
                            continue;
                        };
                        // FIXME: do we have the backpressure to block receiving, or should we
                        // rather drop it here? Or send ECN if so signalled?
                        if let Some(association) = controller.association.get() {
                            controller
                                .slip_to_dect_channel
                                .send((
                                    1,
                                    association.parent,
                                    buffer,
                                ))
                                .await;
                        } else {
                            // FIXME counting
                            warn!("Dropping packet because currently not associated")
                        }
                    }
                    Ok(DecodeStatus::FrameCompleteConfiguration) => {
                        warn!("Peer sent CoAP data, which is unsupported.");
                    }
                }
            }
        }
    }
}
