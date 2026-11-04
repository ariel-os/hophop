<!--
SPDX-FileCopyrightText: Copyright Christian Amsüss <chrysn@fsfe.org>, Silano Systems
SPDX-License-Identifier: MIT OR Apache-2.0
-->
Getting started with hophop
===========================

Basic steps common to all examples and applications
---------------------------------------------------

* Get two (better four) [nRF9151-DK](../doc/hardware.md), connect them and turn them on.

  (Wider hardware support will be added, but this is what the examples assume).

* Ensure you are set up to run Ariel OS examples.

  For the time being, it's easiest to follow the [Getting Started section in the Ariel OS book](https://ariel-os.github.io/ariel-os/dev/docs/book/getting-started.html).

  Note that in particular, you need to have access to the boards you are using as a user;
  this is only mentioned in the Ariel documentation indirectly by referring to the [probe-rs documentation](https://probe.rs/docs/getting-started/probe-setup/#platform-specifics).
  That includes verifying that your user is part of the `plugdev` group,
  and placing [the probe-rs rules file](https://probe.rs/files/69-probe-rs.rules) inside `/etc/udev/rules.d`.
  The typical error that indicates that this step is missing is "USB device: permission denied".

* [Ensure that you have the DECT firmware running](../doc/dect-firmware.md).

* Beware that this is a research example,
  and that depending on your location, regulation on operating these devices does apply,
  especially as some examples emit transmissions controlled by the user alone.

Examples to run
---------------

* To get the most high-level functionality, run:

  - the [`embedded-pt`] application, which includes running Nordic's DECT shell as a FT (Fixed Terminal, a bit like a base station).

    (By the way, there is a [glossary](./glossary.md) for terms and abbreviations).

    This sets up a basic testable network connection between two devices.
    It transports IPv6 traffic between the devices;
    beware that this and other examples are *not yet* running proper IPv6 over DECT (as in ETSI TS 103 874-3),
    but merely compatible with Nordic's workaround (NI6W).

  - the [`bridge-pt`] application, which lets your Linux PC join into the network.

    It ends with setting up an encrypted CoAP data exchange between the PC and the previous example's device.

  - To see what is actually happening on the radio, you can run the RX [example].
    Note that this only sees little useful data;
    turn MAC layer encryption off in the DECT shell and in the examples' `main.rs` files to see more.
    (Replace `dect_mac::security_mode_values::_1,` with `…::NONE` in the hophop source or its callers, and reconfigure FT in the DECT shell).

* The `rssi` example contains tools to visualize how much traffic there is in the DECT spectrum.

[`embedded-pt`]: ../applications/embedded-pt/README.md
[`bridge-pt`]: ../applications/bridge-pt/README.md
[example]: ../examples/README.md

Tips and recovery
-----------------

* Depending on the program and programmer,
  flashing steps might complain about locked cores.

  To recover, run `laze build -b nrf9151-dk flash-erase-all-uicr`.

  This typically works for errors such as:

  "Core 0 is not accessible. Erase procedure will be started to recover access."

  It is the current assumption that this allows recovering from all lock-up situations.
  As there might be situations that were not recenlty accounted,
  you might need some older recovery mechanisms.
  If the above does not work, please try those, **and report** at the recovery issue [#82](https://github.com/ariel-os/hophop/issues/82).

  * With a non-hophop application, run the Ariel OS hello-world example,
    and run the `flash-erase-all` target and the `run` target (possibly twice; please document the Ariel version and whether the first run already showed the hello message).

  * The `nrfutil` ([from Nordic](https://github.com/ariel-os/hophop/issues/82)) has an `nrfutil device recover` command.
    Note that this only works with JLink based programmers (as on their devkits).

  * Press the reset button shortly before the flashing step (and stop pressing once you see any progress).

* If ever you have multiple DKs connected,
  probe-rs will give you interactive options.

  You can skip that and statically set the probe to use
  by adding `-- --probe 1366:1059:xxxxxxxxxxxx` after the laze call (or without the `--` if you already).
