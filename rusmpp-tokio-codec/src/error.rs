//! Error types for the [`CommandCodec`](crate::CommandCodec).

use std::num::TryFromIntError;

use rusmpp_core::command::owned::CommandDecodeError;

/// An error that can occur when encoding a `Command`.
#[derive(Debug)]
#[non_exhaustive]
pub enum EncodeError {
    /// I/O error.
    Io(std::io::Error),
}

impl From<std::io::Error> for EncodeError {
    fn from(e: std::io::Error) -> Self {
        EncodeError::Io(e)
    }
}

impl core::fmt::Display for EncodeError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            EncodeError::Io(e) => write!(f, "I/O error: {e}"),
        }
    }
}

impl core::error::Error for EncodeError {
    fn source(&self) -> Option<&(dyn core::error::Error + 'static)> {
        match self {
            EncodeError::Io(e) => Some(e),
        }
    }

    fn cause(&self) -> Option<&dyn core::error::Error> {
        self.source()
    }
}

/// An error that can occur when decoding a `Command`.
#[derive(Debug)]
#[non_exhaustive]
pub enum DecodeError {
    /// I/O error.
    Io(std::io::Error),
    /// Decode error.
    Decode(std::boxed::Box<CommandDecodeError>),
    /// Minimum command length not met.
    MinLength { actual: usize, min: usize },
    /// Maximum command length exceeded.
    MaxLength { actual: usize, max: usize },
    /// Integral type conversion failed.
    InvalidLength(TryFromIntError),
}

impl From<std::io::Error> for DecodeError {
    fn from(e: std::io::Error) -> Self {
        DecodeError::Io(e)
    }
}

impl core::fmt::Display for DecodeError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            DecodeError::Io(e) => write!(f, "I/O error: {e}"),
            DecodeError::Decode(e) => write!(f, "Decode error: {e}"),
            DecodeError::MinLength { actual, min } => {
                write!(
                    f,
                    "Minimum command length not met. actual: {actual}, min: {min}"
                )
            }
            DecodeError::MaxLength { actual, max } => {
                write!(
                    f,
                    "Maximum command length exceeded. actual: {actual}, max: {max}"
                )
            }
            DecodeError::InvalidLength(e) => {
                write!(f, "Integral type conversion failed: {e}")
            }
        }
    }
}

impl core::error::Error for DecodeError {
    fn source(&self) -> Option<&(dyn core::error::Error + 'static)> {
        match self {
            DecodeError::Io(e) => Some(e),
            DecodeError::Decode(e) => Some(e),
            DecodeError::MinLength { .. } => None,
            DecodeError::MaxLength { .. } => None,
            DecodeError::InvalidLength(e) => Some(e),
        }
    }

    fn cause(&self) -> Option<&dyn core::error::Error> {
        self.source()
    }
}
