//! <!--
//! SPDX-FileCopyrightText: Copyright Christian Amsüss <chrysn@fsfe.org>, Silano Systems
//! SPDX-License-Identifier: MIT OR Apache-2.0
//! -->
#![no_std]
// FIXME: As the crate matures, we'll want this tested as with the other crates, but until the
// crate is a bit less experimental, this is excessive.
#![allow(clippy::pedantic)]

pub mod nrfxlib_phy;

pub mod nrfxlib_mac;
