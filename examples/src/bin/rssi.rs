// SPDX-FileCopyrightText: Copyright Christian Amsüss <chrysn@fsfe.org>, Silano Systems
// SPDX-License-Identifier: MIT OR Apache-2.0
#![no_main]
#![no_std]

use ariel_os::debug::{ExitCode, exit, log::{info, error}};

#[ariel_os::task(autostart)]
async fn main() {
    let mut dect = hophop::nrfxlib_phy::DectPhy::init_after_modem_init(())
    .await
    .unwrap();

    for _ in 0..60 {
        info!("DECT time is {:?}", dect.time_get().await);

        info!("Scanning band 1");
        let scans = &[
            1657,
            1659,
            1661,
            1663,
            1665,
            1667,
            1669,
            1671,
            1673,
            1675,
            1677,
        ];
        match dect.rssi_bulk(scans).await {
            Ok(results) => {
                for (channel, result) in scans.iter().zip(results.into_iter()) {
                    // Reporting just one item of data stays compatible with the older output
                    // format, and right now we know that it's just one frame per band anyway.
                    info!("RSSI for {} at {}: {:?}", channel, result.start_time(), result.data());
                }
            }
            Err(e) => error!("Failed to scan: {:?}", e),
        }

        // Not waiting: The rssi_bulk scan takes some time to start up anyway.
    }

    exit(ExitCode::SUCCESS);
}
