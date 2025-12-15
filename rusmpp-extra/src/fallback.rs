//! Fallback behavior for encoding/decoding and concatenation.

/// Fallback error combining two errors.
#[derive(Debug, thiserror::Error)]
#[error("Both operations failed: {first}, {second}")]
pub struct FallbackError<T, U> {
    pub first: T,
    pub second: U,
}

impl<T, U> FallbackError<T, U> {
    /// Creates a new [`FallbackError`].
    pub const fn new(first: T, second: U) -> Self {
        Self { first, second }
    }
}

/// A wrapper that tries the first operation, and if it fails, tries the second.
#[derive(Debug)]
pub struct Fallback<T, U> {
    pub first: T,
    pub second: U,
}

impl<T, U> Fallback<T, U> {
    /// Creates a new [`Fallback`].
    pub const fn new(first: T, second: U) -> Self {
        Self { first, second }
    }
}
