use rusmpp_macros::Rusmpp;

#[repr(u8)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Rusmpp)]
#[rusmpp(from_into = skip)]
#[cfg_attr(feature = "arbitrary", derive(::arbitrary::Arbitrary))]
#[cfg_attr(feature = "serde", derive(::serde::Serialize))]
#[cfg_attr(feature = "serde-deserialize-unchecked", derive(::serde::Deserialize))]
pub enum NumberOfMessages {
    Allowed(u8),
    Other(u8),
}

impl Default for NumberOfMessages {
    fn default() -> Self {
        NumberOfMessages::Allowed(0)
    }
}

impl From<u8> for NumberOfMessages {
    fn from(value: u8) -> Self {
        match value {
            0..=99 => NumberOfMessages::Allowed(value),
            _ => NumberOfMessages::Other(value),
        }
    }
}

impl From<NumberOfMessages> for u8 {
    fn from(value: NumberOfMessages) -> Self {
        match value {
            NumberOfMessages::Allowed(value) => value,
            NumberOfMessages::Other(value) => value,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from_u8() {
        assert_eq!(NumberOfMessages::from(0), NumberOfMessages::Allowed(0));
        assert_eq!(NumberOfMessages::from(50), NumberOfMessages::Allowed(50));
        assert_eq!(NumberOfMessages::from(99), NumberOfMessages::Allowed(99));
        assert_eq!(NumberOfMessages::from(100), NumberOfMessages::Other(100));
        assert_eq!(NumberOfMessages::from(255), NumberOfMessages::Other(255));
    }

    #[test]
    fn encode_decode() {
        #[cfg(feature = "alloc")]
        crate::tests::owned::encode_decode_test_instances::<NumberOfMessages>();
        crate::tests::borrowed::encode_decode_test_instances::<NumberOfMessages>();
    }
}
