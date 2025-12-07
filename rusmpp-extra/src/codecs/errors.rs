//! Errors related to encoded message creation.

use rusmpp_core::types::OctetStringError;

/// Errors that can occur during encoded message creation.
#[derive(Debug, thiserror::Error)]
#[non_exhaustive]
pub enum EncodeError<E> {
    #[error("Encode error: {0}")]
    Encode(E),
    #[error("Encoder produced invalid short message: {0}")]
    ShortMessage(
        #[from]
        #[source]
        OctetStringError,
    ),
}

impl<E> EncodeError<E> {
    pub(crate) const fn encode(error: E) -> Self {
        Self::Encode(error)
    }
}
