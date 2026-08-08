use rusmpp_macros::TlvValue;

use crate::{
    tlvs::{
        TlvTag,
        borrowed::{Tlv, TlvValue},
    },
    types::borrowed::{AnyOctetString, COctetString, OctetString},
    values::{borrowed::*, *},
};

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, TlvValue)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(bound(deserialize = "'de: 'a")))]
pub enum MessageSubmissionRequestTlvValue<'a> {
    AlertOnMessageDelivery(AlertOnMessageDelivery),
    BillingIdentification(OctetString<'a, 0, 1024>),
    CallbackNum(OctetString<'a, 4, 19>),
    CallbackNumAtag(OctetString<'a, 0, 65>),
    CallbackNumPresInd(CallbackNumPresInd),
    DestAddrNpCountry(OctetString<'a, 1, 5>),
    DestAddrNpInformation(OctetString<'a, 0, 10>),
    DestAddrNpResolution(DestAddrNpResolution),
    DestAddrSubunit(AddrSubunit),
    DestBearerType(BearerType),
    DestNetworkId(COctetString<'a, 7, 66>),
    DestNetworkType(NetworkType),
    DestNodeId(OctetString<'a, 6, 6>),
    DestSubaddress(Subaddress<'a>),
    DestTelematicsId(u16),
    DestPort(u16),
    DisplayTime(DisplayTime),
    ItsReplyType(ItsReplyType),
    ItsSessionInfo(ItsSessionInfo),
    LanguageIndicator(LanguageIndicator),
    MessagePayload(MessagePayload<'a>),
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
    SourceNetworkId(COctetString<'a, 7, 66>),
    SourceNetworkType(NetworkType),
    SourceNodeId(OctetString<'a, 6, 6>),
    SourcePort(u16),
    SourceSubaddress(Subaddress<'a>),
    SourceTelematicsId(u16),
    UserMessageReference(UserMessageReference),
    UserResponseCode(u8),
    UssdServiceOp(UssdServiceOp),
    Other {
        tag: TlvTag,
        value: AnyOctetString<'a>,
    },
}
