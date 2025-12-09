use rusmpp_macros::Rusmpp;

/// Type of Number.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Default, Rusmpp)]
#[repr(u8)]
#[cfg_attr(feature = "arbitrary", derive(::arbitrary::Arbitrary))]
#[cfg_attr(feature = "serde", derive(::serde::Serialize))]
#[cfg_attr(feature = "serde-deserialize-unchecked", derive(::serde::Deserialize))]
pub enum Ton {
    #[default]
    Unknown = 0b00000000,
    International = 0b00000001,
    National = 0b00000010,
    NetworkSpecific = 0b00000011,
    SubscriberNumber = 0b00000100,
    Alphanumeric = 0b00000101,
    Abbreviated = 0b00000110,
    Other(u8),
}

impl Ton {
    /// Create a new [`Ton`] with a value of 0.
    ///
    /// Equivalent to [`Ton::Unknown`].
    pub fn null() -> Self {
        Self::default()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn encode_decode() {
        crate::tests::borrowed::encode_decode_test_instances::<Ton>();
        #[cfg(feature = "alloc")]
        crate::tests::owned::encode_decode_test_instances::<Ton>();
    }
}
