<!--
SPDX-FileCopyrightText: Copyright Christian Amsüss <chrysn@fsfe.org>, Silano Systems
SPDX-License-Identifier: MIT OR Apache-2.0
-->
Current options:

* Run on bare nrf-modem
    - Missing many Ariel conveniences, but quick to get started as it can do without said pull request
    - Most comprehensive demo to date of DECT on Rust filed as https://github.com/diondokter/nrf-modem/pull/52
* Run on Ariel OS
    - Currently being established in https://github.com/ariel-os/ariel-os/pull/1562
* embassy-net-nrf91 seems to have reverse engineerd the IPC <https://github.com/embassy-rs/embassy/blob/main/embassy-net-nrf91/src/context.rs>, and running it on the application core without any C code and running the DECT backend goes on for long enough that it becomes apparent that app and network core do talk (and eventually only fails parsing AT stuff, to no surprise as they do different AT command sets).
  - [modem\_updater](https://github.com/circuitdojo/modem_updater) covers some aspects of IPC (firmware updates) that are not preent there.
