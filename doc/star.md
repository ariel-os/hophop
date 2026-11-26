<!--
SPDX-FileCopyrightText: Copyright Christian Amsüss <chrysn@fsfe.org>, Silano Systems
SPDX-License-Identifier: MIT OR Apache-2.0
-->
hophop star: A practical network
================================

This document describes how to run a star shaped IPv6 network:
the first practical network usable with hophop.

The network is orchestrated by a central node (the "sink" in DECT terminology)
that runs the [`bridge-ft`](../applications/bridge-ft/`) application.
It interconnects different nodes that run the [`bridge-pt`](../applications/bridge-pt/`) application (connecting another PC),
the [`embedded-pt`](../applications/embedded-pt/`) (running a stand-alone embedded network node),
or a custom applications that follow the `embedded-pt` example.

![Illustration of the network described above in prose (two bridge-pt and an embedded-pt linked to a Nordic DECT shell, with an extra sniffer connected to one PC)](./.media/scenarios.svg#page-combination)

**Materials:** To run this, you need at a minimum two [supported embedded devices](hardware.md) (3 or more are recommended).
The examples will assume that you use an `nrf9151-dk` everywhere;
where you use different hardware, substitute the names in the command line prompts accordingly.
You also need at least one PC style system.
Running multiple bridge nodes on a single PC is possible, but requires some care to avoid inadvertently taking shortcuts.
Using link-local addresses exclusively is recommended in those situations, as it is more explicit in which interface is used where;
using network isolation is possible, but beyond the scope of this document.
All was tested on Linux,
but is likely usable in a similar way on MacOS[^1].

[^1]: If the example works for you there, especially if you need to make slight modifications, please open an [issue](https://github.com/ariel-os/hophop/issues) or [PR](https://github.com/ariel-os/hophop/pulls) describing what works or what changes were required.

Basic setup
-----------

As with all applications, the ["Getting started" documentation](getting-started.md)
documents all required first steps.

Setting up the sink
-------------------

Your PC will participate in the network,
but the hophop devkits currently just expose network access through SLIP,
so your operating system will not pick up on the new network interface being availble.
Create one using:

```console
$ sudo ip tuntap add tun0 mode tun user $(id -u)
$ sudo ip link set up tun0
$ echo 1 | sudo tee /proc/sys/net/ipv6/conf/tun0/addr_gen_mode
```

> *Note for curious readers:* The last line is not essential, but
> setting `addr_gen_mode` to 1 ("do no generate a link-local address") keeps the kernel from adding link-local addresses at random.
> We set an explicit link-local address later, but
> not having unusable addresses that would interfere with DECT operation reduces potential causes for errors.
>
> For context, DECT NR+ uses a particular way of assigning IPv6 addresses that makes routing more stateless in some situations.
> This is efficient, but requires either of
> 1. the operating systems understanding that they participate in a DECT NR+ network,
> 2. a border router in the embedded device, or
> 3. manual IP address configuration.
> <!-- there are some wonky inbetween options similar to ND proxying, but those fall somewhere between 2 and 3 usually. -->
>
> While the hophop project is working towards providing 2. through Ariel OS, 3. is a practical solution.

Pick one of the development kits and connect it to a PC.
To keep an overview of what does what,
take note of its programmer's serial number,
and place a post-it note on the device that says "sink" and the last few digits of the serial (in this case 23C2):

```console
$ probe-rs list
The following debug probes were found:
[0]: J-Link -- 1366:1059:001051299572 (J-Link)
```

Inside the `applications/bridge-ft` directory, now run (with example output abbreviated to some noteworty lines):

```console
$ laze build -b nrf9151-dk -D LOG=info run -- --probe 1366:1059:001051299572
laze: project root: …
…
   Compiling hophop v0.0.1 …
…
  Programming ⠦  42% [#########-----------]
…
[INFO ] Running cluster.
[INFO ] Please set this IPv6 address on the host: fe80:0000:0000:0000:0994:b32e:0994:b32e
[INFO ] CoAP server identity: {8:{1:{1:2,2:'',-1:1,-2:h'a7d5c6705a6f215325384e6759ab498687e656fd29ace0be3bb57134094fd8d1'}}}
```

Following the instructions of the debug output,
set the address (note the added `/64` after the address):

```console
$ sudo ip -6 a add dev tun0 fe80:0000:0000:0000:0994:b32e:0994:b32e/64
```

> *Note for curious readers:* See the duplicate groups of `0994:b32e` in the address?
> That is the DECT NR+ addressing structure showing up,
> and it says that we are reaching the sink itself, as opposed to any other radio node,
> where the first group would be the address of the sink (`0994:b32e`), and the rest varies by device.

Now you can run Jelly to both locally interact with the device, and forward network traffic:

```console
$ Jelly /dev/serial/by-id/usb-SEGGER_J-Link_001051299572-if00 --network tun0

Overview (F1) Text (F2) CoAP (F3) Commands (F4) Net (F5) Help (F12) ly 🪼─
┌Text & Commands───────────────────────────────┐┌CoAP Req & Resp─────────┐
│                                              ││ ← Req(Get /.well-known/│
│                                              ││ → Res(Content/Applicati│
│                                              ││</dect-ft/assoc>;rt="tag│
│                                              ││</id>;rel="impl-info"   │
│                                              ││────────────────────────│
│                                              ││ ← Req(Get /id)         │
│                                              ││ → Res(Content/Applicati│
│                                              ││3050242967({1: "Ariel OS│
│                                              ││────────────────────────│
│                                              ││                        │
│                                              ││                        │
│                                              ││                        │
│                                              ││                        │
│                                              ││                        │
│                                              ││                        │
│                                              ││                        │
│                                              ││                        │
│                                              │└────────────────────────┘
│                                              │┌Board Info──────────────┐
└──────────────────────────────────────────────┘│OS: Ariel OS            │
┌User Input────────────────────────────────────┐│Board: nrf9151-dk       │
│Type a command, for example: help, Help, Force││0994b32e6c8e4b55        │
└──────────────────────────────────────────────┘└────────────────────────┘
✅ connected via /dev/serial/by-id/usb-SEGGER_J-Link_001051299572-if00 | N
```

This shows you that the serial port is responsive, and already exchanged some CoAP control commands,
populating the board info field and discovering all resources.

> *Note for setups beyond the tutorial:*
> When setting this up in a more permanent location (or any other non-interactive system),
> Jelly can also be run non-interactively with `--headless-network` or `--headless-configuration.
> Details of that setup are beyond the scope of this introduction.

Type `/dect-ft/assoc` and press return (and avail yourself of the tab completion provided by resource discovery);
output will be shown in the CoAP box (which can be magnified by pressing F3):

```
┌User Input────────────────────────────────────┐
│ /dect-ft/assoc                               │
└──────────────────────────────────────────────┘
 ← Req(Get /dect-ft/assoc)
 → Res(Content/ApplicationCBOR)
[_ ]
```

This indicates that the list of currently associated peers is empty.

Let's do something about that!

Setting up an embedded device
-----------------------------

TBD; until complete, see [the embedded-pt docs](../applications/embedded-pt/README.md) etc.

