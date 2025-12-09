use rusmpp_macros::Rusmpp;

#[repr(u8)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Default, Rusmpp)]
#[cfg_attr(feature = "arbitrary", derive(::arbitrary::Arbitrary))]
#[cfg_attr(feature = "serde", derive(::serde::Serialize))]
#[cfg_attr(feature = "serde-deserialize-unchecked", derive(::serde::Deserialize))]
pub enum BearerType {
    #[default]
    Unknown = 0x00,
    Sms = 0x01,
    Csd = 0x02,
    PacketData = 0x03,
    Ussd = 0x04,
    Cdpd = 0x05,
    DataTac = 0x06,
    FlexReFlex = 0x07,
    CellBroadcast = 0x08,
    Other(u8),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn encode_decode() {
        crate::tests::borrowed::encode_decode_test_instances::<BearerType>();
        #[cfg(feature = "alloc")]
        crate::tests::owned::encode_decode_test_instances::<BearerType>();
    }
}
