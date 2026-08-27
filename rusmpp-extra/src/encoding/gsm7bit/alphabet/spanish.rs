use super::Encoded;

/// Spanish GSM 7-bit alphabet.
#[non_exhaustive]
#[derive(Debug)]
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
    /// Encodes the given character into a spanish GSM 7-bit encoded byte.
    ///
    /// # Returns
    ///
    /// - `Some(Self)` if the character is found in the GSM 7-bit tables.
    /// - `None` if the character is not found.
    pub const fn encode(&self, ch: char) -> Option<Encoded> {
        if let Some(byte) = Standard::encode(ch) {
            Some(Encoded::Standard(byte))
        } else if let Some(byte) = Extended::encode(ch) {
            Some(Encoded::Extended(byte))
        } else {
            None
        }
    }

    /// Decodes the given spanish GSM 7-bit encoded byte into a character.
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
        let mut i = 0;

        while i < EXTENDED.len() {
            let (c, byte) = EXTENDED[i];

            if c == ch {
                return Some(byte);
            }

            i += 1;
        }

        None
    }

    const fn decode(byte: u8) -> Option<char> {
        let mut i = 0;

        while i < EXTENDED.len() {
            let (c, b) = EXTENDED[i];

            if b == byte {
                return Some(c);
            }

            i += 1;
        }

        None
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
