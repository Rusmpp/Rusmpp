use rusmpp_macros::Rusmpp;

/// This field indicates the number of repeated broadcasts requested by the Submitter.
#[derive(Default, Debug, Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Rusmpp)]
#[cfg_attr(feature = "arbitrary", derive(::arbitrary::Arbitrary))]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]

pub struct BroadcastRepNum {
    pub value: u8,
}

impl BroadcastRepNum {
    pub const fn new(value: u8) -> Self {
        Self { value }
    }
}

impl From<u8> for BroadcastRepNum {
    fn from(value: u8) -> Self {
        Self::new(value)
    }
}

impl From<BroadcastRepNum> for u8 {
    fn from(value: BroadcastRepNum) -> Self {
        value.value
    }
}
