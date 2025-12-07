use crate::concatenation::MAX_PARTS;

/// Errors that can occur during Latin1 encoding.

#[derive(Debug, thiserror::Error, PartialEq, Eq)]
pub enum Latin1EncodeError {
    /// Input contains un-encodable character.
    #[error("Input contains un-encodable character")]
    UnencodableCharacter,
}

/// Errors that can occur during Latin1 concatenation.
#[derive(Debug, thiserror::Error, PartialEq, Eq)]
pub enum Latin1ConcatenateError {
    /// Encoding error.
    #[error("Encoding error: {0}")]
    Encode(
        #[from]
        #[source]
        Latin1EncodeError,
    ),
    /// Part cannot fit even a single character.
    ///
    /// This error is returned when `max_message_size - part_header_size == 0`.
    #[error(
        "Cannot fit even a single character into a part with the given header and size constraints"
    )]
    PartCapacityExceeded,
    #[error("The number of parts exceeds the maximum allowed. actual: {actual}, max: {max}")]
    /// The number of parts exceeds the maximum allowed.
    PartsCountExceeded {
        /// The maximum allowed number of parts.
        max: usize,
        /// The actual number of parts.
        actual: usize,
    },
}

impl Latin1ConcatenateError {
    pub(crate) const fn parts_count_exceeded(actual: usize) -> Self {
        Self::PartsCountExceeded {
            max: MAX_PARTS,
            actual,
        }
    }
}
