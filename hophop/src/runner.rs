// SPDX-FileCopyrightText: Copyright Christian Amsüss <chrysn@fsfe.org>, Silano Systems
// SPDX-License-Identifier: MIT OR Apache-2.0
//! Managing a full stack.
//!
//! Code in here should eventually be independent of the underlying implementation of nr+, but is
//! currently tied to the [`crate::nrfxlib_mac`].

use defmt::*;
use embassy_futures::select::{Either3, select3};

use ts_103_636_utils::identifiers::LongRdId;

/// An instance of a stack that is run in one task and provides access to operations that would
/// usually need to be called in the right sequence and from a mutable reference to independent
/// consumers.
pub struct Stack {
    dect: crate::nrfxlib_mac::DectMac,
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
pub type DlcTxItem = (u8, LongRdId, heapless::Vec<u8, 1024>);

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
    /// FIXME Currently this is only awaited once; generally, the producer should just be allowed
    /// to send eventually consistent configs through there.
    fn next_config(
        &mut self,
    ) -> impl core::future::Future<Output = super::association::PtConfiguration<'static>> + use<Self>;
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
    pub fn new(dect: crate::nrfxlib_mac::DectMac) -> Self {
        Self { dect }
    }

    pub async fn run<MC: MacController>(
        mut self,
        mut time: impl embedded_hal_async::delay::DelayNs,
        controller: &mut MC,
    ) -> ! {
        controller.on_assoc_change(None);
        // FIXME keep polling that for config changes
        let config = controller.next_config().await;
        loop {
            let association = super::association::associate(&mut self.dect, &config).await;

            let Some((association, mut release)) = association else {
                warn!("No network beacons found, sleeping before retrying");
                // FIXME: make configurable
                time.delay_ms(5000).await;
                // FIXME race with next_action
                continue;
            };

            controller.on_assoc_change(Some(association));

            loop {
                // FIXME: Or just wait for disassociation
                match select3(
                    // of all the dect functions, this one fortunately is already cancel safe
                    self.dect.dlc_data_rx(),
                    controller.next_dlc_tx(),
                    // FIXME: This should be properly drained, which might mean in face of race
                    // conditions that we'd really rather have a complete "disassociate fully" step
                    // before we associate again, as otherwise we can't tell if "still not getting
                    // beacons" is from the last association of the current one.
                    &mut release,
                )
                .await
                {
                    Either3::First(received) => controller.on_dlc_rx(received),
                    Either3::Second((flow_id, dest, data)) => {
                        // FIXME: Should we back-signal?
                        let _ = self.dect.dlc_data_tx(flow_id, dest, &data).await;
                    }
                    Either3::Third(release) => {
                        info!("Association released ({}), going back to scanning", release);
                        break;
                    }
                }
            }

            controller.on_assoc_change(None);
        }
    }
}
