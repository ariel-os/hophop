// SPDX-FileCopyrightText: Copyright Christian Amsüss <chrysn@fsfe.org>, Silano Systems
// SPDX-License-Identifier: MIT OR Apache-2.0
//! Helpers for running in Nordic's NI6W mode
//!
//! Some parts of hophop currently do **not** implement proper IPv6 over nr+ (ETSI TS 103 874-3),
//! but merely "dump data directly into DLC in any flow" -- Nordic's IPv6 Workaround (NI6W) (see
//! also [their DevZone entry](https://devzone.nordicsemi.com/f/nordic-q-a/128194/dect-shell-ipv6-and-etsi-ts-103-874-3)).
//!
//! As the nRF MAC also does keepalives differently than the standard, NI6W is extended for hophop
//! by sending a single `0xff` byte ([`KEEPALIVE_MESSAGE`]) to avoid getting removed from the
//! network due to timeouts.

pub const KEEPALIVE_MESSAGE: [u8; 1] = [0xff];
