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
    (Replace `mode: nrfxlib_sys::nrf_modem_dect_mac_security_mode_NRF_MODEM_DECT_MAC_SECURITY_MODE_1` with `…_SECURITY_MODE_NONE` in the hophop source, and reconfigure FT in the DECT shell).

* The `rssi` example contains tools to visualize how much traffic there is in the DECT spectrum.

[`embedded-pt`]: ../applications/embedded-pt/README.md
[`bridge-pt`]: ../applications/bridge-pt/README.md
[example]: ../examples/README.md

Unsorted tips and tricks
------------------------

* If ever you have multiple DKs connected,
  probe-rs will give you interactive options.

  You can skip that and statically set the probe to use
  by adding `-- --probe 1366:1059:xxxxxxxxxxxx` after the laze call (or without the `--` if you already).

* Depending on the program,
  flashing steps might complain about locked cores.
  Until the programmer's defaults are configured more aggressively,
  pressing the reset button shortly before the flashing step is actually executed usually helps.
