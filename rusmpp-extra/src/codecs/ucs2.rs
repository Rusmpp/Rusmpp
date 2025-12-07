//! Ucs2 encoding/decoding support.

mod errors;
pub use errors::{Ucs2ConcatenateError, Ucs2EncodeError};
use rusmpp_core::values::DataCoding;

/// UCS2 codec.
#[derive(Debug)]
pub struct Ucs2 {
    /// Whether to allow splitting characters across message parts.
    allow_split_character: bool,
}

impl Default for Ucs2 {
    fn default() -> Self {
        Self::new()
    }
}

impl Ucs2 {
    /// Creates a new [`Ucs2`] codec.
    ///
    /// # Defaults
    ///
    /// - `allow_split_character`: `false`
    pub const fn new() -> Self {
        Self {
            allow_split_character: false,
        }
    }

    /// Returns whether splitting characters is allowed.
    pub const fn allow_split_character(&self) -> bool {
        self.allow_split_character
    }

    /// Sets whether to allow splitting characters across message parts.
    pub const fn with_allow_split_character(mut self, allow: bool) -> Self {
        self.allow_split_character = allow;
        self
    }

    /// Returns the associated [`DataCoding`].
    pub const fn data_coding(&self) -> DataCoding {
        DataCoding::Ucs2
    }
}

#[cfg(any(test, feature = "alloc"))]
#[cfg_attr(docsrs, doc(cfg(feature = "alloc")))]
mod impl_owned {
    use alloc::vec::Vec;

    use crate::{
        codecs::owned::Encoder,
        concatenation::{
            MAX_PARTS,
            owned::{Concatenation, Concatenator},
        },
    };

    use super::*;

    impl Ucs2 {
        /// Encodes the given message into a vector of bytes.
        pub fn encode_to_vec(&self, input: &str) -> Result<Vec<u8>, Ucs2EncodeError> {
            // Maximum possible UCS-2 units = number of chars
            let char_count = input.chars().count();
            let mut buffer = alloc::vec![0u16; char_count];

            match ucs2::encode(input, &mut buffer) {
                Ok(len) => {
                    let mut encoded = Vec::with_capacity(len * 2);

                    for &code_unit in &buffer[..len] {
                        encoded.push((code_unit >> 8) as u8);
                        encoded.push((code_unit & 0xFF) as u8);
                    }

                    Ok(encoded)
                }
                Err(err) => match err {
                    ucs2::Error::BufferOverflow => {
                        unreachable!("We allocated more than enough space")
                    }
                    ucs2::Error::MultiByte => Err(Ucs2EncodeError::UnencodableCharacter),
                },
            }
        }
    }

    impl Encoder for Ucs2 {
        type Error = Ucs2EncodeError;

        fn encode(&self, message: &str) -> Result<(Vec<u8>, DataCoding), Self::Error> {
            self.encode_to_vec(message)
                .map(|vec| (vec, self.data_coding()))
        }
    }

    impl Concatenator for Ucs2 {
        type Error = Ucs2ConcatenateError;

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
                return Err(Ucs2ConcatenateError::PartCapacityExceeded);
            }

            let mut parts: Vec<Vec<u8>> = Vec::new();
            let mut i = 0;

            while i < total {
                let mut end = (i + part_payload_size).min(total);

                if !self.allow_split_character {
                    // If not at the end and our cut is *not* on a 2-byte boundary,
                    // shrink the part by 1 byte to align to even boundary.
                    if end < total && (end % 2 != 0) {
                        end -= 1;

                        // If shrinking removed the entire part -> impossible
                        if end == i {
                            return Err(Ucs2ConcatenateError::InvalidBoundary);
                        }
                    }
                }

                // If allow_split_character == true, we accept uneven boundaries as-is.

                parts.push(encoded[i..end].to_vec());
                i = end;
            }

            if parts.len() > MAX_PARTS {
                return Err(Ucs2ConcatenateError::parts_count_exceeded(parts.len()));
            }

            Ok((Concatenation::concatenated(parts), self.data_coding()))
        }
    }
}

#[cfg(test)]
mod tests;
