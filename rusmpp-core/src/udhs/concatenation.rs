//! Short message concatenation UDHs.

mod concatenated_short_message_16_bit;
pub use concatenated_short_message_16_bit::ConcatenatedShortMessage16Bit;

mod concatenated_short_message_8_bit;
pub use concatenated_short_message_8_bit::ConcatenatedShortMessage8Bit;

pub mod parts {
    pub use super::concatenated_short_message_8_bit::ConcatenatedShortMessage8BitParts;
    pub use super::concatenated_short_message_16_bit::ConcatenatedShortMessage16BitParts;
}

/// Concatenated short message.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum ConcatenatedShortMessage {
    /// 8-bit reference number concatenated short message.
    EightBit(ConcatenatedShortMessage8Bit),
    /// 16-bit reference number concatenated short message.
    SixteenBit(ConcatenatedShortMessage16Bit),
}

impl ConcatenatedShortMessage {
    /// Returns the length of the UDH type in bytes.
    ///
    /// See [`ConcatenatedShortMessageType::udh_length`].
    pub const fn udh_length(&self) -> usize {
        match self {
            Self::EightBit(_) => ConcatenatedShortMessage8Bit::UDH_LENGTH,
            Self::SixteenBit(_) => ConcatenatedShortMessage16Bit::UDH_LENGTH,
        }
    }

    /// Converts [`ConcatenatedShortMessage`] to its UDH bytes representation.
    pub const fn udh_bytes(&self) -> ConcatenatedShortMessageUdhBytes {
        match self {
            Self::EightBit(concatenation) => {
                ConcatenatedShortMessageUdhBytes::EightBit(concatenation.udh_bytes())
            }
            Self::SixteenBit(concatenation) => {
                ConcatenatedShortMessageUdhBytes::SixteenBit(concatenation.udh_bytes())
            }
        }
    }
}

/// Bytes representation of [`ConcatenatedShortMessage`] as full UDH.
#[derive(Debug)]
pub enum ConcatenatedShortMessageUdhBytes {
    /// 8-bit UDH bytes
    EightBit([u8; 6]),
    /// 16-bit UDH bytes
    SixteenBit([u8; 7]),
}

impl ConcatenatedShortMessageUdhBytes {
    /// Returns the bytes as a slice.
    pub const fn as_bytes(&self) -> &[u8] {
        match self {
            Self::EightBit(bytes) => bytes,
            Self::SixteenBit(bytes) => bytes,
        }
    }
}

/// Concatenated short message type.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum ConcatenatedShortMessageType {
    /// 8-bit reference number concatenated short message.
    EightBit { reference: u8 },
    /// 16-bit reference number concatenated short message.
    SixteenBit { reference: u16 },
}

impl ConcatenatedShortMessageType {
    /// Creates a new [`ConcatenatedShortMessageType::EightBit`].
    pub const fn u8(reference: u8) -> Self {
        Self::EightBit { reference }
    }

    /// Creates a new [`ConcatenatedShortMessageType::SixteenBit`].
    pub const fn u16(reference: u16) -> Self {
        Self::SixteenBit { reference }
    }

    /// Returns the length of the UDH type in bytes.
    pub const fn udh_length(self) -> usize {
        match self {
            Self::EightBit { .. } => ConcatenatedShortMessage8Bit::UDH_LENGTH,
            Self::SixteenBit { .. } => ConcatenatedShortMessage16Bit::UDH_LENGTH,
        }
    }

    /// Creates a new [`ConcatenatedShortMessage`]without checking invariants.
    ///
    /// See [`ConcatenatedShortMessage8Bit::new_unchecked`] and [`ConcatenatedShortMessage16Bit::new_unchecked`].
    pub const fn concatenated_short_message_unchecked(
        self,
        total_parts: u8,
        part_number: u8,
    ) -> ConcatenatedShortMessage {
        match self {
            Self::EightBit { reference } => ConcatenatedShortMessage::EightBit(
                ConcatenatedShortMessage8Bit::new_unchecked(reference, total_parts, part_number),
            ),
            Self::SixteenBit { reference } => ConcatenatedShortMessage::SixteenBit(
                ConcatenatedShortMessage16Bit::new_unchecked(reference, total_parts, part_number),
            ),
        }
    }
}
