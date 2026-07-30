use rusmpp_macros::Rusmpp;

use crate::types::borrowed::AnyOctetString;

use super::BroadcastAreaFormat;

/// The broadcast_area_identifier defines the Broadcast Area in terms of a geographical descriptor.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Default, Rusmpp)]
#[rusmpp(decode = borrowed)]
#[cfg_attr(feature = "arbitrary", derive(::arbitrary::Arbitrary))]
#[cfg_attr(feature = "serde", derive(::serde::Serialize))]
pub struct BroadcastAreaIdentifier<'a> {
    pub format: BroadcastAreaFormat,
    #[rusmpp(length = "unchecked")]
    pub area: AnyOctetString<'a>,
}

impl<'a> BroadcastAreaIdentifier<'a> {
    pub const fn new(format: BroadcastAreaFormat, area: AnyOctetString<'a>) -> Self {
        Self { format, area }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn encode_decode() {
        crate::tests::borrowed::encode_decode_with_length_test_instances::<BroadcastAreaIdentifier>(
        );
    }
}
