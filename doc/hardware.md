<!--
SPDX-FileCopyrightText: Copyright Christian Amsüss <chrysn@fsfe.org>, Silano Systems
SPDX-License-Identifier: MIT OR Apache-2.0
-->
# Hardware usable with hophop

Currently, the project aims to support the [nRF9151](https://www.nordicsemi.com/Products/nRF9151) controller,
with the expectation to support all nRF9120 based ones
(which are nRF9151 and nRF9131;
note that the nRF916x series, while using identical radio firmware, contains different components).

Supported boards are (names in parentheses are the Ariel OS target names):

* [nRF9151-DK](https://www.nordicsemi.com/Products/Development-hardware/nRF9151-DK) ([`nrf9151-dk`](https://ariel-os.github.io/ariel-os/dev/docs/book/boards/nrf9151-dk.html)), including the SMA variant
* [makerdiary nRF9151 Connect Kit](https://makerdiary.com/products/nrf9151-connectkit) ([`makerdiary-nrf9151-connect-kit`](https://ariel-os.github.io/ariel-os/dev/docs/book/boards/makerdiary-nrf9151-connect-kit.html))
* [Thingy:91 X](https://www.nordicsemi.com/Products/Development-hardware/Nordic-Thingy-91-X) ([`nordic-thingy-91-x-nrf9151`](https://ariel-os.github.io/ariel-os/dev/docs/book/boards/nordic-thingy-91-x.html), using the default firmware on the nRF5340)

Examples are usually given for the `nrf9151-dk`; that target name (`-b` argument) needs to be adjusted depending on your hardware, but should then run everywhere.

Note that the devices as shipped are running an LTE firmware on their network core;
see [the DECT firmware page](./dect-firmware.md) on how to change that.
