use rusmpp_macros::Rusmpp;

#[repr(u8)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Default, Rusmpp)]
#[rusmpp(from_into = skip)]
#[cfg_attr(feature = "arbitrary", derive(::arbitrary::Arbitrary))]
#[cfg_attr(feature = "serde", derive(::serde::Serialize))]
#[cfg_attr(feature = "serde-deserialize-unchecked", derive(::serde::Deserialize))]
pub enum CongestionState {
    #[default]
    Idle,
    LowLoad(u8),
    MediumLoad(u8),
    HighLoad(u8),
    OptimumLoad(u8),
    NearingCongestion(u8),
    Congested,
    Other(u8),
}

impl From<u8> for CongestionState {
    fn from(value: u8) -> Self {
        match value {
            0 => Self::Idle,
            1..=29 => Self::LowLoad(value),
            30..=49 => Self::MediumLoad(value),
            50..=79 => Self::HighLoad(value),
            80..=89 => Self::OptimumLoad(value),
            90..=99 => Self::NearingCongestion(value),
            100 => Self::Congested,
            _ => Self::Other(value),
        }
    }
}

impl From<CongestionState> for u8 {
    fn from(value: CongestionState) -> Self {
        match value {
            CongestionState::Idle => 0,
            CongestionState::LowLoad(v) => v,
            CongestionState::MediumLoad(v) => v,
            CongestionState::HighLoad(v) => v,
            CongestionState::OptimumLoad(v) => v,
            CongestionState::NearingCongestion(v) => v,
            CongestionState::Congested => 100,
            CongestionState::Other(v) => v,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn encode_decode() {
        #[cfg(feature = "alloc")]
        crate::tests::owned::encode_decode_test_instances::<CongestionState>();
        crate::tests::borrowed::encode_decode_test_instances::<CongestionState>();
    }
}
