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
pub enum BroadcastAreaFormat {
    #[default]
    AliasName = 0x00,
    EllipsoidArc = 0x01,
    Polygon = 0x02,
    Other(u8),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn encode_decode() {
        #[cfg(feature = "alloc")]
        crate::tests::owned::encode_decode_test_instances::<BroadcastAreaFormat>();
        crate::tests::borrowed::encode_decode_test_instances::<BroadcastAreaFormat>();
    }
}
