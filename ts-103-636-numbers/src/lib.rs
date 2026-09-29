//! <!--
//! SPDX-FileCopyrightText: Copyright Christian Amsüss <chrysn@fsfe.org>, Silano Systems
//! SPDX-License-Identifier: MIT OR Apache-2.0
//! -->
//! This crate contains numeric constants for ETSI TS 103-636 "DECT-2020 New Radio (NR)".
//!
//! This encompasses bitfield values from the specification itself as well as entries in the
//! extensible [DECT-2020 NR Endpoint Multiplexing Address Allocation] mapping.
//!
//! This crate tries to strike a balance between not being opinionated in terms of types and being
//! practical: It uses newtypes of the minimal suitable bit length to express constants, which
//! provide Debug implementations that may include a lot of code (relying on dead code eliminiation
//! to not weigh down users needlessly).
//!
//! [DECT-2020 NR Endpoint Multiplexing Address Allocation]: https://portal.etsi.org/PNNS/Protocol-Specification-Allocation/DECT-2020-NR-Endpoint-Multiplexing-Addresses
#![no_std]

pub mod endpoint_multiplexing;

pub mod mac_details;
pub mod mac_ie;
pub mod mac_pdu;

/// Error used in fallible construction when bits that should have been masked as part of
/// processing an incoming data structure are set, eg. when the MSB of an u8 is set when it gets
/// passed into the constructor of a 6-bit long field's value.
#[derive(Debug)]
pub struct ExcessiveBitsSet;
