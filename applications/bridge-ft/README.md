<!--
SPDX-FileCopyrightText: Copyright Christian Amsüss <chrysn@fsfe.org>, Silano Systems
SPDX-License-Identifier: MIT OR Apache-2.0
-->
bridge-ft
===========

This application sets up DECT IPv6 networking (in NI6W mode)
and connects it out via SLIP. It runs as a sink.

This is incomplete in that

* it does not take scanning results into account,
* it is not configurable (other than by altering the source),
* configuration needs to happen even inside the hophop library (i.e., it hard-codes parameters that should be configurable),
* it does not provide any usable feedback about peers, and
* it does not forward traffic out through slipmux (as the name would imply).

Nonetheless, it can be used as a replacement for the DECT shell at least to the extent that the `-pt` examples can forward traffic across it.
