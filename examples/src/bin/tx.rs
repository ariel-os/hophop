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

#[ariel_os::task(autostart, peripherals)]
async fn blinky(peripherals: pins::ButtonPeripherals) {
    let mut dect = hophop::nrfxlib_phy::DectPhy::init_after_modem_init(())
    .await
    .unwrap();

    let mut button0 = ariel_os::gpio::Input::builder(peripherals.button0, ariel_os::gpio::Pull::Up)
        .build_with_interrupt()
        .unwrap();

    loop {
        // Gives the pull-up time to actually pull up
        Timer::after_millis(5).await;
        button0.wait_for_low().await;

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
                    1, 18, 52, 86, 0, 0, 0, 38, 73, 5, 176, 16, 6, 0, 13, 83, 7, 8, 12, 138, 160, 215,
                    2, 100, 64, 24, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                    0, 0,
                ],
            )
            .await
            .unwrap();
        }
        info!("Release: Not sending.");
    }

    panic!(
        "If we want to be able to re-flash, we better things at some point to avoid going through unlock again."
    );
}
