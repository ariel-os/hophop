// SPDX-FileCopyrightText: Copyright Christian Amsüss <chrysn@fsfe.org>, Silano Systems
// SPDX-License-Identifier: MIT OR Apache-2.0
//! Processing between a MAC PDU and its IEs (Information Elements)

use super::ParsingError;

use ts_103_636_numbers as numbers;

/// The MAC header as defined in Section 6.3.2 of ETSI TS 103 636-4 V2.1.1.
#[derive(Debug)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct MacHeaderType(pub u8);

impl MacHeaderType {
    /// The 2-bit Version field.
    #[must_use]
    pub fn version(&self) -> u8 {
        self.0 >> 6
    }

    /// The 2-bit MAC Security field.
    #[must_use]
    pub fn mac_security(&self) -> u8 {
        (self.0 >> 4) & 0x03
    }

    /// The 4-bit MAC Header Type field.
    #[must_use]
    pub fn mac_header_type(&self) -> u8 {
        self.0 & 0x0f
    }
}

#[derive(Debug)]
pub struct DataMacPdu<'buf>(pub &'buf [u8; 2]);

/// The DATA MAC PDU header as defined in Section 6.3.3.1 of ETSI TS 103 636-4 V2.1.1.
impl DataMacPdu<'_> {
    /// The single reset bit.
    #[must_use]
    pub fn reset(&self) -> bool {
        ((self.0[0] >> 4) & 1) != 0
    }

    /// The 12 bit sequence number.
    #[must_use]
    pub fn sequence_number(&self) -> u16 {
        (u16::from(self.0[0]) & 0x0f) << 8 | u16::from(self.0[1])
    }
}

#[cfg(feature = "defmt")]
impl defmt::Format for DataMacPdu<'_> {
    fn format(&self, fmt: defmt::Formatter) {
        defmt::write!(
            fmt,
            "DataMacPdu {{ reset: {=bool}, sequence number: {=u16} }}",
            self.reset(),
            self.sequence_number(),
        );
    }
}

/// The Beacon header as defined in Section 6.3.3.2 of ETSI TS 103 636-4 V2.1.1.
#[derive(Debug)]
pub struct Beacon<'buf>(pub &'buf [u8; 7]);

impl Beacon<'_> {
    /// The 24 bit network ID.
    #[must_use]
    pub fn network_id(&self) -> u32 {
        u32::from_be_bytes([0, self.0[0], self.0[1], self.0[2]])
    }

    #[must_use]
    pub fn transmitter_address(&self) -> u32 {
        #[allow(clippy::missing_panics_doc, reason = "panic is unreachable")]
        u32::from_be_bytes(self.0[3..7].try_into().expect("sizes fit"))
    }
}

#[cfg(feature = "defmt")]
impl defmt::Format for Beacon<'_> {
    fn format(&self, fmt: defmt::Formatter) {
        defmt::write!(
            fmt,
            "Beacon {{ network id: {=u32}, transmitter address: {=u32} }}",
            self.network_id(),
            self.transmitter_address(),
        );
    }
}

/// The Unicast header as defined in Section 6.3.3.3 of ETSI TS 103 636-4 V2.1.1.
#[derive(Debug)]
pub struct Unicast<'buf>(pub &'buf [u8; 10]);

impl Unicast<'_> {
    /// The single reset bit.
    #[must_use]
    pub fn reset(&self) -> bool {
        ((self.0[0] >> 4) & 1) != 0
    }

    /// The 12 bit sequence number.
    ///
    /// Note that Figure 6.3.3.3-1 of ETSI TS 103 636-4 V2.1.1 gives separate names to this field,
    /// but the prose speaks of a 12-bit sequence number.
    #[must_use]
    pub fn sequence_number(&self) -> u16 {
        (u16::from(self.0[0]) & 0x0f) << 8 | u16::from(self.0[1])
    }

    /// The 32 bit receiver address.
    #[must_use]
    pub fn receiver_address(&self) -> u32 {
        #[allow(clippy::missing_panics_doc, reason = "panic is unreachable")]
        u32::from_be_bytes(self.0[2..6].try_into().expect("sizes fit"))
    }

    /// The 32 bit transmitter address.
    #[must_use]
    pub fn transmitter_address(&self) -> u32 {
        #[allow(clippy::missing_panics_doc, reason = "panic is unreachable")]
        u32::from_be_bytes(self.0[6..10].try_into().expect("sizes fit"))
    }
}

#[cfg(feature = "defmt")]
impl defmt::Format for Unicast<'_> {
    fn format(&self, fmt: defmt::Formatter) {
        defmt::write!(
            fmt,
            "Unicast {{ reset: {=bool}, sequence number: {=u16}, receiver address: {=u32}, transmitter address: {=u32} }}",
            self.reset(),
            self.sequence_number(),
            self.receiver_address(),
            self.transmitter_address(),
        );
    }
}

/// The RD Broadcasting header as defined in Section 6.3.3.4 of ETSI TS 103 636-4 V2.1.1.
#[derive(Debug)]
pub struct RdBroadcast<'buf>(pub &'buf [u8; 6]);

impl RdBroadcast<'_> {
    /// The single reset bit.
    #[must_use]
    pub fn reset(&self) -> bool {
        ((self.0[0] >> 4) & 1) != 0
    }

    /// The 12 bit sequence number.
    #[must_use]
    pub fn sequence_number(&self) -> u16 {
        (u16::from(self.0[0]) & 0x0f) << 8 | u16::from(self.0[1])
    }

    /// The 32 bit transmitter address.
    #[must_use]
    pub fn transmitter_address(&self) -> u32 {
        #[allow(clippy::missing_panics_doc, reason = "panic is unreachable")]
        u32::from_be_bytes(self.0[2..6].try_into().expect("sizes fit"))
    }
}

#[cfg(feature = "defmt")]
impl defmt::Format for RdBroadcast<'_> {
    fn format(&self, fmt: defmt::Formatter) {
        defmt::write!(
            fmt,
            "RdBroadcast {{ reset: {=bool}, sequence number: {=u16}, transmitter address: {=u32} }}",
            self.reset(),
            self.sequence_number(),
            self.transmitter_address(),
        );
    }
}

#[derive(Debug)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MacCommonHeader<'buf> {
    DataMacPdu(DataMacPdu<'buf>),
    Beacon(Beacon<'buf>),
    Unicast(Unicast<'buf>),
    RdBroadcast(RdBroadcast<'buf>),
}

#[deprecated(note = "renamed to Message")]
pub type Header<'buf> = Message<'buf>;

#[derive(Debug)]
pub struct Message<'buf> {
    pub head: MacHeaderType,
    pub common: MacCommonHeader<'buf>,
    /// IE items; best used with the [`Self::tail_items()`] iterator
    pub tail: &'buf [u8],
}

impl<'buf> Message<'buf> {
    /// Parses a buffer as a MAC PDU, with a header indicating the common header type, a common
    /// header, and a tail of IEs.
    ///
    /// # Errors
    ///
    /// This errs if the MAC version is not the one specified and understood, or on various length
    /// errors.
    #[expect(clippy::missing_panics_doc, reason = "panics are unreachable")]
    pub fn parse(mut buffer: &'buf [u8]) -> Result<Self, ParsingError> {
        let head = *buffer.split_off_first().ok_or(ParsingError)?;
        let head = MacHeaderType(head);
        if head.version() != numbers::mac_pdu::VERSION {
            // version could have different structure
            return Err(ParsingError);
        }

        let common = match head.mac_header_type() {
            numbers::mac_pdu::header_type::DATA_MAC_PDU => MacCommonHeader::DataMacPdu(DataMacPdu(
                buffer
                    .split_off(..2)
                    .ok_or(ParsingError)?
                    .try_into()
                    .expect("split_off length is guaranteed"),
            )),
            numbers::mac_pdu::header_type::BEACON => MacCommonHeader::Beacon(Beacon(
                buffer
                    .split_off(..7)
                    .ok_or(ParsingError)?
                    .try_into()
                    .expect("split_off length is guaranteed"),
            )),
            numbers::mac_pdu::header_type::UNICAST => MacCommonHeader::Unicast(Unicast(
                buffer
                    .split_off(..10)
                    .ok_or(ParsingError)?
                    .try_into()
                    .expect("split_off length is guaranteed"),
            )),
            numbers::mac_pdu::header_type::RD_BROADCAST => {
                MacCommonHeader::RdBroadcast(RdBroadcast(
                    buffer
                        .split_off(..6)
                        .ok_or(ParsingError)?
                        .try_into()
                        .expect("split_off length is guaranteed"),
                ))
            }
            _ => return Err(ParsingError),
        };

        Ok(Self {
            head,
            common,
            tail: buffer,
        })
    }

    /// Convenience accessor for
    /// [`InformationElement::parse_stream(Self::tail)`][crate::mac_ie::InformationElement::parse_stream].
    pub fn tail_items(
        &self,
    ) -> impl Iterator<Item = Result<crate::mac_ie::InformationElement<'_>, ParsingError>> {
        crate::mac_ie::InformationElement::parse_stream(self.tail)
    }
}

#[cfg(feature = "defmt")]
impl defmt::Format for Message<'_> {
    fn format(&self, fmt: defmt::Formatter) {
        defmt::write!(
            fmt,
            // Version not shown because it is asserted at parse time that this is the one that
            // exists.
            //
            // Header type not shown because that is shown right after in .common's type name.
            "Security: {=u8}, ",
            self.head.mac_security(),
        );
        match &self.common {
            MacCommonHeader::DataMacPdu(inner) => inner.format(fmt),
            MacCommonHeader::Beacon(inner) => inner.format(fmt),
            MacCommonHeader::Unicast(inner) => inner.format(fmt),
            MacCommonHeader::RdBroadcast(inner) => inner.format(fmt),
        }
        defmt::write!(fmt, ", IEs:");
        for ie in self.tail_items() {
            if let Ok(ie) = ie {
                defmt::write!(fmt, "\n    - {}", ie);
            } else {
                defmt::write!(fmt, "\n    Rest is unparsable: {=[u8]:02x}", self.tail);
                break;
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parse_header() {
        let beacon = &[
            1, 18, 52, 86, 0, 0, 0, 38, 73, 5, 176, 16, 6, 0, 13, 83, 7, 8, 12, 138, 160, 215, 2,
            100, 64, 24, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        ];

        let beacon = Header::parse(&beacon[..]).unwrap();
        let MacCommonHeader::Beacon(common) = beacon.common else {
            panic!("Test vector encodes a beacon but was not recognized as such");
        };
        assert_eq!(common.network_id(), 0x123456);
        assert_eq!(common.transmitter_address(), 0x26);
        // Detailed parsing of that very string is tested in mac_ie.rs
        assert!(matches!(beacon.tail, [73, 5, .., 0]));
    }
}
