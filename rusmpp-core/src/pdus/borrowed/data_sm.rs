use rusmpp_macros::Rusmpp;

use crate::{
    pdus::borrowed::Pdu,
    tlvs::borrowed::{MessageSubmissionRequestTlvValue, Tlv},
    types::borrowed::COctetString,
    values::{borrowed::*, *},
};

/// The data_sm operation is similar to the submit_sm in that it provides a means to submit a
/// mobile-terminated message. However, data_sm is intended for packet-based applications
/// such as WAP in that it features a reduced PDU body containing fields relevant to WAP or
/// packet-based applications.
#[derive(Default, Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Rusmpp)]
#[rusmpp(decode = borrowed, test = skip)]
#[cfg_attr(feature = "arbitrary", derive(::arbitrary::Arbitrary))]
#[cfg_attr(feature = "serde", derive(::serde::Serialize))]
pub struct DataSm<'a, const N: usize> {
    /// The service_type parameter can be used to indicate the
    /// SMS Application service associated with the message.
    /// Specifying the service_type allows the ESME to avail of
    /// enhanced messaging services such as “replace by
    /// service_type” or control the teleservice used on the air
    /// interface.
    ///
    /// Set to NULL for default MC
    /// settings.
    pub service_type: ServiceType<'a>,
    /// Type of Number for source
    /// address.
    ///
    /// If not known, set to NULL
    /// (Unknown).
    pub source_addr_ton: Ton,
    /// Numbering Plan Indicator for
    /// source address.
    ///
    /// If not known, set to NULL
    /// (Unknown).
    pub source_addr_npi: Npi,
    /// Address of SME which
    /// originated this message.
    ///
    /// If not known, set to NULL
    /// (Unknown).
    pub source_addr: COctetString<'a, 1, 21>,
    /// Type of Number for destination.
    pub dest_addr_ton: Ton,
    /// Numbering Plan Indicator for destination.
    pub dest_addr_npi: Npi,
    /// Destination address of this short message For mobile
    /// terminated messages, this is the directory number of the
    /// recipient MS.
    pub destination_addr: COctetString<'a, 1, 21>,
    /// Indicates Message Mode and Message Type.
    pub esm_class: EsmClass,
    /// Indicator to signify if a MC
    /// delivery receipt or an SME
    /// acknowledgement is required.
    pub registered_delivery: RegisteredDelivery,
    /// Defines the encoding scheme
    /// of the short message user data.
    pub data_coding: DataCoding,
    /// Message submission request TLVs ([`MessageSubmissionRequestTlvValue`])
    #[rusmpp(length = "unchecked")]
    #[cfg_attr(feature = "arbitrary", arbitrary(default))]
    tlvs: heapless::vec::Vec<Tlv<'a>, N>,
}

impl<'a, const N: usize> DataSm<'a, N> {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        service_type: ServiceType<'a>,
        source_addr_ton: Ton,
        source_addr_npi: Npi,
        source_addr: COctetString<'a, 1, 21>,
        dest_addr_ton: Ton,
        dest_addr_npi: Npi,
        destination_addr: COctetString<'a, 1, 21>,
        esm_class: EsmClass,
        registered_delivery: RegisteredDelivery,
        data_coding: DataCoding,
        tlvs: heapless::vec::Vec<MessageSubmissionRequestTlvValue<'a>, N>,
    ) -> Self {
        let tlvs = tlvs.into_iter().map(From::from).collect();

        Self {
            service_type,
            source_addr_ton,
            source_addr_npi,
            source_addr,
            dest_addr_ton,
            dest_addr_npi,
            destination_addr,
            esm_class,
            registered_delivery,
            data_coding,
            tlvs,
        }
    }

    pub fn tlvs(&'_ self) -> &'_ [Tlv<'_>] {
        &self.tlvs
    }

    pub fn set_tlvs(&mut self, tlvs: heapless::vec::Vec<MessageSubmissionRequestTlvValue<'a>, N>) {
        self.tlvs = tlvs.into_iter().map(From::from).collect();
    }

    pub fn clear_tlvs(&mut self) {
        self.tlvs.clear();
    }

    pub fn push_tlv(
        &mut self,
        tlv: impl Into<MessageSubmissionRequestTlvValue<'a>>,
    ) -> Result<(), Tlv<'a>> {
        self.tlvs.push(Tlv::from(tlv.into()))?;
        Ok(())
    }

    pub fn builder() -> DataSmBuilder<'a, N> {
        DataSmBuilder::new()
    }
}

impl<'a, const N: usize> From<DataSm<'a, N>> for Pdu<'a, N> {
    fn from(value: DataSm<'a, N>) -> Self {
        Self::DataSm(value)
    }
}

#[derive(Debug, Default)]
pub struct DataSmBuilder<'a, const N: usize> {
    inner: DataSm<'a, N>,
}

impl<'a, const N: usize> DataSmBuilder<'a, N> {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn service_type(mut self, service_type: ServiceType<'a>) -> Self {
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

    pub fn source_addr(mut self, source_addr: COctetString<'a, 1, 21>) -> Self {
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

    pub fn destination_addr(mut self, destination_addr: COctetString<'a, 1, 21>) -> Self {
        self.inner.destination_addr = destination_addr;
        self
    }

    pub fn esm_class(mut self, esm_class: EsmClass) -> Self {
        self.inner.esm_class = esm_class;
        self
    }

    pub fn registered_delivery(mut self, registered_delivery: RegisteredDelivery) -> Self {
        self.inner.registered_delivery = registered_delivery;
        self
    }

    pub fn data_coding(mut self, data_coding: DataCoding) -> Self {
        self.inner.data_coding = data_coding;
        self
    }

    pub fn tlvs(
        mut self,
        tlvs: heapless::vec::Vec<MessageSubmissionRequestTlvValue<'a>, N>,
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
        tlv: impl Into<MessageSubmissionRequestTlvValue<'a>>,
    ) -> Result<Self, Tlv<'a>> {
        self.inner.push_tlv(tlv)?;
        Ok(self)
    }

    pub fn build(self) -> DataSm<'a, N> {
        self.inner
    }
}

#[cfg(any(test, feature = "test"))]
mod tests {
    use crate::tests::TestInstance;

    use super::*;

    impl<const N: usize> TestInstance for DataSm<'_, N> {
        fn instances() -> alloc::vec::Vec<Self> {
            alloc::vec![
                Self::default(),
                Self::builder()
                    .service_type(ServiceType::default())
                    .source_addr_ton(Ton::International)
                    .source_addr_npi(Npi::Isdn)
                    .source_addr(COctetString::new(b"source_addr\0").unwrap())
                    .dest_addr_ton(Ton::International)
                    .dest_addr_npi(Npi::Isdn)
                    .destination_addr(COctetString::new(b"destination_addr\0").unwrap())
                    .esm_class(EsmClass::default())
                    .registered_delivery(RegisteredDelivery::request_all())
                    .data_coding(DataCoding::Ucs2)
                    .build(),
                Self::builder()
                    .service_type(ServiceType::default())
                    .source_addr_ton(Ton::International)
                    .source_addr_npi(Npi::Isdn)
                    .source_addr(COctetString::new(b"source_addr\0").unwrap())
                    .dest_addr_ton(Ton::International)
                    .dest_addr_npi(Npi::Isdn)
                    .destination_addr(COctetString::new(b"destination_addr\0").unwrap())
                    .esm_class(EsmClass::default())
                    .registered_delivery(RegisteredDelivery::new(
                        MCDeliveryReceipt::NoMcDeliveryReceiptRequested,
                        SmeOriginatedAcknowledgement::SmeUserAcknowledgementRequested,
                        IntermediateNotification::IntermediateNotificationRequested,
                        0,
                    ))
                    .data_coding(DataCoding::Ucs2)
                    .push_tlv(MessageSubmissionRequestTlvValue::SourceAddrSubunit(
                        AddrSubunit::MobileEquipment,
                    ))
                    .unwrap()
                    .push_tlv(MessageSubmissionRequestTlvValue::UssdServiceOp(
                        UssdServiceOp::UssnConfirm,
                    ))
                    .unwrap()
                    .build(),
            ]
        }
    }

    #[test]
    fn encode_decode() {
        crate::tests::borrowed::encode_decode_with_length_test_instances::<DataSm<'static, 16>>();
    }
}
