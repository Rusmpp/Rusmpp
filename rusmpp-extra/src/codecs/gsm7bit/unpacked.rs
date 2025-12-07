use rusmpp_core::values::DataCoding;

use crate::codecs::gsm7bit::alphabet::Gsm7BitAlphabet;

/// GSM 7-bit unpacked codec.
#[non_exhaustive]
#[derive(Debug)]
pub struct Gsm7BitUnpacked {
    /// The GSM 7-bit alphabet to use for encoding.
    alphabet: Gsm7BitAlphabet,
    /// Whether to allow splitting extended characters across message parts.
    allow_split_extended_character: bool,
}

impl Default for Gsm7BitUnpacked {
    fn default() -> Self {
        Self::new()
    }
}

impl Gsm7BitUnpacked {
    /// Creates a new [`Gsm7BitUnpacked`] with [`Gsm7BitAlphabet::Default`].
    ///
    /// # Defaults
    ///
    /// - `alphabet`: [`Gsm7BitAlphabet::Default`]
    /// - `allow_split_extended_character`: `false`
    pub const fn new() -> Self {
        Self {
            alphabet: Gsm7BitAlphabet::default(),
            allow_split_extended_character: false,
        }
    }

    /// Sets the alphabet for the codec.
    pub const fn with_alphabet(mut self, alphabet: Gsm7BitAlphabet) -> Self {
        self.alphabet = alphabet;
        self
    }

    /// Returns whether splitting extended characters is allowed.
    pub const fn allow_split_extended_character(&self) -> bool {
        self.allow_split_extended_character
    }

    /// Sets whether to allow splitting extended characters across message parts.
    pub const fn with_allow_split_extended_character(mut self, allow: bool) -> Self {
        self.allow_split_extended_character = allow;
        self
    }

    /// Returns the associated [`Gsm7BitAlphabet`].
    pub const fn alphabet(&self) -> &Gsm7BitAlphabet {
        &self.alphabet
    }

    /// Returns the associated [`DataCoding`].
    pub const fn data_coding(&self) -> DataCoding {
        DataCoding::McSpecific
    }
}

#[cfg(any(test, feature = "alloc"))]
#[cfg_attr(docsrs, doc(cfg(feature = "alloc")))]
mod impl_owned {
    use alloc::vec::Vec;

    use crate::{
        codecs::{
            gsm7bit::errors::{Gsm7BitConcatenateError, Gsm7BitEncodeError},
            owned::Encoder,
        },
        concatenation::{
            MAX_PARTS,
            owned::{Concatenation, Concatenator},
        },
    };

    use super::*;

    impl Gsm7BitUnpacked {
        /// Encodes the given message into a vector of bytes.
        pub fn encode_to_vec(&self, input: &str) -> Result<Vec<u8>, Gsm7BitEncodeError> {
            self.alphabet
                .encode_to_vec(input)
                .map_err(Gsm7BitEncodeError::UnencodableCharacter)
        }
    }

    impl Encoder for Gsm7BitUnpacked {
        type Error = Gsm7BitEncodeError;

        fn encode(&self, message: &str) -> Result<(Vec<u8>, DataCoding), Self::Error> {
            self.encode_to_vec(message)
                .map(|vec| (vec, self.data_coding()))
        }
    }

    impl Concatenator for Gsm7BitUnpacked {
        type Error = Gsm7BitConcatenateError;

        fn concatenate(
            &self,
            message: &str,
            max_message_size: usize,
            part_header_size: usize,
        ) -> Result<(Concatenation, DataCoding), Self::Error> {
            let encoded = self.encode_to_vec(message)?;

            let total = encoded.len();

            if total <= max_message_size {
                return Ok((Concatenation::single(encoded), self.data_coding()));
            }

            let part_payload_size = max_message_size.saturating_sub(part_header_size);

            if part_payload_size == 0 {
                return Err(Gsm7BitConcatenateError::PartCapacityExceeded);
            }

            let mut parts: Vec<Vec<u8>> = Vec::new();
            let mut i = 0;

            while i < total {
                let mut end = (i + part_payload_size).min(total);

                // avoid splitting extended characters unless allow_split_extended_character == true
                if !self.allow_split_extended_character {
                    // If not last part AND the last byte of this part is 0x1B,
                    // we must shrink the part to avoid splitting ESC + next byte.
                    if end < total && encoded[end - 1] == 0x1B {
                        end -= 1;

                        // If shrinking removed the entire part -> impossible
                        if end == i {
                            return Err(Gsm7BitConcatenateError::InvalidBoundary);
                        }
                    }
                }

                parts.push(encoded[i..end].to_vec());

                i = end;
            }

            if parts.len() > MAX_PARTS {
                return Err(Gsm7BitConcatenateError::parts_count_exceeded(parts.len()));
            }

            Ok((Concatenation::concatenated(parts), self.data_coding()))
        }
    }
}
