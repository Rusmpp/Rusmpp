use rusmpp_macros::Rusmpp;

use crate::{
    pdus::owned::Pdu,
    tlvs::owned::{MessageSubmissionResponseTlvValue, Tlv},
    types::owned::COctetString,
    values::owned::*,
};

#[derive(Default, Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Rusmpp)]
#[rusmpp(decode = owned, test = skip)]
#[cfg_attr(feature = "arbitrary", derive(::arbitrary::Arbitrary))]
#[cfg_attr(feature = "serde", derive(::serde::Serialize))]
#[cfg_attr(feature = "serde-deserialize-unchecked", derive(::serde::Deserialize))]
pub struct SubmitMultiResp {
    /// This field contains the MC message ID of the submitted
    /// message. It may be used at a later stage to query the status
    /// of a message, cancel or replace the message.
    pub message_id: COctetString<1, 65>,
    /// The number of messages to destination SME addresses that
    /// were unsuccessfully submitted to the MC. This is followed by
    /// the specified number of unsuccessful SMEs, each
    /// specified in a unsuccess_sme field.
    no_unsuccess: u8,
    /// Unsuccessful SME.
    ///
    /// (Composite Field).
    #[rusmpp(count = no_unsuccess)]
    unsuccess_sme: alloc::vec::Vec<UnsuccessSme>,
    /// Message submission response TLVs ([`MessageSubmissionResponseTlvValue`])
    #[rusmpp(length = "unchecked")]
    tlvs: alloc::vec::Vec<Tlv>,
}

impl SubmitMultiResp {
    pub fn new(
        message_id: COctetString<1, 65>,
        unsuccess_sme: alloc::vec::Vec<UnsuccessSme>,
        tlvs: alloc::vec::Vec<impl Into<MessageSubmissionResponseTlvValue>>,
    ) -> Self {
        let no_unsuccess = unsuccess_sme.len() as u8;

        let tlvs = tlvs.into_iter().map(Into::into).map(From::from).collect();

        Self {
            message_id,
            no_unsuccess,
            unsuccess_sme,
            tlvs,
        }
    }

    pub fn no_unsuccess(&self) -> u8 {
        self.no_unsuccess
    }

    pub fn unsuccess_sme(&self) -> &[UnsuccessSme] {
        &self.unsuccess_sme
    }

    pub fn set_unsuccess_sme(&mut self, unsuccess_sme: alloc::vec::Vec<UnsuccessSme>) {
        self.unsuccess_sme = unsuccess_sme;
        self.no_unsuccess = self.unsuccess_sme.len() as u8;
    }

    pub fn push_unsuccess_sme(&mut self, unsuccess_sme: UnsuccessSme) {
        self.unsuccess_sme.push(unsuccess_sme);
        self.no_unsuccess = self.unsuccess_sme.len() as u8;
    }

    pub fn clear_unsuccess_sme(&mut self) {
        self.unsuccess_sme.clear();
        self.no_unsuccess = self.unsuccess_sme.len() as u8;
    }

    pub fn tlvs(&self) -> &[Tlv] {
        &self.tlvs
    }

    pub fn set_tlvs(
        &mut self,
        tlvs: alloc::vec::Vec<impl Into<MessageSubmissionResponseTlvValue>>,
    ) {
        self.tlvs = tlvs.into_iter().map(Into::into).map(From::from).collect();
    }

    pub fn clear_tlvs(&mut self) {
        self.tlvs.clear();
    }

    pub fn push_tlv(&mut self, tlv: impl Into<MessageSubmissionResponseTlvValue>) {
        self.tlvs.push(Tlv::from(tlv.into()));
    }

    pub fn builder() -> SubmitMultiRespBuilder {
        SubmitMultiRespBuilder::new()
    }
}

impl From<SubmitMultiResp> for Pdu {
    fn from(value: SubmitMultiResp) -> Self {
        Self::SubmitMultiResp(value)
    }
}

#[derive(Debug, Default)]
pub struct SubmitMultiRespBuilder {
    inner: SubmitMultiResp,
}

impl SubmitMultiRespBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn message_id(mut self, message_id: COctetString<1, 65>) -> Self {
        self.inner.message_id = message_id;
        self
    }

    pub fn unsuccess_sme(mut self, unsuccess_sme: alloc::vec::Vec<UnsuccessSme>) -> Self {
        self.inner.set_unsuccess_sme(unsuccess_sme);
        self
    }

    pub fn push_unsuccess_sme(mut self, unsuccess_sme: UnsuccessSme) -> Self {
        self.inner.push_unsuccess_sme(unsuccess_sme);
        self
    }

    pub fn clear_unsuccess_sme(mut self) -> Self {
        self.inner.clear_unsuccess_sme();
        self
    }

    pub fn tlvs(
        mut self,
        tlvs: alloc::vec::Vec<impl Into<MessageSubmissionResponseTlvValue>>,
    ) -> Self {
        self.inner.set_tlvs(tlvs);
        self
    }

    pub fn clear_tlvs(mut self) -> Self {
        self.inner.clear_tlvs();
        self
    }

    pub fn push_tlv(mut self, tlv: impl Into<MessageSubmissionResponseTlvValue>) -> Self {
        self.inner.push_tlv(tlv);
        self
    }

    pub fn build(self) -> SubmitMultiResp {
        self.inner
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use crate::{CommandStatus, tests::TestInstance, values::*};

    use super::*;

    impl TestInstance for SubmitMultiResp {
        fn instances() -> alloc::vec::Vec<Self> {
            alloc::vec![
                Self::default(),
                Self::builder()
                    .message_id(COctetString::from_str("1234567890").unwrap())
                    .build(),
                Self::builder()
                    .message_id(COctetString::from_str("1234567890").unwrap())
                    .push_unsuccess_sme(UnsuccessSme::default())
                    .push_unsuccess_sme(UnsuccessSme::new(
                        Ton::International,
                        Npi::Data,
                        COctetString::from_str("1234567890").unwrap(),
                        CommandStatus::EsmeRunknownerr,
                    ))
                    .build(),
                Self::builder()
                    .message_id(COctetString::from_str("1234567890").unwrap())
                    .push_tlv(MessageSubmissionResponseTlvValue::DpfResult(
                        DpfResult::NotSet,
                    ))
                    .push_tlv(MessageSubmissionResponseTlvValue::NetworkErrorCode(
                        NetworkErrorCode::new(ErrorCodeNetworkType::Is95AccessDeniedReason, 0x01),
                    ))
                    .build(),
            ]
        }
    }

    #[test]
    fn encode_decode() {
        #[cfg(feature = "alloc")]
        crate::tests::owned::encode_decode_with_length_test_instances::<SubmitMultiResp>();
    }

    #[test]
    fn count() {
        let submit_multi = SubmitMultiResp::default();

        assert_eq!(submit_multi.no_unsuccess(), 0);
        assert!(submit_multi.unsuccess_sme().is_empty());

        let submit_multi = SubmitMultiResp::builder()
            .unsuccess_sme(alloc::vec![
                UnsuccessSme::default(),
                UnsuccessSme::new(
                    Ton::International,
                    Npi::Data,
                    COctetString::from_str("1234567890").unwrap(),
                    CommandStatus::EsmeRunknownerr,
                ),
            ])
            .build();

        assert_eq!(submit_multi.no_unsuccess(), 2);
        assert_eq!(submit_multi.unsuccess_sme().len(), 2);

        let submit_multi = SubmitMultiResp::builder()
            .push_unsuccess_sme(UnsuccessSme::default())
            .push_unsuccess_sme(UnsuccessSme::new(
                Ton::International,
                Npi::Data,
                COctetString::from_str("1234567890").unwrap(),
                CommandStatus::EsmeRunknownerr,
            ))
            .build();

        assert_eq!(submit_multi.no_unsuccess(), 2);
        assert_eq!(submit_multi.unsuccess_sme().len(), 2);

        let submit_multi = SubmitMultiResp::builder()
            .push_unsuccess_sme(UnsuccessSme::default())
            .push_unsuccess_sme(UnsuccessSme::new(
                Ton::International,
                Npi::Data,
                COctetString::from_str("1234567890").unwrap(),
                CommandStatus::EsmeRunknownerr,
            ))
            .clear_unsuccess_sme()
            .build();

        assert_eq!(submit_multi.no_unsuccess(), 0);
        assert!(submit_multi.unsuccess_sme().is_empty());
    }
}
