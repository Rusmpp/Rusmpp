//! Error types for the [`CommandCodec`](crate::CommandCodec).

use core::num::TryFromIntError;

/// An error that can occur when encoding a [`Command`].
#[derive(Debug)]
#[non_exhaustive]
pub enum EncodeError {
    /// The input buffer is too small to fit the encoded [`Command`].
    BufferTooSmall,
}

impl core::fmt::Display for EncodeError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::BufferTooSmall => write!(f, "Buffer too small"),
        }
    }
}

impl core::error::Error for EncodeError {}

/// An error that can occur when decoding a [`Command`].
#[derive(Debug)]
#[non_exhaustive]
pub enum DecodeError {
    /// Decode error.
    Decode(rusmpp_core::decode::DecodeError),
    /// Minimum command length not met.
    MinLength {
        /// The actual length of the command.
        actual: usize,
        /// The minimum required length of the command.
        min: usize,
    },
    /// Integral type conversion failed.
    InvalidLength(TryFromIntError),
}

impl core::fmt::Display for DecodeError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            DecodeError::Decode(e) => write!(f, "Decode error: {e}"),
            DecodeError::MinLength { actual, min } => {
                write!(
                    f,
                    "Minimum command length not met. actual: {actual}, min: {min}"
                )
            }

            DecodeError::InvalidLength(e) => {
                write!(f, "Integral type conversion failed: {e}")
            }
        }
    }
}

impl core::error::Error for DecodeError {}
