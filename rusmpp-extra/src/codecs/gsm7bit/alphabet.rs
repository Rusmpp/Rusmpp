mod default;

pub use default::Gsm7BitDefaultAlphabet;

/// GSM 7-bit alphabet.
#[derive(Debug)]
#[non_exhaustive]
pub enum Gsm7BitAlphabet {
    /// Default GSM 7-bit alphabet.
    Default(Gsm7BitDefaultAlphabet),
}

impl Default for Gsm7BitAlphabet {
    fn default() -> Self {
        Self::default()
    }
}

impl Gsm7BitAlphabet {
    pub const fn default() -> Self {
        Self::Default(Gsm7BitDefaultAlphabet::new())
    }

    /// # Returns
    ///
    /// - `Some(Encoded)` if the character is found in the GSM 7-bit tables.
    /// - `None` if the character is not found.
    const fn encode(&self, ch: char) -> Option<Encoded> {
        match self {
            Self::Default(alphabet) => alphabet.encode(ch),
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
                    encoded.push(0x1B);
                    encoded.push(byte);
                }
                None => return Err(ch),
            }
        }

        encoded.truncate(encoded.len());

        Ok(encoded)
    }
}

enum Encoded {
    /// Standard GSM 7-bit character.
    Standard(u8),
    /// Extended GSM 7-bit character.
    ///
    /// Requires the escape character `0x1B` before the value.
    Extended(u8),
}
