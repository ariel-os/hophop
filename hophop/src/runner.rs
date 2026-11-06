// SPDX-FileCopyrightText: Copyright Christian Amsüss <chrysn@fsfe.org>, Silano Systems
// SPDX-License-Identifier: MIT OR Apache-2.0
//! Managing a full stack.
//!
//! Code in here should eventually be independent of the underlying implementation of nr+, but is
//! currently tied to the [`crate::nrfxlib_mac`].

use defmt::*;
use embassy_futures::select::{Either4, select, select4};

use ts_103_636_utils::identifiers::LongRdId;

/// An instance of a stack that is run in one task and provides access to operations that would
/// usually need to be called in the right sequence and from a mutable reference to independent
/// consumers.
pub struct Stack {
    dect: crate::nrfxlib_mac::DectMac,
    dlc_rx: crate::nrfxlib_mac::DlcRx,
    dlc_tx: crate::nrfxlib_mac::DlcTx,
}

/// Message sent into a running stack
// This interface might need to change if there are more variants coming in where you can't always
// discard any unexpected ones.
pub enum Poke {
    /// If currently unconnected, pause any sleeping and go right into scanning again.
    ConnectNow,
    /// Disassociate from the current association (if any), and start reconnecting.
    DisconnectAndRescan,
}

/// Message encapsulating a to-be-transmitted DLC data item
pub type DlcTxItem = (u8, LongRdId, heapless::Vec<u8, 1501>);

/// Interface which higher-layer stack components have to imlpement to talk to an
/// association-managing PT MAC layer.
///
/// For all the `next_…` futures, the provider may deliver the update to any currently obtained
/// future; in practice, no two futures of a single type will exist at any point in time.
pub trait MacController {
    fn on_dlc_rx(&mut self, data: crate::nrfxlib_mac::DlcDataRx);
    fn on_assoc_change(&mut self, assoc: Option<super::association::Association>);
    fn next_dlc_tx(&mut self) -> impl core::future::Future<Output = DlcTxItem> + use<Self>;
    /// Producer of whichever is the current configuration.
    ///
    /// FIXME: The `use<Self>` is an indication that we don't really need the mut self, so we should
    /// consider either of
    /// * explaining how this is needed, or
    /// * let this run on `&self`, or
    /// * use a message passing architecture where the runner can wait for any message from the
    ///   controller, or
    /// * wait for field projections to express this more sharply.
    fn next_config(
        &mut self,
    ) -> impl core::future::Future<Output = super::association::PtConfiguration> + use<Self>;
    /// Next simple operation the MAC layer should perform.
    ///
    /// This lumps together a bunch of operations that can either be performed immediately or be
    /// disregarded; for example, a "disconnect" that becomes available during disassociated time
    /// can just be ignored, while a "skip the rescan timeout, scan now" action during active
    /// association is ignored. (Future versions might change semantics there to make that a
    /// "disconnect briefly to see if there are stronger beacons").
    fn next_action(&mut self) -> impl core::future::Future<Output = Poke> + use<Self>;
}

impl Stack {
    pub fn new(
        dect: crate::nrfxlib_mac::DectMac,
        dlc_tx: crate::nrfxlib_mac::DlcTx,
        dlc_rx: crate::nrfxlib_mac::DlcRx,
    ) -> Self {
        Self {
            dect,
            dlc_tx,
            dlc_rx,
        }
    }

    pub async fn run<MC: MacController>(
        mut self,
        mut time: impl embedded_hal_async::delay::DelayNs,
        controller: &mut MC,
    ) -> ! {
        // FIXME: This should be simplifyable now that the MAC, DLC RX and TX can be used in
        // parallel.

        controller.on_assoc_change(None);

        let mut config = controller.next_config().await;
        loop {
            // associate() must be polled to completion, so we're not racing it with next_config()
            // here -- whoever switches away from a setting that has many channels and long beacon
            // intervals will just have to wait for their config to take effect.
            let association = super::association::associate(&mut self.dect, &config).await;

            let Some(association) = association else {
                warn!("No network beacons found, sleeping before retrying");
                select(
                    // FIXME: make configurable
                    time.delay_ms(5000),
                    async {
                        config = controller.next_config().await;
                    },
                )
                .await;
                continue;
            };

            controller.on_assoc_change(Some(association));

            // Set at the moment when we ask to release -- because then, we start ignoring
            // "released because of peer" or "of timeout" until we see "on our own", at which point
            // everything else is properly drained.
            let mut wait_for_our_release = false;
            loop {
                match select4(
                    // of all the dect functions, this one fortunately is already cancel safe
                    self.dlc_rx.dlc_data_rx(),
                    controller.next_dlc_tx(),
                    // FIXME: This should be properly drained, which might mean in face of race
                    // conditions that we'd really rather have a complete "disassociate fully" step
                    // before we associate again, as otherwise we can't tell if "still not getting
                    // beacons" is from the last association of the current one.
                    self.dect.next_association_end(),
                    controller.next_config(),
                )
                .await
                {
                    Either4::First(received) => controller.on_dlc_rx(received),
                    Either4::Second((flow_id, dest, data)) => {
                        // FIXME: Should we back-signal?
                        let _ = self.dlc_tx.dlc_data_tx(flow_id, dest, &data).await;
                    }
                    Either4::Third(release) => {
                        defmt::assert_eq!(release.long_rd_id, association.parent);
                        if wait_for_our_release
                            && release.release_cause != crate::nrfxlib_mac::Release::OurRelease
                        {
                            warn!(
                                "Got release notification {} but still waiting for acknowledgement of the release triggered by this module. This is a warning because it hinges on unclear aspects of the Nordic MAC layer.",
                                release
                            );
                            continue;
                        }
                        info!("Association released ({}), going back to scanning", release);
                        break;
                    }
                    Either4::Fourth(new_config) => {
                        config = new_config;
                        if self
                            .dect
                            .initiate_release_association(
                                association.parent,
                                nrfxlib_sys_pretty::dect_mac::release_cause_values::OTHER_REASON,
                            )
                            .is_ok()
                        {
                            wait_for_our_release = true;
                        } else {
                            warn!(
                                "Failed to kick off release association. The working hypothesis about the Nordic MAC layer's workings is that this would only happen if it thinks that that association is already released, and that a Third case is already on its way in our queue to tell us that it's fine and we did release."
                            );
                        }
                    }
                }
            }

            controller.on_assoc_change(None);
        }
    }
}
