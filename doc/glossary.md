<!--
SPDX-FileCopyrightText: Copyright Julian Dickert <julian@systemscape.de>, Christian Amsüss <chrysn@fsfe.org>, Silano Systems
SPDX-License-Identifier: MIT OR Apache-2.0
-->

Abbreviations and terms relevant to hophop:

<!-- Editing remarks: Most generic terms X used mean nr+'s X; this is is pointed out explicitly when nr+'s implementation is a bit more specific than the "usual" concrete imlpementation of the generic term. -->

- **ARQ**: Automatic Repeat reQuest ([generic](https://en.wikipedia.org/wiki/Automatic_repeat_request), [nr+])
- **BCC**: Broadcast Control ([nr+])
- **BCCH**: Broadcast Control Channel ([nr+])
- **BSC**: Beacon Scanning Control ([nr+])
- **BRSKI**: Bootstrapping Remote Secure Key Infrastructure ([IETF](https://brski.org/))
- **CCC**: Connection Configuration Control ([nr+])
- **CCCH**: Connection Control Channel ([nr+])
- **CoAP**: Constrained Application Protocol ([RFC7252](https://datatracker.ietf.org/doc/html/rfc7252))
- **CoJP**: Constrained Join Protocol ([RFC9031](https://www.rfc-editor.org/rfc/rfc9031.html))
- **CP-OFDM**: Cyclic-Prefix [Orthogonal Frequency Division Multiplexing](https://en.wikipedia.org/wiki/Orthogonal_frequency-division_multiplexing) ([generic](https://en.wikipedia.org/wiki/Cyclic_prefix), [nr+]; method for improving OFDM reception in multi-path environments)
- **CVG**: Convergence (layer) ([nr+])
- **CWT**: CBOR Web token ([RFC8392](https://www.rfc-editor.org/rfc/rfc8392); like JWT (JSON Web Token) but based on CBOR/COSE)
- **DCCH**: Dedicated Control Channel ([nr+])
- **DCH**: Dedicated Channel ([nr+])
- **DL**: Downlink (generic, [nr+])
- **DLC**: Data Link Control ([nr+])
- **DTCH**: Dedicated Traffic Channel ([nr+])
- **edge**: Connection between two nodes. **We mostly avoid this term** due to ambiguity, as it is also used for edge computing, where it refers to a node between two parts of a network.
- **EDHOC**: Ephemeral Diffie-Hellman Over COSE ([RFC9528](https://www.rfc-editor.org/rfc/rfc9528.html); asymmetric key establishment that provides all relevant security properties for CoAP; works in tandem with OSCORE)
- **EP**: Endpoint (generic, but here: [nr+] endpoint; beware that CoAP and OAuth have different and more specific definition)
- **FDMA**: Frequency Division Multiple Access ([generic](https://en.wikipedia.org/wiki/Frequency-division_multiple_access), [nr+])
- **FEC**: Forward Error Correction ([generic](https://en.wikipedia.org/wiki/Error_correction_code), [nr+])
- **FFT**: Fast Fourier Transform ([generic](https://en.wikipedia.org/wiki/Fast_Fourier_transform), [nr+])
- **FT**: Fixed Termination point ([nr+])
- **GI**: Guard Interval ([generic](https://en.wikipedia.org/wiki/Guard_interval), [nr+])
- **LBT**: Listen Before Talk ([generic](https://en.wikipedia.org/wiki/Contention-based_protocol), [nr+])
- **MAC**: Medium Access Control (generic, but here: the [nr+] MAC)
- **mesh network**: A multiply connected network, which has more connections than spanning tree.
  **We mostly avoid this term** due to ambiguity: It is used in the nr+ context to merely indicate a tree beyond a star,
  whereas other networks realize actual mutli-parent topologies.
- **MIMO**: Multiple Input Multiple Output ([generic](https://en.wikipedia.org/wiki/MIMO), [nr+])
- **MTCH**: Multicast (Broadcast) Traffic Channel ([nr+])
- **NI6W**: Nordic's IPv6 Workaround ([hophop](../hophop/src/nrfxlib_mac/embassy_net.rs) [for lack of an official name](https://devzone.nordicsemi.com/f/nordic-q-a/128194/dect-shell-ipv6-and-etsi-ts-103-874-3))
- **node**: Participant in a generic radio network (especially mesh), especially in the context of network topology ([generic](https://en.wikipedia.org/wiki/Node)<!-- pointing to disambiguation as that also has the 'entity in a mesh network' entry -->)
- **NR**: New Radio ([nr+])
- **OFDM**: Orthogonal Frequency Division Multiplexing ([generic](https://en.wikipedia.org/wiki/Orthogonal_frequency-division_multiplexing), [nr+])
- **OSCORE**: Object Security for Constrained RESTful Environments ([RFC8613](https://datatracker.ietf.org/doc/html/rfc8613); symmetric security for CoAP, set up through EDHOC)
- **PCC**: Physical Control Channel ([nr+])
- **PCCH**: Physical Control Channel ([nr+])
- **PCH/BCH**: Paging and Broadcast Channel ([nr+])
- **PDC**: Physical Data Channel ([nr+])
- **PHY**: Physical layer (generic, but here: the [nr+] PHY)
- **PT**: Portable Termination point ([nr+])
- **PTC**: Paging Transmission Control ([nr+])
- **RAC**: Random Access Control ([nr+])
- **RACH**: Random Access Channel ([nr+])
- **RD**: Radio Device ([nr+])
- **RF**: Radio Frequency (generic, [nr+])
- **RSSI**: Received Signal Strength Indicator ([generic](https://en.wikipedia.org/wiki/Received_signal_strength_indicator), [nr+])
- **RX**: Receiver (generic, [nr+])
- **RX-TX**: Receive-Transmit (generic, [nr+])
- **SAP**: Service Access Point ([nr+])
- **TDMA**: Time Division Multiple Access ([generic](https://en.wikipedia.org/wiki/Time-division_multiple_access), [nr+])
- **TX**: Transmitter (generic, [nr+])
- **UE**: User Equipment ([nr+])
- **URLLC**: Ultra-Reliable Low Latency Communications (generic, [nr+])

[nr+]: https://www.etsi.org/deliver/etsi_ts/103600_103699/10363601/02.01.01_60/ts_10363601v020101p.pdf#page=7
