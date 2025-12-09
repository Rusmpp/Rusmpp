use rusmpp_macros::Rusmpp;

/// The success rate indicator, defined as the ratio of the
/// number of BTSs that accepted the message and the total
/// number of BTSs that should have accepted the message, for
/// a particular broadcast_area_identifier.
#[repr(u8)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Default, Rusmpp)]
#[rusmpp(from_into = skip)]
#[cfg_attr(feature = "arbitrary", derive(::arbitrary::Arbitrary))]
#[cfg_attr(feature = "serde", derive(::serde::Serialize))]
#[cfg_attr(feature = "serde-deserialize-unchecked", derive(::serde::Deserialize))]
pub enum BroadcastAreaSuccess {
    #[default]
    InformationNotAvailable,
    ZeroToHundred(u8),
    Other(u8),
}

impl From<BroadcastAreaSuccess> for u8 {
    fn from(value: BroadcastAreaSuccess) -> Self {
        match value {
            BroadcastAreaSuccess::InformationNotAvailable => 255,
            BroadcastAreaSuccess::ZeroToHundred(value) => value,
            BroadcastAreaSuccess::Other(value) => value,
        }
    }
}

impl From<u8> for BroadcastAreaSuccess {
    fn from(value: u8) -> Self {
        match value {
            0..=100 => Self::ZeroToHundred(value),
            255 => Self::InformationNotAvailable,
            _ => Self::Other(value),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn encode_decode() {
        #[cfg(feature = "alloc")]
        crate::tests::owned::encode_decode_test_instances::<BroadcastAreaSuccess>();
        crate::tests::borrowed::encode_decode_test_instances::<BroadcastAreaSuccess>();
    }
}
