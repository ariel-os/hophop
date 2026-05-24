// SPDX-FileCopyrightText: Copyright Christian Amsüss <chrysn@fsfe.org>, Silano Systems
// SPDX-License-Identifier: MIT OR Apache-2.0
//! Runs a DECT PT and forwards IP traffic to a UART.

#![no_std]
#![no_main]

use ariel_os::log::{Hex, error, info, warn};

use ts_103_636_utils::identifiers::{LongRdId, NetworkId32};
use embedded_io_async::{Read, Write};

use nrfxlib_sys;

mod pins {
    use ariel_os::hal::{peripherals, uart};

    pub type Uart<'a> = uart::SERIAL3<'a>;
    // For nrf9151-dk VCOM0
    ariel_os::hal::define_peripherals!(UartPins {
        uart_rx: P0_26,
        uart_tx: P0_27,
    });
}

use pins::*;

#[ariel_os::task(autostart, peripherals)]
async fn main(peripherals: UartPins) {
    let mut config = ariel_os::hal::uart::Config::default();
    config.baudrate = ariel_os::uart::Baudrate::_115200;
    info!("Selected UART configuration: {:?}", config);

    let mut uart_rx_buf = [0u8; 32];
    let mut uart_tx_buf = [0u8; 32];

    let mut uart = pins::Uart::new(
        peripherals.uart_rx,
        peripherals.uart_tx,
        &mut uart_rx_buf,
        &mut uart_tx_buf,
        config,
    )
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

    /* FIXME: end duplicate */

    let gateway_long = params.transmitter_long_rd_id;

    let mut slipmux = SingleFrameDecoder::default();
    let mut decoder = slipmux::Decoder::new();

    // This is heavily inspired by hophop::nrfxlib_mac::embassy_net
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
                        dect.dlc_data_tx(1, gateway_long, data).await;
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
