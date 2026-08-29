//! Latin1 encoding/decoding support.

use rusmpp_core::values::DataCoding;

mod decode;

#[cfg(any(test, feature = "alloc"))]
#[cfg_attr(docsrs, doc(cfg(feature = "alloc")))]
pub use decode::owned::Latin1Decoder;

mod errors;
pub use errors::{Latin1ConcatenateError, Latin1DecodeError, Latin1EncodeError};

/// Latin1 encoder.
#[derive(Debug)]
#[non_exhaustive]
pub struct Latin1Encoder {}

impl Default for Latin1Encoder {
    fn default() -> Self {
        Self::new()
    }
}

impl Latin1Encoder {
    /// Creates a new [`Latin1Encoder`] encoder.
    pub const fn new() -> Self {
        Self {}
    }

    /// Returns the associated [`DataCoding`].
    pub const fn data_coding(&self) -> DataCoding {
        DataCoding::Latin1
    }
}

#[cfg(any(test, feature = "alloc"))]
#[cfg_attr(docsrs, doc(cfg(feature = "alloc")))]
mod impl_owned {
    use alloc::vec::Vec;

    use crate::{
        concatenation::{
            MAX_PARTS,
            owned::{Concatenation, Concatenator},
        },
        encoding::owned::Encoder,
    };

    use super::*;

    impl Latin1Encoder {
        /// Encodes the given message into a vector of bytes.
        pub fn encode_to_vec(&self, input: &str) -> Result<Vec<u8>, Latin1EncodeError> {
            let mut buffer = Vec::with_capacity(input.len());

            for ch in input.chars() {
                let code_point = ch as u32;

                // Latin1 only covers the Unicode range U+0000..=U+00FF.
                if code_point > 0xFF {
                    return Err(Latin1EncodeError::InvalidCharacter(ch));
                }

                buffer.push(code_point as u8);
            }

            Ok(buffer)
        }
    }

    impl Encoder for Latin1Encoder {
        type Error = Latin1EncodeError;

        fn encode(&self, message: &str) -> Result<(Vec<u8>, DataCoding), Self::Error> {
            self.encode_to_vec(message)
                .map(|vec| (vec, self.data_coding()))
        }
    }

    impl Concatenator for Latin1Encoder {
        type Error = Latin1ConcatenateError;

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
                return Err(Latin1ConcatenateError::PartCapacityExceeded);
            }

            let parts = encoded
                .chunks(part_payload_size)
                .map(|chunk| chunk.to_vec())
                .collect::<Vec<Vec<u8>>>();

            if parts.len() > MAX_PARTS {
                return Err(Latin1ConcatenateError::parts_count_exceeded(parts.len()));
            }

            Ok((Concatenation::concatenated(parts), self.data_coding()))
        }
    }
}

#[cfg(test)]
mod tests;
