use rusmpp_macros::Rusmpp;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Default, Rusmpp)]
#[cfg_attr(feature = "arbitrary", derive(::arbitrary::Arbitrary))]
#[cfg_attr(feature = "serde", derive(::serde::Serialize))]
#[cfg_attr(feature = "serde-deserialize-unchecked", derive(::serde::Deserialize))]
pub struct ItsSessionInfo {
    pub session_number: u8,
    pub sequence_number: u8,
}

impl ItsSessionInfo {
    pub fn new(session_number: u8, sequence_number: u8) -> Self {
        Self {
            session_number,
            sequence_number,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn encode_decode() {
        #[cfg(feature = "alloc")]
        crate::tests::owned::encode_decode_test_instances::<ItsSessionInfo>();
        crate::tests::borrowed::encode_decode_test_instances::<ItsSessionInfo>();
    }
}
