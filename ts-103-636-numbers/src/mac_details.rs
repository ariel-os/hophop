// SPDX-FileCopyrightText: Copyright Christian Amsüss <chrysn@fsfe.org>, Silano Systems
// SPDX-License-Identifier: MIT OR Apache-2.0
//! Various bitfield constants in subsections of Section 6.4 of ETSI TS 103 636-4 V2.1.1

/// Fields of the Association Relase Message IE of 6.4.2.6 of ETSI TS 103 636-4 V2.1.1
pub mod release_message {
    /// The "Release Cause" field.
    ///
    /// The value is technically 4 bit; it is a type invariant (under penalty of panics) that the
    /// inner value only has the lowest 4 bits set.
    // FIXME: This type should be a macro that merely takes copy-pasted data.
    #[derive(Copy, Clone, PartialEq, Eq)]
    pub struct ReleaseCause(u8);

    impl ReleaseCause {
        /// Constructs a new release cause with a value from the wire.
        ///
        /// Even though being a coded value, this code does not err on reserved values; it is up to
        /// the user to act on Section 6.4.1 of ETSI TS 103 636-4 V2.1.1 when processing the
        /// containing element. (Although realistically, callers might err on the side of
        /// practicality here).
        ///
        /// # Panics
        ///
        /// … if any bit of the high nibble is set.
        #[must_use]
        pub fn new(value: u8) -> Self {
            assert!(value >> 4 == 0, "bits outside the size are set");
            Self(value)
        }

        // Coded values
        pub const CONNECTION_TERMINATION: Self = Self(0);
        pub const MOBILITY: Self = Self(1);
        pub const LONG_INACTIVITY: Self = Self(2);
        pub const INCOMPATIBLE_CONFIGURATION: Self = Self(3);
        pub const NO_SUFFICIENT_HW_OR_MEMORY_RESOURCE: Self = Self(4);
        pub const NO_SUFFICIENT_RADIO_RESOURCES: Self = Self(5);
        pub const BAD_RADIO_QUALITY: Self = Self(6);
        pub const SECURITY_ERROR: Self = Self(7);
        pub const SHORT_RD_ID_CONFLICT_DETECTED_IN_PT_SIDE: Self = Self(8);
        pub const SHORT_RD_ID_CONFLICT_DETECTED_IN_FT_SIDE: Self = Self(9);
        pub const NOT_ASSOCIATED: Self = Self(10);
        pub const NOT_OPERATING_IN_FT_MODE: Self = Self(12);
        pub const OTHER_ERROR: Self = Self(13);
    }

    #[cfg(feature = "defmt")]
    impl defmt::Format for ReleaseCause {
        fn format(&self, fmt: defmt::Formatter) {
            use defmt::write;
            match *self {
                ReleaseCause::CONNECTION_TERMINATION => write!(fmt, "connection termination"),
                ReleaseCause::MOBILITY => write!(fmt, "mobility"),
                ReleaseCause::LONG_INACTIVITY => write!(fmt, "long Inactivity"),
                ReleaseCause::INCOMPATIBLE_CONFIGURATION => {
                    write!(fmt, "incompatible configuration");
                }
                ReleaseCause::NO_SUFFICIENT_HW_OR_MEMORY_RESOURCE => {
                    write!(fmt, "No sufficient HW/memory resource");
                }
                ReleaseCause::NO_SUFFICIENT_RADIO_RESOURCES => {
                    write!(fmt, "No sufficient radio resources");
                }
                ReleaseCause::BAD_RADIO_QUALITY => write!(fmt, "bad radio quality"),
                ReleaseCause::SECURITY_ERROR => write!(fmt, "security error"),
                ReleaseCause::SHORT_RD_ID_CONFLICT_DETECTED_IN_PT_SIDE => {
                    write!(fmt, "Short RD ID Conflict detected in PT side");
                }
                ReleaseCause::SHORT_RD_ID_CONFLICT_DETECTED_IN_FT_SIDE => {
                    write!(fmt, "Short RD ID Conflict detected in FT side");
                }
                ReleaseCause::NOT_ASSOCIATED => write!(fmt, "not associated"),
                ReleaseCause::NOT_OPERATING_IN_FT_MODE => write!(fmt, "not operating in FT mode"),
                ReleaseCause::OTHER_ERROR => write!(fmt, "other error"),
                _ => write!(fmt, "reserved"),
            }
        }
    }
}
