// SPDX-FileCopyrightText: Copyright Christian Amsüss <chrysn@fsfe.org>, Silano Systems
// SPDX-License-Identifier: MIT OR Apache-2.0
//! Runs in RX mode and sends data out via slipmux

#![no_std]
#![no_main]

use ariel_os::log::{info, warn};

use ts_103_636_utils::identifiers::{AbsoluteChannel, NetworkId32};

use ariel_os::coap::{self, IP2SLIPMUX, SLIPMUX2IP};

#[ariel_os::task(autostart)]
async fn main() {
    let mut dect = hophop::nrfxlib_phy::DectPhy::init_after_modem_init(())
        .await
        .unwrap();

    // Buffer through which we send data to slipmux
    use static_cell::ConstStaticCell;
    static BUFFER: ConstStaticCell<ariel_os::coap::SingleBuffer> =
        ConstStaticCell::new(ariel_os::coap::SingleBuffer::new());
    // Just so that it's in there and we can do the regular thing of "take it back, write to it,
    // put it in again" in the loop.
    IP2SLIPMUX.signal(BUFFER.take());

    sniff(dect).await;
}

async fn sniff(mut dect: hophop::nrfxlib_phy::DectPhy) {
    loop {
        dect.rssi_bulk(
            // an array of 10 gives us about 100ms of continous capture; slow enough that we don't
            // miss too much at the edges, fast enough that we can switch around quickly.
            &[const { AbsoluteChannel::new(1665).unwrap() }; 10],
            const { NetworkId32::new(0x87654321).unwrap() },
            async |event| {
                // FIXME: We'll want to have this at some point, but right now, it's using too much
                // of the 115200 default UART link.
                #[cfg(false)]
                if let Some(event) = event {
                    // we don't block for this, making RSSI reports kind of lower priority
                    if let Some(buffer) = SLIPMUX2IP.try_take() {
                        buffer.extend_from_slice(&[0x60, 0x00, 0x00]);
                        IP2SLIPMUX.signal(buffer);
                    }
                } else {
                    warn!("RSSI pool overflow");
                }
            },
            async |receive| match receive {
                Ok(recv) => send(recv.pcc(), recv.pdc().unwrap_or(&[])).await.unwrap(),
                Err(pcc_error) => warn!("Can not express PCC error for Wireshark"),
            },
        )
        .await
        .unwrap();
    }
}

#[ariel_os::task(autostart)]
async fn coap() {
    coap::coap_run(coap_handler_implementations::new_dispatcher()).await;
}

async fn send(pcc: &[u8], pdc: &[u8]) -> Result<(), heapless::CapacityError> {
    // If this actually awaits, we might miss a transmission
    let buffer = SLIPMUX2IP.wait().await;

    buffer.clear();

    let length: u16 = 10 + u16::try_from(pdc.len()).unwrap();

    // Building a packet out from raw here, because through Jelly it's easy to get it
    // into a Linux tun interface on which we can sniff.
    buffer.extend_from_slice(&[0x60, 0x00, 0x00, 0x00])?;
    // payload length
    buffer.extend_from_slice(&(length + 8).to_be_bytes())?;
    // next header: UDP; hop limit 1
    buffer.extend_from_slice(&[17, 1])?;
    // source address
    buffer.extend_from_slice(&[0; 16])?;
    // destination: link-local all nodes
    buffer.extend_from_slice(&[0xff, 0x02, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1])?;
    // UDP: source port, dest port
    buffer.extend_from_slice(&[0xb1, 0x07, 0xb1, 0x07])?;
    // UDP length
    buffer.extend_from_slice(&(length + 8).to_be_bytes())?;
    // Checksum: ignoring
    buffer.extend_from_slice(&[0, 0])?;

    buffer.extend_from_slice(pcc)?;
    match pcc.len() {
        5 => buffer.extend_from_slice(&[0; 5])?,
        10 => (),
        _ => unreachable!(),
    }

    buffer.extend_from_slice(pdc)?;

    IP2SLIPMUX.signal(buffer);

    Ok(())
}
