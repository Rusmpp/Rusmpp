use rusmpp_macros::Rusmpp;

/// Defines the encoding scheme of the short
/// message user data.
#[repr(u8)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Default, Rusmpp)]
#[cfg_attr(feature = "arbitrary", derive(::arbitrary::Arbitrary))]
#[cfg_attr(feature = "serde", derive(::serde::Serialize))]
#[cfg_attr(feature = "serde-deserialize-unchecked", derive(::serde::Deserialize))]
pub enum DataCoding {
    /// GSM 7-bit default alphabet
    #[default]
    McSpecific = 0b00000000,
    /// IA5 (CCITT T.50)/ASCII (ANSI X3.4).
    Ia5 = 0b00000001,
    /// Octet unspecified (8-bit binary).
    OctetUnspecified = 0b00000010,
    /// Latin 1 (ISO-8859-1).
    Latin1 = 0b00000011,
    /// Octet unspecified (8-bit binary) 2.
    OctetUnspecified2 = 0b00000100,
    /// JIS (X 0208-1990).
    Jis = 0b00000101,
    /// Cyrillic (ISO-8859-5).
    Cyrillic = 0b00000110,
    /// Latin/Hebrew (ISO-8859-8).
    LatinHebrew = 0b00000111,
    /// UCS2 (ISO/IEC-10646).
    Ucs2 = 0b00001000,
    /// Pictogram Encoding.
    PictogramEncoding = 0b00001001,
    /// ISO-2022-JP (Music Codes).
    Iso2022JpMusicCodes = 0b00001010,
    /// Extended Kanji JIS (X 0212-1990).
    ExtendedKanjiJis = 0b00001101,
    /// KS C 5601.
    Ksc5601 = 0b00001110,
    /// GSM MWI control - see [GSM 03.38].
    GsmMwiControl = 0b11000000,
    /// GSM MWI control 2- see [GSM 03.38].
    GsmMwiControl2 = 0b11010000,
    /// GSM message class control - see [GSM 03.38].
    GsmMessageClassControl = 0b11100000,
    Other(u8),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn encode_decode() {
        #[cfg(feature = "alloc")]
        crate::tests::owned::encode_decode_test_instances::<DataCoding>();
        crate::tests::borrowed::encode_decode_test_instances::<DataCoding>();
    }
}
