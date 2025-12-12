use rusmpp_macros::Rusmpp;

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Default, Rusmpp)]
#[cfg_attr(feature = "arbitrary", derive(::arbitrary::Arbitrary))]
#[cfg_attr(feature = "serde", derive(::serde::Serialize))]
#[cfg_attr(feature = "serde-deserialize-unchecked", derive(::serde::Deserialize))]
pub struct NetworkErrorCode {
    pub network_type: ErrorCodeNetworkType,
    pub error_code: u16,
}

impl NetworkErrorCode {
    pub fn new(network_type: ErrorCodeNetworkType, error_code: u16) -> Self {
        Self {
            network_type,
            error_code,
        }
    }
}

#[repr(u8)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Default, Rusmpp)]
#[cfg_attr(feature = "arbitrary", derive(::arbitrary::Arbitrary))]
#[cfg_attr(feature = "serde", derive(::serde::Serialize))]
#[cfg_attr(feature = "serde-deserialize-unchecked", derive(::serde::Deserialize))]
pub enum ErrorCodeNetworkType {
    Ansi136AccessDeniedReason = 1,
    Is95AccessDeniedReason = 2,
    Gsm = 3,
    Ansi136CauseCode = 4,
    Is95CauseCode = 5,
    Ansi41Error = 6,
    #[default]
    SmppError = 7,
    MessageCenterSpecific = 8,
    Other(u8),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn encode_decode() {
        #[cfg(feature = "alloc")]
        crate::tests::owned::encode_decode_test_instances::<NetworkErrorCode>();
        crate::tests::borrowed::encode_decode_test_instances::<NetworkErrorCode>();
        #[cfg(feature = "alloc")]
        crate::tests::owned::encode_decode_test_instances::<ErrorCodeNetworkType>();
        crate::tests::borrowed::encode_decode_test_instances::<ErrorCodeNetworkType>();
    }
}
