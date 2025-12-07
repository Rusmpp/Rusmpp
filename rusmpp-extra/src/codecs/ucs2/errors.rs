use crate::concatenation::MAX_PARTS;

/// Errors that can occur during UCS2 encoding.
#[derive(Debug, thiserror::Error, PartialEq, Eq)]
pub enum Ucs2EncodeError {
    /// Input contains un-encodable character.
    #[error("Input contains un-encodable character")]
    UnencodableCharacter,
}

/// Errors that can occur during UCS2 concatenation.
#[derive(Debug, thiserror::Error, PartialEq, Eq)]
pub enum Ucs2ConcatenateError {
    /// Encoding error.
    #[error("Encoding error: {0}")]
    Encode(
        #[from]
        #[source]
        Ucs2EncodeError,
    ),
    /// Part cannot fit even a single character.
    ///
    /// This error is returned when `max_message_size - part_header_size == 0`.
    #[error(
        "Cannot fit even a single character into a part with the given header and size constraints"
    )]
    PartCapacityExceeded,
    /// A part would end with a leading surrogate, which is not allowed unless allow_split_character=true.
    ///
    /// This error might be returned when `max_message_size - part_header_size < 2 && allow_split_character == false`.
    #[error(
        "A part would end with a leading surrogate, which is not allowed unless allow_split_character=true"
    )]
    InvalidBoundary,
    #[error("The number of parts exceeds the maximum allowed. actual: {actual}, max: {max}")]
    /// The number of parts exceeds the maximum allowed.
    PartsCountExceeded {
        /// The maximum allowed number of parts.
        max: usize,
        /// The actual number of parts.
        actual: usize,
    },
}

impl Ucs2ConcatenateError {
    pub(crate) const fn parts_count_exceeded(actual: usize) -> Self {
        Self::PartsCountExceeded {
            max: MAX_PARTS,
            actual,
        }
    }
}
