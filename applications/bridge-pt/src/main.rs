// SPDX-FileCopyrightText: Copyright Christian Amsüss <chrysn@fsfe.org>, Silano Systems
// SPDX-License-Identifier: MIT OR Apache-2.0
//! Runs a DECT PT and forwards IP traffic to a UART.

#![no_std]
#![no_main]

use ariel_os::log::{Hex, error, info, warn};

use ts_103_636_utils::identifiers::{LongRdId, NetworkId32};
use embedded_io_async::{Read, Write};

use nrfxlib_sys;

type UartPeripherals = ariel_os_boards::pins::HOST_FACING_UART;

#[ariel_os::task(autostart, peripherals)]
async fn main(peripherals: UartPeripherals) {
    let mut config = ariel_os::hal::uart::Config::default();
    config.baudrate = ariel_os::uart::Baudrate::_115200;
    info!("Selected UART configuration: {:?}", config);

    let mut uart_rx_buf = [0u8; 32];
    let mut uart_tx_buf = [0u8; 32];

    let mut uart = peripherals
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
        networks: const { &[
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
        ] },
        rd_id: our_long_id,
    };

    /* end duplicate */

    let dlc_tx = embassy_sync::channel::Channel::new();
    let config_poke = embassy_sync::channel::Channel::new();

    let mut on_rx = |dlc_data_rx: crate::nrfxlib_mac::DlcDataRx| {
//         if let Some(rx_buf) = rx_runner.try_rx_buf() {
//             let len = dlc_data_rx.data().len();
//             rx_buf[..len].copy_from_slice(dlc_data_rx.data());
//             rx_runner.rx_done(len);
//         } else {
//             warn!("Dropping packet -- overflow");
//         }
    };

    let mut on_assoc_change = |assoc: Option<crate::association::Association>| {
//         info!("Learned about association: {}", assoc);
//         if let Some(assoc) = assoc {
//             current_assoc.set(Some(assoc.parent));
//             control_hub.association.set(Some(assoc));
//             state_runner.set_link_state(LinkState::Up);
//         } else {
//             current_assoc.set(None);
//             control_hub.association.set(None);
//             state_runner.set_link_state(LinkState::Down);
//         }
    };

    let running_dect = crate::runner::Stack::new(dect).run(
        time,
        config,
        dlc_tx.receiver(),
        config_poke.receiver(),
        &mut on_rx,
        &mut on_assoc_change,
    );

    let gateway_long = params.transmitter_long_rd_id;

    let mut slipmux = SingleFrameDecoder::default();
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
            dect.dlc_data_rx(),
            uart.read(&mut uart_app_buffer),
        )
        .await
        {
            Either::First(from_network) => {
                let mut encoder = slipmux::ChunkedEncoder::new(slipmux::FrameType::Ip, from_network.data());
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
                        let (Ok(buffer) | Err(buffer)) = slipmux.data();
                        let text = core::str::from_utf8(buffer);
                        warn!(
                            "Peer sent diagnostic data. This will no be forwarded; content was {:?}{}",
                            text.map_err(|_| &buffer),
                            if slipmux.data().is_err() { "..." } else { "" },
                        );
                    }
                    Ok(DecodeStatus::FrameCompleteIp) => {
                        let Ok(data) = slipmux.data() else {
                            warn!("Frame overflew slipmux buffer, won't relay it.");
                            continue;
                        };
                        // FIXME count errors
                        let _ = dect.dlc_data_tx(1, gateway_long, data).await;
                    }
                    Ok(DecodeStatus::FrameCompleteConfiguration) => {
                        warn!("Peer sent CoAP data {}, which is unsupported.", Hex(slipmux.data().unwrap()));
                    }
                }
            }
        }
    }
}

// FIXME: copied unmodified from https://github.com/ariel-os/ariel-os/pull/1613

struct SingleFrameDecoder {
    // See https://github.com/t2trg/slipmux/issues/1 for expectations on how big this should be
    buffer: heapless::Vec<u8, 1280>,
    overflow: bool,
}
impl SingleFrameDecoder {
    /// Returns the decoded data if complete.
    ///
    /// # Errors
    ///
    /// If the buffer overflew, it returns the initial decoded bytes.
    fn data(&self) -> Result<&[u8], &[u8]> {
        if self.overflow {
            Err(&self.buffer)
        } else {
            Ok(&self.buffer)
        }
    }
}
impl Default for SingleFrameDecoder {
    fn default() -> Self {
        SingleFrameDecoder {
            buffer: Default::default(),
            overflow: false,
        }
    }
}
impl slipmux::FrameHandler for SingleFrameDecoder {
    fn begin_frame(&mut self, _: slipmux::FrameType) {
        self.buffer.clear();
        self.overflow = false;
    }
    fn write_byte(&mut self, byte: u8) {
        if self.buffer.push(byte).is_err() {
            self.overflow = true;
        }
    }
    fn end_frame(&mut self, _: Option<slipmux::Error>) {}
}
