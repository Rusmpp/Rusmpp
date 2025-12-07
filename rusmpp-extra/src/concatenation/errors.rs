//! Errors related to concatenated message creation.

use rusmpp_core::types::OctetStringError;

use crate::concatenation::{MAX_PARTS, MIN_PARTS};

/// Errors that can occur during multipart message creation.
#[derive(Debug, thiserror::Error)]
#[non_exhaustive]
pub enum MultipartError<E> {
    #[error("Concatenation error: {0}")]
    Concatenation(E),
    #[error("Encoder produced invalid short message: {0}")]
    ShortMessage(
        #[from]
        #[source]
        OctetStringError,
    ),
    #[error("The number of parts is less than the minimum required. actual: {actual}, min: {min}")]
    MinPartCount {
        /// The minimum required number of parts.
        min: usize,
        /// The actual number of parts.
        actual: usize,
    },
    #[error("The number of parts exceeds the maximum allowed. actual: {actual}, max: {max}")]
    MaxPartsCount {
        /// The maximum allowed number of parts.
        max: usize,
        /// The actual number of parts.
        actual: usize,
    },
}

impl<E> MultipartError<E> {
    pub(crate) const fn concatenation(error: E) -> Self {
        Self::Concatenation(error)
    }

    pub(crate) const fn min_part_count(actual: usize) -> Self {
        Self::MinPartCount {
            min: MIN_PARTS,
            actual,
        }
    }

    pub(crate) const fn max_parts_count(actual: usize) -> Self {
        Self::MaxPartsCount {
            max: MAX_PARTS,
            actual,
        }
    }
}
