// SPDX-FileCopyrightText: Copyright Christian Amsüss <chrysn@fsfe.org>, Silano Systems
// SPDX-License-Identifier: MIT OR Apache-2.0
//! Types for numbers commonly used in ETSI TS 103-636 "DECT-2020 New Radio (NR)".
//!
//! This covers types that are predominantly not backed by constants, as those would be provided in
//! [`ts_103_636_numbers`]. (Some types still may have constants, eg. for a single "broadcast"
//! value).

use core::num::NonZero;

/// An absolute channel number.
///
/// Following Section 5.4.2 of ETSI TS 103 636-2 V2.1.1
///
/// This type uses 13 bit (expressed in a u16).
#[derive(Copy, Clone, PartialEq, Eq)]
pub struct AbsoluteChannel(NonZero<u16>);

impl AbsoluteChannel {
    #[must_use]
    pub const fn new(number: u16) -> Option<Self> {
        // Could be
        //   NonZero::new(number).map(Self)
        // but we have to do this until <https://github.com/rust-lang/rust/issues/143956> is
        // stable:
        match NonZero::new(number) {
            Some(n) => Some(Self(n)),
            None => None,
        }
    }
}

impl From<AbsoluteChannel> for u16 {
    fn from(value: AbsoluteChannel) -> Self {
        value.0.into()
    }
}

impl From<AbsoluteChannel> for NonZero<u16> {
    fn from(value: AbsoluteChannel) -> Self {
        value.0
    }
}

impl core::fmt::Debug for AbsoluteChannel {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        // Does using a write rather than just deferring really get us rid of a :x on a surrounding
        // struct? (It should: We use hex for identifiers but decimal for channels).
        write!(f, "{}", self.0.get())
    }
}

impl core::fmt::Display for AbsoluteChannel {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}", self.0.get())
    }
}

#[cfg(feature = "defmt")]
impl defmt::Format for AbsoluteChannel {
    fn format(&self, fmt: defmt::Formatter) {
        defmt::write!(fmt, "{=u16}", self.0.get());
    }
}

/// A full (32-bit) Network ID.
///
/// Following Section 4.2.3.1 of ETSI TS 103 636-4 V2.1.1.
///
/// It can be split into the MSB [`NetworkId24`] and the LSB [`NetworkId8`].
///
/// An invariant of this type is that neither part of it is zero. Violating it may cause panics;
/// unsoundness would only arrive from violating the more stronly enforced `NonZero` (which covers
/// the special case of both being zero, and allows niche optimizaiton for options).
#[derive(Copy, Clone, PartialEq, Eq)]
pub struct NetworkId32(NonZero<u32>);

impl NetworkId32 {
    const MASK_MSB: u32 = 0xffff_ff00;
    const MASK_LSB: u32 = 0x0000_00ff;

    #[must_use]
    pub const fn new(network_id: u32) -> Option<Self> {
        if network_id & Self::MASK_MSB == 0 || network_id & Self::MASK_LSB == 0 {
            return None;
        }

        // At this point it is sure that this will succeed; trying to be clever about it is not
        // worth it.

        // Could be
        //   NonZero::new(network_id).map(Self)
        // but we have to do this until <https://github.com/rust-lang/rust/issues/143956> is
        // stable:
        match NonZero::new(network_id) {
            Some(n) => Some(Self(n)),
            None => None,
        }
    }

    #[must_use]
    #[allow(clippy::missing_panics_doc, reason = "using type invariant")]
    pub const fn msb(self) -> NetworkId24 {
        NetworkId24::new(self.0.get() >> 8).expect("own value is nonzero even there")
    }

    #[must_use]
    #[allow(clippy::missing_panics_doc, reason = "using type invariant")]
    pub const fn lsb(self) -> NetworkId8 {
        #[allow(clippy::cast_possible_truncation, reason = "truncation is intended")]
        NetworkId8::new(self.0.get() as u8).expect("own value is nonzero even there")
    }
}

impl From<NetworkId32> for u32 {
    fn from(value: NetworkId32) -> Self {
        value.0.into()
    }
}

impl From<NetworkId32> for NonZero<u32> {
    fn from(value: NetworkId32) -> Self {
        value.0
    }
}

impl core::fmt::Debug for NetworkId32 {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "0x{:08x}", u32::from(self.0))
    }
}

#[cfg(feature = "defmt")]
impl defmt::Format for NetworkId32 {
    fn format(&self, fmt: defmt::Formatter) {
        defmt::write!(fmt, "0x{=u32:08x}", u32::from(self.0));
    }
}

/// The most significant part of a [`NetworkId32`], transmitted in beacons and used for scrambling
/// non-beacon packets.
///
/// An invariant of this type is that it is not zero.
#[derive(Copy, Clone, PartialEq, Eq)]
pub struct NetworkId24(
    /// This is stored in right-shifted form (i.e. of the shape 0x00NNNNN)
    NonZero<u32>,
);

impl NetworkId24 {
    const MASK: u32 = 0xff_ffff;

    #[must_use]
    pub const fn new(network_id: u32) -> Option<Self> {
        if network_id & Self::MASK == 0 {
            return None;
        }

        // At this point it is sure that this will succeed; trying to be clever about it is not
        // worth it.

        // Could be
        //   NonZero::new(network_id).map(Self)
        // but we have to do this until <https://github.com/rust-lang/rust/issues/143956> is
        // stable:
        match NonZero::new(network_id) {
            Some(n) => Some(Self(n)),
            None => None,
        }
    }

    /// The ID stored in a u32 in the highest bit positions, where it would be in a full RD ID.
    #[must_use]
    pub fn into_high_u32(self) -> u32 {
        u32::from(self.0) << 8
    }

    /// The ID stored in a u32 in the lower bit positions, matching its numeric value.
    #[must_use]
    pub fn into_low_u32(self) -> u32 {
        u32::from(self.0) << 8
    }
}

impl core::fmt::Debug for NetworkId24 {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "0x{:06x}..", u32::from(self.0))
    }
}

#[cfg(feature = "defmt")]
impl defmt::Format for NetworkId24 {
    fn format(&self, fmt: defmt::Formatter) {
        defmt::write!(fmt, "0x{=u32:06x}..", u32::from(self.0));
    }
}

/// The least significant part of a [`NetworkId32`], transmitted in the PHY control field.
///
/// An invariant of this type is that it is not zero.
#[derive(Copy, Clone, PartialEq, Eq)]
pub struct NetworkId8(NonZero<u8>);

impl NetworkId8 {
    #[must_use]
    pub const fn new(network_id: u8) -> Option<Self> {
        // Could be
        //   NonZero::new(network_id).map(Self)
        // but we have to do this until <https://github.com/rust-lang/rust/issues/143956> is
        // stable:
        match NonZero::new(network_id) {
            Some(n) => Some(Self(n)),
            None => None,
        }
    }
}

impl From<NetworkId8> for u8 {
    fn from(value: NetworkId8) -> Self {
        value.0.into()
    }
}

impl From<NetworkId8> for NonZero<u8> {
    fn from(value: NetworkId8) -> Self {
        value.0
    }
}

impl core::fmt::Debug for NetworkId8 {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "0x…{:06x}", u8::from(self.0))
    }
}

#[cfg(feature = "defmt")]
impl defmt::Format for NetworkId8 {
    fn format(&self, fmt: defmt::Formatter) {
        defmt::write!(fmt, "0x…{=u8:02x}", u8::from(self.0));
    }
}

/// A Long Radio Device ID (Long RD ID).
///
/// Following Section 4.2.3.2 of ETSI TS 103 636-4 V2.1.1.
///
/// An invariant of this type is that it does not use the reserved address zero.
#[derive(Copy, Clone, PartialEq, Eq)]
pub struct LongRdId(NonZero<u32>);

impl LongRdId {
    pub const BACKEND: Self = LongRdId(NonZero::new(0xffff_fffe).unwrap());
    pub const BROADCAST: Self = LongRdId(NonZero::new(0xffff_ffff).unwrap());

    /// Loads a 32bit numeric RD ID.
    ///
    /// This accepts all of the regular ranges (1-0xfffffffd), the backend and the broadcast
    /// address, but the reserved address produces None.
    #[must_use]
    pub const fn new(rd_id: u32) -> Option<Self> {
        // Could be
        //   NonZero::new(network_id).map(Self)
        // but we have to do this until <https://github.com/rust-lang/rust/issues/143956> is
        // stable:
        match NonZero::new(rd_id) {
            Some(n) => Some(Self(n)),
            None => None,
        }
    }
}

impl From<LongRdId> for u32 {
    fn from(value: LongRdId) -> Self {
        value.0.into()
    }
}

impl From<LongRdId> for NonZero<u32> {
    fn from(value: LongRdId) -> Self {
        value.0
    }
}

/// A Short Radio Device ID (Short RD ID).
///
/// Following Section 4.2.3.3 of ETSI TS 103 636-4 V2.1.1.
///
/// An invariant of this type is that it does not use the reserved address zero.
#[derive(Copy, Clone, PartialEq, Eq)]
pub struct ShortRdId(NonZero<u16>);

impl ShortRdId {
    pub const BROADCAST: Self = ShortRdId(NonZero::new(0xffff).unwrap());

    /// Loads a 16bit numeric RD ID.
    ///
    /// This accepts both the regular ranges (1-0xfffe), and the broadcast address, but the
    /// reserved address produces None.
    #[must_use]
    pub const fn new(rd_id: u16) -> Option<Self> {
        // Could be
        //   NonZero::new(network_id).map(Self)
        // but we have to do this until <https://github.com/rust-lang/rust/issues/143956> is
        // stable:
        match NonZero::new(rd_id) {
            Some(n) => Some(Self(n)),
            None => None,
        }
    }
}

impl From<ShortRdId> for u16 {
    fn from(value: ShortRdId) -> Self {
        value.0.into()
    }
}

impl From<ShortRdId> for NonZero<u16> {
    fn from(value: ShortRdId) -> Self {
        value.0
    }
}
