use rusmpp_macros::Rusmpp;

use crate::{
    pdus::borrowed::Pdu,
    tlvs::borrowed::{MessageSubmissionResponseTlvValue, Tlv},
    types::borrowed::COctetString,
    values::borrowed::*,
};

#[derive(Default, Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Rusmpp)]
#[rusmpp(decode = borrowed, test = skip)]
#[cfg_attr(feature = "arbitrary", derive(::arbitrary::Arbitrary))]
#[cfg_attr(feature = "serde", derive(::serde::Serialize))]
pub struct SubmitMultiResp<'a, const N: usize> {
    /// This field contains the MC message ID of the submitted
    /// message. It may be used at a later stage to query the status
    /// of a message, cancel or replace the message.
    pub message_id: COctetString<'a, 1, 65>,
    /// The number of messages to destination SME addresses that
    /// were unsuccessfully submitted to the MC. This is followed by
    /// the specified number of unsuccessful SMEs, each
    /// specified in a unsuccess_sme field.
    no_unsuccess: u8,
    /// Unsuccessful SME.
    ///
    /// (Composite Field).
    #[rusmpp(count = no_unsuccess)]
    #[cfg_attr(feature = "arbitrary", arbitrary(default))]
    unsuccess_sme: heapless::vec::Vec<UnsuccessSme<'a>, N>,
    /// Message submission response TLVs ([`MessageSubmissionResponseTlvValue`])
    #[rusmpp(length = "unchecked")]
    #[cfg_attr(feature = "arbitrary", arbitrary(default))]
    tlvs: heapless::vec::Vec<Tlv<'a>, N>,
}

impl<'a, const N: usize> SubmitMultiResp<'a, N> {
    pub fn new(
        message_id: COctetString<'a, 1, 65>,
        unsuccess_sme: heapless::vec::Vec<UnsuccessSme<'a>, N>,
        tlvs: heapless::vec::Vec<MessageSubmissionResponseTlvValue<'a>, N>,
    ) -> Self {
        let no_unsuccess = unsuccess_sme.len() as u8;

        let tlvs = tlvs.into_iter().map(From::from).collect();

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

    pub fn unsuccess_sme(&'_ self) -> &'_ [UnsuccessSme<'_>] {
        &self.unsuccess_sme
    }

    pub fn set_unsuccess_sme(&mut self, unsuccess_sme: heapless::vec::Vec<UnsuccessSme<'a>, N>) {
        self.unsuccess_sme = unsuccess_sme;
        self.no_unsuccess = self.unsuccess_sme.len() as u8;
    }

    pub fn push_unsuccess_sme(
        &mut self,
        unsuccess_sme: UnsuccessSme<'a>,
    ) -> Result<(), UnsuccessSme<'a>> {
        self.unsuccess_sme.push(unsuccess_sme)?;
        self.no_unsuccess = self.unsuccess_sme.len() as u8;
        Ok(())
    }

    pub fn clear_unsuccess_sme(&mut self) {
        self.unsuccess_sme.clear();
        self.no_unsuccess = self.unsuccess_sme.len() as u8;
    }

    pub fn tlvs(&'_ self) -> &'_ [Tlv<'_>] {
        &self.tlvs
    }

    pub fn set_tlvs(&mut self, tlvs: heapless::vec::Vec<MessageSubmissionResponseTlvValue<'a>, N>) {
        self.tlvs = tlvs.into_iter().map(From::from).collect();
    }

    pub fn clear_tlvs(&mut self) {
        self.tlvs.clear();
    }

    pub fn push_tlv(
        &mut self,
        tlv: impl Into<MessageSubmissionResponseTlvValue<'a>>,
    ) -> Result<(), Tlv<'a>> {
        self.tlvs.push(Tlv::from(tlv.into()))?;
        Ok(())
    }

    pub fn builder() -> SubmitMultiRespBuilder<'a, N> {
        SubmitMultiRespBuilder::new()
    }
}

impl<'a, const N: usize> From<SubmitMultiResp<'a, N>> for Pdu<'a, N> {
    fn from(value: SubmitMultiResp<'a, N>) -> Self {
        Self::SubmitMultiResp(value)
    }
}

#[derive(Debug, Default)]
pub struct SubmitMultiRespBuilder<'a, const N: usize> {
    inner: SubmitMultiResp<'a, N>,
}

impl<'a, const N: usize> SubmitMultiRespBuilder<'a, N> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn message_id(mut self, message_id: COctetString<'a, 1, 65>) -> Self {
        self.inner.message_id = message_id;
        self
    }

    pub fn unsuccess_sme(mut self, unsuccess_sme: heapless::vec::Vec<UnsuccessSme<'a>, N>) -> Self {
        self.inner.set_unsuccess_sme(unsuccess_sme);
        self
    }

    pub fn push_unsuccess_sme(
        mut self,
        unsuccess_sme: UnsuccessSme<'a>,
    ) -> Result<Self, UnsuccessSme<'a>> {
        self.inner.push_unsuccess_sme(unsuccess_sme)?;
        Ok(self)
    }

    pub fn clear_unsuccess_sme(mut self) -> Self {
        self.inner.clear_unsuccess_sme();
        self
    }

    pub fn tlvs(
        mut self,
        tlvs: heapless::vec::Vec<MessageSubmissionResponseTlvValue<'a>, N>,
    ) -> Self {
        self.inner.set_tlvs(tlvs);
        self
    }

    pub fn clear_tlvs(mut self) -> Self {
        self.inner.clear_tlvs();
        self
    }

    pub fn push_tlv(
        mut self,
        tlv: impl Into<MessageSubmissionResponseTlvValue<'a>>,
    ) -> Result<Self, Tlv<'a>> {
        self.inner.push_tlv(tlv)?;
        Ok(self)
    }

    pub fn build(self) -> SubmitMultiResp<'a, N> {
        self.inner
    }
}

#[cfg(any(test, feature = "test"))]
mod tests {
    use crate::{CommandStatus, tests::TestInstance, values::*};

    use super::*;

    impl<const N: usize> TestInstance for SubmitMultiResp<'static, N> {
        fn instances() -> alloc::vec::Vec<Self> {
            alloc::vec![
                Self::default(),
                Self::builder()
                    .message_id(COctetString::new(b"1234567890\0").unwrap())
                    .build(),
                Self::builder()
                    .message_id(COctetString::new(b"1234567890\0").unwrap())
                    .push_unsuccess_sme(UnsuccessSme::default())
                    .unwrap()
                    .push_unsuccess_sme(UnsuccessSme::new(
                        Ton::International,
                        Npi::Data,
                        COctetString::new(b"1234567890\0").unwrap(),
                        CommandStatus::EsmeRunknownerr,
                    ))
                    .unwrap()
                    .build(),
                Self::builder()
                    .message_id(COctetString::new(b"1234567890\0").unwrap())
                    .push_tlv(MessageSubmissionResponseTlvValue::DpfResult(
                        DpfResult::NotSet,
                    ))
                    .unwrap()
                    .push_tlv(MessageSubmissionResponseTlvValue::NetworkErrorCode(
                        NetworkErrorCode::new(ErrorCodeNetworkType::Is95AccessDeniedReason, 0x01),
                    ))
                    .unwrap()
                    .build(),
            ]
        }
    }

    #[test]
    fn encode_decode() {
        crate::tests::borrowed::encode_decode_with_length_test_instances::<
            SubmitMultiResp<'static, 16>,
        >();
    }

    #[test]
    fn count() {
        let submit_multi = SubmitMultiResp::<'static, 16>::default();

        assert_eq!(submit_multi.no_unsuccess(), 0);
        assert!(submit_multi.unsuccess_sme().is_empty());

        let submit_multi = SubmitMultiResp::<'static, 16>::builder()
            .unsuccess_sme(
                [
                    UnsuccessSme::default(),
                    UnsuccessSme::new(
                        Ton::International,
                        Npi::Data,
                        COctetString::new(b"1234567890\0").unwrap(),
                        CommandStatus::EsmeRunknownerr,
                    ),
                ]
                .into(),
            )
            .build();

        assert_eq!(submit_multi.no_unsuccess(), 2);
        assert_eq!(submit_multi.unsuccess_sme().len(), 2);

        let submit_multi = SubmitMultiResp::<'static, 16>::builder()
            .push_unsuccess_sme(UnsuccessSme::default())
            .unwrap()
            .push_unsuccess_sme(UnsuccessSme::new(
                Ton::International,
                Npi::Data,
                COctetString::new(b"1234567890\0").unwrap(),
                CommandStatus::EsmeRunknownerr,
            ))
            .unwrap()
            .build();

        assert_eq!(submit_multi.no_unsuccess(), 2);
        assert_eq!(submit_multi.unsuccess_sme().len(), 2);

        let submit_multi = SubmitMultiResp::<'static, 16>::builder()
            .push_unsuccess_sme(UnsuccessSme::default())
            .unwrap()
            .push_unsuccess_sme(UnsuccessSme::new(
                Ton::International,
                Npi::Data,
                COctetString::new(b"1234567890\0").unwrap(),
                CommandStatus::EsmeRunknownerr,
            ))
            .unwrap()
            .clear_unsuccess_sme()
            .build();

        assert_eq!(submit_multi.no_unsuccess(), 0);
        assert!(submit_multi.unsuccess_sme().is_empty());
    }
}
