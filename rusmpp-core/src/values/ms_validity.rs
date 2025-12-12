use rusmpp_macros::Rusmpp;

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Default, Rusmpp)]
#[cfg_attr(feature = "arbitrary", derive(::arbitrary::Arbitrary))]
#[cfg_attr(feature = "serde", derive(::serde::Serialize))]
#[cfg_attr(feature = "serde-deserialize-unchecked", derive(::serde::Deserialize))]
pub struct MsValidity {
    pub validity_behavior: MsValidityBehavior,
    #[rusmpp(length = "checked")]
    pub validity_information: Option<MsValidityInformation>,
}

impl MsValidity {
    pub fn new(
        validity_behavior: MsValidityBehavior,
        validity_information: Option<MsValidityInformation>,
    ) -> Self {
        Self {
            validity_behavior,
            validity_information,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Default, Rusmpp)]
#[cfg_attr(feature = "arbitrary", derive(::arbitrary::Arbitrary))]
#[cfg_attr(feature = "serde", derive(::serde::Serialize))]
#[cfg_attr(feature = "serde-deserialize-unchecked", derive(::serde::Deserialize))]
pub struct MsValidityInformation {
    pub units_of_time: UnitsOfTime,
    pub number_of_time_units: u16,
}

impl MsValidityInformation {
    pub fn new(units_of_time: UnitsOfTime, number_of_time_units: u16) -> Self {
        Self {
            units_of_time,
            number_of_time_units,
        }
    }
}

#[repr(u8)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Default, Rusmpp)]
#[cfg_attr(feature = "arbitrary", derive(::arbitrary::Arbitrary))]
#[cfg_attr(feature = "serde", derive(::serde::Serialize))]
#[cfg_attr(feature = "serde-deserialize-unchecked", derive(::serde::Deserialize))]
pub enum MsValidityBehavior {
    #[default]
    StoreIndefinitely = 0,
    PowerDown = 1,
    ValidUntilRegistrationAreaChanges = 2,
    DisplayOnly = 3,
    RelativeTimePeriod = 4,
    Other(u8),
}

#[repr(u8)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Default, Rusmpp)]
#[cfg_attr(feature = "arbitrary", derive(::arbitrary::Arbitrary))]
#[cfg_attr(feature = "serde", derive(::serde::Serialize))]
#[cfg_attr(feature = "serde-deserialize-unchecked", derive(::serde::Deserialize))]
pub enum UnitsOfTime {
    #[default]
    Seconds = 0b00000000,
    Minutes = 0b00000001,
    Hours = 0b00000010,
    Days = 0b00000011,
    Weeks = 0b00000100,
    Months = 0b00000101,
    Years = 0b00000110,
    Other(u8),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn encode_decode() {
        #[cfg(feature = "alloc")]
        crate::tests::owned::encode_decode_with_length_test_instances::<MsValidity>();
        crate::tests::borrowed::encode_decode_with_length_test_instances::<MsValidity>();
        #[cfg(feature = "alloc")]
        crate::tests::owned::encode_decode_test_instances::<MsValidityInformation>();
        crate::tests::borrowed::encode_decode_test_instances::<MsValidityInformation>();
        #[cfg(feature = "alloc")]
        crate::tests::owned::encode_decode_test_instances::<MsValidityBehavior>();
        crate::tests::borrowed::encode_decode_test_instances::<MsValidityBehavior>();
        #[cfg(feature = "alloc")]
        crate::tests::owned::encode_decode_test_instances::<UnitsOfTime>();
        crate::tests::borrowed::encode_decode_test_instances::<UnitsOfTime>();
    }
}
