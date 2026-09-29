// SPDX-FileCopyrightText: Copyright Christian Amsüss <chrysn@fsfe.org>, Silano Systems
// SPDX-License-Identifier: MIT OR Apache-2.0
//! Glue between the MAC and the [`embassy_net_driver`]
//!
//! # Compatibility warning
//!
//! This does **not** implement proper IPv6 over nr+ (ETSI TS 103 874-3), but merely "dump data
//! directly into DLC in any flow" -- Nordic's IPv6 Workaround (NI6W) (see also [their DevZone
//! entry](https://devzone.nordicsemi.com/f/nordic-q-a/128194/dect-shell-ipv6-and-etsi-ts-103-874-3)).
//!
//! The intention for this module is to grow into an actual imlementation (possibly retaining the
//! NI6W version for compatibility).

use defmt::*;

/// Interface through which a running [`run_ni6w`] can be influenced.
///
/// It is generally multi-controller and single-controllee.
///
/// It currently does this by noting down state received from the controlled process (such as
/// whether or not it is associated), so that it can be polled by any controller, and by forwarding
/// actions into a queue (as that is an easy single-consumer direction).
///
/// # Further development
///
/// It is yet unclear whether this should really be around here or in the [`crate::runner`] module.
pub struct ControlHub {
    // Kind of duplicated with the inner current_assoc, but that is special-purpose and narrow,
    // while this is user visible.
    association: core::cell::Cell<Option<crate::association::Association>>,
    poke: embassy_sync::channel::Channel<
        embassy_sync::blocking_mutex::raw::CriticalSectionRawMutex,
        crate::runner::Poke,
        // Senders can just wait, or use fallible pushing
        1,
    >,
}

/// Error returned in the exotic case of a [`ControlHub`] fallible function currently not working.
///
/// When returned, retry later, or use a corresponding async function.
pub struct RetryLater(());

impl ControlHub {
    #[expect(clippy::new_without_default, reason = "only used with const")]
    pub const fn new() -> Self {
        Self {
            association: core::cell::Cell::new(None),
            poke: embassy_sync::channel::Channel::new(),
        }
    }

    pub fn association_status(&self) -> Option<crate::association::Association> {
        self.association.get()
    }

    // not async right now because we can't trigger that from coap anyway
    pub fn try_connect_now(&self) -> Result<(), RetryLater> {
        self.poke
            .try_send(crate::runner::Poke::ConnectNow)
            .map_err(|_| RetryLater(()))
    }

    pub fn try_disconnect_and_rescan(&self) -> Result<(), RetryLater> {
        self.poke
            .try_send(crate::runner::Poke::DisconnectAndRescan)
            .map_err(|_| RetryLater(()))
    }
}

/// An embassy network driver that transmits and receives packets via the Nordic nrfxlib MAC
/// following the NI6W mode (see module level documentation).
///
/// This is implemented in terms of [`embassy_net_driver_channel`], and thus takes a
/// [Runner][embassy_net_driver_channel::Runner].
///
/// `gateway_long` is the long RD address of the FT; in a sense, the MAC address of the
/// default gateway. This is probably how NI6W works, and packets will be sent correctly even to
/// other nodes when addressed that way.
pub async fn run_ni6w<'cfg, 'd, const MTU: usize>(
    time: impl embedded_hal_async::delay::DelayNs,
    config: &crate::association::PtConfiguration<'cfg>,
    net_runner: embassy_net_driver_channel::Runner<'d, MTU>,
    dect: super::DectMac,
    control_hub: &ControlHub,
) -> ! {
    use embassy_net_driver::LinkState;

    let (state_runner, mut rx_runner, mut tx_runner) = net_runner.split();

    let dlc_tx = embassy_sync::channel::Channel::new();
    let config_poke = embassy_sync::channel::Channel::new();

    let current_assoc = core::cell::Cell::new(None);

    let mut on_rx = |dlc_data_rx: crate::nrfxlib_mac::DlcDataRx| {
        if let Some(rx_buf) = rx_runner.try_rx_buf() {
            let len = dlc_data_rx.data().len();
            rx_buf[..len].copy_from_slice(dlc_data_rx.data());
            rx_runner.rx_done(len);
        } else {
            warn!("Dropping packet -- overflow");
        }
    };

    let mut on_assoc_change = |assoc: Option<crate::association::Association>| {
        info!("Learned about association: {}", assoc);
        if let Some(assoc) = assoc {
            current_assoc.set(Some(assoc.parent));
            control_hub.association.set(Some(assoc));
            state_runner.set_link_state(LinkState::Up);
        } else {
            current_assoc.set(None);
            control_hub.association.set(None);
            state_runner.set_link_state(LinkState::Down);
        }
    };

    let running_dect = crate::runner::Stack::new(dect).run(
        time,
        config,
        dlc_tx.receiver(),
        config_poke.receiver(),
        &mut on_rx,
        &mut on_assoc_change,
    );

    let tx_runner = async {
        loop {
            let tx_buf = tx_runner.tx_buf().await;

            // FIXME: Once we go from mesh to tree, we need more explicit information.
            let Some(gateway) = current_assoc.get() else {
                warn!("Dropping packetet: can not transmit while not associated.");
                // FIXME move into Dropper with Rust 1.100
                tx_runner.tx_done();
                continue;
            };
            let Ok(tx_buf) = heapless::Vec::try_from(&*tx_buf) else {
                // FIXME: Ensure at type level (passing owned pieces of the network buffer?)
                warn!("Dropping over-long packet; MTUs need to be aligned");
                tx_runner.tx_done();
                continue;
            };
            // It's a network driver, we can't do anything about lost packets.
            let _ = dlc_tx.try_send((1, gateway, tx_buf));
            tx_runner.tx_done();
        }
    };

    use embassy_futures::select::{Either, select};

    match select(running_dect, tx_runner).await {
        Either::First(never) => never,
        Either::Second(never) => never,
    }
}
