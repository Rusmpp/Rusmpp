use alloc::vec::Vec;

/// Represents either a single encoded message or a vector of concatenated message parts.
#[derive(Debug)]
pub enum Concatenation {
    /// A single encoded message.
    Single(Vec<u8>),
    /// A vector of concatenated message parts.
    Concatenated(Vec<Vec<u8>>),
}

impl Concatenation {
    /// Creates a new [`Concatenation::Single`] instance.
    pub(crate) const fn single(value: Vec<u8>) -> Self {
        Self::Single(value)
    }

    /// Creates a new [`Concatenation::Concatenated`] instance.
    pub(crate) const fn concatenated(value: Vec<Vec<u8>>) -> Self {
        Self::Concatenated(value)
    }

    #[cfg(test)]
    /// Collects the [`Concatenation`] into a vector of `Vec<u8>`.
    pub(crate) fn collect(self) -> Vec<Vec<u8>> {
        match self {
            Self::Single(part) => alloc::vec![part],
            Self::Concatenated(vec) => vec,
        }
    }
}
