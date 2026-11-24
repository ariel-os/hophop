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

/// Message sent regularly to keep inactivity notifications from firing.
pub const KEEPALIVE_MESSAGE: [u8; 1] = [0xff];

/// Interval in which a PT has to show some activity before it gets removed.
///
/// Careful, this does not seem to follow the standard ULPeriod / UL_PERIOD timings, but needs to
/// be set on the Nordic MAC per FT. (Issue tracked at
/// <https://github.com/ariel-os/hophop/issues/108>).
///
/// Note that once the notification is issued in the FT, data can not be sent from there any more
/// through DLC, although a "release" message can still be sent.
///
/// This is the timeout as used by the FT; the PTs will create traffic a bit more frequently (after
/// 3/4 of the time).
// Picking a nice round number around 1 minute, not sure if that helps anyone
pub const INACTIVITY_MS: u32 = 65_536;
