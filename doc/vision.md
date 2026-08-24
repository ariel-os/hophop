<!--
SPDX-FileCopyrightText: Copyright Christian Amsüss <chrysn@fsfe.org>, Silano Systems
SPDX-License-Identifier: MIT OR Apache-2.0
-->
What hophop should be
=====================

*This document outlines the vision for the project, not the current implementation state.*

hophop makes the ETSI TS 103 636 ("DECT NR+") standard family accessible to embedded developers,
in an auditably secure way
and with a focus on IoT applications.

Between library and application
-------------------------------

hophop is built from several components, many of which can be used independently:

- tools for parsing and composing NR+ protocol elements
- drivers and their interfaces for different back-ends at different layers:
  This includes drivers for Nordic's MAC layer (possibly limiting the supported features), Nordic's PHY layer, other implementations (such as the Opener implementation), other vendors' interfaces, as well as simulation or SDR interfaces.
  Where needed, hophop either provides layers itself or uses other libraries.
- full device control:
  hophop can be run as a module that takes control over some radio hardware and be told which functions to perform.
  For example, it can be instructed to run as an FT, as a PT, or to just listen for general traffic.
- full stack:
  hophop can be run as a system network component.
  Its behavior is confiugured by some policy,
  but in the most general mode, it would join a network for which it has credentials,
  or open a network if it has the right connectivity.

  When combined with a secondary network interface, this includes the option to facilitate IP network access to additional devices.

  When run as a full stack,
  hophop can expose its policy through a CoAP server,
  either for local configuration
  (particularly when acting as a network access device)
  or through secure remote configuration
  (particularly when run standalone).

Beyond that, hophop offers some example applications.
Those focus on illustrating any DECT NR+ specific features,
since general IP networking examples that are transport independent apply automatically,
e.g. from Ariel OS's collection of examples.

DECT features
-------------

* Low power operation
* Using scheduled resources on demand
* Multicast
* All roles (sink, FT, PT, mesh with mobility inside and between sinks, Joining Proxy)
* Data without association
* Security on all layers:
  - Link-layer
  - end-to-end security
  - Secure Joining
* High-level support for:
  - IP traffic
  - audio traffic

Compatibility and openness
--------------------------

Being Free Software,
hophop can be used to participate in networks that admit a diverse set of devices.
Network operators are free to choose their admission policies.

Hophop aims to be compliant with all applicable DECT profiles where high-level support for application data is provided,
and facilitate compliance with others.

Pre-certification for the parts of the stack where this is sensible is aimed for.

Security
--------

hophop aims to follow best security practices;
this includes:

- regular rotation of symmetric key material
- source authentication / avoiding group secrets
- minimizing unencrypted information (also for privacy reasons)
- algorithm agility

In particular,
hophop implements an onboarding procedure
that follows IETF standards, but is bespoke from a DECT point of view
(but might be contributed to ETSI):
It combines the gist of RFCs [8995](https://www.rfc-editor.org/rfc/rfc8995.html), [9031](https://www.rfc-editor.org/rfc/rfc9031.html), [9528](https://www.rfc-editor.org/rfc/rfc9528.html) and [the ELA draft](https://www.ietf.org/archive/id/draft-ietf-lake-authz-08.html),
and thus are a concrete implementation of the options sketched in Section 7.2 of ETSI TS 103 636-1.

*The concrete procedure is subject to change as it is being implemented.*

* In network beacons,<!-- because apparently using Joining Beacons means everything is unecnrypted?? --> an initial handshake message is broadcast;
  this does not reveal the identity of the network but contains mostly random data.
* Along with its unencrypted association request<!-- or data without association? -->,
  the joining node sends information about its credential,
  encrypted to the (so far unauthenticated) network.

  Depending on the deployment scenario,
  this can be a short identifier of the public key (if the device expects to be known to the network),
  or can additionally include information about how to look it up in the manufacturer's / MASA's database
  (for zero-touch rollouts).
* Then, 
  - If the beaconing node can evaluate the credential immediately,
    it can provide a mutually authenticated and encrypted response in an association response:
    formally, it rejects the association,
    but it also provides a 3rd EDHOC message that is sufficient for the joining node to then join the network with link-layer encryption.
  - If the beaconing node needs to check back (with the manufacturer or just a key distributor),
    it accepts the association tentatively,
    and relays a final response whenever it arrives from the network.
* As with RFC9031,
  key updates are transmitted whenever keys are rolled over.

  If individual updates reach their limitations,
  key update mechanisms from [the Group OSCORE draft](https://www.ietf.org/archive/id/draft-ietf-core-oscore-groupcomm-28.html)
  are being considered.
