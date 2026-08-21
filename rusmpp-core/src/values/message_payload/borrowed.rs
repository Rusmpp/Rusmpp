use rusmpp_macros::Rusmpp;

use crate::types::borrowed::AnyOctetString;

#[repr(transparent)]
#[derive(Default, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Rusmpp)]
#[rusmpp(decode = borrowed)]
#[cfg_attr(feature = "arbitrary", derive(::arbitrary::Arbitrary))]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(bound(deserialize = "'de: 'a"), transparent))]
pub struct MessagePayload<'a> {
    #[rusmpp(length = "unchecked")]
    pub value: AnyOctetString<'a>,
}

impl ::core::fmt::Debug for MessagePayload<'_> {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        ::core::fmt::Debug::fmt(&self.value, f)
    }
}

impl<'a> MessagePayload<'a> {
    pub const fn new(value: AnyOctetString<'a>) -> Self {
        Self { value }
    }
}

impl<'a> From<AnyOctetString<'a>> for MessagePayload<'a> {
    fn from(value: AnyOctetString<'a>) -> Self {
        Self::new(value)
    }
}

impl<'a> From<MessagePayload<'a>> for AnyOctetString<'a> {
    fn from(value: MessagePayload<'a>) -> Self {
        value.value
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn encode_decode() {
        crate::tests::borrowed::encode_decode_with_length_test_instances::<MessagePayload>();
    }
}
