use rusmpp_macros::Rusmpp;

use crate::{
    encode::Length,
    pdus::owned::Pdu,
    tlvs::owned::{Tlv, TlvValue},
    types::owned::{COctetString, EmptyOrFullCOctetString, OctetString},
    values::{owned::*, *},
};

/// This command is issued by the ESME to replace a previously submitted short message that
/// is pending delivery. The matching mechanism is based on the message_id and source
/// address of the original message.
///
/// Where the original submit_sm ‘source address’ was defaulted to NULL, then the source
/// address in the replace_sm command should also be NULL.
#[derive(Default, Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Rusmpp)]
#[rusmpp(decode = owned, test = skip)]
#[cfg_attr(feature = "arbitrary", derive(::arbitrary::Arbitrary))]
#[cfg_attr(feature = "serde", derive(::serde::Serialize))]
#[cfg_attr(feature = "serde-deserialize-unchecked", derive(::serde::Deserialize))]
pub struct ReplaceSm {
    /// Message ID of the message to be replaced.
    /// This must be the MC assigned Message ID
    /// allocated to the original short message when
    /// submitted to the MC by the submit_sm, data_sm or
    /// submit_multi command, and returned in the
    /// response PDU by the MC.
    pub message_id: COctetString<1, 65>,
    /// Type of Number of message originator. This is used for
    /// verification purposes, and must match that supplied in
    /// the original request PDU (e.g. submit_sm).
    ///
    /// If not known, set to NULL.
    pub source_addr_ton: Ton,
    /// Numbering Plan Indicator for source address of
    /// original message.
    ///
    /// If not known, set to NULL (Unknown).
    pub source_addr_npi: Npi,
    /// Address of SME, which originated this message.
    /// If not known, set to NULL (Unknown).
    pub source_addr: COctetString<1, 21>,
    /// New scheduled delivery time for the short message.
    /// Set to NULL to preserve the original scheduled
    /// delivery time.
    pub schedule_delivery_time: EmptyOrFullCOctetString<17>,
    /// New expiry time for the short message.
    ///
    /// Set to NULL to preserve
    /// the original validity period
    /// setting.
    pub validity_period: EmptyOrFullCOctetString<17>,
    /// Indicator to signify if a MC delivery receipt,
    /// user/manual or delivery ACK or intermediate
    /// notification is required.
    pub registered_delivery: RegisteredDelivery,
    /// Indicates the short message to send from a list
    /// of predefined (‘canned’) short messages stored on
    /// the MC.
    ///
    /// If not using a MC canned message, set to NULL.
    pub sm_default_msg_id: u8,
    /// Length in octets of the short_message user data.
    sm_length: u8,
    /// Up to 255 octets of short message user data.
    /// The exact physical limit for short_message size may
    /// vary according to the underlying network
    ///
    /// Note: this field is superceded by the message_payload TLV if specified.
    ///
    /// Applications which need to send messages longer than
    /// 255 octets should use the message_payload TLV. In
    /// this case the sm_length field should be set to zero.
    #[rusmpp(length = sm_length)]
    short_message: OctetString<0, 255>,
    /// Message replacement request TLVs. [`MessagePayload`].
    #[rusmpp(length = "checked")]
    message_payload: Option<Tlv>,
}

impl ReplaceSm {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        message_id: COctetString<1, 65>,
        source_addr_ton: Ton,
        source_addr_npi: Npi,
        source_addr: COctetString<1, 21>,
        schedule_delivery_time: EmptyOrFullCOctetString<17>,
        validity_period: EmptyOrFullCOctetString<17>,
        registered_delivery: RegisteredDelivery,
        sm_default_msg_id: u8,
        short_message: OctetString<0, 255>,
        message_payload: Option<MessagePayload>,
    ) -> Self {
        let message_payload = message_payload
            .map(TlvValue::MessagePayload)
            .map(From::from);

        let sm_length = short_message.length() as u8;

        let mut replace_sm = Self {
            message_id,
            source_addr_ton,
            source_addr_npi,
            source_addr,
            schedule_delivery_time,
            validity_period,
            registered_delivery,
            sm_default_msg_id,
            sm_length,
            short_message,
            message_payload,
        };

        replace_sm.clear_short_message_if_message_payload_exists();

        replace_sm
    }

    pub const fn sm_length(&self) -> u8 {
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
        self.sm_length = short_message.length() as u8;
        self.short_message = short_message;

        !self.clear_short_message_if_message_payload_exists()
    }

    pub const fn message_payload_tlv(&self) -> Option<&Tlv> {
        self.message_payload.as_ref()
    }

    pub fn message_payload(&self) -> Option<&MessagePayload> {
        self.message_payload_tlv()
            .and_then(|tlv| match tlv.value() {
                Some(TlvValue::MessagePayload(value)) => Some(value),
                _ => None,
            })
    }

    /// Sets the message payload.
    /// Updates the short message and short message length accordingly.
    pub fn set_message_payload(&mut self, message_payload: Option<MessagePayload>) {
        self.message_payload = message_payload
            .map(TlvValue::MessagePayload)
            .map(From::from);

        self.clear_short_message_if_message_payload_exists();
    }

    /// Clears the short message and short message length if the message payload is set.
    /// Returns true if the short message and short message length were cleared.
    fn clear_short_message_if_message_payload_exists(&mut self) -> bool {
        if self.message_payload.is_some() {
            self.short_message = OctetString::empty();
            self.sm_length = 0;

            return true;
        };

        false
    }

    pub fn builder() -> ReplaceSmBuilder {
        ReplaceSmBuilder::new()
    }
}

impl From<ReplaceSm> for Pdu {
    fn from(value: ReplaceSm) -> Self {
        Self::ReplaceSm(value)
    }
}

#[derive(Debug, Default)]
pub struct ReplaceSmBuilder {
    inner: ReplaceSm,
}

impl ReplaceSmBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn message_id(mut self, message_id: COctetString<1, 65>) -> Self {
        self.inner.message_id = message_id;
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

    pub fn sm_default_msg_id(mut self, sm_default_msg_id: u8) -> Self {
        self.inner.sm_default_msg_id = sm_default_msg_id;
        self
    }

    pub fn short_message(mut self, short_message: OctetString<0, 255>) -> Self {
        self.inner.set_short_message(short_message);
        self
    }

    pub fn message_payload(mut self, message_payload: Option<MessagePayload>) -> Self {
        self.inner.set_message_payload(message_payload);
        self
    }

    pub fn build(self) -> ReplaceSm {
        self.inner
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use crate::{tests::TestInstance, types::owned::AnyOctetString};

    use super::*;

    impl TestInstance for ReplaceSm {
        fn instances() -> alloc::vec::Vec<Self> {
            alloc::vec![
                Self::default(),
                Self::builder()
                    .message_id(COctetString::from_str("123456789012345678901234").unwrap())
                    .source_addr_ton(Ton::International)
                    .source_addr_npi(Npi::Isdn)
                    .source_addr(COctetString::from_str("Source Addr").unwrap())
                    .schedule_delivery_time(
                        EmptyOrFullCOctetString::new(b"2023-10-01T12:00\0".to_vec()).unwrap(),
                    )
                    .validity_period(
                        EmptyOrFullCOctetString::new(b"2023-10-01T12:00\0".to_vec()).unwrap()
                    )
                    .registered_delivery(RegisteredDelivery::default())
                    .sm_default_msg_id(0)
                    .short_message(OctetString::new(b"Short Message".to_vec()).unwrap())
                    .build(),
                Self::builder()
                    .message_id(COctetString::from_str("123456789012345678901234").unwrap())
                    .source_addr_ton(Ton::International)
                    .source_addr_npi(Npi::Isdn)
                    .source_addr(COctetString::from_str("Source Addr").unwrap())
                    .schedule_delivery_time(
                        EmptyOrFullCOctetString::new(b"2023-10-01T12:00\0".to_vec()).unwrap(),
                    )
                    .validity_period(
                        EmptyOrFullCOctetString::new(b"2023-10-01T12:00\0".to_vec()).unwrap()
                    )
                    .registered_delivery(RegisteredDelivery::default())
                    .sm_default_msg_id(0)
                    .message_payload(Some(MessagePayload::new(AnyOctetString::new(
                        b"Message Payload".to_vec(),
                    ))))
                    .build(),
                Self::builder()
                    .validity_period(
                        EmptyOrFullCOctetString::new(b"2023-10-01T12:00\0".to_vec()).unwrap()
                    )
                    .short_message(OctetString::new(b"Short Message".to_vec()).unwrap())
                    .message_payload(Some(MessagePayload::new(AnyOctetString::new(
                        b"Message Payload".to_vec(),
                    ))))
                    .build(),
            ]
        }
    }

    #[test]
    fn encode_decode() {
        crate::tests::owned::encode_decode_with_length_test_instances::<ReplaceSm>();
    }

    #[test]
    fn short_message_length() {
        let short_message = OctetString::new(b"Short Message".to_vec()).unwrap();

        let submit_sm = ReplaceSm::builder()
            .short_message(short_message.clone())
            .build();

        assert_eq!(submit_sm.short_message(), &short_message);
        assert_eq!(submit_sm.sm_length(), short_message.length() as u8);
    }

    #[test]
    fn short_message_override() {
        let short_message_1 = OctetString::new(b"Short Message 101".to_vec()).unwrap();
        let short_message_2 = OctetString::new(b"Short Message 2".to_vec()).unwrap();

        let submit_sm = ReplaceSm::builder()
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
        let replace_sm = ReplaceSm::builder()
            .short_message(short_message.clone())
            .message_payload(Some(message_payload.clone()))
            .build();

        assert_eq!(replace_sm.short_message(), &OctetString::empty());
        assert_eq!(replace_sm.sm_length(), 0);

        // Even setting the short message after the message payload should not set the short message
        let submit_sm = ReplaceSm::builder()
            .short_message(short_message.clone())
            .message_payload(Some(message_payload.clone()))
            .short_message(short_message.clone())
            .build();

        assert_eq!(submit_sm.short_message(), &OctetString::empty());
        assert_eq!(submit_sm.sm_length(), 0);

        // Removing the message payload and then setting the short message should set the short message
        let submit_sm = ReplaceSm::builder()
            .message_payload(Some(message_payload.clone()))
            .message_payload(None)
            .short_message(short_message.clone())
            .build();

        assert_eq!(submit_sm.short_message(), &short_message);
        assert_eq!(submit_sm.sm_length(), short_message.length() as u8);
    }
}
