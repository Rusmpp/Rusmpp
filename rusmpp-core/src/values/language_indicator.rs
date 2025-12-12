use rusmpp_macros::Rusmpp;

/// Refer to [CMT-136] for other values.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Default, Rusmpp)]
#[repr(u8)]
#[cfg_attr(feature = "arbitrary", derive(::arbitrary::Arbitrary))]
#[cfg_attr(feature = "serde", derive(::serde::Serialize))]
#[cfg_attr(feature = "serde-deserialize-unchecked", derive(::serde::Deserialize))]
pub enum LanguageIndicator {
    #[default]
    Unspecified = 0,
    English = 1,
    French = 2,
    Spanish = 3,
    German = 4,
    Portuguese = 5,
    Other(u8),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn encode_decode() {
        #[cfg(feature = "alloc")]
        crate::tests::owned::encode_decode_test_instances::<LanguageIndicator>();
        crate::tests::borrowed::encode_decode_test_instances::<LanguageIndicator>();
    }
}
