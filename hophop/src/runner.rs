// SPDX-FileCopyrightText: Copyright Christian Amsüss <chrysn@fsfe.org>, Silano Systems
// SPDX-License-Identifier: MIT OR Apache-2.0
//! Managing a full stack.
//!
//! Code in here should eventually be independent of the underlying implementation of nr+, but is
//! currently tied to the [`crate::nrfxlib_mac`].

use defmt::*;
use embassy_futures::select::{Either3, select3};

use embassy_sync::blocking_mutex::raw::CriticalSectionRawMutex;
use embassy_sync::channel::Receiver;

use ts_103_636_utils::identifiers::LongRdId;

/// An instance of a stack that is run in one task and provides access to operations that would
/// usually need to be called in the right sequence and from a mutable reference to independent
/// consumers.
pub struct Stack {
    dect: crate::nrfxlib_mac::DectMac,
}

type DlcTxReceiver<'a> =
    Receiver<'a, CriticalSectionRawMutex, (u8, LongRdId, heapless::Vec<u8, 1024>), 1>;
type ConfigPokeReceiver<'a> = Receiver<'a, CriticalSectionRawMutex, (), 1>;

impl Stack {
    pub fn new(dect: crate::nrfxlib_mac::DectMac) -> Self {
        Self { dect }
    }

    pub async fn run<'cfg, 'txch, 'cfgch>(
        mut self,
        mut time: impl embedded_hal_async::delay::DelayNs,
        config: &super::association::PtConfiguration<'cfg>,
        // FIXME we'll probably need a better interface in several places.
        // We can not `impl futures::Stream` because StreamExt needs `Unpin`.
        dlc_tx: DlcTxReceiver<'txch>,
        // FIXME we'll want to send in more than just "hey maybe rescan now"; really, we'll
        // probably want to send in the config. Or do we want to send in messages? Or will this
        // become an internal interface anyway because there'll be a split between "create a
        // shared-accessible item" and "and run this to service it"?
        config_poke: ConfigPokeReceiver<'cfgch>,
        on_dlc_rx: &mut impl FnMut(crate::nrfxlib_mac::DlcDataRx),
        on_assoc_change: &mut impl FnMut(Option<super::association::Association>),
    ) -> ! {
        on_assoc_change(None);
        loop {
            let association = super::association::associate(&mut self.dect, config).await;

            let Some((association, mut release)) = association else {
                warn!("No network beacons found, sleeping before retrying");
                // FIXME: make configurable
                time.delay_ms(5000).await;
                let _ = config_poke;
                continue;
            };

            on_assoc_change(Some(association));

            loop {
                // FIXME: Or just wait for disassociation
                match select3(
                    // of all the dect functions, this one fortunately is already cancel safe
                    self.dect.dlc_data_rx(),
                    dlc_tx.receive(),
                    // FIXME: This should be properly drained, which might mean in face of race
                    // conditions that we'd really rather have a complete "disassociate fully" step
                    // before we associate again, as otherwise we can't tell if "still not getting
                    // beacons" is from the last association of the current one.
                    &mut release,
                )
                .await
                {
                    Either3::First(received) => on_dlc_rx(received),
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

            // FIXME when we can fall out of the loop:
            // on_assoc_change(None);
        }
    }
}
