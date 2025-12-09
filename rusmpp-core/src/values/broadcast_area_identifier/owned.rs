use rusmpp_macros::Rusmpp;

use crate::types::owned::AnyOctetString;

use super::BroadcastAreaFormat;

/// The broadcast_area_identifier defines the Broadcast Area in terms of a geographical descriptor.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Default, Rusmpp)]
#[rusmpp(decode = owned)]
#[cfg_attr(feature = "arbitrary", derive(::arbitrary::Arbitrary))]
#[cfg_attr(feature = "serde", derive(::serde::Serialize))]
#[cfg_attr(feature = "serde-deserialize-unchecked", derive(::serde::Deserialize))]
pub struct BroadcastAreaIdentifier {
    pub format: BroadcastAreaFormat,
    #[rusmpp(length = "unchecked")]
    pub area: AnyOctetString,
}

impl BroadcastAreaIdentifier {
    pub fn new(format: BroadcastAreaFormat, area: AnyOctetString) -> Self {
        Self { format, area }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn encode_decode() {
        #[cfg(feature = "alloc")]
        crate::tests::owned::encode_decode_with_length_test_instances::<BroadcastAreaIdentifier>();
    }
}
