use rusmpp_macros::Rusmpp;

#[repr(u8)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Default, Rusmpp)]
#[rusmpp(from_into = skip)]
#[cfg_attr(feature = "arbitrary", derive(::arbitrary::Arbitrary))]
#[cfg_attr(feature = "serde", derive(::serde::Serialize))]
#[cfg_attr(feature = "serde-deserialize-unchecked", derive(::serde::Deserialize))]
pub enum InterfaceVersion {
    Smpp3_3OrEarlier(u8),
    Smpp3_4 = 0x34,
    #[default]
    Smpp5_0 = 0x50,
    Other(u8),
}

impl From<InterfaceVersion> for u8 {
    fn from(value: InterfaceVersion) -> Self {
        match value {
            InterfaceVersion::Smpp3_3OrEarlier(value) => value,
            InterfaceVersion::Smpp3_4 => 0x34,
            InterfaceVersion::Smpp5_0 => 0x50,
            InterfaceVersion::Other(value) => value,
        }
    }
}

impl From<u8> for InterfaceVersion {
    fn from(value: u8) -> Self {
        match value {
            0x00..=0x33 => Self::Smpp3_3OrEarlier(value),
            0x34 => Self::Smpp3_4,
            0x50 => Self::Smpp5_0,
            _ => Self::Other(value),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn encode_decode() {
        crate::tests::borrowed::encode_decode_test_instances::<InterfaceVersion>();
        #[cfg(feature = "alloc")]
        crate::tests::owned::encode_decode_test_instances::<InterfaceVersion>();
    }
}
