use rusmpp_macros::Rusmpp;

/// The ms_availability_status parameter is used in the alert_notification operation to indicate the
/// availability state of the MS to the ESME.
///
/// If the MC does not include the parameter in the alert_notification operation, the ESME should
/// assume that the MS is in an “available” state.
#[repr(u8)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Default, Rusmpp)]
#[cfg_attr(feature = "arbitrary", derive(::arbitrary::Arbitrary))]
#[cfg_attr(feature = "serde", derive(::serde::Serialize))]
#[cfg_attr(feature = "serde-deserialize-unchecked", derive(::serde::Deserialize))]
pub enum MsAvailabilityStatus {
    #[default]
    Available = 0,
    Denied = 1,
    Unavailable = 2,
    Other(u8),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn encode_decode() {
        #[cfg(feature = "alloc")]
        crate::tests::owned::encode_decode_test_instances::<MsAvailabilityStatus>();
        crate::tests::borrowed::encode_decode_test_instances::<MsAvailabilityStatus>();
    }
}
