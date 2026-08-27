mod default;
mod spanish;

pub use default::Gsm7BitDefaultAlphabet;

pub use spanish::Gsm7BitSpanishAlphabet;

/// Gsm 7-bit escape character.
pub const ESCAPE_CHARACTER: u8 = 0x1B;

/// GSM 7-bit alphabet.
#[derive(Debug)]
#[non_exhaustive]
pub enum Gsm7BitAlphabet {
    /// Default GSM 7-bit alphabet.
    Default(Gsm7BitDefaultAlphabet),
    /// Spanish GSM 7-bit alphabet.
    Spanish(Gsm7BitSpanishAlphabet),
}

impl Default for Gsm7BitAlphabet {
    fn default() -> Self {
        Self::default()
    }
}

impl Gsm7BitAlphabet {
    /// Returns the GSM 7-bit escape character `0x1B` ([`ESCAPE_CHARACTER`]).
    pub const fn escape_character() -> u8 {
        ESCAPE_CHARACTER
    }

    /// Creates a new [`Gsm7BitAlphabet`] with the [`Gsm7BitDefaultAlphabet`].
    pub const fn default() -> Self {
        Self::Default(Gsm7BitDefaultAlphabet::new())
    }

    /// Creates a new [`Gsm7BitAlphabet`] with the [`Gsm7BitSpanishAlphabet`].
    pub const fn spanish() -> Self {
        Self::Spanish(Gsm7BitSpanishAlphabet::new())
    }

    /// Returns the standard GSM 7-bit character set.
    pub const fn standard(&self) -> &'static [(char, u8)] {
        match self {
            Self::Default(_) => Gsm7BitDefaultAlphabet::standard(),
            Self::Spanish(_) => Gsm7BitSpanishAlphabet::standard(),
        }
    }

    /// Returns the extended GSM 7-bit character set.
    pub const fn extended(&self) -> &'static [(char, u8)] {
        match self {
            Self::Default(_) => Gsm7BitDefaultAlphabet::extended(),
            Self::Spanish(_) => Gsm7BitSpanishAlphabet::extended(),
        }
    }

    /// Encodes the given character into a GSM 7-bit encoded byte.
    ///
    /// # Returns
    ///
    /// - `Some(Encoded)` if the character is found in the GSM 7-bit tables.
    /// - `None` if the character is not found.
    pub const fn encode(&self, ch: char) -> Option<Encoded> {
        match self {
            Self::Default(alphabet) => alphabet.encode(ch),
            Self::Spanish(alphabet) => alphabet.encode(ch),
        }
    }

    /// Decodes the given GSM 7-bit encoded byte into a character.
    ///
    /// # Returns
    ///
    /// - `Some(char)` if the byte is found in the GSM 7-bit tables.
    /// - `None` if the byte is not found.
    pub const fn decode(&self, encoded: Encoded) -> Option<char> {
        match self {
            Self::Default(alphabet) => alphabet.decode(encoded),
            Self::Spanish(alphabet) => alphabet.decode(encoded),
        }
    }

    /// Encodes the given message into a vector of GSM 7-bit encoded bytes.
    ///
    /// # Errors
    ///
    /// - Returns `Err(char)` if a character in the message cannot be encoded.
    #[cfg(any(test, feature = "alloc"))]
    #[cfg_attr(docsrs, doc(cfg(feature = "alloc")))]
    pub(crate) fn encode_to_vec(&self, message: &str) -> Result<alloc::vec::Vec<u8>, char> {
        // We double the amount of `bytes` we have in the worst case.
        //
        // If the amount of `bytes` is equals to the amount of `chars`
        //      (str = `[[[`, chars = [`[`, `[`, `[`] bytes = `[[[`)
        //      => we have 6 bytes of space, which is enough for standard/extended chars.
        //
        // If the amount of `bytes` is more than the amount of `chars`
        //      (str = `Ä`, , chars = [`Ä`], bytes = [195, 132])
        //      => we have 4 bytes of space, which is enough for standard/extended chars.
        let mut encoded = alloc::vec::Vec::with_capacity(message.len() * 2);

        for ch in message.chars() {
            match self.encode(ch) {
                Some(Encoded::Standard(byte)) => encoded.push(byte),
                Some(Encoded::Extended(byte)) => {
                    encoded.push(ESCAPE_CHARACTER);
                    encoded.push(byte);
                }
                None => return Err(ch),
            }
        }

        encoded.truncate(encoded.len());

        Ok(encoded)
    }
}

/// Encoded GSM 7-bit character.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Encoded {
    /// Standard GSM 7-bit character.
    Standard(u8),
    /// Extended GSM 7-bit character.
    ///
    /// Requires the escape character `0x1B` ([`ESCAPE_CHARACTER`]) before the value.
    Extended(u8),
}
