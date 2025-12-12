use rusmpp_macros::Rusmpp;

#[repr(u8)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Default, Rusmpp)]
#[cfg_attr(feature = "arbitrary", derive(::arbitrary::Arbitrary))]
#[cfg_attr(feature = "serde", derive(::serde::Serialize))]
#[cfg_attr(feature = "serde-deserialize-unchecked", derive(::serde::Deserialize))]
pub enum UnitOfTime {
    #[default]
    AsFrequentlyAsPossible = 0x00,
    Seconds = 0x08,
    Minutes = 0x09,
    Hours = 0x0A,
    Days = 0x0B,
    Weeks = 0x0C,
    Months = 0x0D,
    Years = 0x0E,
    Other(u8),
}

/// This field indicates the frequency interval at which
/// the broadcasts of a message should be repeated.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Default, Rusmpp)]
#[cfg_attr(feature = "arbitrary", derive(::arbitrary::Arbitrary))]
#[cfg_attr(feature = "serde", derive(::serde::Serialize))]
#[cfg_attr(feature = "serde-deserialize-unchecked", derive(::serde::Deserialize))]
pub struct BroadcastFrequencyInterval {
    pub unit: UnitOfTime,
    pub value: u16,
}

impl BroadcastFrequencyInterval {
    pub fn new(unit: UnitOfTime, value: u16) -> Self {
        Self { unit, value }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn encode_decode() {
        #[cfg(feature = "alloc")]
        crate::tests::owned::encode_decode_test_instances::<UnitOfTime>();
        crate::tests::borrowed::encode_decode_test_instances::<UnitOfTime>();
        #[cfg(feature = "alloc")]
        crate::tests::owned::encode_decode_test_instances::<BroadcastFrequencyInterval>();
        crate::tests::borrowed::encode_decode_test_instances::<BroadcastFrequencyInterval>();
    }
}
