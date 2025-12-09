use rusmpp_macros::Rusmpp;

use crate::{types::owned::OctetString, values::sub_address::SubaddressTag};

// https://smpp.org/SMPP_v5.pdf#page=165
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Default, Rusmpp)]
#[rusmpp(decode = owned)]
#[cfg_attr(feature = "arbitrary", derive(::arbitrary::Arbitrary))]
#[cfg_attr(feature = "serde", derive(::serde::Serialize))]
#[cfg_attr(feature = "serde-deserialize-unchecked", derive(::serde::Deserialize))]
pub struct Subaddress {
    pub tag: SubaddressTag,
    // addr can not be empty, because the whole source_subaddress tlv value is between 2 and 23 bytes long, and the tag is 1 byte long
    #[rusmpp(length = "unchecked")]
    pub addr: OctetString<1, 22>,
}

impl Subaddress {
    pub fn new(tag: SubaddressTag, addr: OctetString<1, 22>) -> Self {
        Self { tag, addr }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn encode_decode() {
        #[cfg(feature = "alloc")]
        crate::tests::owned::encode_decode_with_length_test_instances::<Subaddress>();
    }
}
