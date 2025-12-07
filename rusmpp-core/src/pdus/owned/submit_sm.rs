use rusmpp_macros::Rusmpp;

use crate::{
    encode::Length,
    pdus::owned::Pdu,
    tlvs::{
        TlvTag,
        owned::{MessageSubmissionRequestTlvValue, Tlv},
    },
    types::owned::{COctetString, EmptyOrFullCOctetString, OctetString},
    values::{owned::*, *},
};

/// This operation is used by an ESME to submit a short message to the MC for onward
/// transmission to a specified short message entity (SME).
#[derive(Default, Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Rusmpp)]
#[rusmpp(decode = owned, test = skip)]
#[cfg_attr(feature = "arbitrary", derive(::arbitrary::Arbitrary))]
#[cfg_attr(feature = "serde", derive(::serde::Serialize))]
#[cfg_attr(feature = "serde-deserialize-unchecked", derive(::serde::Deserialize))]
pub struct SubmitSm {
    /// The service_type parameter can be used to
    /// indicate the SMS Application service
    /// associated with the message. Specifying the
    /// service_type allows the ESME to avail of enhanced
    /// messaging services such as “replace by service_type”
    /// or to control the teleservice used on the
    /// air interface.
    ///
    /// Set to NULL for default MC settings.
    pub service_type: ServiceType,
    /// Type of Number for source address.
    pub source_addr_ton: Ton,
    /// Numbering Plan Indicator for source address.
    pub source_addr_npi: Npi,
    /// Address of SME which originated this message.
    pub source_addr: COctetString<1, 21>,
    /// Type of Number for destination.
    pub dest_addr_ton: Ton,
    /// Numbering Plan Indicator for destination.
    pub dest_addr_npi: Npi,
    /// Destination address of this
    /// short message For mobile
    /// terminated messages, this
    /// is the directory number of
    /// the recipient MS
    pub destination_addr: COctetString<1, 21>,
    /// Indicates Message Mode
    /// and Message Type.
    pub esm_class: EsmClass,
    /// Protocol Identifier.
    /// Network specific field.
    pub protocol_id: u8,
    /// Designates the priority level of the message.
    pub priority_flag: PriorityFlag,
    /// The short message is to be
    /// scheduled by the MC for delivery.
    /// Set to NULL for immediate message delivery.
    pub schedule_delivery_time: EmptyOrFullCOctetString<17>,
    /// The validity period of this message.
    /// Set to NULL to request the MC default validity period.
    ///
    /// Note: this is superseded by the qos_time_to_live TLV if
    /// specified.
    pub validity_period: EmptyOrFullCOctetString<17>,
    /// Indicator to signify if a MC delivery receipt, manual
    /// ACK, delivery ACK or an intermediate notification is required.
    pub registered_delivery: RegisteredDelivery,
    /// Flag indicating if the submitted message should replace an existing message.
    pub replace_if_present_flag: ReplaceIfPresentFlag,
    /// Defines the encoding scheme of the short message user data.
    pub data_coding: DataCoding,
    /// Indicates the short message to send from a list of pre- defined (‘canned’)
    /// short messages stored on the MC. If not using a MC canned message, set to NULL.
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
    #[rusmpp(length = sm_length)]
    short_message: OctetString<0, 255>,
    /// Message submission request TLVs ([`MessageSubmissionRequestTlvValue`]).
    #[rusmpp(length = "unchecked")]
    tlvs: alloc::vec::Vec<Tlv>,
}

impl SubmitSm {
    /// The default maximum size of the short message in bytes (octets).
    const DEFAULT_MAX_SHORT_MESSAGE_SIZE: usize = 140;

    /// Returns the default maximum size of the short message in bytes (octets).
    ///
    /// # Note
    ///
    /// Depending on the [`DataCoding`] used, the amount of characters* `(not bytes)`
    /// that can fit in a short message may vary.
    ///
    /// * `GSM 7-bit` encoding allows for up to `160` characters in `140` bytes.
    ///   `GSM 7-bit` encoding performs character packing to fit more characters in less bytes.
    ///   The formula to calculate the maximum number of characters is: `(bytes * 8) / 7`.
    ///   Therefore the maximum number of characters is: `(140 * 8) / 7 = 160`.
    /// * `GSM 7-bit unpacked` encoding allows for up to `140` characters in `140` bytes.
    ///   `GSM 7-bit unpacked` does not perform character packing, so each character takes up to `2` bytes.
    ///   In the worst case, each character may require an escape character `(0x1B)` followed by the actual character byte,
    ///   ending up fitting only `70` characters in `140` bytes.
    /// * `UCS2` encoding allows for up to `70` characters in `140` bytes.
    /// * `ISO-8859-1` encoding allows for up to `140` characters in `140` bytes.
    ///
    /// `*` A character is a single textual unit, which may be represented by one or more bytes depending on the encoding scheme.
    pub const fn default_max_short_message_size() -> usize {
        Self::DEFAULT_MAX_SHORT_MESSAGE_SIZE
    }

    #[allow(clippy::too_many_arguments)]
    pub fn new(
        service_type: ServiceType,
        source_addr_ton: Ton,
        source_addr_npi: Npi,
        source_addr: COctetString<1, 21>,
        dest_addr_ton: Ton,
        dest_addr_npi: Npi,
        destination_addr: COctetString<1, 21>,
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
        tlvs: alloc::vec::Vec<impl Into<MessageSubmissionRequestTlvValue>>,
    ) -> Self {
        let tlvs = tlvs.into_iter().map(Into::into).map(From::from).collect();

        let sm_length = short_message.length() as u8;

        let mut submit_sm = Self {
            service_type,
            source_addr_ton,
            source_addr_npi,
            source_addr,
            dest_addr_ton,
            dest_addr_npi,
            destination_addr,
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
        };

        submit_sm.clear_short_message_if_message_payload_exists();

        submit_sm
    }

    pub fn sm_length(&self) -> u8 {
        self.sm_length
    }

    pub fn short_message(&self) -> &OctetString<0, 255> {
        &self.short_message
    }

    /// Sets the short message and short message length.
    /// Updates the short message and short message length accordingly.
    /// Has no effect if the message payload is set.
    /// Returns true if the short message and short message length were set.
    pub fn set_short_message(&mut self, short_message: OctetString<0, 255>) -> bool {
        self.short_message = short_message;
        self.sm_length = self.short_message.length() as u8;

        !self.clear_short_message_if_message_payload_exists()
    }

    pub fn tlvs(&self) -> &[Tlv] {
        &self.tlvs
    }

    pub fn set_tlvs(&mut self, tlvs: alloc::vec::Vec<impl Into<MessageSubmissionRequestTlvValue>>) {
        self.tlvs = tlvs.into_iter().map(Into::into).map(From::from).collect();

        self.clear_short_message_if_message_payload_exists();
    }

    pub fn clear_tlvs(&mut self) {
        self.tlvs.clear();
    }

    pub fn push_tlv(&mut self, tlv: impl Into<MessageSubmissionRequestTlvValue>) {
        self.tlvs.push(Tlv::from(tlv.into()));

        self.clear_short_message_if_message_payload_exists();
    }

    /// Clears the short message and short message length if the message payload is set.
    /// Returns true if the short message and short message length were cleared.
    fn clear_short_message_if_message_payload_exists(&mut self) -> bool {
        let message_payload_exists = self
            .tlvs
            .iter()
            .any(|value| matches!(value.tag(), TlvTag::MessagePayload));

        if message_payload_exists {
            self.short_message = OctetString::empty();
            self.sm_length = 0;

            return true;
        };

        false
    }

    pub fn builder() -> SubmitSmBuilder {
        SubmitSmBuilder::new()
    }

    /// Sets the [`SubmitSm::data_coding`].
    pub fn with_data_coding(mut self, data_coding: DataCoding) -> Self {
        self.data_coding = data_coding;
        self
    }

    /// Sets the UDH Indicator bit in the GSM Features field of the [`SubmitSm::esm_class`].
    pub fn with_udhi_indicator(mut self) -> Self {
        self.esm_class = self.esm_class.with_udhi_indicator();
        self
    }

    /// Sets the [`SubmitSm::short_message`] and [`SubmitSm::sm_length`](SubmitSm::sm_length).
    ///
    /// See [`SubmitSm::set_short_message`] for details.
    pub fn with_short_message(mut self, short_message: OctetString<0, 255>) -> Self {
        self.set_short_message(short_message);
        self
    }
}

impl From<SubmitSm> for Pdu {
    fn from(value: SubmitSm) -> Self {
        Self::SubmitSm(value)
    }
}

#[derive(Debug, Default, Clone)]
pub struct SubmitSmBuilder {
    inner: SubmitSm,
}

impl SubmitSmBuilder {
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

    pub fn dest_addr_ton(mut self, dest_addr_ton: Ton) -> Self {
        self.inner.dest_addr_ton = dest_addr_ton;
        self
    }

    pub fn dest_addr_npi(mut self, dest_addr_npi: Npi) -> Self {
        self.inner.dest_addr_npi = dest_addr_npi;
        self
    }

    pub fn destination_addr(mut self, destination_addr: COctetString<1, 21>) -> Self {
        self.inner.destination_addr = destination_addr;
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

    pub fn tlvs(
        mut self,
        tlvs: alloc::vec::Vec<impl Into<MessageSubmissionRequestTlvValue>>,
    ) -> Self {
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

    pub fn build(self) -> SubmitSm {
        self.inner
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use crate::{tests::TestInstance, types::owned::AnyOctetString};

    use super::*;

    impl TestInstance for SubmitSm {
        fn instances() -> alloc::vec::Vec<Self> {
            alloc::vec![
                Self::default(),
                Self::builder()
                    .service_type(ServiceType::new(
                        GenericServiceType::CellularMessaging.into(),
                    ))
                    .source_addr_ton(Ton::International)
                    .source_addr_npi(Npi::Isdn)
                    .source_addr(COctetString::from_str("Source Address").unwrap())
                    .dest_addr_ton(Ton::International)
                    .dest_addr_npi(Npi::Isdn)
                    .destination_addr(COctetString::from_str("Destination Address").unwrap())
                    .esm_class(EsmClass::new(
                        MessagingMode::StoreAndForward,
                        MessageType::ShortMessageContainsMCDeliveryReceipt,
                        Ansi41Specific::ShortMessageContainsDeliveryAcknowledgement,
                        GsmFeatures::SetUdhiAndReplyPath,
                    ))
                    .protocol_id(0)
                    .priority_flag(PriorityFlag::from(PriorityFlagType::from(Ansi136::Bulk)))
                    .schedule_delivery_time(
                        EmptyOrFullCOctetString::new(b"2023-09-01T12:00\0".to_vec()).unwrap(),
                    )
                    .validity_period(EmptyOrFullCOctetString::from_str("2023-10-01T12:00").unwrap())
                    .registered_delivery(RegisteredDelivery::request_all())
                    .replace_if_present_flag(ReplaceIfPresentFlag::Replace)
                    .data_coding(DataCoding::Ksc5601)
                    .sm_default_msg_id(69)
                    .short_message(OctetString::new(b"Short Message".to_vec()).unwrap())
                    .build(),
                Self::builder()
                    .service_type(ServiceType::new(
                        GenericServiceType::CellularMessaging.into(),
                    ))
                    .source_addr_ton(Ton::International)
                    .source_addr_npi(Npi::Isdn)
                    .source_addr(COctetString::new(b"Source Address\0".to_vec()).unwrap())
                    .dest_addr_ton(Ton::International)
                    .dest_addr_npi(Npi::Isdn)
                    .destination_addr(COctetString::new(b"Destination Address\0".to_vec()).unwrap())
                    .esm_class(EsmClass::new(
                        MessagingMode::Default,
                        MessageType::ShortMessageContainsIntermediateDeliveryNotification,
                        Ansi41Specific::ShortMessageContainsUserAcknowledgment,
                        GsmFeatures::SetUdhiAndReplyPath,
                    ))
                    .protocol_id(0)
                    .priority_flag(PriorityFlag::from(PriorityFlagType::from(
                        Ansi136::VeryUrgent,
                    )))
                    .schedule_delivery_time(
                        EmptyOrFullCOctetString::new(b"2023-09-01T12:01\0".to_vec()).unwrap(),
                    )
                    .validity_period(EmptyOrFullCOctetString::from_str("2023-10-01T12:20").unwrap())
                    .registered_delivery(RegisteredDelivery::request_all())
                    .replace_if_present_flag(ReplaceIfPresentFlag::DoNotReplace)
                    .data_coding(DataCoding::Jis)
                    .sm_default_msg_id(96)
                    .short_message(OctetString::new(b"Short Message".to_vec()).unwrap())
                    .tlvs(alloc::vec![
                        MessageSubmissionRequestTlvValue::MessagePayload(MessagePayload::new(
                            AnyOctetString::new(b"Message Payload".to_vec())
                        ),)
                    ])
                    .build(),
                Self::builder()
                    .short_message(OctetString::new(b"Short Message".to_vec()).unwrap())
                    .tlvs(alloc::vec![
                        MessageSubmissionRequestTlvValue::MessagePayload(MessagePayload::new(
                            AnyOctetString::new(b"Message Payload".to_vec()),
                        )),
                        MessageSubmissionRequestTlvValue::UserResponseCode(3),
                        MessageSubmissionRequestTlvValue::DestBearerType(BearerType::FlexReFlex),
                        MessageSubmissionRequestTlvValue::SourceSubaddress(Subaddress::new(
                            SubaddressTag::NsapOdd,
                            OctetString::from_str("Subaddress :D").unwrap(),
                        )),
                    ])
                    .build(),
            ]
        }
    }

    #[test]
    fn encode_decode() {
        crate::tests::owned::encode_decode_with_length_test_instances::<SubmitSm>();
    }

    #[test]
    fn short_message_length() {
        let short_message = OctetString::new(b"Short Message".to_vec()).unwrap();

        let submit_sm = SubmitSm::builder()
            .short_message(short_message.clone())
            .build();

        assert_eq!(submit_sm.short_message(), &short_message);
        assert_eq!(submit_sm.sm_length(), short_message.length() as u8);
    }

    #[test]
    fn short_message_override() {
        let short_message_1 = OctetString::new(b"Short Message 101".to_vec()).unwrap();
        let short_message_2 = OctetString::new(b"Short Message 2".to_vec()).unwrap();

        let submit_sm = SubmitSm::builder()
            .short_message(short_message_1)
            .short_message(short_message_2.clone())
            .build();

        assert_eq!(submit_sm.short_message(), &short_message_2);
        assert_eq!(submit_sm.sm_length(), short_message_2.length() as u8);
    }

    #[test]
    fn message_payload_suppresses_short_message() {
        let short_message = OctetString::new(b"Short Message".to_vec()).unwrap();
        let message_payload = MessagePayload::new(AnyOctetString::new(b"Message Payload".to_vec()));

        // Using push_tlv
        let submit_sm = SubmitSm::builder()
            .short_message(short_message.clone())
            .push_tlv(MessageSubmissionRequestTlvValue::MessagePayload(
                message_payload.clone(),
            ))
            .build();

        assert_eq!(submit_sm.short_message(), &OctetString::empty());
        assert_eq!(submit_sm.sm_length(), 0);

        // Using tlvs
        let submit_sm = SubmitSm::builder()
            .short_message(short_message.clone())
            .tlvs(alloc::vec![
                MessageSubmissionRequestTlvValue::MessagePayload(message_payload.clone(),)
            ])
            .build();

        assert_eq!(submit_sm.short_message(), &OctetString::empty());
        assert_eq!(submit_sm.sm_length(), 0);

        // Even setting the short message after the message payload should not set the short message
        // Using push_tlv
        let submit_sm = SubmitSm::builder()
            .short_message(short_message.clone())
            .push_tlv(MessageSubmissionRequestTlvValue::MessagePayload(
                message_payload.clone(),
            ))
            .short_message(short_message.clone())
            .build();

        assert_eq!(submit_sm.short_message(), &OctetString::empty());
        assert_eq!(submit_sm.sm_length(), 0);

        // Using tlvs
        let submit_sm = SubmitSm::builder()
            .short_message(short_message.clone())
            .tlvs(alloc::vec![
                MessageSubmissionRequestTlvValue::MessagePayload(message_payload.clone(),)
            ])
            .short_message(short_message.clone())
            .build();

        assert_eq!(submit_sm.short_message(), &OctetString::empty());
        assert_eq!(submit_sm.sm_length(), 0);

        // Removing the message payload and then setting the short message should set the short message
        let submit_sm = SubmitSm::builder()
            .push_tlv(MessageSubmissionRequestTlvValue::MessagePayload(
                message_payload.clone(),
            ))
            .clear_tlvs()
            .short_message(short_message.clone())
            .build();

        assert_eq!(submit_sm.short_message(), &short_message);
        assert_eq!(submit_sm.sm_length(), short_message.length() as u8);
    }
}
