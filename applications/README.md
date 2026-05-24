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

Currently, this contains two applications:

* [`embedded-pt`](./embedded-pt/): IPv6 processed in the device, which takes the PT role.
* [`bridge-pt`](./bridge-pt/): IPv6 forwarded to SLIP, with the device taking the PT role.

Other envisioned applications are
and `bridge-ft` (serving as a sink) and `embedded-ft` (which makes most sense for isolated networks or when Ariel OS gains border router functionality).
On the long run, those might become unified.
