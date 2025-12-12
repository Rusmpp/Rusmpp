use rusmpp_macros::Rusmpp;

#[repr(u8)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Default, Rusmpp)]
#[cfg_attr(feature = "arbitrary", derive(::arbitrary::Arbitrary))]
#[cfg_attr(feature = "serde", derive(::serde::Serialize))]
#[cfg_attr(feature = "serde-deserialize-unchecked", derive(::serde::Deserialize))]
pub enum BroadcastMessageClass {
    #[default]
    NoClassSpecified = 0x00,
    Class1 = 0x01,
    Class2 = 0x02,
    Class3 = 0x03,
    Other(u8),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn encode_decode() {
        #[cfg(feature = "alloc")]
        crate::tests::owned::encode_decode_test_instances::<BroadcastMessageClass>();
        crate::tests::borrowed::encode_decode_test_instances::<BroadcastMessageClass>();
    }
}
