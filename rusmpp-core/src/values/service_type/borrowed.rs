use rusmpp_macros::Rusmpp;

use crate::types::borrowed::COctetString;

use super::GenericServiceType;

impl From<GenericServiceType> for COctetString<'static, 1, 6> {
    fn from(value: GenericServiceType) -> Self {
        match value {
            GenericServiceType::Default => COctetString::null(),
            GenericServiceType::CellularMessaging => COctetString::new_unchecked(b"CMT\0"),
            GenericServiceType::CellularPaging => COctetString::new_unchecked(b"CPT\0"),
            GenericServiceType::VoiceMailNotification => COctetString::new_unchecked(b"VMN\0"),
            GenericServiceType::VoiceMailAlerting => COctetString::new_unchecked(b"VMA\0"),
            GenericServiceType::WirelessApplicationProtocol => {
                COctetString::new_unchecked(b"WAP\0")
            }
            GenericServiceType::UnstructuredSupplementaryServicesData => {
                COctetString::new_unchecked(b"USSD\0")
            }
            GenericServiceType::CellBroadcastService => COctetString::new_unchecked(b"CBS\0"),
            GenericServiceType::GenericUDPTransportService => {
                COctetString::new_unchecked(b"GUTS\0")
            }
        }
    }
}

impl<'a> From<GenericServiceType> for ServiceType<'a> {
    fn from(value: GenericServiceType) -> Self {
        ServiceType::new(value.into())
    }
}

/// The service_type parameter can be used to indicate the SMS Application service associated
/// with the message. Specifying the service_type allows the ESME to:
///
/// * Avail of enhanced messaging services such as replace_if_present by service type
///   (generic to all network types).
/// * Control the teleservice used on the air interface (e.g. ANSI-136/TDMA, IS-95/CDMA).
///
/// MCs may implicitly associate a ‘replace if present’ function from the indicated service_type in
/// a message submission operation, i.e., the MC will always replace an existing message
/// pending delivery, that has the same originating and destination address as the submitted
/// message. For example, a MC can ensure that a Voice Mail System using a service_type of
/// “VMA” has at most one outstanding notification per destination MS by automatically invoking
/// the “replace if present” function.
///
/// Note: In the case of Cell Broadcast Service replace functionality by service type is not
/// supported.
///
/// See [`GenericServiceType`].
#[derive(Default, Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Rusmpp)]
#[rusmpp(decode = borrowed)]
#[cfg_attr(feature = "arbitrary", derive(::arbitrary::Arbitrary))]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(bound(deserialize = "'de: 'a")))]
pub struct ServiceType<'a> {
    value: COctetString<'a, 1, 6>,
}

impl<'a> ServiceType<'a> {
    pub const fn new(value: COctetString<'a, 1, 6>) -> Self {
        Self { value }
    }

    /// Create a new [`ServiceType`] with a value of 0.
    pub const fn null() -> Self {
        Self {
            value: COctetString::null(),
        }
    }

    pub const fn value(&'_ self) -> &'_ COctetString<'_, 1, 6> {
        &self.value
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn encode_decode() {
        crate::tests::borrowed::encode_decode_test_instances::<ServiceType>();
    }
}
