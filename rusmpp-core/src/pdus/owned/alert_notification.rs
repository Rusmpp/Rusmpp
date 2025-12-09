use rusmpp_macros::Rusmpp;

use crate::{
    pdus::owned::Pdu,
    tlvs::owned::{Tlv, TlvValue},
    types::owned::COctetString,
    values::*,
};

/// The alert_notification PDU is sent by the MC to the ESME across a Receiver or Transceiver
/// session. It is sent when the MC has detected that a particular mobile subscriber has become
/// available and a delivery pending flag had been previously set for that subscriber by means of
/// the set_dpf TLV.
///
/// A typical use of this operation is to trigger a data content ‘Push’ to the subscriber from a WAP
/// Proxy Server.
///
/// Note: There is no associated alert_notification_resp PDU.
#[derive(Default, Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Rusmpp)]
#[rusmpp(decode = owned, test = skip)]
#[cfg_attr(feature = "arbitrary", derive(::arbitrary::Arbitrary))]
#[cfg_attr(feature = "serde", derive(::serde::Serialize))]
#[cfg_attr(feature = "serde-deserialize-unchecked", derive(::serde::Deserialize))]
pub struct AlertNotification {
    /// Type of Number for alert SME.
    pub source_addr_ton: Ton,
    /// Numbering Plan Indicator for alert SME.
    pub source_addr_npi: Npi,
    /// Address of alert SME.
    pub source_addr: COctetString<1, 65>,
    /// Type of Number for ESME address
    /// which requested the alert.
    pub esme_addr_ton: Ton,
    /// Numbering Plan Indicator for ESME
    /// address which requested the alert.
    pub esme_addr_npi: Npi,
    /// Address for ESME which requested the alert.
    pub esme_addr: COctetString<1, 65>,
    /// The status of the mobile station [`MsAvailabilityStatus`].
    #[rusmpp(length = "checked")]
    ms_availability_status: Option<Tlv>,
}

impl AlertNotification {
    pub fn new(
        source_addr_ton: Ton,
        source_addr_npi: Npi,
        source_addr: COctetString<1, 65>,
        esme_addr_ton: Ton,
        esme_addr_npi: Npi,
        esme_addr: COctetString<1, 65>,
        ms_availability_status: Option<MsAvailabilityStatus>,
    ) -> Self {
        Self {
            source_addr_ton,
            source_addr_npi,
            source_addr,
            esme_addr_ton,
            esme_addr_npi,
            esme_addr,
            ms_availability_status: ms_availability_status
                .map(TlvValue::MsAvailabilityStatus)
                .map(From::from),
        }
    }

    pub const fn ms_availability_status_tlv(&self) -> Option<&Tlv> {
        self.ms_availability_status.as_ref()
    }

    pub fn ms_availability_status(&self) -> Option<MsAvailabilityStatus> {
        self.ms_availability_status_tlv()
            .and_then(|tlv| match tlv.value() {
                Some(TlvValue::MsAvailabilityStatus(value)) => Some(value),
                _ => None,
            })
            .copied()
    }

    pub fn set_ms_availability_status(
        &mut self,
        ms_availability_status: Option<MsAvailabilityStatus>,
    ) {
        self.ms_availability_status = ms_availability_status
            .map(TlvValue::MsAvailabilityStatus)
            .map(From::from);
    }

    pub fn builder() -> AlertNotificationBuilder {
        AlertNotificationBuilder::new()
    }
}

impl From<AlertNotification> for Pdu {
    fn from(value: AlertNotification) -> Self {
        Self::AlertNotification(value)
    }
}

#[derive(Debug, Default)]
pub struct AlertNotificationBuilder {
    inner: AlertNotification,
}

impl AlertNotificationBuilder {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn source_addr_ton(mut self, source_addr_ton: Ton) -> Self {
        self.inner.source_addr_ton = source_addr_ton;
        self
    }

    pub fn source_addr_npi(mut self, source_addr_npi: Npi) -> Self {
        self.inner.source_addr_npi = source_addr_npi;
        self
    }

    pub fn source_addr(mut self, source_addr: COctetString<1, 65>) -> Self {
        self.inner.source_addr = source_addr;
        self
    }

    pub fn esme_addr_ton(mut self, esme_addr_ton: Ton) -> Self {
        self.inner.esme_addr_ton = esme_addr_ton;
        self
    }

    pub fn esme_addr_npi(mut self, esme_addr_npi: Npi) -> Self {
        self.inner.esme_addr_npi = esme_addr_npi;
        self
    }

    pub fn esme_addr(mut self, esme_addr: COctetString<1, 65>) -> Self {
        self.inner.esme_addr = esme_addr;
        self
    }

    pub fn ms_availability_status(
        mut self,
        ms_availability_status: Option<MsAvailabilityStatus>,
    ) -> Self {
        self.inner
            .set_ms_availability_status(ms_availability_status);
        self
    }

    pub fn build(self) -> AlertNotification {
        self.inner
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use crate::tests::TestInstance;

    use super::*;

    impl TestInstance for AlertNotification {
        fn instances() -> alloc::vec::Vec<Self> {
            alloc::vec![
                Self::default(),
                Self::builder()
                    .ms_availability_status(Some(MsAvailabilityStatus::Available))
                    .build(),
                Self::builder()
                    .source_addr_ton(Ton::International)
                    .source_addr_npi(Npi::Isdn)
                    .source_addr(COctetString::from_str("1234567890").unwrap())
                    .esme_addr_ton(Ton::International)
                    .esme_addr_npi(Npi::Isdn)
                    .esme_addr(COctetString::from_str("0987654321").unwrap())
                    .ms_availability_status(Some(MsAvailabilityStatus::Available))
                    .build(),
                Self::builder()
                    .source_addr_ton(Ton::NetworkSpecific)
                    .source_addr_npi(Npi::LandMobile)
                    .source_addr(COctetString::from_str("1234567890").unwrap())
                    .esme_addr_ton(Ton::Abbreviated)
                    .esme_addr_npi(Npi::WapClientId)
                    .esme_addr(COctetString::from_str("0987654321").unwrap())
                    .ms_availability_status(Some(MsAvailabilityStatus::Other(255)))
                    .build(),
            ]
        }
    }

    #[test]
    fn encode_decode() {
        #[cfg(feature = "alloc")]
        crate::tests::owned::encode_decode_with_length_test_instances::<AlertNotification>();
    }
}
