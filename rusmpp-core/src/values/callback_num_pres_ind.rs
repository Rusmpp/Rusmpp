use rusmpp_macros::Rusmpp;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Default, Rusmpp)]
#[rusmpp(repr = "u8")]
#[cfg_attr(feature = "arbitrary", derive(::arbitrary::Arbitrary))]
#[cfg_attr(feature = "serde", derive(::serde::Serialize))]
#[cfg_attr(feature = "serde-deserialize-unchecked", derive(::serde::Deserialize))]
pub struct CallbackNumPresInd {
    pub presentation: Presentation,
    pub screening: Screening,
}

impl CallbackNumPresInd {
    pub fn new(presentation: Presentation, screening: Screening) -> Self {
        Self {
            presentation,
            screening,
        }
    }
}

impl From<u8> for CallbackNumPresInd {
    fn from(value: u8) -> Self {
        Self {
            presentation: Presentation::from(value & 0b00000011),
            screening: Screening::from(value & 0b00001100),
        }
    }
}

impl From<CallbackNumPresInd> for u8 {
    fn from(value: CallbackNumPresInd) -> Self {
        u8::from(value.presentation) | u8::from(value.screening)
    }
}

#[repr(u8)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Default, Rusmpp)]
#[cfg_attr(feature = "arbitrary", derive(::arbitrary::Arbitrary))]
#[cfg_attr(feature = "serde", derive(::serde::Serialize))]
#[cfg_attr(feature = "serde-deserialize-unchecked", derive(::serde::Deserialize))]
pub enum Presentation {
    #[default]
    PresentationAllowed = 0b00000000,
    PresentationRestricted = 0b00000001,
    NumberNotAvailable = 0b00000010,
    Other(u8),
}

#[repr(u8)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Default, Rusmpp)]
#[cfg_attr(feature = "arbitrary", derive(::arbitrary::Arbitrary))]
#[cfg_attr(feature = "serde", derive(::serde::Serialize))]
#[cfg_attr(feature = "serde-deserialize-unchecked", derive(::serde::Deserialize))]
pub enum Screening {
    #[default]
    NotScreened = 0b00000000,
    VerifiedAndPassed = 0b00000100,
    VerifiedAndFailed = 0b00001000,
    NetworkProvided = 0b00001100,
    Other(u8),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn to_u8() {
        let callback_num_pres_ind = CallbackNumPresInd {
            presentation: Presentation::PresentationRestricted,
            screening: Screening::VerifiedAndFailed,
        };

        assert_eq!(u8::from(callback_num_pres_ind), 0b00001001);
    }

    #[test]
    fn from_u8() {
        let callback_num_pres_ind = CallbackNumPresInd::from(0b00001001);

        assert_eq!(
            callback_num_pres_ind,
            CallbackNumPresInd {
                presentation: Presentation::PresentationRestricted,
                screening: Screening::VerifiedAndFailed,
            }
        );
    }

    #[test]
    fn encode_decode() {
        #[cfg(feature = "alloc")]
        crate::tests::owned::encode_decode_test_instances::<CallbackNumPresInd>();
        crate::tests::borrowed::encode_decode_test_instances::<CallbackNumPresInd>();
        #[cfg(feature = "alloc")]
        crate::tests::owned::encode_decode_test_instances::<Presentation>();
        crate::tests::borrowed::encode_decode_test_instances::<Presentation>();
        #[cfg(feature = "alloc")]
        crate::tests::owned::encode_decode_test_instances::<Screening>();
        crate::tests::borrowed::encode_decode_test_instances::<Screening>();
    }
}
