use rusmpp_macros::Rusmpp;

pub mod borrowed;
#[cfg(feature = "alloc")]
#[cfg_attr(docsrs, doc(cfg(feature = "alloc")))]
pub mod owned;

#[repr(u8)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Default, Rusmpp)]
#[cfg_attr(feature = "arbitrary", derive(::arbitrary::Arbitrary))]
#[cfg_attr(feature = "serde", derive(::serde::Serialize))]
#[cfg_attr(feature = "serde-deserialize-unchecked", derive(::serde::Deserialize))]
pub enum SubaddressTag {
    #[default]
    NsapEven = 0b10000000,
    NsapOdd = 0b10001000,
    UserSpecified = 0b10100000,
    Other(u8),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn encode_decode() {
        crate::tests::borrowed::encode_decode_test_instances::<SubaddressTag>();
        #[cfg(feature = "alloc")]
        crate::tests::owned::encode_decode_test_instances::<SubaddressTag>();
    }
}
