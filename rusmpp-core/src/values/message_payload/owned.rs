use rusmpp_macros::Rusmpp;

use crate::types::owned::AnyOctetString;

#[derive(Default, Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Rusmpp)]
#[rusmpp(decode = owned)]
#[cfg_attr(feature = "arbitrary", derive(::arbitrary::Arbitrary))]
#[cfg_attr(feature = "serde", derive(::serde::Serialize))]
#[cfg_attr(feature = "serde-deserialize-unchecked", derive(::serde::Deserialize))]
pub struct MessagePayload {
    #[rusmpp(length = "unchecked")]
    pub value: AnyOctetString,
}

impl MessagePayload {
    pub const fn new(value: AnyOctetString) -> Self {
        Self { value }
    }
}

impl From<AnyOctetString> for MessagePayload {
    fn from(value: AnyOctetString) -> Self {
        Self::new(value)
    }
}

impl From<MessagePayload> for AnyOctetString {
    fn from(value: MessagePayload) -> Self {
        value.value
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn encode_decode() {
        #[cfg(feature = "alloc")]
        crate::tests::owned::encode_decode_with_length_test_instances::<MessagePayload>();
    }
}
