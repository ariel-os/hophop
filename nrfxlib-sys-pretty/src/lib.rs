//! <!--
//! SPDX-FileCopyrightText: Copyright Christian Amsüss <chrysn@fsfe.org>, Silano Systems
//! SPDX-License-Identifier: MIT OR Apache-2.0
//! -->
//! This crate serves several purposes at once around the [`nrfxlib_sys`] crate, focusing on the
//! DECT aspects (but could be extended to ther APIs in the same style):
//!
//! * It re-exports its items by splitting them in semantic components. This way, code using it
//!   does not become cluttered with `nrf_modem_dect_mac_…` all over the place, and more
//!   importantly, names such as `nrf_modem_dect_phy_err_NRF_MODEM_DECT_PHY_ERR_COMBINED_OP_FAILED`
//!   are accessible as [`dect_phy::err_values::ERR_COMBINED_OP_FAILED`].
//!
//!   This include splitting enums into modules. (Ideally, they'd just be associated consts on the
//!   type, but that'd require changing bindgen output).
//!
//! * It enhances documentation by cross-linking items:
//!
//!   * Enum values are not linked from the enum type in bindgen. (This might also be addressed at
//!     bindgen, probably).
//!   * FIXME: It moves bindgen pieces (for anonymous structs or unions) out of the way.
//!   * FIXME: It splits event and notify things and links them
//!
//! * FIXME: It gathers manually added comments where nrfxlib documentation is lacking.
#![no_std]
#![allow(clippy::needless_raw_string_hashes, reason = "simplifies inlining")]

#[rustfmt::skip]
mod built;
pub use built::*;

// Or should we make things searchable this way:

// impl DectCall for calling_function {
//     return_callback:
//     return_type:JustStatus into arg
// }
