use rusmpp_macros::Rusmpp;

#[repr(u8)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Default, Rusmpp)]
#[cfg_attr(feature = "arbitrary", derive(::arbitrary::Arbitrary))]
#[cfg_attr(feature = "serde", derive(::serde::Serialize))]
#[cfg_attr(feature = "serde-deserialize-unchecked", derive(::serde::Deserialize))]
pub enum AddrSubunit {
    #[default]
    Unknown = 0x00,
    MSDisplay = 0x01,
    MobileEquipment = 0x02,
    SmartCard = 0x03,
    ExternalUnit = 0x04,
    Other(u8),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn encode_decode() {
        crate::tests::borrowed::encode_decode_test_instances::<AddrSubunit>();
        #[cfg(feature = "alloc")]
        crate::tests::owned::encode_decode_test_instances::<AddrSubunit>();
    }
}
