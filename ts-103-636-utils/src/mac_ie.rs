// SPDX-FileCopyrightText: Copyright Christian Amsüss <chrysn@fsfe.org>, Silano Systems
// SPDX-License-Identifier: MIT OR Apache-2.0
//! Processing of MAC IEs (Information Elements)

use super::{InputLengthError, ParsingError};
use ts_103_636_numbers as numbers;

/// A single IE of the MAC layer.
///
/// This is mainly for parsing, but can also be used to serialize from owned values into a buffer.
///
/// # Invariant
///
/// The payload length is always consistent the header (i.e. it is 0 or 1 if it's a short header,
/// matches the constant length for no-length headers, and fits in the length type indicated
/// otherwise).
///
/// Failure to upholad this in internal code will result in panics or inconsistent output, but this
/// is no soundness requirement, so it will not result in UB.
// FIXME Might be useful to use the `yoke` crate?
pub struct InformationElement<'a> {
    /// The first byte of the IE.
    head: u8,
    /// The payload of the IE.
    ///
    /// Depending on the head, this may be bytes that are serialized directly after the head, or
    /// 1-2 bytes (length) later.
    payload: &'a [u8],
}

impl core::fmt::Debug for InformationElement<'_> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("InformationElement")
            .field("head", &self.head)
            .field("head.ie", &format_args!("{:?}", self.ie_number()))
            .field("payload", &self.payload)
            .finish()
    }
}

#[cfg(feature = "defmt")]
impl defmt::Format for InformationElement<'_> {
    fn format(&self, fmt: defmt::Formatter) {
        if self.ie_number().is_padding() && self.payload.iter().all(|x| *x == 0) {
            defmt::write!(
                fmt,
                "{}, {} byte 0x00",
                self.ie_number(),
                self.payload.len()
            );
            return;
        }
        defmt::write!(fmt, "{}, payload: {=[u8]}", self.ie_number(), self.payload,);
    }
}

/// Unifying type over different IE registries
///
/// It might make sense to shape `ts_103_636_numbers` more in that direction; then it'll be a plain
/// enum and just have instance methods to turn it into a header with any given payload length.
#[derive(PartialEq)]
pub enum AnyIeType {
    Type6bit(numbers::mac_ie::IEType6bit),
    Type5bit(numbers::mac_ie::IEType5bit),
}

impl AnyIeType {
    fn is_padding(&self) -> bool {
        match self {
            AnyIeType::Type6bit(numbers::mac_ie::ie6bit::PADDING) => true,
            AnyIeType::Type5bit(
                numbers::mac_ie::ie5bit_len0::PADDING | numbers::mac_ie::ie5bit_len1::PADDING,
            ) => true,
            _ => false,
        }
    }
}

// Convenience while we don't use a unified enum (for `.ie_number() == constant`)
impl PartialEq<numbers::mac_ie::IEType6bit> for AnyIeType {
    fn eq(&self, other: &numbers::mac_ie::IEType6bit) -> bool {
        if let AnyIeType::Type6bit(slef) = self {
            slef == other
        } else {
            false
        }
    }
}

impl core::fmt::Debug for AnyIeType {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Type6bit(t) => t.fmt(f),
            Self::Type5bit(t) => t.fmt(f),
        }
    }
}

#[cfg(feature = "defmt")]
impl defmt::Format for AnyIeType {
    fn format(&self, f: defmt::Formatter<'_>) {
        match self {
            Self::Type6bit(t) => t.format(f),
            Self::Type5bit(t) => t.format(f),
        }
    }
}

impl<'a> InformationElement<'a> {
    /// Creates an IE out of a 6-bit MAC IE with explicit length.
    ///
    /// # Errors
    ///
    /// This errs if the input data can not be expressed in 16 bit length.
    ///
    /// # Future development
    ///
    /// If it turns out that the [`ts_103_636_numbers`] module is better off with a unified MAC IE
    /// type, and there is never a reason to use a long version of an option when a short (5-bit)
    /// or even implicit-length version would do, then this will be deprecated in favor of a less
    /// explicit version.
    pub fn new_6bit_with_length(
        type_: numbers::mac_ie::IEType6bit,
        payload: &'a [u8],
    ) -> Result<Self, InputLengthError> {
        if payload.len() > u16::MAX.into() {
            return Err(InputLengthError);
        }

        let head = if payload.len() <= u8::MAX.into() {
            u8::from(type_) | (numbers::mac_pdu::mux_ext::LENGTH_8BIT << 6)
        } else {
            u8::from(type_) | (numbers::mac_pdu::mux_ext::LENGTH_16BIT << 6)
        };

        Ok(Self { head, payload })
    }

    /// Creates an IE out of a 5-bit MAC IE ("Short IE")
    ///
    /// # Errors
    ///
    /// This errs if the input data length does not match the IE type.
    ///
    /// # Future development
    ///
    /// See [`Self::new_6bit_with_length`].
    pub fn new_5bit(
        type_: numbers::mac_ie::IEType5bit,
        payload: &'a [u8],
    ) -> Result<Self, InputLengthError> {
        if payload.len() != type_.len() {
            return Err(InputLengthError);
        }

        let head = type_.composite() | (numbers::mac_pdu::mux_ext::SHORT_IE << 6);

        Ok(Self { head, payload })
    }

    /// Reads an IE out of a MAC IE stream.
    ///
    /// The success result is an item in that buffer; in that case, the input slice is shortened to
    /// the remaining data.
    ///
    /// # Errors
    ///
    /// Given the structure of the IEs, the only syntactic parsing error produced by this is not
    /// having enough data.
    ///
    /// A second case that can produce parsing errors are MAC_Ext 00 (IE type defines length)
    /// values where the IE is not known; those too produce a paring error, as they have the same
    /// effect of terminating processing prematurely and irrecoverably.
    ///
    /// On error, the input slice is still advanced, but not left in a position where further
    /// elements can be expected to be decoded.
    #[expect(
        clippy::missing_panics_doc,
        reason = "Local invariant ensures this does not happen"
    )]
    pub fn parse<'buf>(data: &mut &'buf [u8]) -> Result<InformationElement<'buf>, ParsingError> {
        let head = *data.split_off_first().ok_or(ParsingError)?;
        let mac_ext = head >> 6;
        let len: u16 = match mac_ext {
            numbers::mac_pdu::mux_ext::NO_LENGTH_FIELD => {
                let ie_type = head & 0x3f;
                let _ie_type = numbers::mac_ie::IEType6bit::try_from(ie_type).expect("Bits masked");
                // FIXME: We won't just have to query the IE type, but also let the remaining
                // length and the data in the IE influence it -- see
                // <https://github.com/ariel-os/hophop/issues/11>
                let len: Option<u16> = None;
                len.ok_or(ParsingError)?
            }
            numbers::mac_pdu::mux_ext::LENGTH_8BIT => {
                let len = *data.split_off_first().ok_or(ParsingError)?;
                len.into()
            }
            numbers::mac_pdu::mux_ext::LENGTH_16BIT => {
                let high = *data.split_off_first().ok_or(ParsingError)?;
                let low = *data.split_off_first().ok_or(ParsingError)?;
                (u16::from(high) << 8) | u16::from(low)
            }
            numbers::mac_pdu::mux_ext::SHORT_IE => ((head >> 5) & 1).into(),
            _ => unreachable!("Bit shift only admits those values"),
        };
        let payload = data.split_off(..len as usize).ok_or(ParsingError)?;
        Ok(InformationElement { head, payload })
    }

    pub fn parse_stream(
        mut data: &[u8],
    ) -> impl Iterator<Item = Result<InformationElement<'_>, ParsingError>> {
        // There might be room for a bit more optimized and a bit more ideal from_result_fn that
        // iterates as long as there are Ok items, but this works, especially as we can exhaust the
        // data slice.
        core::iter::from_fn(move || {
            if data.is_empty() {
                return None;
            }
            match Self::parse(&mut data) {
                Ok(item) => Some(Ok(item)),
                Err(e) => {
                    data = &[];
                    Some(Err(e))
                }
            }
        })
    }

    /// Number encoded in the header.
    ///
    /// This may not be ideal API yet; it produces what can easily be expressed in terms of the
    /// [`ts_103_636_numbers::mac_ie`] as wrapped in an [`AnyIeType`] union.
    #[expect(
        clippy::missing_panics_doc,
        reason = "Local invariant ensures this does not happen"
    )]
    #[must_use]
    pub fn ie_number(&self) -> AnyIeType {
        let low_6_bits = self.head & 0x3f;
        match self.head >> 6 {
            numbers::mac_pdu::mux_ext::SHORT_IE => AnyIeType::Type5bit(
                numbers::mac_ie::IEType5bit::try_from_composite(low_6_bits)
                    .expect("Value was masked"),
            ),
            _ => AnyIeType::Type6bit(
                numbers::mac_ie::IEType6bit::try_from(low_6_bits).expect("Bits masked"),
            ),
        }
    }

    /// Payload data in the IE.
    #[must_use]
    pub fn payload(&self) -> &'a [u8] {
        self.payload
    }

    /// Serializes into any [`embedded_io::Write`]r.
    ///
    /// # Errors
    ///
    /// This merely forwards any errors of the writer.
    #[expect(
        clippy::missing_panics_doc,
        reason = "Panics only happen when type level invariants are not upheld"
    )]
    pub fn serialize<W: embedded_io::Write>(&self, w: &mut W) -> Result<(), W::Error> {
        w.write(&[self.head])?;
        let mac_ext = self.head >> 6;
        match mac_ext {
            numbers::mac_pdu::mux_ext::LENGTH_8BIT => {
                w.write(&[self
                    .payload
                    .len()
                    .try_into()
                    .expect("Payload length mismatches header")])?;
            }
            numbers::mac_pdu::mux_ext::LENGTH_16BIT => {
                w.write(
                    &(u16::try_from(self.payload.len()).expect("Payload length mismatches header"))
                        .to_be_bytes(),
                )?;
            }
            _ => (),
        }
        w.write(self.payload)?;

        Ok(())
    }
}

#[cfg(test)]
mod test {
    /// A test across both the parser and the formatting.
    ///
    /// The output strings are not set in stone, but conveniently serve as a check that the data is
    /// correct.
    #[test]
    fn test_parse_ie_stream() {
        extern crate alloc;
        use alloc::{format, string::String, vec::Vec};

        let data = [
            73, 5, 176, 16, 6, 0, 13, 83, 7, 8, 12, 138, 160, 215, 2, 100, 64, 24, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        ];
        let expected = [
            r#"IE IEType6bit { .0: 0x09, description: "Cluster Beacon" }, payload [176, 16, 6, 0, 13]"#,
            r#"IE IEType6bit { .0: 0x13, description: "Random Access Resource" }, payload [8, 12, 138, 160, 215, 2, 100]"#,
            r#"IE IEType6bit { .0: 0x00, description: "Padding" }, payload [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]"#,
        ];
        let output: Vec<String> = crate::mac_ie::InformationElement::parse_stream(&data)
            .map(|i| i.unwrap())
            .map(|item| format!("IE {:?}, payload {:?}", item.ie_number(), item.payload()))
            .collect();

        assert_eq!(output, expected);
    }
}
