use rusmpp_macros::Rusmpp;

use crate::{
    encode::Length,
    pdus::owned::Pdu,
    tlvs::owned::{MessageSubmissionRequestTlvValue, Tlv},
    types::owned::{COctetString, EmptyOrFullCOctetString, OctetString},
    values::{owned::*, *},
};

#[derive(Default, Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Rusmpp)]
#[rusmpp(decode = owned, test = skip)]
#[cfg_attr(feature = "arbitrary", derive(::arbitrary::Arbitrary))]
#[cfg_attr(feature = "serde", derive(::serde::Serialize))]
#[cfg_attr(feature = "serde-deserialize-unchecked", derive(::serde::Deserialize))]
pub struct SubmitMulti {
    /// The service_type parameter can be used to indicate the
    /// SMS Application service associated with the message.
    /// Specifying the service_type allows the ESME to avail of
    /// enhanced messaging services such as “replace by
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
    /// Number of destination addresses – indicates the
    /// number of destinations that are to follow.
    ///
    /// A maximum of 255 destination addresses are allowed.
    ///
    /// Note: Set to 1 when submitting to one SME Address or when
    /// submitting to one Distribution List.
    number_of_dests: u8,
    /// Composite field.
    #[rusmpp(count = number_of_dests)]
    dest_address: alloc::vec::Vec<DestAddress>,
    /// Indicates Message Mode and Message Type.
    pub esm_class: EsmClass,
    /// Protocol Identifier.
    ///
    /// Network specific field.
    pub protocol_id: u8,
    /// Designates the priority level of the message.
    pub priority_flag: PriorityFlag,
    /// The short message is to be scheduled by the MC for delivery.
    ///
    /// Set to NULL for immediate message delivery.
    pub schedule_delivery_time: EmptyOrFullCOctetString<17>,
    /// The validity period of this message.
    ///
    /// Set to NULL to request the SMSC default validity period.
    ///
    /// Note: this is superseded by the qos_time_to_live TLV if specified.
    pub validity_period: EmptyOrFullCOctetString<17>,
    /// Indicator to signify if a MC delivery receipt or an SME
    /// acknowledgement is required.
    pub registered_delivery: RegisteredDelivery,
    /// Flag indicating if submitted message should replace an
    /// existing message.
    pub replace_if_present_flag: ReplaceIfPresentFlag,
    /// Defines the encoding scheme of the short message user data.
    pub data_coding: DataCoding,
    /// Indicates the short message to send from a list of pre- defined
    /// (‘canned’) short messages stored on the MC.
    ///
    /// If not using a MC canned message, set to NULL.
    pub sm_default_msg_id: u8,
    /// Length in octets of the short_message user data.
    sm_length: u8,
    /// Up to 255 octets of short message user data.
    ///
    /// The exact physical limit for short_message size may
    /// vary according to the underlying network
    ///
    /// Note: this field is superceded by the message_payload TLV if
    /// specified.
    ///
    /// Applications which need to send messages longer than
    /// 255 octets should use the message_payload TLV. In
    /// this case the sm_length field should be set to zero.
    #[rusmpp(length = sm_length)]
    short_message: OctetString<0, 255>,
    /// Message submission request TLVs ([`MessageSubmissionRequestTlvValue`]).
    #[rusmpp(length = "unchecked")]
    tlvs: alloc::vec::Vec<Tlv>,
}

impl SubmitMulti {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        service_type: ServiceType,
        source_addr_ton: Ton,
        source_addr_npi: Npi,
        source_addr: COctetString<1, 21>,
        dest_address: alloc::vec::Vec<DestAddress>,
        esm_class: EsmClass,
        protocol_id: u8,
        priority_flag: PriorityFlag,
        schedule_delivery_time: EmptyOrFullCOctetString<17>,
        validity_period: EmptyOrFullCOctetString<17>,
        registered_delivery: RegisteredDelivery,
        replace_if_present_flag: ReplaceIfPresentFlag,
        data_coding: DataCoding,
        sm_default_msg_id: u8,
        short_message: OctetString<0, 255>,
        tlvs: alloc::vec::Vec<MessageSubmissionRequestTlvValue>,
    ) -> Self {
        let sm_length = short_message.length() as u8;
        let number_of_dests = dest_address.len() as u8;

        let tlvs = tlvs.into_iter().map(From::from).collect();

        Self {
            service_type,
            source_addr_ton,
            source_addr_npi,
            source_addr,
            number_of_dests,
            dest_address,
            esm_class,
            protocol_id,
            priority_flag,
            schedule_delivery_time,
            validity_period,
            registered_delivery,
            replace_if_present_flag,
            data_coding,
            sm_default_msg_id,
            sm_length,
            short_message,
            tlvs,
        }
    }

    pub const fn number_of_dests(&self) -> u8 {
        self.number_of_dests
    }

    pub fn dest_address(&self) -> &[DestAddress] {
        &self.dest_address
    }

    pub fn set_dest_address(&mut self, dest_address: alloc::vec::Vec<DestAddress>) {
        self.dest_address = dest_address;
        self.number_of_dests = self.dest_address.len() as u8;
    }

    pub fn push_dest_address(&mut self, dest_address: DestAddress) {
        self.dest_address.push(dest_address);
        self.number_of_dests = self.dest_address.len() as u8;
    }

    pub fn clear_dest_address(&mut self) {
        self.dest_address.clear();
        self.number_of_dests = self.dest_address.len() as u8;
    }

    pub const fn sm_length(&self) -> u8 {
        self.sm_length
    }

    pub const fn short_message(&self) -> &OctetString<0, 255> {
        &self.short_message
    }

    /// Sets the `short_message` and `sm_length`.
    ///
    /// # Note
    ///
    /// `short_message` is superceded by [`TlvValue::MessagePayload`](crate::tlvs::owned::TlvValue::MessagePayload) and should only be used if
    /// [`TlvValue::MessagePayload`](crate::tlvs::owned::TlvValue::MessagePayload) is not present.
    pub fn set_short_message(&mut self, short_message: OctetString<0, 255>) {
        self.short_message = short_message;
        self.sm_length = self.short_message.length() as u8;
    }

    pub fn tlvs(&self) -> &[Tlv] {
        &self.tlvs
    }

    pub fn set_tlvs(&mut self, tlvs: alloc::vec::Vec<MessageSubmissionRequestTlvValue>) {
        self.tlvs = tlvs.into_iter().map(From::from).collect();
    }

    pub fn clear_tlvs(&mut self) {
        self.tlvs.clear();
    }

    pub fn push_tlv(&mut self, tlv: impl Into<MessageSubmissionRequestTlvValue>) {
        self.tlvs.push(Tlv::from(tlv.into()));
    }

    pub fn builder() -> SubmitMultiBuilder {
        SubmitMultiBuilder::new()
    }
}

impl From<SubmitMulti> for Pdu {
    fn from(value: SubmitMulti) -> Self {
        Self::SubmitMulti(value)
    }
}

#[derive(Debug, Default)]
pub struct SubmitMultiBuilder {
    inner: SubmitMulti,
}

impl SubmitMultiBuilder {
    pub fn new() -> Self {
        Default::default()
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

    pub fn dest_address(mut self, dest_address: alloc::vec::Vec<DestAddress>) -> Self {
        self.inner.set_dest_address(dest_address);
        self
    }

    pub fn push_dest_address(mut self, dest_address: DestAddress) -> Self {
        self.inner.push_dest_address(dest_address);
        self
    }

    pub fn clear_dest_address(mut self) -> Self {
        self.inner.clear_dest_address();
        self
    }

    pub fn esm_class(mut self, esm_class: EsmClass) -> Self {
        self.inner.esm_class = esm_class;
        self
    }

    pub fn protocol_id(mut self, protocol_id: u8) -> Self {
        self.inner.protocol_id = protocol_id;
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

    pub fn registered_delivery(mut self, registered_delivery: RegisteredDelivery) -> Self {
        self.inner.registered_delivery = registered_delivery;
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

    pub fn short_message(mut self, short_message: OctetString<0, 255>) -> Self {
        self.inner.set_short_message(short_message);
        self
    }

    pub fn tlvs(mut self, tlvs: alloc::vec::Vec<MessageSubmissionRequestTlvValue>) -> Self {
        self.inner.set_tlvs(tlvs);
        self
    }

    pub fn clear_tlvs(mut self) -> Self {
        self.inner.clear_tlvs();
        self
    }

    pub fn push_tlv(mut self, tlv: impl Into<MessageSubmissionRequestTlvValue>) -> Self {
        self.inner.push_tlv(tlv);
        self
    }

    pub fn build(self) -> SubmitMulti {
        self.inner
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use crate::{tests::TestInstance, types::owned::AnyOctetString};

    use super::*;

    impl TestInstance for SubmitMulti {
        fn instances() -> alloc::vec::Vec<Self> {
            alloc::vec![
                Self::default(),
                Self::builder()
                    .service_type(ServiceType::default())
                    .source_addr_ton(Ton::International)
                    .source_addr_npi(Npi::Isdn)
                    .source_addr(COctetString::from_str("Source Address").unwrap())
                    .esm_class(EsmClass::default())
                    .protocol_id(0)
                    .priority_flag(PriorityFlag::default())
                    .schedule_delivery_time(EmptyOrFullCOctetString::empty())
                    .validity_period(EmptyOrFullCOctetString::empty())
                    .registered_delivery(RegisteredDelivery::default())
                    .replace_if_present_flag(ReplaceIfPresentFlag::default())
                    .data_coding(DataCoding::default())
                    .sm_default_msg_id(0)
                    .short_message(OctetString::from_static_slice(b"Short Message").unwrap())
                    .build(),
                Self::builder()
                    .short_message(OctetString::from_static_slice(b"Short Message").unwrap())
                    .push_tlv(MessageSubmissionRequestTlvValue::MessagePayload(
                        MessagePayload::new(AnyOctetString::from_static_slice(b"Message Payload")),
                    ))
                    .build(),
                Self::builder()
                    .push_dest_address(DestAddress::new(SmeAddress::new(
                        Ton::International,
                        Npi::Isdn,
                        COctetString::from_static_slice(b"1234567890123456789\0").unwrap(),
                    )))
                    .push_dest_address(DestAddress::new(DistributionListName::new(
                        COctetString::from_static_slice(b"1234567890123456789\0").unwrap(),
                    ),))
                    .short_message(OctetString::from_static_slice(b"Short Message").unwrap())
                    .push_tlv(MessageSubmissionRequestTlvValue::MessagePayload(
                        MessagePayload::new(AnyOctetString::from_static_slice(b"Message Payload")),
                    ))
                    .push_tlv(MessageSubmissionRequestTlvValue::DestTelematicsId(16))
                    .build(),
            ]
        }
    }

    #[test]
    fn encode_decode() {
        crate::tests::owned::encode_decode_with_length_test_instances::<SubmitMulti>();
    }

    #[test]
    fn short_message_length() {
        let short_message = OctetString::from_static_slice(b"Short Message").unwrap();

        let submit_sm = SubmitMulti::builder()
            .short_message(short_message.clone())
            .build();

        assert_eq!(submit_sm.short_message(), &short_message);
        assert_eq!(submit_sm.sm_length(), short_message.length() as u8);
    }

    #[test]
    fn count() {
        let submit_multi = SubmitMulti::default();

        assert_eq!(submit_multi.number_of_dests(), 0);
        assert!(submit_multi.dest_address().is_empty());

        let submit_sm = SubmitMulti::builder()
            .dest_address(alloc::vec![
                DestAddress::new(SmeAddress::new(
                    Ton::International,
                    Npi::Isdn,
                    COctetString::from_static_slice(b"1234567890123456789\0").unwrap(),
                )),
                DestAddress::new(DistributionListName::new(
                    COctetString::from_static_slice(b"1234567890123456789\0").unwrap(),
                )),
            ])
            .build();

        assert_eq!(submit_sm.number_of_dests(), 2);
        assert_eq!(submit_sm.dest_address().len(), 2);

        let submit_sm = SubmitMulti::builder()
            .push_dest_address(DestAddress::new(SmeAddress::new(
                Ton::International,
                Npi::Isdn,
                COctetString::from_static_slice(b"1234567890123456789\0").unwrap(),
            )))
            .push_dest_address(DestAddress::new(DistributionListName::new(
                COctetString::from_static_slice(b"1234567890123456789\0").unwrap(),
            )))
            .build();

        assert_eq!(submit_sm.number_of_dests(), 2);
        assert_eq!(submit_sm.dest_address().len(), 2);

        let submit_sm = SubmitMulti::builder()
            .push_dest_address(DestAddress::new(SmeAddress::new(
                Ton::International,
                Npi::Isdn,
                COctetString::from_static_slice(b"1234567890123456789\0").unwrap(),
            )))
            .push_dest_address(DestAddress::new(DistributionListName::new(
                COctetString::from_static_slice(b"1234567890123456789\0").unwrap(),
            )))
            .clear_dest_address()
            .build();

        assert_eq!(submit_sm.number_of_dests(), 0);
        assert!(submit_sm.dest_address().is_empty());
    }
}
