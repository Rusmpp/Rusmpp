use rusmpp_macros::TlvValue;

use crate::{
    tlvs::{
        TlvTag,
        owned::{Tlv, TlvValue},
    },
    types::owned::{AnyOctetString, COctetString, OctetString},
    values::{owned::*, *},
};

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, TlvValue)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize))]
#[cfg_attr(feature = "serde-deserialize-unchecked", derive(::serde::Deserialize))]
pub enum MessageSubmissionRequestTlvValue {
    AlertOnMessageDelivery(AlertOnMessageDelivery),
    BillingIdentification(OctetString<0, 1024>),
    CallbackNum(OctetString<4, 19>),
    CallbackNumAtag(OctetString<0, 65>),
    CallbackNumPresInd(CallbackNumPresInd),
    DestAddrNpCountry(OctetString<1, 5>),
    DestAddrNpInformation(OctetString<0, 10>),
    DestAddrNpResolution(DestAddrNpResolution),
    DestAddrSubunit(AddrSubunit),
    DestBearerType(BearerType),
    DestNetworkId(COctetString<7, 66>),
    DestNetworkType(NetworkType),
    DestNodeId(OctetString<6, 6>),
    DestSubaddress(Subaddress),
    DestTelematicsId(u16),
    DestPort(u16),
    DisplayTime(DisplayTime),
    ItsReplyType(ItsReplyType),
    ItsSessionInfo(ItsSessionInfo),
    LanguageIndicator(LanguageIndicator),
    MessagePayload(MessagePayload),
    MoreMessagesToSend(MoreMessagesToSend),
    MsMsgWaitFacilities(MsMsgWaitFacilities),
    MsValidity(MsValidity),
    NumberOfMessages(NumberOfMessages),
    PayloadType(PayloadType),
    PrivacyIndicator(PrivacyIndicator),
    QosTimeToLive(u32),
    SarMsgRefNum(u16),
    SarSegmentSeqnum(u8),
    SarTotalSegments(u8),
    SetDpf(SetDpf),
    SmsSignal(u16),
    SourceAddrSubunit(AddrSubunit),
    SourceBearerType(BearerType),
    SourceNetworkId(COctetString<7, 66>),
    SourceNetworkType(NetworkType),
    SourceNodeId(OctetString<6, 6>),
    SourcePort(u16),
    SourceSubaddress(Subaddress),
    SourceTelematicsId(u16),
    UserMessageReference(UserMessageReference),
    UserResponseCode(u8),
    UssdServiceOp(UssdServiceOp),
    Other { tag: TlvTag, value: AnyOctetString },
}

// TODO: macro?
impl MessageSubmissionRequestTlvValue {
    /// Attempts to create a [`MessageSubmissionRequestTlvValue`] from a generic [`TlvValue`].
    pub fn from_tlv_value(value: TlvValue) -> Option<Self> {
        let value = match value {
            TlvValue::AlertOnMessageDelivery(value) => Self::AlertOnMessageDelivery(value),
            TlvValue::BillingIdentification(value) => Self::BillingIdentification(value),
            TlvValue::CallbackNum(value) => Self::CallbackNum(value),
            TlvValue::CallbackNumAtag(value) => Self::CallbackNumAtag(value),
            TlvValue::CallbackNumPresInd(value) => Self::CallbackNumPresInd(value),
            TlvValue::DestAddrNpCountry(value) => Self::DestAddrNpCountry(value),
            TlvValue::DestAddrNpInformation(value) => Self::DestAddrNpInformation(value),
            TlvValue::DestAddrNpResolution(value) => Self::DestAddrNpResolution(value),
            TlvValue::DestAddrSubunit(value) => Self::DestAddrSubunit(value),
            TlvValue::DestBearerType(value) => Self::DestBearerType(value),
            TlvValue::DestNetworkId(value) => Self::DestNetworkId(value),
            TlvValue::DestNetworkType(value) => Self::DestNetworkType(value),
            TlvValue::DestNodeId(value) => Self::DestNodeId(value),
            TlvValue::DestSubaddress(value) => Self::DestSubaddress(value),
            TlvValue::DestTelematicsId(value) => Self::DestTelematicsId(value),
            TlvValue::DestPort(value) => Self::DestPort(value),
            TlvValue::DisplayTime(value) => Self::DisplayTime(value),
            TlvValue::ItsReplyType(value) => Self::ItsReplyType(value),
            TlvValue::ItsSessionInfo(value) => Self::ItsSessionInfo(value),
            TlvValue::LanguageIndicator(value) => Self::LanguageIndicator(value),
            TlvValue::MessagePayload(value) => Self::MessagePayload(value),
            TlvValue::MoreMessagesToSend(value) => Self::MoreMessagesToSend(value),
            TlvValue::MsMsgWaitFacilities(value) => Self::MsMsgWaitFacilities(value),
            TlvValue::MsValidity(value) => Self::MsValidity(value),
            TlvValue::NumberOfMessages(value) => Self::NumberOfMessages(value),
            TlvValue::PayloadType(value) => Self::PayloadType(value),
            TlvValue::PrivacyIndicator(value) => Self::PrivacyIndicator(value),
            TlvValue::QosTimeToLive(value) => Self::QosTimeToLive(value),
            TlvValue::SarMsgRefNum(value) => Self::SarMsgRefNum(value),
            TlvValue::SarSegmentSeqnum(value) => Self::SarSegmentSeqnum(value),
            TlvValue::SarTotalSegments(value) => Self::SarTotalSegments(value),
            TlvValue::SetDpf(value) => Self::SetDpf(value),
            TlvValue::SmsSignal(value) => Self::SmsSignal(value),
            TlvValue::SourceAddrSubunit(value) => Self::SourceAddrSubunit(value),
            TlvValue::SourceBearerType(value) => Self::SourceBearerType(value),
            TlvValue::SourceNetworkId(value) => Self::SourceNetworkId(value),
            TlvValue::SourceNetworkType(value) => Self::SourceNetworkType(value),
            TlvValue::SourceNodeId(value) => Self::SourceNodeId(value),
            TlvValue::SourcePort(value) => Self::SourcePort(value),
            TlvValue::SourceSubaddress(value) => Self::SourceSubaddress(value),
            TlvValue::SourceTelematicsId(value) => Self::SourceTelematicsId(value),
            TlvValue::UserMessageReference(value) => Self::UserMessageReference(value),
            TlvValue::UserResponseCode(value) => Self::UserResponseCode(value),
            TlvValue::UssdServiceOp(value) => Self::UssdServiceOp(value),
            TlvValue::Other { tag, value } => Self::Other { tag, value },
            _ => return None,
        };

        Some(value)
    }
}
