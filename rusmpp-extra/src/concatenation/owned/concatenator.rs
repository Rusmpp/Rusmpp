use rusmpp_core::values::DataCoding;

use crate::concatenation::owned::concatenation::Concatenation;

/// A trait for concatenating messages into smaller parts.
pub trait Concatenator {
    /// The type of errors that can occur during concatenation.
    type Error;

    /// Splits the encoded message into concatenated parts and their associated [`DataCoding`].
    ///
    /// # Arguments
    ///
    /// * `message` - The message to encode and concatenate.
    /// * `max_message_size` - The maximum size of each message part.
    /// * `part_header_size` - The size of the header for each part.
    ///
    /// # Notes
    ///
    /// * The returned `Vec<u8>` in the [`Concatenation::Single`] must *`NOT`* exceed `max_message_size` in length. (this is considered a bug in the implementation)
    /// * Each `Vec<u8>` in the [`Concatenation::Concatenated`] must *`NOT`* exceed `max_message_size  - part_header_size` in length. (this is considered a bug in the implementation)
    /// * The parts count in [`Concatenation::Concatenated`] must be at least [`concatenation::MIN_PARTS`](crate::concatenation::MIN_PARTS). (this is considered a bug in the implementation)
    /// * The parts count in [`Concatenation::Concatenated`] must *`NOT`* exceed [`concatenation::MAX_PARTS`](crate::concatenation::MAX_PARTS). (this is considered an error that might be returned as a [`Concatenator::Error`])
    ///     - Why is this an error? Some encoders might use less bytes per part than others. This allows us to implement a `FallbackConcatenator` that tries different concatenation strategies until one works within the parts limit.
    ///        - E.g., `Ucs2` encoding uses 2 bytes per character, while `Gsm7Bit` uses 7 bits per character. This means that a message that fits in 3 parts with `Gsm7Bit` might require 5 parts with `Ucs2`.
    fn concatenate(
        &self,
        message: &str,
        max_message_size: usize,
        part_header_size: usize,
    ) -> Result<(Concatenation, DataCoding), Self::Error>;
}
