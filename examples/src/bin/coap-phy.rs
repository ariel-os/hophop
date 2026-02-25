// SPDX-FileCopyrightText: Copyright Christian Amsüss <chrysn@fsfe.org>, Silano Systems
// SPDX-License-Identifier: MIT OR Apache-2.0
//! Minimal transmit example
//!
//! This sends a hand-crafted beacon message whenever the first button is pressed.
#![no_std]
#![no_main]

use ariel_os::debug::log::info;
use ariel_os::time::Timer;

use ariel_os_boards::pins;

use coap_handler_implementations::{
    GetRenderable, HandlerBuilder, SimpleRendered, TypeHandler, new_dispatcher, with_get,
};

#[ariel_os::task(autostart, peripherals)]
async fn main(peripherals: pins::ButtonPeripherals) {
    let dect = hophop::nrfxlib_phy::DectPhy::init_after_modem_init(())
        .await
        .unwrap();

    let dect = core::cell::RefCell::new(dect);

    embassy_futures::select::select(
        // For the purpose of the CoAP application, "press a button to transmit" is not so much a
        // specialty of the button, but more there to show that other system components can still
        // use the PHY.
        transmit_beacon_on_button(peripherals, &dect),
        run_coap_with_dect(DectByReference(&dect)),
    )
    .await;
}

async fn transmit_beacon_on_button(
    buttons: pins::ButtonPeripherals,
    dect: &core::cell::RefCell<hophop::nrfxlib_phy::DectPhy>,
) -> ! {
    let mut button0 = ariel_os::gpio::Input::builder(buttons.button0, ariel_os::gpio::Pull::Up)
        .build_with_interrupt()
        .unwrap();

    loop {
        // Gives the pull-up time to actually pull up, or CoAP to finish
        Timer::after_millis(5).await;
        button0.wait_for_low().await;

        let Ok(mut dect) = dect.try_borrow_mut() else {
            // CoAP is *just* doing the same
            info!("Delaying transmission from button press until CoAP has released");
            continue;
        };

        info!("Press: Starting to transmit.");
        let mut last_tx = dect.time_get().await.unwrap();
        const TICKS_PER_FRAME: u64 = 691200;

        while button0.is_low() {
            last_tx = last_tx.wrapping_add(TICKS_PER_FRAME * 2);
            dect.tx(
                last_tx,
                1665,
                // FIXME: Not using a proper network ID yet
                0x12345678,
                // Beacon as seen by the dect_shell
                &[17, 120, 150, 24, 112],
                &[
                    1, 18, 52, 86, 0, 0, 0, 38, 73, 5, 176, 16, 6, 0, 13, 83, 7, 8, 12, 138, 160,
                    215, 2, 100, 64, 24, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                    0, 0, 0, 0, 0,
                ],
            )
            .await
            .unwrap();
        }
        info!("Release: Not sending.");
    }
}

async fn run_coap_with_dect(dect: DectByReference<'_>) -> ! {
    let handler = new_dispatcher()
        // That's not what we'll want to use for advertising who we are, but as long as this is
        // what Jelly queries, it doesn't hurt.
        .at(
            &["jelly", "board"],
            SimpleRendered(ariel_os::buildinfo::BOARD),
        )
        .at(&["jelly", "ver"], SimpleRendered("unversioned hophop demo"))
        // Those will need a better description too
        .at_with_attributes(
            &["phy", "time"],
            &[],
            TypeHandler::new_minicbor_2(with_get(TimeResource(dect))),
        )
        .at_with_attributes(
            &["phy", "rssi"],
            &[],
            TypeHandler::new_minicbor_2(with_get(RssiResource(dect))),
        );

    ariel_os::coap::coap_run(handler).await
}

// FIXME: This only works because we don't use a main runner task. Don't do this at home, and let's
// fix the CoAP crates ASAP.
fn block<F: core::future::Future>(future: F) -> F::Output {
    let mut future = core::pin::pin!(future);
    let mut context = core::task::Context::from_waker(&core::task::Waker::noop());
    loop {
        match future.as_mut().poll(&mut context) {
            core::task::Poll::Ready(v) => break v,
            _ => (),
        }
    }
}

/// Simple wrapper around the RefCell reference that has an accessor that does the CoAP error
/// conversion.
#[derive(Copy, Clone)]
struct DectByReference<'d>(&'d core::cell::RefCell<hophop::nrfxlib_phy::DectPhy>);

impl<'d> DectByReference<'d> {
    fn use_mut(
        &self,
    ) -> Result<core::cell::RefMut<'_, hophop::nrfxlib_phy::DectPhy>, coap_message_utils::Error>
    {
        self.0
            .try_borrow_mut()
            .map_err(|_| coap_message_utils::Error::service_unavailable().with_max_age(1))
    }
}

struct TimeResource<'d>(DectByReference<'d>);

impl<'d> GetRenderable for TimeResource<'d> {
    type Get = u64;

    fn get(&mut self) -> Result<u64, coap_message_utils::Error> {
        let mut dect = self.0.use_mut()?;

        block(dect.time_get()).map_err(|_| coap_message_utils::Error::internal_server_error())
    }
}

struct RssiResource<'d>(DectByReference<'d>);

#[derive(minicbor::Encode)]
// FIXME: Should we define a minicbor encoding for the main RssiEvent?
//
// (Like, with no guarantees on stability, but at least it's a usable form).
pub struct RssiEvent {
    #[n(0)]
    start_time: u64,
    #[n(1)]
    data: [u8; 240],
}

impl<'d> GetRenderable for RssiResource<'d> {
    type Get = RssiEvent;

    fn get(&mut self) -> Result<RssiEvent, coap_message_utils::Error> {
        let mut dect = self.0.use_mut()?;

        let result = block(dect.rssi(1657)).unwrap();

        Ok(RssiEvent {
            start_time: result.start_time(),
            data: result.data().try_into().unwrap(),
        })
    }
}
