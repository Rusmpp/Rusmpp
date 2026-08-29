//! ASCII encoding/decoding support.

use rusmpp_core::values::DataCoding;

mod decode;

#[cfg(any(test, feature = "alloc"))]
#[cfg_attr(docsrs, doc(cfg(feature = "alloc")))]
pub use decode::owned::AsciiDecoder;

mod errors;
pub use errors::{AsciiConcatenateError, AsciiDecodeError, AsciiEncodeError};

/// ASCII encoder.
#[derive(Debug)]
#[non_exhaustive]
pub struct AsciiEncoder {
    /// The associated [`DataCoding`] for ASCII encoding.
    data_coding: DataCoding,
}

impl Default for AsciiEncoder {
    fn default() -> Self {
        Self::new()
    }
}

impl AsciiEncoder {
    /// Creates a new [`AsciiEncoder`] with [`DataCoding::McSpecific`].
    ///
    /// # Defaults
    ///
    /// - `data_coding`: [`DataCoding::McSpecific`]
    pub const fn new() -> Self {
        Self {
            data_coding: DataCoding::McSpecific,
        }
    }

    /// Returns the associated [`DataCoding`] of the [`AsciiEncoder`] encoder.
    pub const fn data_coding(&self) -> DataCoding {
        self.data_coding
    }

    /// Sets the associated [`DataCoding`] of the [`AsciiEncoder`] encoder.
    pub const fn with_data_coding(mut self, data_coding: DataCoding) -> Self {
        self.data_coding = data_coding;
        self
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

    impl AsciiEncoder {
        /// Encodes the given message into a vector of bytes.
        pub fn encode_to_vec(&self, input: &str) -> Result<Vec<u8>, AsciiEncodeError> {
            if let Some(ch) = input.chars().find(|c| !c.is_ascii()) {
                return Err(AsciiEncodeError::UnencodableCharacter(ch));
            }

            Ok(input.as_bytes().to_vec())
        }
    }

    impl Encoder for AsciiEncoder {
        type Error = AsciiEncodeError;

        fn encode(&self, message: &str) -> Result<(Vec<u8>, DataCoding), Self::Error> {
            self.encode_to_vec(message)
                .map(|vec| (vec, self.data_coding()))
        }
    }

    impl Concatenator for AsciiEncoder {
        type Error = AsciiConcatenateError;

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
                return Err(AsciiConcatenateError::PartCapacityExceeded);
            }

            let parts = encoded
                .chunks(part_payload_size)
                .map(|chunk| chunk.to_vec())
                .collect::<Vec<Vec<u8>>>();

            if parts.len() > MAX_PARTS {
                return Err(AsciiConcatenateError::parts_count_exceeded(parts.len()));
            }

            Ok((Concatenation::concatenated(parts), self.data_coding()))
        }
    }
}

#[cfg(test)]
mod tests;
