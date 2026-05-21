// SPDX-FileCopyrightText: Copyright Christian Amsüss <chrysn@fsfe.org>, Silano Systems
// SPDX-License-Identifier: MIT OR Apache-2.0
//! Glue between the MAC and the [`embassy-net-driver`]
//!
//! This is compatible with the DECT shell, but not necessarily proper IPv6 over nr+ (ETSI TS 103
//! 874-3). Implementing this for the time being, but
//! <https://devzone.nordicsemi.com/f/nordic-q-a/128194/dect-shell-ipv6-and-etsi-ts-103-874-3> is
//! pending to clarify whether we're just missing .

use defmt::warn;

/// An embassy network driver that transmits and receives packets via the Nordic nrfxlib MAC.
///
/// This is implemented in terms of [`embassy_net_driver_channel`], and thus takes a
/// [Runner][embassy_net_driver_channel::Runner].
///
/// FIXME: `gateway_long` is the long RD address of the FT; in a sense, the MAC address of the
/// default gateway. We shouldn't need that on the long run.
pub async fn run<'d, const MTU: usize>(
    runner: &mut embassy_net_driver_channel::Runner<'d, MTU>,
    dect: &mut super::DectMac,
    gateway_long: u32,
) {
    use embassy_net_driver::LinkState;

    runner.set_link_state(LinkState::Up);

    loop {
        use embassy_futures::select::{Either, select};

        match select(
            // of all the dect functions, this one fortunately is already cancel safe
            dect.dlc_data_rx(),
            runner.tx_buf(),
        )
        .await
        {
            Either::First(received) => {
                if let Some(rx_buf) = runner.try_rx_buf() {
                    let len = received.data().len();
                    rx_buf[..len].copy_from_slice(received.data());
                    runner.rx_done(len);
                } else {
                    warn!("Dropping packet -- overflow");
                }
            }
            Either::Second(tx_buf) => {
                // It's a network driver, we can't do anything about lost packets.
                let _ = dect
                    .dlc_data_tx(
                        1,
                        // BIG FIXME -- probably the solution will be to pretend to be MAC or 802154
                        // and cram these MAC addresses into their fields (but we'd have to parse them
                        // out and put them into the buffer, right)?
                        gateway_long,
                        tx_buf,
                    )
                    .await;
                // FIXME: Actually we don't have to await the dlc_data_tx to mark it as done
                runner.tx_done();
            }
        }
    }
}
