use rusmpp_macros::Rusmpp;

#[repr(u8)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Default, Rusmpp)]
#[cfg_attr(feature = "arbitrary", derive(::arbitrary::Arbitrary))]
#[cfg_attr(feature = "serde", derive(::serde::Serialize))]
#[cfg_attr(feature = "serde-deserialize-unchecked", derive(::serde::Deserialize))]
pub enum DestAddrNpResolution {
    #[default]
    QueryNotPerformed = 0,
    QueryPerformedNumberNotPorted = 1,
    QueryPerformedNumberPorted = 2,
    Other(u8),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn encode_decode() {
        #[cfg(feature = "alloc")]
        crate::tests::owned::encode_decode_test_instances::<DestAddrNpResolution>();
        crate::tests::borrowed::encode_decode_test_instances::<DestAddrNpResolution>();
    }
}
