// SPDX-FileCopyrightText: Copyright Christian Amsüss <chrysn@fsfe.org>, Silano Systems
// SPDX-License-Identifier: MIT OR Apache-2.0
//! Minimal receive example
//!
//! While this is running, peers executing the nRF dect_shell example and running `dect ping -c`
//! produce visible traffic.
#![no_std]
#![no_main]

use ariel_os::debug::log::{Hex, info, warn};

use ts_103_636_numbers as numbers;
use ts_103_636_utils as utils;

fn log_header(header: &[u8]) {
    // Following ETSI TS 103 636-4 V2.1.1 Section 6.2
    let hdr_format = header[0] >> 5;
    let packet_len = header[0] & 0x0f;
    let packet_len_units = if header[0] & 0x10 == 0 {
        "subslots"
    } else {
        "slots"
    };
    let short_nid = header[1];
    let transmitter_id = u16::from_be_bytes(header[2..4].try_into().unwrap());
    let transmit_power = header[4] >> 4;
    // Weird enough, in the 5-byte header the MSB of this is reserved, but we can still
    // decode it this way:
    let df_mcs = header[4] & 0x0f;

    match header.len() {
        5 => {
            info!(
                "Header details: format {} length {} {}, nid {}, from {}, tx power {}, df_mcs {}",
                hdr_format,
                packet_len,
                packet_len_units,
                short_nid,
                transmitter_id,
                transmit_power,
                df_mcs
            );
        }
        10 => {
            let receiver_id = u16::from_be_bytes(header[5..7].try_into().unwrap());
            // Ignoring remaining feedback info for the moment; its interpretation depends on hdr_format
            // (although that really only tells if that's reserved or used).
            info!(
                "Header details: format {} length {} {}, nid {}, from {} to {}, tx power {}, df_mcs {}",
                hdr_format,
                packet_len,
                packet_len_units,
                short_nid,
                transmitter_id,
                receiver_id,
                transmit_power,
                df_mcs
            );
        }
        _ => unreachable!("Header length is always 5 or 10"),
    }
}

#[ariel_os::task(autostart)]
async fn main() {
    let mut dect = hophop::nrfxlib_phy::DectPhy::init_after_modem_init(())
    .await
    .unwrap();

    for _ in 0..300 {
        if let Some(received) = dect
            .rx()
            .await
            .expect("Receive operation failed as a whole")
        {
            if let Ok(received) = received {
                let start = received.pcc.time;
                let pcc = received.pcc();
                match received.pdc() {
                    Ok(pdc) => {
                        info!("Received at {}: {:?} {:?}", start, pcc, pdc);
                        log_header(pcc);
                        info!("PDC: {:?}", utils::mac_pdu::Header::parse(pdc));
                    },
                    Err(pdc) => {
                        info!("Received at {}: {:?} but data was not received: {:?}", start, pcc, pdc);
                        log_header(pcc);
                    }
                }
            } else {
                warn!(
                    "Received partial transmission",
                );
            }
        }
    }

    panic!(
        "If we want to be able to re-flash, we better things at some point to avoid going through unlock again."
    );
}
