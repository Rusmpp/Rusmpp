use rusmpp_macros::Rusmpp;

/// See [`PriorityFlagType`].
#[derive(Default, Debug, Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Rusmpp)]
#[rusmpp(repr = "u8")]
#[cfg_attr(feature = "arbitrary", derive(::arbitrary::Arbitrary))]
#[cfg_attr(feature = "serde", derive(::serde::Serialize))]
#[cfg_attr(feature = "serde-deserialize-unchecked", derive(::serde::Deserialize))]
pub struct PriorityFlag {
    pub value: u8,
}

impl PriorityFlag {
    pub fn new(value: u8) -> Self {
        Self { value }
    }
}

impl From<PriorityFlagType> for PriorityFlag {
    fn from(value: PriorityFlagType) -> Self {
        Self {
            value: value.into(),
        }
    }
}

impl From<u8> for PriorityFlag {
    fn from(value: u8) -> Self {
        Self { value }
    }
}

impl From<PriorityFlag> for u8 {
    fn from(value: PriorityFlag) -> Self {
        value.value
    }
}

impl From<GsmSms> for PriorityFlag {
    fn from(value: GsmSms) -> Self {
        Self {
            value: value.into(),
        }
    }
}

impl From<GsmCbs> for PriorityFlag {
    fn from(value: GsmCbs) -> Self {
        Self {
            value: value.into(),
        }
    }
}

impl From<Ansi136> for PriorityFlag {
    fn from(value: Ansi136) -> Self {
        Self {
            value: value.into(),
        }
    }
}

impl From<Is95> for PriorityFlag {
    fn from(value: Is95) -> Self {
        Self {
            value: value.into(),
        }
    }
}

impl From<Ansi41Cbs> for PriorityFlag {
    fn from(value: Ansi41Cbs) -> Self {
        Self {
            value: value.into(),
        }
    }
}

/// Helper for creating a priority_flag.
///
/// A priority flag type can not be created from u8.
/// Depending on the variant you want to create, you need to use the variant itself.
///
/// # Example
///
/// ```rust
/// use rusmpp_core::values::{GsmSms, PriorityFlag, PriorityFlagType};
///
/// let gsm_sms = GsmSms::from(1);
/// assert_eq!(gsm_sms, GsmSms::Priority1);
///
/// let priority_flag_type = PriorityFlagType::from(gsm_sms);
/// assert!(matches!(priority_flag_type, PriorityFlagType::GsmSms(GsmSms::Priority1)));
///
/// let priority_flag = PriorityFlag::from(priority_flag_type);
/// assert_eq!(priority_flag, PriorityFlag::new(1));
///
/// let priority_flag: PriorityFlag = GsmSms::from(1).into();
/// assert_eq!(priority_flag, PriorityFlag::new(1));
/// ```
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum PriorityFlagType {
    GsmSms(GsmSms),
    GsmCbs(GsmCbs),
    Ansi136(Ansi136),
    Is95(Is95),
    Ansi41Cbs(Ansi41Cbs),
}

impl From<PriorityFlagType> for u8 {
    fn from(value: PriorityFlagType) -> Self {
        match value {
            PriorityFlagType::GsmSms(value) => value.into(),
            PriorityFlagType::GsmCbs(value) => value.into(),
            PriorityFlagType::Ansi136(value) => value.into(),
            PriorityFlagType::Is95(value) => value.into(),
            PriorityFlagType::Ansi41Cbs(value) => value.into(),
        }
    }
}

#[repr(u8)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
pub enum GsmSms {
    #[default]
    None = 0,
    Priority1 = 1,
    Priority2 = 2,
    Priority3 = 3,
}

impl From<u8> for GsmSms {
    fn from(value: u8) -> Self {
        match value {
            0 => GsmSms::None,
            1 => GsmSms::Priority1,
            2 => GsmSms::Priority2,
            3 => GsmSms::Priority3,
            _ => GsmSms::None,
        }
    }
}

impl From<GsmSms> for u8 {
    fn from(value: GsmSms) -> Self {
        match value {
            GsmSms::None => 0,
            GsmSms::Priority1 => 1,
            GsmSms::Priority2 => 2,
            GsmSms::Priority3 => 3,
        }
    }
}

impl From<GsmSms> for PriorityFlagType {
    fn from(value: GsmSms) -> Self {
        PriorityFlagType::GsmSms(value)
    }
}

#[repr(u8)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
pub enum GsmCbs {
    #[default]
    Normal = 0,
    ImmediateBroadcast = 1,
    HighPriority = 2,
    Reserved = 3,
    PriorityBackground = 4,
}

impl From<u8> for GsmCbs {
    fn from(value: u8) -> Self {
        match value {
            0 => GsmCbs::Normal,
            1 => GsmCbs::ImmediateBroadcast,
            2 => GsmCbs::HighPriority,
            3 => GsmCbs::Reserved,
            4 => GsmCbs::PriorityBackground,
            _ => GsmCbs::Normal,
        }
    }
}

impl From<GsmCbs> for u8 {
    fn from(value: GsmCbs) -> Self {
        match value {
            GsmCbs::Normal => 0,
            GsmCbs::ImmediateBroadcast => 1,
            GsmCbs::HighPriority => 2,
            GsmCbs::Reserved => 3,
            GsmCbs::PriorityBackground => 4,
        }
    }
}

impl From<GsmCbs> for PriorityFlagType {
    fn from(value: GsmCbs) -> Self {
        PriorityFlagType::GsmCbs(value)
    }
}

#[repr(u8)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
pub enum Ansi136 {
    #[default]
    Bulk = 0,
    Normal = 1,
    Urgent = 2,
    VeryUrgent = 3,
}

impl From<u8> for Ansi136 {
    fn from(value: u8) -> Self {
        match value {
            0 => Ansi136::Bulk,
            1 => Ansi136::Normal,
            2 => Ansi136::Urgent,
            3 => Ansi136::VeryUrgent,
            _ => Ansi136::Normal,
        }
    }
}

impl From<Ansi136> for u8 {
    fn from(value: Ansi136) -> Self {
        match value {
            Ansi136::Bulk => 0,
            Ansi136::Normal => 1,
            Ansi136::Urgent => 2,
            Ansi136::VeryUrgent => 3,
        }
    }
}

impl From<Ansi136> for PriorityFlagType {
    fn from(value: Ansi136) -> Self {
        PriorityFlagType::Ansi136(value)
    }
}

#[repr(u8)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
pub enum Is95 {
    #[default]
    Normal = 0,
    Interactive = 1,
    Urgent = 2,
    Emergency = 3,
}

impl From<u8> for Is95 {
    fn from(value: u8) -> Self {
        match value {
            0 => Is95::Normal,
            1 => Is95::Interactive,
            2 => Is95::Urgent,
            3 => Is95::Emergency,
            _ => Is95::Normal,
        }
    }
}

impl From<Is95> for u8 {
    fn from(value: Is95) -> Self {
        match value {
            Is95::Normal => 0,
            Is95::Interactive => 1,
            Is95::Urgent => 2,
            Is95::Emergency => 3,
        }
    }
}

impl From<Is95> for PriorityFlagType {
    fn from(value: Is95) -> Self {
        PriorityFlagType::Is95(value)
    }
}

#[repr(u8)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
pub enum Ansi41Cbs {
    #[default]
    Normal = 0,
    Interactive = 1,
    Urgent = 2,
    Emergency = 3,
}

impl From<u8> for Ansi41Cbs {
    fn from(value: u8) -> Self {
        match value {
            0 => Ansi41Cbs::Normal,
            1 => Ansi41Cbs::Interactive,
            2 => Ansi41Cbs::Urgent,
            3 => Ansi41Cbs::Emergency,
            _ => Ansi41Cbs::Normal,
        }
    }
}

impl From<Ansi41Cbs> for u8 {
    fn from(value: Ansi41Cbs) -> Self {
        match value {
            Ansi41Cbs::Normal => 0,
            Ansi41Cbs::Interactive => 1,
            Ansi41Cbs::Urgent => 2,
            Ansi41Cbs::Emergency => 3,
        }
    }
}

impl From<Ansi41Cbs> for PriorityFlagType {
    fn from(value: Ansi41Cbs) -> Self {
        PriorityFlagType::Ansi41Cbs(value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn gsm_sms() {
        let gsm_sms = GsmSms::from(0);
        assert_eq!(gsm_sms, GsmSms::None);

        let gsm_sms = GsmSms::from(1);
        assert_eq!(gsm_sms, GsmSms::Priority1);

        let gsm_sms = GsmSms::from(2);
        assert_eq!(gsm_sms, GsmSms::Priority2);

        let gsm_sms = GsmSms::from(3);
        assert_eq!(gsm_sms, GsmSms::Priority3);

        let gsm_sms = GsmSms::from(4);
        assert_eq!(gsm_sms, GsmSms::None);
    }

    #[test]
    fn encode_decode() {
        #[cfg(feature = "alloc")]
        crate::tests::owned::encode_decode_test_instances::<PriorityFlag>();
        crate::tests::borrowed::encode_decode_test_instances::<PriorityFlag>();
    }
}
