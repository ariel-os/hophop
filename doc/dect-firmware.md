<!--
SPDX-FileCopyrightText: Copyright Christian Amsüss <chrysn@fsfe.org>, Silano Systems
SPDX-License-Identifier: MIT OR Apache-2.0
-->
# Steps to get the DECT firmware onto the nRF9151 network core

For any of the code here to work, the nRF9151's network core
(which is not usually programmed by Ariel OS or any other RTOS)
needs to be flashed with dedicated firmware provided by Nordic
(`mfw-nr+-phy_nrf91x1_2.0.0.zip`).

This image is currently available through Nordic's "DECT NR+ early access program".
To obtain it,
fill your EMail address in [the early access program access form](https://share.hsforms.com/1O1l57xxoSgilkmPp8nhJHA1618t);
you will recevive an invitation to their "JFrog" platform (might take 1-2 days).
Beware that this EMail address may be shown to other JFrog participants.
Once you can log in to the platform,
you find the firmware image under Artifactory / Artifacts / MFW.

If that link should become unavailable,
as per [their docs](https://docs.nordicsemi.com/bundle/ncs-latest/page/nrfxlib/nrf_modem/doc/dectphy.html),
"you must contact the Nordic Semiconductor sales department".

Flashing this may be obvious for regular users of Nordic's tools,
but not to those using typical Rust workflows:

* Download `nrfutil` from <https://www.nordicsemi.com/Products/Development-tools/nRF-Util/Download>
* Run `nrfutil install 91`, which downloads extra code into your home directory (`~/.nrfutil`).
* Ensure that Segger's JLink is installed in the exact location nrfutil expects it to be
  (eg. by unpacking <https://www.segger.com/downloads/jlink/JLink_Linux_V880_x86_64.tgz> in /opt/SEGGER/JLink).
* Run `nrfutil 91 modem-firmware-upgrade --firmware mfw-nr+-phy_nrf91x1_2.0.0.zip --all-jlinks`
  (or specify the concrete JLink device).
  * If there is any error, try factory resetting the device: `nrfutil device recover --serial-number 001....`
    (unlike the firmware-upgrade, this has no `--all-jlinks` option).

## Supported firmware variants

As of writing, there were no observable differences in the way
the 2.0 PHY firmware (`mfw-nr+-phy_nrf91x1_2.0.0`) and the 2.0 MAC (`mfw-nr+_nrf91x1_2.0.0`)
behave when used for PHY operations;
either will work.

Older versions (1.1) currently fail a start-up test we perform (checking whether the latencies changed from our expectations);
that check could be relaxed to allow older firmware versions to run
until we start using any features of the new version.

## Going back to the cellular firmware

While there is no known way to read the network core's firmware
(i.e., there is no way to perform a backup before overwriting it),
the vendor firmware for LTE is [publically available ](https://www.nordicsemi.com/Products/nRF9151/Download#636701B052474E5A8EB903C56678D26E),
and should be usable the same way to restore the original state.
