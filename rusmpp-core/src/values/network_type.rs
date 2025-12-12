use rusmpp_macros::Rusmpp;

#[repr(u8)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Default, Rusmpp)]
#[cfg_attr(feature = "arbitrary", derive(::arbitrary::Arbitrary))]
#[cfg_attr(feature = "serde", derive(::serde::Serialize))]
#[cfg_attr(feature = "serde-deserialize-unchecked", derive(::serde::Deserialize))]
pub enum NetworkType {
    #[default]
    Unknown = 0x00,
    Gsm = 0x01,
    Ansi136 = 0x02,
    Is95 = 0x03,
    Pdc = 0x04,
    Phs = 0x05,
    IDen = 0x06,
    Amps = 0x07,
    PagingNetwork = 0x08,
    Other(u8),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn encode_decode() {
        #[cfg(feature = "alloc")]
        crate::tests::owned::encode_decode_test_instances::<NetworkType>();
        crate::tests::borrowed::encode_decode_test_instances::<NetworkType>();
    }
}
