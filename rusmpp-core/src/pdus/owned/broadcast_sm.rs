use rusmpp_macros::Rusmpp;

use crate::{
    pdus::owned::Pdu,
    tlvs::owned::{BroadcastRequestTlvValue, Tlv},
    types::owned::{COctetString, EmptyOrFullCOctetString},
    values::{owned::*, *},
};

/// This operation is issued by the ESME to submit a message to the Message Centre for
/// broadcast to a specified geographical area or set of geographical areas.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Default, Rusmpp)]
#[rusmpp(decode = owned, test = skip)]
#[cfg_attr(feature = "arbitrary", derive(::arbitrary::Arbitrary))]
#[cfg_attr(feature = "serde", derive(::serde::Serialize))]
#[cfg_attr(feature = "serde-deserialize-unchecked", derive(::serde::Deserialize))]
pub struct BroadcastSm {
    /// The service_type parameter can be used to
    /// indicate the SMS Application service
    /// associated with the message. Specifying the
    /// service_type allows the ESME to avail of enhanced
    /// messaging services such as “replace by
    /// service_type” or control the teleservice used on the air
    /// interface.
    ///
    /// Set to NULL for default MC settings.
    pub service_type: ServiceType,
    /// Type of Number for source address.
    ///
    /// If not known, set to NULL (Unknown).
    pub source_addr_ton: Ton,
    /// Numbering Plan Indicator for source address.
    ///
    /// If not known, set to NULL (Unknown).
    pub source_addr_npi: Npi,
    /// Address of SME which originated this message.
    ///
    /// If not known, set to NULL (Unknown).
    pub source_addr: COctetString<1, 21>,
    /// If using broadcast_sm to replace a message,
    /// previously submitted for broadcast, then set
    /// message_id to the MC assigned message ID
    /// allocated to the original message and returned in
    /// the broadcast_sm_resp  (to the original broadcast_sm request).
    ///
    /// Note: For "broadcast replace", either the message_id or the
    /// user_message_reference field should be used. Both
    /// fields must not be used simultaneously.
    /// Set to NULL:
    ///
    /// * if not using MC message ID in broadcast_sm to
    ///   replace a message, previously submitted for broadcast.
    /// * if setting user_message_reference TLV.
    pub message_id: COctetString<1, 65>,
    /// Designates the propriety level of the message.
    pub priority_flag: PriorityFlag,
    /// The short message is to be scheduled by the MC for
    /// delivery.
    ///
    /// Set to NULL for immediate message broadcast.
    pub schedule_delivery_time: EmptyOrFullCOctetString<17>,
    /// The validity period of this message.
    ///
    /// Set to NULL to specify that a ‘broadcast_rep_num’
    /// parameter and a ‘broadcast_frequency_interval’
    /// parameter have been specified from which a
    /// default value should be derived.
    pub validity_period: EmptyOrFullCOctetString<17>,
    /// Flag indicating if the submitted message should
    /// replace an existing message which has:
    /// * (1) MC message ID matching the ID supplied in
    ///   the message_id field.
    /// * (2) or ESME assigned message reference number
    ///   supplied in the user_message_reference field.
    pub replace_if_present_flag: ReplaceIfPresentFlag,
    /// Defines the encoding scheme of the short
    /// message user data.
    pub data_coding: DataCoding,
    /// Indicates the short message to send from a list of pre-
    /// defined (‘canned’) short messages stored on the MC.
    ///
    /// If not using a MC canned message, set to NULL.
    pub sm_default_msg_id: u8,
    /// Broadcast request TLVs ([`BroadcastRequestTlvValue`]).
    #[rusmpp(length = "unchecked")]
    tlvs: alloc::vec::Vec<Tlv>,
}

impl BroadcastSm {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        service_type: ServiceType,
        source_addr_ton: Ton,
        source_addr_npi: Npi,
        source_addr: COctetString<1, 21>,
        message_id: COctetString<1, 65>,
        priority_flag: PriorityFlag,
        schedule_delivery_time: EmptyOrFullCOctetString<17>,
        validity_period: EmptyOrFullCOctetString<17>,
        replace_if_present_flag: ReplaceIfPresentFlag,
        data_coding: DataCoding,
        sm_default_msg_id: u8,
        tlvs: alloc::vec::Vec<impl Into<BroadcastRequestTlvValue>>,
    ) -> Self {
        let tlvs = tlvs.into_iter().map(Into::into).map(From::from).collect();

        Self {
            service_type,
            source_addr_ton,
            source_addr_npi,
            source_addr,
            message_id,
            priority_flag,
            schedule_delivery_time,
            validity_period,
            replace_if_present_flag,
            data_coding,
            sm_default_msg_id,
            tlvs,
        }
    }

    pub fn tlvs(&self) -> &[Tlv] {
        &self.tlvs
    }

    pub fn set_tlvs(&mut self, tlvs: alloc::vec::Vec<impl Into<BroadcastRequestTlvValue>>) {
        self.tlvs = tlvs.into_iter().map(Into::into).map(From::from).collect();
    }

    pub fn clear_tlvs(&mut self) {
        self.tlvs.clear();
    }

    pub fn push_tlv(&mut self, tlv: impl Into<BroadcastRequestTlvValue>) {
        self.tlvs.push(Tlv::from(tlv.into()));
    }

    pub fn builder() -> BroadcastSmBuilder {
        BroadcastSmBuilder::new()
    }
}

impl From<BroadcastSm> for Pdu {
    fn from(value: BroadcastSm) -> Self {
        Self::BroadcastSm(value)
    }
}

#[derive(Debug, Default)]
pub struct BroadcastSmBuilder {
    inner: BroadcastSm,
}

impl BroadcastSmBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn service_type(mut self, service_type: ServiceType) -> Self {
        self.inner.service_type = service_type;
        self
    }

    pub fn source_addr_ton(mut self, source_addr_ton: Ton) -> Self {
        self.inner.source_addr_ton = source_addr_ton;
        self
    }

    pub fn source_addr_npi(mut self, source_addr_npi: Npi) -> Self {
        self.inner.source_addr_npi = source_addr_npi;
        self
    }

    pub fn source_addr(mut self, source_addr: COctetString<1, 21>) -> Self {
        self.inner.source_addr = source_addr;
        self
    }

    pub fn message_id(mut self, message_id: COctetString<1, 65>) -> Self {
        self.inner.message_id = message_id;
        self
    }

    pub fn priority_flag(mut self, priority_flag: PriorityFlag) -> Self {
        self.inner.priority_flag = priority_flag;
        self
    }

    pub fn schedule_delivery_time(
        mut self,
        schedule_delivery_time: EmptyOrFullCOctetString<17>,
    ) -> Self {
        self.inner.schedule_delivery_time = schedule_delivery_time;
        self
    }

    pub fn validity_period(mut self, validity_period: EmptyOrFullCOctetString<17>) -> Self {
        self.inner.validity_period = validity_period;
        self
    }

    pub fn replace_if_present_flag(
        mut self,
        replace_if_present_flag: ReplaceIfPresentFlag,
    ) -> Self {
        self.inner.replace_if_present_flag = replace_if_present_flag;
        self
    }

    pub fn data_coding(mut self, data_coding: DataCoding) -> Self {
        self.inner.data_coding = data_coding;
        self
    }

    pub fn sm_default_msg_id(mut self, sm_default_msg_id: u8) -> Self {
        self.inner.sm_default_msg_id = sm_default_msg_id;
        self
    }

    pub fn tlvs(mut self, tlvs: alloc::vec::Vec<impl Into<BroadcastRequestTlvValue>>) -> Self {
        self.inner.set_tlvs(tlvs);
        self
    }

    pub fn clear_tlvs(mut self) -> Self {
        self.inner.clear_tlvs();
        self
    }

    pub fn push_tlv(mut self, tlv: impl Into<BroadcastRequestTlvValue>) -> Self {
        self.inner.push_tlv(tlv);
        self
    }

    pub fn build(self) -> BroadcastSm {
        self.inner
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use crate::{tests::TestInstance, types::owned::OctetString};

    use super::*;

    impl TestInstance for BroadcastSm {
        fn instances() -> alloc::vec::Vec<Self> {
            alloc::vec![
                Self::default(),
                Self::builder()
                    .service_type(ServiceType::new(
                        GenericServiceType::CellularMessaging.into(),
                    ))
                    .source_addr_ton(Ton::International)
                    .source_addr_npi(Npi::Isdn)
                    .source_addr(COctetString::from_str("SourceAddr").unwrap())
                    .message_id(COctetString::from_str("MessageId").unwrap())
                    .priority_flag(PriorityFlag::from(PriorityFlagType::from(GsmSms::from(1))))
                    .schedule_delivery_time(EmptyOrFullCOctetString::empty())
                    .validity_period(EmptyOrFullCOctetString::empty())
                    .replace_if_present_flag(ReplaceIfPresentFlag::Replace)
                    .data_coding(DataCoding::LatinHebrew)
                    .sm_default_msg_id(0)
                    .build(),
                Self::builder()
                    .service_type(ServiceType::new(
                        GenericServiceType::UnstructuredSupplementaryServicesData.into(),
                    ))
                    .source_addr_ton(Ton::International)
                    .source_addr_npi(Npi::Isdn)
                    .source_addr(COctetString::from_str("SourceAddr").unwrap())
                    .message_id(COctetString::from_str("MessageId").unwrap())
                    .priority_flag(PriorityFlag::from(PriorityFlagType::from(Ansi136::Bulk)))
                    .schedule_delivery_time(
                        EmptyOrFullCOctetString::from_static_slice(b"2023-10-01T00:00\0").unwrap(),
                    )
                    .validity_period(EmptyOrFullCOctetString::empty())
                    .replace_if_present_flag(ReplaceIfPresentFlag::DoNotReplace)
                    .data_coding(DataCoding::GsmMessageClassControl)
                    .sm_default_msg_id(255)
                    .tlvs(alloc::vec![
                        BroadcastRequestTlvValue::CallbackNum(
                            OctetString::from_str("1234567890").unwrap(),
                        ),
                        BroadcastRequestTlvValue::LanguageIndicator(LanguageIndicator::German),
                        BroadcastRequestTlvValue::SmsSignal(1024),
                    ])
                    .build(),
            ]
        }
    }

    #[test]
    fn encode_decode() {
        crate::tests::owned::encode_decode_with_length_test_instances::<BroadcastSm>();
    }
}
