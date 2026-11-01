//! Errors related to User Data Header (UDH).

/// Errors that can occur when creating `ConcatenatedShortMessage8Bit` or `ConcatenatedShortMessage16Bit`.
#[derive(Debug)]
pub enum ConcatenatedShortMessageError {
    /// The total number of parts is zero.
    TotalPartsZero,
    /// The part number is zero.
    PartNumberZero,
    /// The part number exceeds the total number of parts.
    PartNumberExceedsTotalParts { part_number: u8, total_parts: u8 },
}

impl ::core::fmt::Display for ConcatenatedShortMessageError {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        match self {
            Self::PartNumberZero => {
                write!(f, "Part number cannot be zero")
            }
            Self::PartNumberExceedsTotalParts {
                part_number,
                total_parts,
            } => {
                write!(
                    f,
                    "Part number {} exceeds total parts {}",
                    part_number, total_parts
                )
            }
            Self::TotalPartsZero => {
                write!(f, "Total parts cannot be zero")
            }
        }
    }
}

impl ::core::error::Error for ConcatenatedShortMessageError {}
