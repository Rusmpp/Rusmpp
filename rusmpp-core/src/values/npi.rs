use rusmpp_macros::Rusmpp;

/// Numeric Plan Indicator.
#[repr(u8)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Default, Rusmpp)]
#[cfg_attr(feature = "arbitrary", derive(::arbitrary::Arbitrary))]
#[cfg_attr(feature = "serde", derive(::serde::Serialize))]
#[cfg_attr(feature = "serde-deserialize-unchecked", derive(::serde::Deserialize))]
pub enum Npi {
    #[default]
    Unknown = 0b00000000,
    Isdn = 0b00000001,
    Data = 0b00000011,
    Telex = 0b00000100,
    LandMobile = 0b00000110,
    National = 0b00001000,
    Private = 0b00001001,
    Ermes = 0b00001010,
    Internet = 0b00001110,
    WapClientId = 0b00010010,
    Other(u8),
}

impl Npi {
    /// Create a new [`Npi`] with a value of 0.
    ///
    /// Equivalent to [`Npi::Unknown`].
    pub fn null() -> Self {
        Self::default()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn encode_decode() {
        crate::tests::borrowed::encode_decode_test_instances::<Npi>();
        #[cfg(feature = "alloc")]
        crate::tests::owned::encode_decode_test_instances::<Npi>();
    }
}
