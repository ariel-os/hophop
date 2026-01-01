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
        let mut scans = [
            (1657, 0, [[0; 240]]),
            (1659, 0, [[0; 240]]),
            (1661, 0, [[0; 240]]),
            (1663, 0, [[0; 240]]),
            (1665, 0, [[0; 240]]),
            (1667, 0, [[0; 240]]),
            (1669, 0, [[0; 240]]),
            (1671, 0, [[0; 240]]),
            (1673, 0, [[0; 240]]),
            (1675, 0, [[0; 240]]),
            (1677, 0, [[0; 240]]),
        ];
        match dect.rssi_bulk(scans.as_mut()).await {
            Ok(()) => {
                for (channel, start_time, data) in scans {
                    // Reporting just one item of data stays compatible with the older output
                    // format, and right now we know that it's just one frame per band anyway.
                    info!("RSSI for {} at {}: {:?}", channel, start_time, data[0]);
                }
            }
            Err(e) => error!("Failed to scan: {:?}", e),
        }

        // Not waiting: The rssi_bulk scan takes some time to start up anyway.
    }

    exit(ExitCode::SUCCESS);
}
