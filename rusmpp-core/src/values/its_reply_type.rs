use rusmpp_macros::Rusmpp;

#[repr(u8)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Default, Rusmpp)]
#[cfg_attr(feature = "arbitrary", derive(::arbitrary::Arbitrary))]
#[cfg_attr(feature = "serde", derive(::serde::Serialize))]
#[cfg_attr(feature = "serde-deserialize-unchecked", derive(::serde::Deserialize))]
pub enum ItsReplyType {
    #[default]
    Digit = 0,
    Number = 1,
    TelephoneNo = 2,
    Password = 3,
    CharacterLine = 4,
    Menu = 5,
    Date = 6,
    Time = 7,
    Continue = 8,
    Other(u8),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn encode_decode() {
        #[cfg(feature = "alloc")]
        crate::tests::owned::encode_decode_test_instances::<ItsReplyType>();
        crate::tests::borrowed::encode_decode_test_instances::<ItsReplyType>();
    }
}
