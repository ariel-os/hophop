// SPDX-FileCopyrightText: Copyright Christian Amsüss <chrysn@fsfe.org>, Silano Systems
// SPDX-License-Identifier: MIT OR Apache-2.0
#![no_main]
#![no_std]

use ariel_os::debug::{ExitCode, exit};
use ariel_os::log::{error, info, warn};

use ts_103_636_utils::identifiers::{AbsoluteChannel, NetworkId32};

#[ariel_os::task(autostart)]
async fn main() {
    let mut dect = hophop::nrfxlib_phy::DectPhy::init_after_modem_init(())
        .await
        .unwrap();

    for _ in 0..60 {
        info!("DECT time is {:?}", dect.time_get().await);

        info!("Scanning band 1");
        let scans = &const {
            [
                AbsoluteChannel::new(1657).unwrap(),
                AbsoluteChannel::new(1659).unwrap(),
                AbsoluteChannel::new(1661).unwrap(),
                AbsoluteChannel::new(1663).unwrap(),
                AbsoluteChannel::new(1665).unwrap(),
                AbsoluteChannel::new(1667).unwrap(),
                AbsoluteChannel::new(1669).unwrap(),
                AbsoluteChannel::new(1671).unwrap(),
                AbsoluteChannel::new(1673).unwrap(),
                AbsoluteChannel::new(1675).unwrap(),
                AbsoluteChannel::new(1677).unwrap(),
            ]
        };
        let mut scan_iterator = scans.iter();
        if let Err(e) = dect
            .rssi_bulk(
                scans,
                const { NetworkId32::new(0x87654321).unwrap() },
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
