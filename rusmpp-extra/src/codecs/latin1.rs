use rusmpp_core::values::DataCoding;

mod errors;
pub use errors::{Latin1ConcatenateError, Latin1EncodeError};

/// Latin1 codec.
#[derive(Debug)]
#[non_exhaustive]
pub struct Latin1 {}

impl Default for Latin1 {
    fn default() -> Self {
        Self::new()
    }
}

impl Latin1 {
    /// Creates a new [`Latin1`] codec.
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
        codecs::owned::Encoder,
        concatenation::{
            MAX_PARTS,
            owned::{Concatenation, Concatenator},
        },
    };

    use super::*;

    impl Latin1 {
        /// Encodes the given message into a vector of bytes.
        pub fn encode_to_vec(&self, input: &str) -> Result<Vec<u8>, Latin1EncodeError> {
            encoding_rs::mem::is_utf8_latin1(input.as_bytes())
                .then_some(())
                .ok_or(Latin1EncodeError::UnencodableCharacter)?;

            let mut buffer = alloc::vec![0u8; input.len()];
            /*
            Correctness:

            - The input is UTF-8 Latin1.
            - The size of the buffer is at least as large as the encoded output.
            */
            let size =
                encoding_rs::mem::convert_utf8_to_latin1_lossy(input.as_bytes(), &mut buffer);

            buffer.truncate(size);

            Ok(buffer)
        }
    }

    impl Encoder for Latin1 {
        type Error = Latin1EncodeError;

        fn encode(&self, message: &str) -> Result<(Vec<u8>, DataCoding), Self::Error> {
            self.encode_to_vec(message)
                .map(|vec| (vec, self.data_coding()))
        }
    }

    impl Concatenator for Latin1 {
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
