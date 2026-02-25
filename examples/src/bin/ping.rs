// SPDX-FileCopyrightText: Copyright Christian Amsüss <chrysn@fsfe.org>, Silano Systems
// SPDX-License-Identifier: MIT OR Apache-2.0
//! Ping/pong example
//!
//! See <https://github.com/ariel-os/hophop/issues/12> for initial documentation.
#![no_std]
#![no_main]

use ariel_os::debug::log::{info, warn};
use ariel_os_boards::pins;

use ts_103_636_numbers as numbers;
use ts_103_636_utils as utils;

#[ariel_os::task(autostart, peripherals)]
async fn main(peripherals: pins::ButtonPeripherals) {
    let mut dect = hophop::nrfxlib_phy::DectPhy::init_after_modem_init(())
        .await
        .unwrap();

    let transmitter_id = &ariel_os::identity::interface_eui48(0).unwrap();
    let transmitter_id_short = u16::from_be_bytes(transmitter_id.0[..2].try_into().unwrap());
    let transmitter_id_long = u32::from_be_bytes(transmitter_id.0[2..].try_into().unwrap());
    info!(
        "Chosen transmitter ID: short {:?} long {:?}",
        transmitter_id_short, transmitter_id_long,
    );

    let button0 = ariel_os::gpio::Input::new(peripherals.button0, ariel_os::gpio::Pull::Up);

    loop {
        // FIXME: This is an exactly 1-second receive; that'll need API changes, but for this very
        // moment we can just leave it at that.
        if button0.is_high() {
            let received = dect
                .rx()
                .await
                .expect("Receive operation failed as a whole");

            if let Some(Ok(received)) = received {
                let start = received.pcc.time;
                let pcc = received.pcc();
                let pdc = received.pdc();
                if let Ok(pdc) = pdc {
                    let header = utils::mac_pdu::Header::parse(pdc);
                    info!("Received at {}: {:?}. PDC: {:?}", start, pcc, header);
                    if let Ok(header) = utils::mac_pdu::Header::parse(pdc)
                        && let utils::mac_pdu::MacCommonHeader::Beacon(ref beacon) = header.common
                    {
                        for ie in header.tail_items() {
                            if let Ok(ie) = ie
                                && ie.ie_number() == numbers::mac_ie::ie6bit::USER_PLANE_DATA_FLOW_1
                                && ie.payload().len() == 9
                                && ie.payload()[0] == 0x10
                            {
                                info!(
                                    "Sync received. Local ID {} time {}. Remote ID {} time {}.",
                                    transmitter_id_long,
                                    start,
                                    beacon.transmitter_address(),
                                    u64::from_be_bytes(ie.payload()[1..].try_into().unwrap())
                                );
                            }
                        }
                    }
                } else {
                    warn!(
                        "Received partial transmission: {:?} {:?} {:?}",
                        start, pcc, pdc
                    );
                }
            }
        }

        if button0.is_low() {
            #[rustfmt::skip]
            let mut pcc = [
                // header format 000, 2 subslots
                0x02,
                // short networkID
                0x41,
                // Transmitter identity, later overwritten
                0x12, 0x34,
                // Transmit power and DF MCS as in what we've seen from dect_shell beacons
                0x70,
            ];
            // Overwrite short transmitter ID
            pcc[2..4].copy_from_slice(&transmitter_id_short.to_be_bytes());
            let mut pdc_buf = heapless::Vec::<u8, 256>::new();
            // version 0, no security; beacon.
            pdc_buf.push(0x01).unwrap();
            // beacon details:
            // full network ID
            pdc_buf.extend_from_slice(&[0x41, 0x41, 0x41]).unwrap();
            // full sender ID
            pdc_buf
                .extend_from_slice(&transmitter_id_long.to_be_bytes())
                .unwrap();

            // Clock starts ticking for building the message…
            let now = dect.time_get().await.unwrap();
            // Guess: 1ms is long in MCU terms but not too long in clock drift terms.
            let transmit_time = now.wrapping_add(70000);

            // DLC PDU: type 0 (transparent mode) without routing header
            let mut userdata = [0x10, 0, 0, 0, 0, 0, 0, 0, 0];
            userdata[1..9].copy_from_slice(&transmit_time.to_be_bytes());

            // Our convention is that we transmit on data channel 1 -- setting that up or
            // multiplexing is TBD.
            utils::mac_ie::InformationElement::new_6bit_with_length(
                numbers::mac_ie::ie6bit::USER_PLANE_DATA_FLOW_1,
                &userdata,
            )
            .unwrap()
            .serialize(&mut pdc_buf)
            .unwrap();

            // For the time being we accept that the PCC (which is copy-pasted from beacon
            // messages) has a 50 byte payload, and we pad accordingly:
            //
            // ... or we just send in a subslot and then it's 17 length (1 subslot) or 33 (2
            // subslots)
            const LEN: usize = 33;

            assert!(pdc_buf.len() < LEN);
            while pdc_buf.len() < LEN {
                // Single paddings are easier than calculating back whether the rest fits in a u8
                // or u16 padding; on the long run, we'll have a "pad it up" function, or just emit
                // an indefinite-length padding, as that (juding from Wireshark) auto-fills all the
                // rest.
                utils::mac_ie::InformationElement::new_5bit(
                    numbers::mac_ie::ie5bit_len0::PADDING,
                    &[],
                )
                .unwrap()
                .serialize(&mut pdc_buf)
                .unwrap();
            }

            dect.tx(transmit_time, 1665, 0x12345678, &pcc, &pdc_buf)
                .await
                .unwrap();

            info!("Sent {} bytes PDC data", pdc_buf.len());
        }
    }
}
