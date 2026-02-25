// SPDX-FileCopyrightText: Copyright Christian Amsüss <chrysn@fsfe.org>, Silano Systems
// SPDX-License-Identifier: MIT OR Apache-2.0
#![no_main]
#![no_std]

use ariel_os::debug::{ExitCode, exit, log::info};
use ariel_os::time::Timer;

#[ariel_os::task(autostart)]
async fn main() {
    let mut dect = hophop::nrfxlib_phy::DectPhy::init_after_modem_init(())
    .await
    .unwrap();

    for _ in 0..60 {
        info!("DECT time is {:?}", dect.time_get().await);

        info!("Scanning band 1");
        for carrier in 1657..=1677 {
            if let Ok(rssi) = dect.rssi(carrier).await {
                info!("RSSI for {} at {}: {:?}", carrier, rssi.0, rssi.1.data());
            }
        }
    }

    exit(ExitCode::SUCCESS);
}
