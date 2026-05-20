<!--
SPDX-FileCopyrightText: Copyright Christian Amsüss <chrysn@fsfe.org>, Silano Systems
SPDX-License-Identifier: MIT OR Apache-2.0
-->
# Available baseline tools

## PHY based tools

As a baseline for any trouble related to getting the PHY working, see those two projects from <https://github.com/nrfconnect/sdk-nrf>:

* [samples/dect/dect_phy/hello_dect](https://github.com/nrfconnect/sdk-nrf/tree/main/samples/dect/dect_phy/hello_dect)
* [samples/dect/dect_phy/dect_shell](https://github.com/nrfconnect/sdk-nrf/tree/main/samples/dect/dect_phy/dect_shell)

Both are based on Zephyr and the Nordic SDK;
if you are not familiar with that, they are set up as below.

Additional apps (not tested so far):

* [lauri-piikivi/dect-nr-samples](https://github.com/lauri-piikivi/dect-nr-samples/tree/main/dect-nr-samples)

  Working on an older (< 3.x) libmodem version (with per-event-type callbacks in a struct).
  
  Sniffer and Wireshark dissector (which appears to use a different format than the Wireshark 4.6 built-in DECT NR+ dissector).

## MAC based tools

* [samples/dect/dect_shell](https://github.com/nrfconnect/sdk-nrf/tree/main/samples/dect/dect_shell) has a full IPv6 FT and PT implementation

## Building and installing using the Nordic toolchain

Based on <https://docs.nordicsemi.com/bundle/ncs-3.2.0-preview2/page/nrf/installation/install_ncs.html>

* [Ensure the DECT firmware is flashed](./dect-firmware.md) (or don't, just to see which errors happen)
* `pipx install west`
* `pipx runpip west install jsonschema`
* `nrfutil install sdk-manager` (installs into `~/.nrfutil`)
* `nrfutil sdk-manager install v3.3.0` (installs into `~/ncs/`)
* `nrfutil sdk-manager toolchain launch --ncs-version v3.3.0 --shell` (spawns new shell with many environment variables set)
* `source ~/ncs/v3.3.0/zephyr/zephyr-env.sh` (or with added `-- -DEXTRA_CONF_FILE=overlay-eu.conf` for the hello)
* `cd ~/ncs/v3.3.0/nrf/samples/dect/dect_phy/dect_shell/`
  (or `samples/dect/dect_shell` for the MAC-based firmware)
* `west update`
* `west build -p -b nrf9151dk/nrf9151/ns`
* `west flash`, or `probe-rs download --protocol swd --chip nRF9160_xxAA build/merged.hex --binary-format ihex`
* `socat - /dev/ttyACM0,cfmakeraw,b115200`

## Failure modes

### Wrong network core

This is the DECT shell's startup crash when the stock firmware is on the network core:


```
All pins have been configured as non-secure
Booting TF-M v2.1.1-ncs4
[Sec Thread] Secure image initializing!
TF-M isolation level is: 0x00000001
TF-M Float ABI: Hard
Lazy stacking enabled
*** Booting nRF Connect SDK v3.0.0-3bfc46578e42 ***
*** Using Zephyr OS v4.0.99-3e0ce7636fa6 ***

Reset reason: PIN reset

desh:~$ Modem crash reason: 0xfff (Unknown reason), PC: 0x0
ASSERTION FAIL [0] @ WEST_TOPDIR/nrf/samples/dect/dect_phy/dect_shell/src/main.c:112
        Modem crash detected, halting application execution
[00:00:00.578,613] <err> os: r0/a1:  0x00000004  r1/a2:  0x00000070  r2/a3:  0x00000001
[00:00:00.578,643] <err> os: r3/a4:  0x00000004 r12/ip:  0x00000000 r14/lr:  0x0000b725
[00:00:00.578,643] <err> os:  xpsr:  0x0100003a
[00:00:00.578,674] <err> os: s[ 0]:  0x0000b6f1  s[ 1]:  0x0002492b  s[ 2]:  0x00000fff  s[ 3]:  0x00000000
[00:00:00.578,704] <err> os: s[ 4]:  0x00000000  s[ 5]:  0x00024af1  s[ 6]:  0x00000000  s[ 7]:  0x20014f1c
[00:00:00.578,704] <err> os: s[ 8]:  0x00000001  s[ 9]:  0x000246db  s[10]:  0x0000002a  s[11]:  0x00000020
[00:00:00.578,735] <err> os: s[12]:  0x00000000  s[13]:  0x00000000  s[14]:  0x00000000  s[15]:  0x0002060f
[00:00:00.578,735] <err> os: fpscr:  0x00000000
[00:00:00.578,765] <err> os: r4/v1:  0x000bffff  r5/v2:  0x00000002  r6/v3:  0x00000000
[00:00:00.578,796] <err> os: r7/v4:  0x00000000  r8/v5:  0x00000000  r9/v6:  0x00000000
[00:00:00.578,826] <err> os: r10/v7: 0x00000000  r11/v8: 0x00000000    psp:  0x200221a8
[00:00:00.578,857] <err> os: EXC_RETURN: 0x0
[00:00:00.578,857] <err> os: Faulting instruction address (r15/pc): 0x0002e230
[00:00:00.578,887] <err> os: >>> ZEPHYR FATAL ERROR 4: Kernel panic on CPU 0
[00:00:00.578,918] <err> os: Fault during interrupt handling

[00:00:00.578,948] <err> os: Current thread: 0x20013658 (idle)
[00:00:00.757,232] <err> os: Halting system
```
