use crate::concatenation::MAX_PARTS;

// TODO: rename all `UnencodableCharacter` errors to `InvalidCharacter`.

/// Errors that can occur during GSM 7-bit encoding.
#[derive(Debug, thiserror::Error, PartialEq, Eq)]
pub enum Gsm7BitEncodeError {
    /// Input contains un-encodable character.
    #[error("Input contains un-encodable character: '{0}'")]
    UnencodableCharacter(char),
}

/// Errors that can occur during GSM 7-bit decoding.
#[derive(Debug, thiserror::Error)]
pub enum Gsm7BitDecodeError {
    /// A standard-table byte had no mapping to a character.
    #[error("A standard byte had no mapping to a character: {0:#04X}")]
    InvalidByte(u8),
    /// An extended-table byte (following 0x1B) had no mapping.
    #[error("An extended byte (following 0x1B) had no mapping: {0:#04X}")]
    InvalidExtendedByte(u8),
    /// Input ended on an escape byte (0x1B) with no completing byte, and no further chunk resolved it.
    #[error(
        "Input ended on an escape byte (0x1B) with no completing byte, and no further chunk resolved it"
    )]
    TrailingEscape,
}

/// Errors that can occur during GSM 7-bit concatenation.
#[derive(Debug, thiserror::Error, PartialEq, Eq)]
pub enum Gsm7BitConcatenateError {
    /// Encoding error.
    #[error("Encoding error: {0}")]
    Encode(
        #[from]
        #[source]
        Gsm7BitEncodeError,
    ),
    /// Part cannot fit even a single septet.
    ///
    /// This error is returned when `max_message_size - part_header_size == 0`.
    #[error(
        "Cannot fit even a single septet into a part with the given header and size constraints"
    )]
    PartCapacityExceeded,
    /// A part would end with an escape (0x1B) septet, which is not allowed unless allow_split_extended_character=true.
    ///
    /// This error might be returned when `max_message_size - part_header_size < 2 && allow_split_extended_character == false`.
    #[error(
        "A part would end with an escape (0x1B) septet, which is not allowed unless allow_split_extended_character=true"
    )]
    InvalidBoundary,
    /// The number of parts exceeds the maximum allowed.
    #[error("The number of parts exceeds the maximum allowed. actual: {actual}, max: {max}")]
    PartsCountExceeded {
        /// The maximum allowed number of parts.
        max: usize,
        /// The actual number of parts.
        actual: usize,
    },
}

impl Gsm7BitConcatenateError {
    pub(crate) const fn parts_count_exceeded(actual: usize) -> Self {
        Self::PartsCountExceeded {
            max: MAX_PARTS,
            actual,
        }
    }
}
