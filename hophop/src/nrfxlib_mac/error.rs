// SPDX-FileCopyrightText: Copyright Christian Amsüss <chrysn@fsfe.org>, Silano Systems
// SPDX-License-Identifier: MIT OR Apache-2.0
//! Error types that are produced by `nrfxlib` in the MAC layer.

use nrfxlib_sys_pretty::dect_mac;

// Having MacError used in a Result transparently only works if it can use the zero niche.
const _: () = assert!(dect_mac::err_values::OK == 0);

/// Errors reported by the MAC layer
pub struct MacError(core::num::NonZero<u8>);

impl core::fmt::Debug for MacError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.write_str(match self.0.into() {
            dect_mac::err_values::OK => unreachable!(),
            dect_mac::err_values::FAIL => "FAIL",
            dect_mac::err_values::INVALID_PARAM => "INVALID_PARAM",
            dect_mac::err_values::NOT_ALLOWED => "NOT_ALLOWED",
            dect_mac::err_values::NO_CONFIG => "NO_CONFIG",
            dect_mac::err_values::RD_NOT_FOUND => "RD_NOT_FOUND",
            dect_mac::err_values::TEMP_FAILURE => "TEMP_FAILURE",
            dect_mac::err_values::NO_RESOURCES => "NO_RESOURCES",
            dect_mac::err_values::NO_RESPONSE => "NO_RESPONSE",
            dect_mac::err_values::NW_REJECT => "NW_REJECT",
            dect_mac::err_values::NO_MEMORY => "NO_MEMORY",
            dect_mac::err_values::NO_RSSI_RESULTS => "NO_RSSI_RESULTS",
            dect_mac::err_values::DLC_DISCARD_TIMER_EXPIRED => "DLC_DISCARD_TIMER_EXPIRED",
            _ => "(unknown error)",
        })
    }
}

impl defmt::Format for MacError {
    fn format(&self, fmt: defmt::Formatter) {
        match self.0.into() {
            dect_mac::err_values::OK => unreachable!(),
            dect_mac::err_values::FAIL => defmt::write!(fmt, "FAIL"),
            dect_mac::err_values::INVALID_PARAM => defmt::write!(fmt, "INVALID_PARAM"),
            dect_mac::err_values::NOT_ALLOWED => defmt::write!(fmt, "NOT_ALLOWED"),
            dect_mac::err_values::NO_CONFIG => defmt::write!(fmt, "NO_CONFIG"),
            dect_mac::err_values::RD_NOT_FOUND => defmt::write!(fmt, "RD_NOT_FOUND"),
            dect_mac::err_values::TEMP_FAILURE => defmt::write!(fmt, "TEMP_FAILURE"),
            dect_mac::err_values::NO_RESOURCES => defmt::write!(fmt, "NO_RESOURCES"),
            dect_mac::err_values::NO_RESPONSE => defmt::write!(fmt, "NO_RESPONSE"),
            dect_mac::err_values::NW_REJECT => defmt::write!(fmt, "NW_REJECT"),
            dect_mac::err_values::NO_MEMORY => defmt::write!(fmt, "NO_MEMORY"),
            dect_mac::err_values::NO_RSSI_RESULTS => defmt::write!(fmt, "NO_RSSI_RESULTS"),
            dect_mac::err_values::DLC_DISCARD_TIMER_EXPIRED => {
                defmt::write!(fmt, "DLC_DISCARD_TIMER_EXPIRED")
            }
            _ => defmt::write!(fmt, "(unknown error)"),
        }
    }
}

// FIXME: Our APIs should be tight enough that this is not needed outside.
pub trait MacErrorExt {
    fn into_mac_status(self) -> Result<(), MacError>;
}

impl MacErrorExt for u8 {
    fn into_mac_status(self) -> Result<(), MacError> {
        if let Ok(nonzero) = self.try_into() {
            Err(MacError(nonzero))
        } else {
            Ok(())
        }
    }
}
