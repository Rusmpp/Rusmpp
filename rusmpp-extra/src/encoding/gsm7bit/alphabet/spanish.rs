use rusmpp_core::udhs::language::NationalLanguageIndicator;

use crate::encoding::gsm7bit::alphabet::macros::{decode, encode};

use super::Encoded;

/// Spanish GSM 7-bit alphabet.
#[non_exhaustive]
#[derive(Debug, Clone, Copy)]
pub struct Gsm7BitSpanishAlphabet;

impl Default for Gsm7BitSpanishAlphabet {
    fn default() -> Self {
        Self::new()
    }
}

impl Gsm7BitSpanishAlphabet {
    /// Creates a new [`Gsm7BitSpanishAlphabet`].
    pub const fn new() -> Self {
        Self
    }
}

impl Gsm7BitSpanishAlphabet {
    /// Encodes the given character into a `spanish` GSM 7-bit encoded byte using the standard character set.   
    pub const fn encode_standard(&self, ch: char) -> Option<u8> {
        Standard::encode(ch)
    }

    /// Encodes the given character into a `spanish` GSM 7-bit encoded byte using the extended character set.
    pub const fn encode_extended(&self, ch: char) -> Option<u8> {
        Extended::encode(ch)
    }

    /// Decodes the given `spanish` GSM 7-bit encoded byte into a character.
    ///
    /// # Returns
    ///
    /// - `Some(char)` if the byte is found in the GSM 7-bit tables.
    /// - `None` if the byte is not found.
    pub const fn decode(&self, encoded: Encoded) -> Option<char> {
        match encoded {
            Encoded::Standard(byte) => Standard::decode(byte),
            Encoded::Extended(byte) => Extended::decode(byte),
        }
    }

    /// Returns the standard `spanish` GSM 7-bit character set.
    pub const fn standard() -> &'static [(char, u8)] {
        super::default::Gsm7BitDefaultAlphabet::standard()
    }

    /// Returns the extended `spanish` GSM 7-bit character set.
    pub const fn extended() -> &'static [(char, u8)] {
        EXTENDED
    }

    /// Returns the standard [`NationalLanguageIndicator`] for the `spanish` GSM 7-bit alphabet.
    pub const fn standard_national_language_indicator() -> Option<NationalLanguageIndicator> {
        None
    }

    /// Returns the extended [`NationalLanguageIndicator`] for the `spanish` GSM 7-bit alphabet.
    pub const fn extended_national_language_indicator() -> Option<NationalLanguageIndicator> {
        Some(NationalLanguageIndicator::Spanish)
    }
}

struct Extended;

struct Standard;

impl Standard {
    const fn encode(ch: char) -> Option<u8> {
        super::default::Standard::encode(ch)
    }

    const fn decode(byte: u8) -> Option<char> {
        super::default::Standard::decode(byte)
    }
}

impl Extended {
    const fn encode(ch: char) -> Option<u8> {
        encode!(ch, EXTENDED)
    }

    const fn decode(byte: u8) -> Option<char> {
        decode!(byte, EXTENDED)
    }
}

static EXTENDED: &[(char, u8)] = &[
    ('ç', 0x09), // Spanish only
    ('^', 0x14),
    ('{', 0x28),
    ('}', 0x29),
    ('\\', 0x2F),
    ('[', 0x3C),
    ('~', 0x3D),
    (']', 0x3E),
    ('|', 0x40),
    ('Á', 0x41), // Spanish only
    ('Í', 0x49), // Spanish only
    ('Ó', 0x4F), // Spanish only
    ('Ú', 0x55), // Spanish only
    ('á', 0x61), // Spanish only
    ('€', 0x65),
    ('í', 0x69), // Spanish only
    ('ó', 0x6F), // Spanish only
    ('ú', 0x75), // Spanish only
];
