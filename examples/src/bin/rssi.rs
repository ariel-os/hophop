// SPDX-FileCopyrightText: Copyright Christian Amsüss <chrysn@fsfe.org>, Silano Systems
// SPDX-License-Identifier: MIT OR Apache-2.0
#![no_main]
#![no_std]

use ariel_os::debug::{ExitCode, exit};
use ariel_os::log::{error, info, warn};

#[ariel_os::task(autostart)]
async fn main() {
    let mut dect = hophop::nrfxlib_phy::DectPhy::init_after_modem_init(())
        .await
        .unwrap();

    for _ in 0..60 {
        info!("DECT time is {:?}", dect.time_get().await);

        info!("Scanning band 1");
        let scans = &[
            1657, 1659, 1661, 1663, 1665, 1667, 1669, 1671, 1673, 1675, 1677,
        ];
        let mut scan_iterator = scans.iter();
        if let Err(e) = dect
            .rssi_bulk(
                scans,
                async |result| {
                    let channel = scan_iterator.next().unwrap();
                    let Some(result) = result else {
                        info!("RSSI missed for channel {}.", channel);
                        return;
                    };
                    info!(
                        "RSSI for {} at {}: {:?}",
                        channel,
                        result.start_time(),
                        result.data()
                    );
                },
                async |recv| match recv {
                    Ok(recv) => info!(
                        "Data received: time {:?}, PCC {:?}, PDC {:?}.",
                        recv.pcc.time,
                        recv.pcc(),
                        recv.pdc()
                    ),
                    Err(e) => warn!("Receive error: {:?}", e),
                },
            )
            .await
        {
            error!("Failed to scan: {:?}", e);
        }

        // Not waiting: The rssi_bulk scan takes some time to start up anyway.
    }

    exit(ExitCode::SUCCESS);
}
