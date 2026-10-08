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

/// An embassy network driver that transmits and receives packets via the Nordic nrfxlib MAC
/// following the NI6W mode (see module level documentation).
///
/// This is implemented in terms of [`embassy_net_driver_channel`], and thus takes a
/// [Runner][embassy_net_driver_channel::Runner], which it services when the instance of `Self`
/// constructed around the runner is being run inside [`crate::runner::Stack::run()`].
///
/// # Caveats
///
/// Module-level caveats about NI6W, i.e., not being standard IPv6-profile apply.
///
/// The code currently assumes that the associated PT is also the sink, i.e., that this is just a
/// leaf node. This might be a fundamental restriction of the NI6W mode.
pub struct Ni6wNetworkController {
    static_config: crate::association::PtConfiguration<'static>,
    association: core::cell::Cell<Option<crate::association::Association>>,
    poke: embassy_sync::channel::Channel<
        embassy_sync::blocking_mutex::raw::CriticalSectionRawMutex,
        crate::runner::Poke,
        // Senders can just wait, or use fallible pushing
        1,
    >,
    // Having those RefCell is not ideal, but works right now.
    //
    // Better tools would be to do both of
    // - after Ni6wNetworkController construction, allow splitting it into halves
    // - rather than do next_…() all the time, allow the stack runner to take the owned controller
    //   half and split it up further into producer channels
    state_runner: core::cell::RefCell<embassy_net_driver_channel::StateRunner<'static>>,
    rx_runner: core::cell::RefCell<embassy_net_driver_channel::RxRunner<'static, 1500>>,
    tx_runner: core::cell::RefCell<embassy_net_driver_channel::TxRunner<'static, 1500>>,
}

/// Error returned in the exotic case of a [`Ni6wNetworkController`] fallible function currently not working.
///
/// When returned, retry later, or use a corresponding async function.
pub struct RetryLater(());

impl Ni6wNetworkController {
    pub fn new(
        static_config: crate::association::PtConfiguration<'static>,
        net_runner: embassy_net_driver_channel::Runner<'static, 1500>,
    ) -> Self {
        let (state_runner, rx_runner, tx_runner) = net_runner.split();

        Self {
            static_config,
            association: core::cell::Cell::new(None),
            poke: embassy_sync::channel::Channel::new(),
            state_runner: state_runner.into(),
            rx_runner: rx_runner.into(),
            tx_runner: tx_runner.into(),
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

    /// Builds a CoAP sub-tree (suitable for use with `.below()`)
    pub fn coap_server(&self) -> impl coap_handler::Handler + coap_handler::Reporting {
        use coap_handler_implementations::*;
        use coap_message_utils::Error;

        struct ServeStatus<'a>(&'a Ni6wNetworkController);

        impl GetRenderable for ServeStatus<'_> {
            type Get = Option<crate::association::Association>;

            fn get(&mut self) -> Result<Self::Get, Error> {
                Ok(self.0.association_status())
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

impl<'a> crate::runner::MacController for &'a Ni6wNetworkController {
    fn on_dlc_rx(&mut self, data: crate::nrfxlib_mac::DlcDataRx) {
        let mut rx_runner = self.rx_runner.borrow_mut();

        if let Some(rx_buf) = rx_runner.try_rx_buf() {
            let len = data.data().len();
            rx_buf[..len].copy_from_slice(data.data());
            rx_runner.rx_done(len);
        } else {
            warn!("Dropping packet -- overflow");
        }
    }

    fn on_assoc_change(&mut self, assoc: Option<crate::association::Association>) {
        use embassy_net_driver::LinkState;

        info!("Learned about association: {}", assoc);

        let state_runner = self.state_runner.borrow_mut();
        if let Some(assoc) = assoc {
            self.association.set(Some(assoc));
            state_runner.set_link_state(LinkState::Up);
        } else {
            self.association.set(None);
            state_runner.set_link_state(LinkState::Down);
        }
    }

    fn next_dlc_tx(
        &mut self,
    ) -> impl core::future::Future<Output = crate::runner::DlcTxItem> + use<'a> {
        let mut tx_runner = self.tx_runner.borrow_mut();
        // FIXME: refresh; for the time being, we trust that the user drops the future all the time.
        let association = self.association.get();

        async move {
            loop {
                let tx_buf = tx_runner.tx_buf().await;

                // FIXME: Once we go from mesh to tree, we need more explicit information.
                let Some(gateway) = association.map(|a| a.parent) else {
                    // Realistically, we won't ever land in here: This won't even be polled if
                    // we're not connected.

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
                // It's not sent, but we own a copy that we pass on reliably.
                tx_runner.tx_done();

                return (1, gateway, tx_buf);
            }
        }
    }

    fn next_config(
        &mut self,
    ) -> impl core::future::Future<Output = crate::association::PtConfiguration<'static>> + use<'a>
    {
        // FIXME: This relies on the consumer just querying it once; on the long run, we must be
        // sure to always return Pending after the first emission, or already forward changing
        // configurations.

        let static_config = self.static_config.clone();

        async move { static_config }
    }

    fn next_action(&mut self) -> impl core::future::Future<Output = crate::runner::Poke> + use<'a> {
        let receiver = self.poke.receiver();
        async move { receiver.receive().await }
    }
}
