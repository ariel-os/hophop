<!--
SPDX-FileCopyrightText: Copyright Christian Amsüss <chrysn@fsfe.org>, Silano Systems
SPDX-License-Identifier: MIT OR Apache-2.0
-->
Like `../examples`, this contains applications built on hophop;
those here are more standalone in that they come with their own Cargo and laze configuration.

The intention of those examples is more to serve as a practical tool and less to illustrate or research
(like the examples do).
This line is rather wobbly at the time of writing.

---

Currently, this contains those applications:

* [`embedded-pt`](./embedded-pt/): IPv6 processed in the device, which takes the PT role.
* [`bridge-pt`](./bridge-pt/): IPv6 forwarded to SLIP, with the device taking the PT role.
* [`sniffer`](./sniffer/): A packet dumping application, built on the PHY, that dumps the sniffed data out via slipmux and UDP for Wireshark.
* [`bridge-ft`](./bridge-ft/): A sink that forwards IPv6 to SLIP, and manages a DECT NR+ cluster in the FT role.
  This is best explored through the [star network setup](../doc/star.md).

On the long run, those might become unified,
allowing any node to run FT or PT mode;
with non-bridge devices, this will allow running isolated networks.
The bridge applications might be augmented with router applications
that link directly to Ethernet or other uplinks, rather than expecting the host to take a more active role in DECT networking.
