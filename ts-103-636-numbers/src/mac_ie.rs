// SPDX-FileCopyrightText: Copyright Christian Amsüss <chrysn@fsfe.org>, Silano Systems
// SPDX-License-Identifier: MIT OR Apache-2.0
//! IE (Information Element) type
#![allow(
    clippy::unreadable_literal,
    reason = "values are use more with copy-pasting and searching than with actual reaidng of the value"
)]

/// An IE type as uses with MAC Extension field encodings 00/01/10
///
/// ## Invariants
///
/// The inner field only has the lowest 6 bit set. This is not a safety invariant: This module will
/// only either produce wrong results or panic if the upper bits are set.
#[derive(PartialEq, Eq, Copy, Clone)]
pub struct IEType6bit(u8);

impl IEType6bit {
    /// Definition of the element from Table 6.3.4-2
    ///
    /// Editorial liberty is used to convert remove "IE" and "message" suffixes.
    const fn description(self) -> Option<&'static str> {
        Some(match self.0 {
            // Not using the equivalent constants: that wouldn't make it any mor readable, and if
            // this gets extended frequently, it needs X-macro or code generation driven
            // population anyway.
            0b000000 => "Padding",
            0b000001 => "Higher layer signalling - flow 1",
            0b000010 => "Higher layer signalling - flow 2",
            0b000011 => "User plane data - flow 1",
            0b000100 => "User plane data - flow 2",
            0b000101 => "User plane data - flow 3",
            0b000110 => "User plane data - flow 4",
            0b001000 => "Network Beacon",
            0b001001 => "Cluster Beacon",
            0b001010 => "Association Request",
            0b001011 => "Association Response",
            0b001100 => "Association Release",
            0b001101 => "Reconfiguration Request",
            0b001110 => "Reconfiguration Response",
            0b001111 => "Additional MAC messages",
            0b010000 => "MAC Security Info",
            0b010001 => "Route Info",
            0b010010 => "Resource allocation",
            0b010011 => "Random Access Resource",
            0b010100 => "RD capability",
            0b010101 => "Neighbouring",
            0b010110 => "Broadcast Indication",
            0b010111 => "Group Assignment",
            0b011000 => "Load Info",
            0b011001 => "Measurement Report",
            0b011010 => "Source Routing",
            0b011011 => "Joining Beacon",
            0b011100 => "Joining Information",
            0b011110 => "Escape",
            0b011111 => "IE type extension",
            _ => return None,
        })
    }
}

pub mod ie6bit {
    // Those are not associated constants of EndpointMultiplexingAddress because struct associated
    // constants can not be wildcard imported (and for those a wildcard import makes a lot of sense).
    use super::IEType6bit;

    pub const PADDING: IEType6bit = IEType6bit(0b000000);
    pub const HIGHER_LAYER_SIGNALLING_FLOW_1: IEType6bit = IEType6bit(0b000001);
    pub const HIGHER_LAYER_SIGNALLING_FLOW_2: IEType6bit = IEType6bit(0b000010);
    pub const USER_PLANE_DATA_FLOW_1: IEType6bit = IEType6bit(0b000011);
    pub const USER_PLANE_DATA_FLOW_2: IEType6bit = IEType6bit(0b000100);
    pub const USER_PLANE_DATA_FLOW_3: IEType6bit = IEType6bit(0b000101);
    pub const USER_PLANE_DATA_FLOW_4: IEType6bit = IEType6bit(0b000110);
    pub const NETWORK_BEACON: IEType6bit = IEType6bit(0b001000);
    pub const CLUSTER_BEACON: IEType6bit = IEType6bit(0b001001);
    pub const ASSOCIATION_REQUEST: IEType6bit = IEType6bit(0b001010);
    pub const ASSOCIATION_RESPONSE: IEType6bit = IEType6bit(0b001011);
    pub const ASSOCIATION_RELEASE: IEType6bit = IEType6bit(0b001100);
    pub const RECONFIGURATION_REQUEST: IEType6bit = IEType6bit(0b001101);
    pub const RECONFIGURATION_RESPONSE: IEType6bit = IEType6bit(0b001110);
    pub const ADDITIONAL_MAC_MESSAGES: IEType6bit = IEType6bit(0b001111);
    pub const MAC_SECURITY_INFO: IEType6bit = IEType6bit(0b010000);
    pub const ROUTE_INFO: IEType6bit = IEType6bit(0b010001);
    pub const RESOURCE_ALLOCATION: IEType6bit = IEType6bit(0b010010);
    pub const RANDOM_ACCESS_RESOURCE: IEType6bit = IEType6bit(0b010011);
    pub const RD_CAPABILITY: IEType6bit = IEType6bit(0b010100);
    pub const NEIGHBOURING: IEType6bit = IEType6bit(0b010101);
    pub const BROADCAST_INDICATION: IEType6bit = IEType6bit(0b010110);
    pub const GROUP_ASSIGNMENT: IEType6bit = IEType6bit(0b010111);
    pub const LOAD_INFO: IEType6bit = IEType6bit(0b011000);
    pub const MEASUREMENT_REPORT: IEType6bit = IEType6bit(0b011001);
    pub const SOURCE_ROUTING: IEType6bit = IEType6bit(0b011010);
    pub const JOINING_BEACON: IEType6bit = IEType6bit(0b011011);
    pub const JOINING_INFORMATION: IEType6bit = IEType6bit(0b011100);
    pub const ESCAPE: IEType6bit = IEType6bit(0b011110);
    pub const IE_TYPE_EXTENSION: IEType6bit = IEType6bit(0b011111);
}

impl core::fmt::Debug for IEType6bit {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let mut debugtuple = f.debug_struct("IEType6bit");
        debugtuple.field(".0", &format_args!("{:#04x}", self.0));
        if let Some(description) = self.description() {
            debugtuple.field("description", &description);
        }
        debugtuple.finish()
    }
}

#[cfg(feature = "defmt")]
impl defmt::Format for IEType6bit {
    fn format(&self, f: defmt::Formatter<'_>) {
        if let Some(description) = self.description() {
            defmt::write!(
                f,
                "IEType6bit {{ .0: {:#04x}, description: {} }}",
                self.0,
                description
            );
        } else {
            defmt::write!(f, "IEType6bit {{ .0: {:#04x} }}", self.0);
        }
    }
}

impl TryFrom<u8> for IEType6bit {
    type Error = super::ExcessiveBitsSet;

    #[inline]
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        if value & !0x3f == 0 {
            Ok(Self(value))
        } else {
            Err(super::ExcessiveBitsSet)
        }
    }
}

impl From<IEType6bit> for u8 {
    #[inline]
    fn from(value: IEType6bit) -> Self {
        value.0
    }
}

/// An IE type as uses with MAC Extension field encoding 11
///
/// Note that this type also encodes the byte length, making it 6-bit again; in a sense, it joins
/// tables 6.3.4-3 and 6.3.4-4 by composing the length into the key.
///
/// Unlike the 6-bit variant that has a trivial conversion to u8, this uses deciated `from_` and
/// accessor methods to declare that it is the value combined with the length bit that gets transported.
///
/// ## Invariants
///
/// The inner field only has the lowest 6 bit set. This is not a safety invariant: This module will
/// only either produce wrong results or panic if the upper bits are set.
#[derive(PartialEq, Eq, Copy, Clone)]
pub struct IEType5bit(u8);

impl IEType5bit {
    /// Definition of the element from Table 6.3.4-3 and -4
    ///
    /// Editorial liberty is used to convert remove "IE" and "message" suffixes.
    const fn description(self) -> Option<&'static str> {
        #[allow(
            clippy::match_same_arms,
            reason = "items are sorted and semantically grouped"
        )]
        Some(match self.0 {
            0b0_00000 => "Padding",
            0b0_00001 => "Configuration Request",
            0b0_00010 => "Keep alive",
            0b0_10000 => "MAC Security Info",
            0b0_11110 => "Escape",

            0b1_00000 => "Padding",
            0b1_00001 => "Radio Device Status",
            0b1_00010 => "RD capability short",
            0b1_00011 => "Association Control",
            0b1_11110 => "Escape",

            _ => return None,
        })
    }

    /// Returns the length of the IE (0 or 1)
    #[expect(
        clippy::len_without_is_empty,
        reason = "emptiness is not distinct here"
    )]
    #[must_use]
    pub const fn len(self) -> usize {
        self.len_u8() as _
    }

    /// Like [`Self::len()`], but using the knowlege that it fits in a u8.
    #[must_use]
    #[inline]
    const fn len_u8(self) -> u8 {
        self.0 >> 5
    }

    /// Returns the numeric value of the IE (5 bit)
    #[must_use]
    pub const fn value(self) -> u8 {
        self.0 & 0x1f
    }

    /// Returns the combined length-and-value bits
    #[must_use]
    pub const fn composite(self) -> u8 {
        self.0
    }

    /// Creates an IE label from its components.
    ///
    /// This is the inverse function of the tuple created from [`Self::len()`] and
    /// [`Self::value()`].
    ///
    /// # Errors
    ///
    /// Errs if length is not in (0, 1), or the value exceed the 5 lowest bits.
    pub const fn try_from_len_and_value(
        len: usize,
        value: u8,
    ) -> Result<Self, super::ExcessiveBitsSet> {
        if len >= 2 || value & !0x1f != 0 {
            return Err(super::ExcessiveBitsSet);
        }
        #[allow(
            clippy::cast_possible_truncation,
            reason = "length was checked, and idiomatic alternatives don't work in const yet"
        )]
        Ok(Self(((len as u8) << 5) | value))
    }

    /// Creates an IE label from its combined length-and-value bits.
    ///
    /// This is the inverse function of the tuple created from [`Self::composite()`].
    ///
    /// # Errors
    ///
    /// Errs if input exceeds the 6 lowest bits.
    pub const fn try_from_composite(composite: u8) -> Result<Self, super::ExcessiveBitsSet> {
        if composite & !0x3f == 0 {
            Ok(Self(composite))
        } else {
            Err(super::ExcessiveBitsSet)
        }
    }
}

pub mod ie5bit_len0 {
    use super::IEType5bit;

    pub const PADDING: IEType5bit = IEType5bit(0b0_00000);
    pub const CONFIGURATION_REQUEST: IEType5bit = IEType5bit(0b0_00001);
    pub const KEEP_ALIVE: IEType5bit = IEType5bit(0b0_00010);
    pub const MAC_SECURITY_INFO: IEType5bit = IEType5bit(0b0_10000);
    pub const ESCAPE: IEType5bit = IEType5bit(0b0_11110);
}

pub mod ie5bit_len1 {
    use super::IEType5bit;

    pub const PADDING: IEType5bit = IEType5bit(0b1_00000);
    pub const RADIO_DEVICE_STATUS: IEType5bit = IEType5bit(0b1_00001);
    pub const RD_CAPABILITY_SHORT: IEType5bit = IEType5bit(0b1_00010);
    pub const ASSOCIATION_CONTROL: IEType5bit = IEType5bit(0b1_00011);
    pub const ESCAPE: IEType5bit = IEType5bit(0b1_11110);
}

impl core::fmt::Debug for IEType5bit {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let mut debugtuple = f.debug_struct("IEType5bit");
        debugtuple.field(".len", &self.len());
        debugtuple.field(".value", &format_args!("{:#04x}", self.value()));
        if let Some(description) = self.description() {
            debugtuple.field("description", &description);
        }
        debugtuple.finish()
    }
}

#[cfg(feature = "defmt")]
impl defmt::Format for IEType5bit {
    fn format(&self, f: defmt::Formatter<'_>) {
        if let Some(description) = self.description() {
            defmt::write!(
                f,
                "IEType5bit {{ .len: {}, .value {:#04x}, description: {} }}",
                self.len_u8(),
                self.value(),
                description
            );
        } else {
            defmt::write!(
                f,
                "IEType5bit {{ .len {}, .value: {:#04x} }}",
                self.len_u8(),
                self.value(),
            );
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_convert() {
        IEType6bit::try_from(0xff).unwrap_err();

        assert_eq!(u8::from(IEType6bit::try_from(0x3f).unwrap()), 0x3f);

        assert_eq!(IEType5bit::try_from_composite(0).unwrap().len(), 0);
        assert_eq!(IEType5bit::try_from_composite(0b1_10001).unwrap().len(), 1);
        assert_eq!(
            IEType5bit::try_from_composite(0b1_10001).unwrap().value(),
            0b10001
        );
        assert_eq!(
            IEType5bit::try_from_len_and_value(1, 0b10001)
                .unwrap()
                .composite(),
            0b1_10001
        );
        assert_eq!(
            IEType5bit::try_from_len_and_value(0, 0b11111)
                .unwrap()
                .composite(),
            0b11111
        );

        IEType5bit::try_from_composite(0xff).unwrap_err();
        IEType5bit::try_from_len_and_value(0, 0x3f).unwrap_err();
        IEType5bit::try_from_len_and_value(2, 0).unwrap_err();
    }

    #[test]
    fn test_debug() {
        extern crate std;
        use std::format;

        assert_eq!(
            &format!("{:?}", IEType6bit::try_from(0b001000).unwrap()),
            &"IEType6bit { .0: 0x08, description: \"Network Beacon\" }"
        );

        assert_eq!(
            // reserved
            &format!("{:?}", IEType6bit::try_from(0b011101).unwrap()),
            &"IEType6bit { .0: 0x1d }"
        );

        assert_eq!(
            &format!("{:?}", IEType5bit::try_from_composite(0b100010).unwrap()),
            &"IEType5bit { .len: 1, .value: 0x02, description: \"RD capability short\" }"
        );

        assert_eq!(
            // reserved
            &format!("{:?}", IEType5bit::try_from_composite(0b100100).unwrap()),
            &"IEType5bit { .len: 1, .value: 0x04 }"
        );
    }
}
