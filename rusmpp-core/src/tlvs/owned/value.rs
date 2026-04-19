use bytes::BytesMut;

use crate::{
    CommandStatus, Sealed,
    decode::{
        AnyOctetStringDecodeError, COctetStringDecodeError, DecodeResultExt, IntegerDecodeError,
        OctetStringDecodeError,
        owned::{Decode, DecodeErrorType, DecodeWithKey, DecodeWithLength},
    },
    encode::Length,
    tlvs::TlvTag,
    types::owned::{AnyOctetString, COctetString, OctetString},
    values::{
        errors::*,
        owned::{errors::*, *},
        *,
    },
};

/// See module level documentation.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[cfg_attr(feature = "arbitrary", derive(::arbitrary::Arbitrary))]
#[cfg_attr(feature = "serde", derive(::serde::Serialize))]
#[cfg_attr(feature = "serde-deserialize-unchecked", derive(::serde::Deserialize))]
pub enum TlvValue {
    AdditionalStatusInfoText(COctetString<1, 256>),
    AlertOnMessageDelivery(AlertOnMessageDelivery),
    BillingIdentification(OctetString<0, 1024>),
    /// Identifies one or more target Broadcast Area(s) for which the
    /// status information applies.
    ///
    /// The number of instances of this parameter will be exactly equal
    /// to the number of occurrences of the broadcast_area_identifiers
    /// parameter in the corresponding broadcast_sm.
    BroadcastAreaIdentifier(BroadcastAreaIdentifier),
    /// The success rate indicator, defined as the ratio of the
    /// number of BTSs that accepted the message and the total
    /// number of BTSs that should have accepted the message, for
    /// a particular broadcast_area_identifier.
    BroadcastAreaSuccess(BroadcastAreaSuccess),
    BroadcastContentTypeInfo(OctetString<0, 255>),
    BroadcastChannelIndicator(BroadcastChannelIndicator),
    /// Specifies the content type of the message.
    BroadcastContentType(BroadcastContentType),
    /// Absolute time is formatted as a 16-character string (encoded as a 17-octet C-octet String)
    /// “YYMMDDhhmmsstnnp” where:
    ///
    /// | Digits | Meaning |
    /// |--------|---------|
    /// | ‘YY’   | last two digits of the year (00-99)   |
    /// | ‘MM’   | month (01-12)                         |
    /// | ‘DD’   | day (01-31)                           |
    /// | ‘hh’   | hour (00-23)                          |
    /// | ‘mm’   | minute (00-59)                        |
    /// | ‘ss’   | second (00-59)                        |
    /// | ‘t’    | tenths of second (0-9)                |
    /// | ‘nn’   | time difference in quarter hours between local time (as expressed in the first 13 octets) and UTC (Universal Time Constant) time (00-48). |
    /// | ‘p’    | “+” Local time is in quarter hours advanced in relation to UTC time. “-” Local time is in quarter hours retarded in relation to UTC time. |
    BroadcastEndTime(OctetString<0, 17>),
    BroadcastErrorStatus(CommandStatus),
    /// This field indicates the frequency interval at which
    /// the broadcasts of a message should be repeated.
    BroadcastFrequencyInterval(BroadcastFrequencyInterval),
    BroadcastMessageClass(BroadcastMessageClass),
    /// This field indicates the number of repeated
    /// broadcasts of a message requested by the submitter.
    BroadcastRepNum(BroadcastRepNum),
    BroadcastServiceGroup(OctetString<1, 255>),
    CallbackNum(OctetString<4, 19>),
    CallbackNumAtag(OctetString<0, 65>),
    CallbackNumPresInd(CallbackNumPresInd),
    CongestionState(CongestionState),
    DeliveryFailureReason(DeliveryFailureReason),
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
    DpfResult(DpfResult),
    ItsReplyType(ItsReplyType),
    ItsSessionInfo(ItsSessionInfo),
    LanguageIndicator(LanguageIndicator),
    MessagePayload(MessagePayload),
    /// This field indicates the current status of the broadcast message.
    MessageState(MessageState),
    MoreMessagesToSend(MoreMessagesToSend),
    MsAvailabilityStatus(MsAvailabilityStatus),
    MsMsgWaitFacilities(MsMsgWaitFacilities),
    MsValidity(MsValidity),
    NetworkErrorCode(NetworkErrorCode),
    NumberOfMessages(NumberOfMessages),
    PayloadType(PayloadType),
    PrivacyIndicator(PrivacyIndicator),
    QosTimeToLive(u32),
    ReceiptedMessageId(COctetString<1, 65>),
    SarMsgRefNum(u16),
    SarSegmentSeqnum(u8),
    SarTotalSegments(u8),
    ScInterfaceVersion(InterfaceVersion),
    SetDpf(SetDpf),
    /// Encoded as per [CMT-136]
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
    Other {
        tag: TlvTag,
        value: AnyOctetString,
    },
}

impl TlvValue {
    pub const fn tag(&self) -> TlvTag {
        match self {
            TlvValue::AdditionalStatusInfoText(_) => TlvTag::AdditionalStatusInfoText,
            TlvValue::AlertOnMessageDelivery(_) => TlvTag::AlertOnMessageDelivery,
            TlvValue::BillingIdentification(_) => TlvTag::BillingIdentification,
            TlvValue::BroadcastAreaIdentifier(_) => TlvTag::BroadcastAreaIdentifier,
            TlvValue::BroadcastAreaSuccess(_) => TlvTag::BroadcastAreaSuccess,
            TlvValue::BroadcastContentTypeInfo(_) => TlvTag::BroadcastContentTypeInfo,
            TlvValue::BroadcastChannelIndicator(_) => TlvTag::BroadcastChannelIndicator,
            TlvValue::BroadcastContentType(_) => TlvTag::BroadcastContentType,
            TlvValue::BroadcastEndTime(_) => TlvTag::BroadcastEndTime,
            TlvValue::BroadcastErrorStatus(_) => TlvTag::BroadcastErrorStatus,
            TlvValue::BroadcastFrequencyInterval(_) => TlvTag::BroadcastFrequencyInterval,
            TlvValue::BroadcastMessageClass(_) => TlvTag::BroadcastMessageClass,
            TlvValue::BroadcastRepNum(_) => TlvTag::BroadcastRepNum,
            TlvValue::BroadcastServiceGroup(_) => TlvTag::BroadcastServiceGroup,
            TlvValue::CallbackNum(_) => TlvTag::CallbackNum,
            TlvValue::CallbackNumAtag(_) => TlvTag::CallbackNumAtag,
            TlvValue::CallbackNumPresInd(_) => TlvTag::CallbackNumPresInd,
            TlvValue::CongestionState(_) => TlvTag::CongestionState,
            TlvValue::DeliveryFailureReason(_) => TlvTag::DeliveryFailureReason,
            TlvValue::DestAddrNpCountry(_) => TlvTag::DestAddrNpCountry,
            TlvValue::DestAddrNpInformation(_) => TlvTag::DestAddrNpInformation,
            TlvValue::DestAddrNpResolution(_) => TlvTag::DestAddrNpResolution,
            TlvValue::DestAddrSubunit(_) => TlvTag::DestAddrSubunit,
            TlvValue::DestBearerType(_) => TlvTag::DestBearerType,
            TlvValue::DestNetworkId(_) => TlvTag::DestNetworkId,
            TlvValue::DestNetworkType(_) => TlvTag::DestNetworkType,
            TlvValue::DestNodeId(_) => TlvTag::DestNodeId,
            TlvValue::DestSubaddress(_) => TlvTag::DestSubaddress,
            TlvValue::DestTelematicsId(_) => TlvTag::DestTelematicsId,
            TlvValue::DestPort(_) => TlvTag::DestPort,
            TlvValue::DisplayTime(_) => TlvTag::DisplayTime,
            TlvValue::DpfResult(_) => TlvTag::DpfResult,
            TlvValue::ItsReplyType(_) => TlvTag::ItsReplyType,
            TlvValue::ItsSessionInfo(_) => TlvTag::ItsSessionInfo,
            TlvValue::LanguageIndicator(_) => TlvTag::LanguageIndicator,
            TlvValue::MessagePayload(_) => TlvTag::MessagePayload,
            TlvValue::MessageState(_) => TlvTag::MessageState,
            TlvValue::MoreMessagesToSend(_) => TlvTag::MoreMessagesToSend,
            TlvValue::MsAvailabilityStatus(_) => TlvTag::MsAvailabilityStatus,
            TlvValue::MsMsgWaitFacilities(_) => TlvTag::MsMsgWaitFacilities,
            TlvValue::MsValidity(_) => TlvTag::MsValidity,
            TlvValue::NetworkErrorCode(_) => TlvTag::NetworkErrorCode,
            TlvValue::NumberOfMessages(_) => TlvTag::NumberOfMessages,
            TlvValue::PayloadType(_) => TlvTag::PayloadType,
            TlvValue::PrivacyIndicator(_) => TlvTag::PrivacyIndicator,
            TlvValue::QosTimeToLive(_) => TlvTag::QosTimeToLive,
            TlvValue::ReceiptedMessageId(_) => TlvTag::ReceiptedMessageId,
            TlvValue::SarMsgRefNum(_) => TlvTag::SarMsgRefNum,
            TlvValue::SarSegmentSeqnum(_) => TlvTag::SarSegmentSeqnum,
            TlvValue::SarTotalSegments(_) => TlvTag::SarTotalSegments,
            TlvValue::ScInterfaceVersion(_) => TlvTag::ScInterfaceVersion,
            TlvValue::SetDpf(_) => TlvTag::SetDpf,
            TlvValue::SmsSignal(_) => TlvTag::SmsSignal,
            TlvValue::SourceAddrSubunit(_) => TlvTag::SourceAddrSubunit,
            TlvValue::SourceBearerType(_) => TlvTag::SourceBearerType,
            TlvValue::SourceNetworkId(_) => TlvTag::SourceNetworkId,
            TlvValue::SourceNetworkType(_) => TlvTag::SourceNetworkType,
            TlvValue::SourceNodeId(_) => TlvTag::SourceNodeId,
            TlvValue::SourcePort(_) => TlvTag::SourcePort,
            TlvValue::SourceSubaddress(_) => TlvTag::SourceSubaddress,
            TlvValue::SourceTelematicsId(_) => TlvTag::SourceTelematicsId,
            TlvValue::UserMessageReference(_) => TlvTag::UserMessageReference,
            TlvValue::UserResponseCode(_) => TlvTag::UserResponseCode,
            TlvValue::UssdServiceOp(_) => TlvTag::UssdServiceOp,
            TlvValue::Other { tag, .. } => *tag,
        }
    }
}

impl Sealed for TlvValue {}

impl Length for TlvValue {
    fn length(&self) -> usize {
        match self {
            TlvValue::AdditionalStatusInfoText(value) => value.length(),
            TlvValue::AlertOnMessageDelivery(value) => value.length(),
            TlvValue::BillingIdentification(value) => value.length(),
            TlvValue::BroadcastAreaIdentifier(value) => value.length(),
            TlvValue::BroadcastAreaSuccess(value) => value.length(),
            TlvValue::BroadcastContentTypeInfo(value) => value.length(),
            TlvValue::BroadcastChannelIndicator(value) => value.length(),
            TlvValue::BroadcastContentType(value) => value.length(),
            TlvValue::BroadcastEndTime(value) => value.length(),
            TlvValue::BroadcastErrorStatus(value) => value.length(),
            TlvValue::BroadcastFrequencyInterval(value) => value.length(),
            TlvValue::BroadcastMessageClass(value) => value.length(),
            TlvValue::BroadcastRepNum(value) => value.length(),
            TlvValue::BroadcastServiceGroup(value) => value.length(),
            TlvValue::CallbackNum(value) => value.length(),
            TlvValue::CallbackNumAtag(value) => value.length(),
            TlvValue::CallbackNumPresInd(value) => value.length(),
            TlvValue::CongestionState(value) => value.length(),
            TlvValue::DeliveryFailureReason(value) => value.length(),
            TlvValue::DestAddrNpCountry(value) => value.length(),
            TlvValue::DestAddrNpInformation(value) => value.length(),
            TlvValue::DestAddrNpResolution(value) => value.length(),
            TlvValue::DestAddrSubunit(value) => value.length(),
            TlvValue::DestBearerType(value) => value.length(),
            TlvValue::DestNetworkId(value) => value.length(),
            TlvValue::DestNetworkType(value) => value.length(),
            TlvValue::DestNodeId(value) => value.length(),
            TlvValue::DestSubaddress(value) => value.length(),
            TlvValue::DestTelematicsId(value) => value.length(),
            TlvValue::DestPort(value) => value.length(),
            TlvValue::DisplayTime(value) => value.length(),
            TlvValue::DpfResult(value) => value.length(),
            TlvValue::ItsReplyType(value) => value.length(),
            TlvValue::ItsSessionInfo(value) => value.length(),
            TlvValue::LanguageIndicator(value) => value.length(),
            TlvValue::MessagePayload(value) => value.length(),
            TlvValue::MessageState(value) => value.length(),
            TlvValue::MoreMessagesToSend(value) => value.length(),
            TlvValue::MsAvailabilityStatus(value) => value.length(),
            TlvValue::MsMsgWaitFacilities(value) => value.length(),
            TlvValue::MsValidity(value) => value.length(),
            TlvValue::NetworkErrorCode(value) => value.length(),
            TlvValue::NumberOfMessages(value) => value.length(),
            TlvValue::PayloadType(value) => value.length(),
            TlvValue::PrivacyIndicator(value) => value.length(),
            TlvValue::QosTimeToLive(value) => value.length(),
            TlvValue::ReceiptedMessageId(value) => value.length(),
            TlvValue::SarMsgRefNum(value) => value.length(),
            TlvValue::SarSegmentSeqnum(value) => value.length(),
            TlvValue::SarTotalSegments(value) => value.length(),
            TlvValue::ScInterfaceVersion(value) => value.length(),
            TlvValue::SetDpf(value) => value.length(),
            TlvValue::SmsSignal(value) => value.length(),
            TlvValue::SourceAddrSubunit(value) => value.length(),
            TlvValue::SourceBearerType(value) => value.length(),
            TlvValue::SourceNetworkId(value) => value.length(),
            TlvValue::SourceNetworkType(value) => value.length(),
            TlvValue::SourceNodeId(value) => value.length(),
            TlvValue::SourcePort(value) => value.length(),
            TlvValue::SourceSubaddress(value) => value.length(),
            TlvValue::SourceTelematicsId(value) => value.length(),
            TlvValue::UserMessageReference(value) => value.length(),
            TlvValue::UserResponseCode(value) => value.length(),
            TlvValue::UssdServiceOp(value) => value.length(),
            TlvValue::Other { value, .. } => value.length(),
        }
    }
}

impl crate::encode::Encode for TlvValue {
    fn encode(&self, dst: &mut [u8]) -> usize {
        match self {
            TlvValue::AdditionalStatusInfoText(value) => value.encode(dst),
            TlvValue::AlertOnMessageDelivery(value) => value.encode(dst),
            TlvValue::BillingIdentification(value) => value.encode(dst),
            TlvValue::BroadcastAreaIdentifier(value) => value.encode(dst),
            TlvValue::BroadcastAreaSuccess(value) => value.encode(dst),
            TlvValue::BroadcastContentTypeInfo(value) => value.encode(dst),
            TlvValue::BroadcastChannelIndicator(value) => value.encode(dst),
            TlvValue::BroadcastContentType(value) => value.encode(dst),
            TlvValue::BroadcastEndTime(value) => value.encode(dst),
            TlvValue::BroadcastErrorStatus(value) => value.encode(dst),
            TlvValue::BroadcastFrequencyInterval(value) => value.encode(dst),
            TlvValue::BroadcastMessageClass(value) => value.encode(dst),
            TlvValue::BroadcastRepNum(value) => value.encode(dst),
            TlvValue::BroadcastServiceGroup(value) => value.encode(dst),
            TlvValue::CallbackNum(value) => value.encode(dst),
            TlvValue::CallbackNumAtag(value) => value.encode(dst),
            TlvValue::CallbackNumPresInd(value) => value.encode(dst),
            TlvValue::CongestionState(value) => value.encode(dst),
            TlvValue::DeliveryFailureReason(value) => value.encode(dst),
            TlvValue::DestAddrNpCountry(value) => value.encode(dst),
            TlvValue::DestAddrNpInformation(value) => value.encode(dst),
            TlvValue::DestAddrNpResolution(value) => value.encode(dst),
            TlvValue::DestAddrSubunit(value) => value.encode(dst),
            TlvValue::DestBearerType(value) => value.encode(dst),
            TlvValue::DestNetworkId(value) => value.encode(dst),
            TlvValue::DestNetworkType(value) => value.encode(dst),
            TlvValue::DestNodeId(value) => value.encode(dst),
            TlvValue::DestSubaddress(value) => value.encode(dst),
            TlvValue::DestTelematicsId(value) => value.encode(dst),
            TlvValue::DestPort(value) => value.encode(dst),
            TlvValue::DisplayTime(value) => value.encode(dst),
            TlvValue::DpfResult(value) => value.encode(dst),
            TlvValue::ItsReplyType(value) => value.encode(dst),
            TlvValue::ItsSessionInfo(value) => value.encode(dst),
            TlvValue::LanguageIndicator(value) => value.encode(dst),
            TlvValue::MessagePayload(value) => value.encode(dst),
            TlvValue::MessageState(value) => value.encode(dst),
            TlvValue::MoreMessagesToSend(value) => value.encode(dst),
            TlvValue::MsAvailabilityStatus(value) => value.encode(dst),
            TlvValue::MsMsgWaitFacilities(value) => value.encode(dst),
            TlvValue::MsValidity(value) => value.encode(dst),
            TlvValue::NetworkErrorCode(value) => value.encode(dst),
            TlvValue::NumberOfMessages(value) => value.encode(dst),
            TlvValue::PayloadType(value) => value.encode(dst),
            TlvValue::PrivacyIndicator(value) => value.encode(dst),
            TlvValue::QosTimeToLive(value) => value.encode(dst),
            TlvValue::ReceiptedMessageId(value) => value.encode(dst),
            TlvValue::SarMsgRefNum(value) => value.encode(dst),
            TlvValue::SarSegmentSeqnum(value) => value.encode(dst),
            TlvValue::SarTotalSegments(value) => value.encode(dst),
            TlvValue::ScInterfaceVersion(value) => value.encode(dst),
            TlvValue::SetDpf(value) => value.encode(dst),
            TlvValue::SmsSignal(value) => value.encode(dst),
            TlvValue::SourceAddrSubunit(value) => value.encode(dst),
            TlvValue::SourceBearerType(value) => value.encode(dst),
            TlvValue::SourceNetworkId(value) => value.encode(dst),
            TlvValue::SourceNetworkType(value) => value.encode(dst),
            TlvValue::SourceNodeId(value) => value.encode(dst),
            TlvValue::SourcePort(value) => value.encode(dst),
            TlvValue::SourceSubaddress(value) => value.encode(dst),
            TlvValue::SourceTelematicsId(value) => value.encode(dst),
            TlvValue::UserMessageReference(value) => value.encode(dst),
            TlvValue::UserResponseCode(value) => value.encode(dst),
            TlvValue::UssdServiceOp(value) => value.encode(dst),
            TlvValue::Other { value, .. } => value.encode(dst),
        }
    }
}

impl crate::encode::owned::Encode for TlvValue {
    fn encode(&self, dst: &mut bytes::BytesMut) {
        match self {
            TlvValue::AdditionalStatusInfoText(value) => value.encode(dst),
            TlvValue::AlertOnMessageDelivery(value) => value.encode(dst),
            TlvValue::BillingIdentification(value) => value.encode(dst),
            TlvValue::BroadcastAreaIdentifier(value) => value.encode(dst),
            TlvValue::BroadcastAreaSuccess(value) => value.encode(dst),
            TlvValue::BroadcastContentTypeInfo(value) => value.encode(dst),
            TlvValue::BroadcastChannelIndicator(value) => value.encode(dst),
            TlvValue::BroadcastContentType(value) => value.encode(dst),
            TlvValue::BroadcastEndTime(value) => value.encode(dst),
            TlvValue::BroadcastErrorStatus(value) => value.encode(dst),
            TlvValue::BroadcastFrequencyInterval(value) => value.encode(dst),
            TlvValue::BroadcastMessageClass(value) => value.encode(dst),
            TlvValue::BroadcastRepNum(value) => value.encode(dst),
            TlvValue::BroadcastServiceGroup(value) => value.encode(dst),
            TlvValue::CallbackNum(value) => value.encode(dst),
            TlvValue::CallbackNumAtag(value) => value.encode(dst),
            TlvValue::CallbackNumPresInd(value) => value.encode(dst),
            TlvValue::CongestionState(value) => value.encode(dst),
            TlvValue::DeliveryFailureReason(value) => value.encode(dst),
            TlvValue::DestAddrNpCountry(value) => value.encode(dst),
            TlvValue::DestAddrNpInformation(value) => value.encode(dst),
            TlvValue::DestAddrNpResolution(value) => value.encode(dst),
            TlvValue::DestAddrSubunit(value) => value.encode(dst),
            TlvValue::DestBearerType(value) => value.encode(dst),
            TlvValue::DestNetworkId(value) => value.encode(dst),
            TlvValue::DestNetworkType(value) => value.encode(dst),
            TlvValue::DestNodeId(value) => value.encode(dst),
            TlvValue::DestSubaddress(value) => value.encode(dst),
            TlvValue::DestTelematicsId(value) => value.encode(dst),
            TlvValue::DestPort(value) => value.encode(dst),
            TlvValue::DisplayTime(value) => value.encode(dst),
            TlvValue::DpfResult(value) => value.encode(dst),
            TlvValue::ItsReplyType(value) => value.encode(dst),
            TlvValue::ItsSessionInfo(value) => value.encode(dst),
            TlvValue::LanguageIndicator(value) => value.encode(dst),
            TlvValue::MessagePayload(value) => value.encode(dst),
            TlvValue::MessageState(value) => value.encode(dst),
            TlvValue::MoreMessagesToSend(value) => value.encode(dst),
            TlvValue::MsAvailabilityStatus(value) => value.encode(dst),
            TlvValue::MsMsgWaitFacilities(value) => value.encode(dst),
            TlvValue::MsValidity(value) => value.encode(dst),
            TlvValue::NetworkErrorCode(value) => value.encode(dst),
            TlvValue::NumberOfMessages(value) => value.encode(dst),
            TlvValue::PayloadType(value) => value.encode(dst),
            TlvValue::PrivacyIndicator(value) => value.encode(dst),
            TlvValue::QosTimeToLive(value) => value.encode(dst),
            TlvValue::ReceiptedMessageId(value) => value.encode(dst),
            TlvValue::SarMsgRefNum(value) => value.encode(dst),
            TlvValue::SarSegmentSeqnum(value) => value.encode(dst),
            TlvValue::SarTotalSegments(value) => value.encode(dst),
            TlvValue::ScInterfaceVersion(value) => value.encode(dst),
            TlvValue::SetDpf(value) => value.encode(dst),
            TlvValue::SmsSignal(value) => value.encode(dst),
            TlvValue::SourceAddrSubunit(value) => value.encode(dst),
            TlvValue::SourceBearerType(value) => value.encode(dst),
            TlvValue::SourceNetworkId(value) => value.encode(dst),
            TlvValue::SourceNetworkType(value) => value.encode(dst),
            TlvValue::SourceNodeId(value) => value.encode(dst),
            TlvValue::SourcePort(value) => value.encode(dst),
            TlvValue::SourceSubaddress(value) => value.encode(dst),
            TlvValue::SourceTelematicsId(value) => value.encode(dst),
            TlvValue::UserMessageReference(value) => value.encode(dst),
            TlvValue::UserResponseCode(value) => value.encode(dst),
            TlvValue::UssdServiceOp(value) => value.encode(dst),
            TlvValue::Other { value, .. } => value.encode(dst),
        }
    }
}

#[derive(Debug, Clone, thiserror::Error)]
pub enum TlvValueDecodeError {
    #[error("AdditionalStatusInfoText decode error: {0}")]
    AdditionalStatusInfoText(#[source] COctetStringDecodeError),
    #[error("AlertOnMessageDelivery decode error: {0}")]
    AlertOnMessageDelivery(#[source] IntegerDecodeError),
    #[error("BillingIdentification decode error: {0}")]
    BillingIdentification(#[source] OctetStringDecodeError),
    #[error("BroadcastAreaIdentifier decode error: {0}")]
    BroadcastAreaIdentifier(#[source] BroadcastAreaIdentifierDecodeError),
    #[error("BroadcastAreaSuccess decode error: {0}")]
    BroadcastAreaSuccess(#[source] IntegerDecodeError),
    #[error("BroadcastContentTypeInfo decode error: {0}")]
    BroadcastContentTypeInfo(#[source] OctetStringDecodeError),
    #[error("BroadcastChannelIndicator decode error: {0}")]
    BroadcastChannelIndicator(#[source] IntegerDecodeError),
    #[error("BroadcastContentType decode error: {0}")]
    BroadcastContentType(#[source] BroadcastContentTypeDecodeError),
    #[error("BroadcastEndTime decode error: {0}")]
    BroadcastEndTime(#[source] OctetStringDecodeError),
    #[error("BroadcastErrorStatus decode error: {0}")]
    BroadcastErrorStatus(#[source] IntegerDecodeError),
    #[error("BroadcastFrequencyInterval decode error: {0}")]
    BroadcastFrequencyInterval(#[source] BroadcastFrequencyIntervalDecodeError),
    #[error("BroadcastMessageClass decode error: {0}")]
    BroadcastMessageClass(#[source] IntegerDecodeError),
    #[error("BroadcastRepNum decode error: {0}")]
    BroadcastRepNum(#[source] BroadcastRepNumDecodeError),
    #[error("BroadcastServiceGroup decode error: {0}")]
    BroadcastServiceGroup(#[source] OctetStringDecodeError),
    #[error("CallbackNum decode error: {0}")]
    CallbackNum(#[source] OctetStringDecodeError),
    #[error("CallbackNumAtag decode error: {0}")]
    CallbackNumAtag(#[source] OctetStringDecodeError),
    #[error("CallbackNumPresInd decode error: {0}")]
    CallbackNumPresInd(#[source] IntegerDecodeError),
    #[error("CongestionState decode error: {0}")]
    CongestionState(#[source] IntegerDecodeError),
    #[error("DeliveryFailureReason decode error: {0}")]
    DeliveryFailureReason(#[source] IntegerDecodeError),
    #[error("DestAddrNpCountry decode error: {0}")]
    DestAddrNpCountry(#[source] OctetStringDecodeError),
    #[error("DestAddrNpInformation decode error: {0}")]
    DestAddrNpInformation(#[source] OctetStringDecodeError),
    #[error("DestAddrNpResolution decode error: {0}")]
    DestAddrNpResolution(#[source] IntegerDecodeError),
    #[error("DestAddrSubunit decode error: {0}")]
    DestAddrSubunit(#[source] IntegerDecodeError),
    #[error("DestBearerType decode error: {0}")]
    DestBearerType(#[source] IntegerDecodeError),
    #[error("DestNetworkId decode error: {0}")]
    DestNetworkId(#[source] COctetStringDecodeError),
    #[error("DestNetworkType decode error: {0}")]
    DestNetworkType(#[source] IntegerDecodeError),
    #[error("DestNodeId decode error: {0}")]
    DestNodeId(#[source] OctetStringDecodeError),
    #[error("DestSubaddress decode error: {0}")]
    DestSubaddress(#[source] SubaddressDecodeError),
    #[error("DestTelematicsId decode error: {0}")]
    DestTelematicsId(#[source] IntegerDecodeError),
    #[error("DestPort decode error: {0}")]
    DestPort(#[source] IntegerDecodeError),
    #[error("DisplayTime decode error: {0}")]
    DisplayTime(#[source] IntegerDecodeError),
    #[error("DpfResult decode error: {0}")]
    DpfResult(#[source] IntegerDecodeError),
    #[error("ItsReplyType decode error: {0}")]
    ItsReplyType(#[source] IntegerDecodeError),
    #[error("ItsSessionInfo decode error: {0}")]
    ItsSessionInfo(#[source] ItsSessionInfoDecodeError),
    #[error("LanguageIndicator decode error: {0}")]
    LanguageIndicator(#[source] IntegerDecodeError),
    #[error("MessagePayload decode error: {0}")]
    MessagePayload(#[source] MessagePayloadDecodeError),
    #[error("MessageState decode error: {0}")]
    MessageState(#[source] IntegerDecodeError),
    #[error("MoreMessagesToSend decode error: {0}")]
    MoreMessagesToSend(#[source] IntegerDecodeError),
    #[error("MsAvailabilityStatus decode error: {0}")]
    MsAvailabilityStatus(#[source] IntegerDecodeError),
    #[error("MsMsgWaitFacilities decode error: {0}")]
    MsMsgWaitFacilities(#[source] IntegerDecodeError),
    #[error("MsValidity decode error: {0}")]
    MsValidity(#[source] MsValidityDecodeError),
    #[error("NetworkErrorCode decode error: {0}")]
    NetworkErrorCode(#[source] NetworkErrorCodeDecodeError),
    #[error("NumberOfMessages decode error: {0}")]
    NumberOfMessages(#[source] IntegerDecodeError),
    #[error("PayloadType decode error: {0}")]
    PayloadType(#[source] IntegerDecodeError),
    #[error("PrivacyIndicator decode error: {0}")]
    PrivacyIndicator(#[source] IntegerDecodeError),
    #[error("QosTimeToLive decode error: {0}")]
    QosTimeToLive(#[source] IntegerDecodeError),
    #[error("ReceiptedMessageId decode error: {0}")]
    ReceiptedMessageId(#[source] COctetStringDecodeError),
    #[error("SarMsgRefNum decode error: {0}")]
    SarMsgRefNum(#[source] IntegerDecodeError),
    #[error("SarSegmentSeqnum decode error: {0}")]
    SarSegmentSeqnum(#[source] IntegerDecodeError),
    #[error("SarTotalSegments decode error: {0}")]
    SarTotalSegments(#[source] IntegerDecodeError),
    #[error("ScInterfaceVersion decode error: {0}")]
    ScInterfaceVersion(#[source] IntegerDecodeError),
    #[error("SetDpf decode error: {0}")]
    SetDpf(#[source] IntegerDecodeError),
    #[error("SmsSignal decode error: {0}")]
    SmsSignal(#[source] IntegerDecodeError),
    #[error("SourceAddrSubunit decode error: {0}")]
    SourceAddrSubunit(#[source] IntegerDecodeError),
    #[error("SourceBearerType decode error: {0}")]
    SourceBearerType(#[source] IntegerDecodeError),
    #[error("SourceNetworkId decode error: {0}")]
    SourceNetworkId(#[source] COctetStringDecodeError),
    #[error("SourceNetworkType decode error: {0}")]
    SourceNetworkType(#[source] IntegerDecodeError),
    #[error("SourceNodeId decode error: {0}")]
    SourceNodeId(#[source] OctetStringDecodeError),
    #[error("SourcePort decode error: {0}")]
    SourcePort(#[source] IntegerDecodeError),
    #[error("SourceSubaddress decode error: {0}")]
    SourceSubaddress(#[source] SubaddressDecodeError),
    #[error("SourceTelematicsId decode error: {0}")]
    SourceTelematicsId(#[source] IntegerDecodeError),
    #[error("UserMessageReference decode error: {0}")]
    UserMessageReference(#[source] UserMessageReferenceDecodeError),
    #[error("UserResponseCode decode error: {0}")]
    UserResponseCode(#[source] IntegerDecodeError),
    #[error("UssdServiceOp decode error: {0}")]
    UssdServiceOp(#[source] IntegerDecodeError),
    #[error("Other decode error: {0}")]
    Other(#[source] AnyOctetStringDecodeError),
}

impl DecodeErrorType for TlvValue {
    type Error = TlvValueDecodeError;
}

impl DecodeWithKey for TlvValue {
    type Key = TlvTag;

    fn decode(
        key: Self::Key,
        src: &mut BytesMut,
        length: usize,
    ) -> Result<(Self, usize), Self::Error> {
        let (value, size) = match key {
            TlvTag::AdditionalStatusInfoText => Decode::decode(src)
                .map_decoded(Self::AdditionalStatusInfoText)
                .map_err(Self::Error::AdditionalStatusInfoText)?,
            TlvTag::AlertOnMessageDelivery => Decode::decode(src)
                .map_decoded(Self::AlertOnMessageDelivery)
                .map_err(Self::Error::AlertOnMessageDelivery)?,
            TlvTag::BillingIdentification => DecodeWithLength::decode(src, length)
                .map_decoded(Self::BillingIdentification)
                .map_err(Self::Error::BillingIdentification)?,
            TlvTag::BroadcastAreaIdentifier => DecodeWithLength::decode(src, length)
                .map_decoded(Self::BroadcastAreaIdentifier)
                .map_err(Self::Error::BroadcastAreaIdentifier)?,
            TlvTag::BroadcastAreaSuccess => Decode::decode(src)
                .map_decoded(Self::BroadcastAreaSuccess)
                .map_err(Self::Error::BroadcastAreaSuccess)?,
            TlvTag::BroadcastContentTypeInfo => DecodeWithLength::decode(src, length)
                .map_decoded(Self::BroadcastContentTypeInfo)
                .map_err(Self::Error::BroadcastContentTypeInfo)?,
            TlvTag::BroadcastChannelIndicator => Decode::decode(src)
                .map_decoded(Self::BroadcastChannelIndicator)
                .map_err(Self::Error::BroadcastChannelIndicator)?,
            TlvTag::BroadcastContentType => Decode::decode(src)
                .map_decoded(Self::BroadcastContentType)
                .map_err(Self::Error::BroadcastContentType)?,
            TlvTag::BroadcastEndTime => DecodeWithLength::decode(src, length)
                .map_decoded(Self::BroadcastEndTime)
                .map_err(Self::Error::BroadcastEndTime)?,
            TlvTag::BroadcastErrorStatus => Decode::decode(src)
                .map_decoded(Self::BroadcastErrorStatus)
                .map_err(Self::Error::BroadcastErrorStatus)?,
            TlvTag::BroadcastFrequencyInterval => Decode::decode(src)
                .map_decoded(Self::BroadcastFrequencyInterval)
                .map_err(Self::Error::BroadcastFrequencyInterval)?,
            TlvTag::BroadcastMessageClass => Decode::decode(src)
                .map_decoded(Self::BroadcastMessageClass)
                .map_err(Self::Error::BroadcastMessageClass)?,
            TlvTag::BroadcastRepNum => Decode::decode(src)
                .map_decoded(Self::BroadcastRepNum)
                .map_err(Self::Error::BroadcastRepNum)?,
            TlvTag::BroadcastServiceGroup => DecodeWithLength::decode(src, length)
                .map_decoded(Self::BroadcastServiceGroup)
                .map_err(Self::Error::BroadcastServiceGroup)?,
            TlvTag::CallbackNum => DecodeWithLength::decode(src, length)
                .map_decoded(Self::CallbackNum)
                .map_err(Self::Error::CallbackNum)?,
            TlvTag::CallbackNumAtag => DecodeWithLength::decode(src, length)
                .map_decoded(Self::CallbackNumAtag)
                .map_err(Self::Error::CallbackNumAtag)?,
            TlvTag::CallbackNumPresInd => Decode::decode(src)
                .map_decoded(Self::CallbackNumPresInd)
                .map_err(Self::Error::CallbackNumPresInd)?,
            TlvTag::CongestionState => Decode::decode(src)
                .map_decoded(Self::CongestionState)
                .map_err(Self::Error::CongestionState)?,
            TlvTag::DeliveryFailureReason => Decode::decode(src)
                .map_decoded(Self::DeliveryFailureReason)
                .map_err(Self::Error::DeliveryFailureReason)?,
            TlvTag::DestAddrNpCountry => DecodeWithLength::decode(src, length)
                .map_decoded(Self::DestAddrNpCountry)
                .map_err(Self::Error::DestAddrNpCountry)?,
            TlvTag::DestAddrNpInformation => DecodeWithLength::decode(src, length)
                .map_decoded(Self::DestAddrNpInformation)
                .map_err(Self::Error::DestAddrNpInformation)?,
            TlvTag::DestAddrNpResolution => Decode::decode(src)
                .map_decoded(Self::DestAddrNpResolution)
                .map_err(Self::Error::DestAddrNpResolution)?,
            TlvTag::DestAddrSubunit => Decode::decode(src)
                .map_decoded(Self::DestAddrSubunit)
                .map_err(Self::Error::DestAddrSubunit)?,
            TlvTag::DestBearerType => Decode::decode(src)
                .map_decoded(Self::DestBearerType)
                .map_err(Self::Error::DestBearerType)?,
            TlvTag::DestNetworkId => Decode::decode(src)
                .map_decoded(Self::DestNetworkId)
                .map_err(Self::Error::DestNetworkId)?,
            TlvTag::DestNetworkType => Decode::decode(src)
                .map_decoded(Self::DestNetworkType)
                .map_err(Self::Error::DestNetworkType)?,
            TlvTag::DestNodeId => DecodeWithLength::decode(src, length)
                .map_decoded(Self::DestNodeId)
                .map_err(Self::Error::DestNodeId)?,
            TlvTag::DestSubaddress => DecodeWithLength::decode(src, length)
                .map_decoded(Self::DestSubaddress)
                .map_err(Self::Error::DestSubaddress)?,
            TlvTag::DestTelematicsId => Decode::decode(src)
                .map_decoded(Self::DestTelematicsId)
                .map_err(Self::Error::DestTelematicsId)?,
            TlvTag::DestPort => Decode::decode(src)
                .map_decoded(Self::DestPort)
                .map_err(Self::Error::DestPort)?,
            TlvTag::DisplayTime => Decode::decode(src)
                .map_decoded(Self::DisplayTime)
                .map_err(Self::Error::DisplayTime)?,
            TlvTag::DpfResult => Decode::decode(src)
                .map_decoded(Self::DpfResult)
                .map_err(Self::Error::DpfResult)?,
            TlvTag::ItsReplyType => Decode::decode(src)
                .map_decoded(Self::ItsReplyType)
                .map_err(Self::Error::ItsReplyType)?,
            TlvTag::ItsSessionInfo => Decode::decode(src)
                .map_decoded(Self::ItsSessionInfo)
                .map_err(Self::Error::ItsSessionInfo)?,
            TlvTag::LanguageIndicator => Decode::decode(src)
                .map_decoded(Self::LanguageIndicator)
                .map_err(Self::Error::LanguageIndicator)?,
            TlvTag::MessagePayload => DecodeWithLength::decode(src, length)
                .map_decoded(Self::MessagePayload)
                .map_err(Self::Error::MessagePayload)?,
            TlvTag::MessageState => Decode::decode(src)
                .map_decoded(Self::MessageState)
                .map_err(Self::Error::MessageState)?,
            TlvTag::MoreMessagesToSend => Decode::decode(src)
                .map_decoded(Self::MoreMessagesToSend)
                .map_err(Self::Error::MoreMessagesToSend)?,
            TlvTag::MsAvailabilityStatus => Decode::decode(src)
                .map_decoded(Self::MsAvailabilityStatus)
                .map_err(Self::Error::MsAvailabilityStatus)?,
            TlvTag::MsMsgWaitFacilities => Decode::decode(src)
                .map_decoded(Self::MsMsgWaitFacilities)
                .map_err(Self::Error::MsMsgWaitFacilities)?,
            TlvTag::MsValidity => DecodeWithLength::decode(src, length)
                .map_decoded(Self::MsValidity)
                .map_err(Self::Error::MsValidity)?,
            TlvTag::NetworkErrorCode => Decode::decode(src)
                .map_decoded(Self::NetworkErrorCode)
                .map_err(Self::Error::NetworkErrorCode)?,
            TlvTag::NumberOfMessages => Decode::decode(src)
                .map_decoded(Self::NumberOfMessages)
                .map_err(Self::Error::NumberOfMessages)?,
            TlvTag::PayloadType => Decode::decode(src)
                .map_decoded(Self::PayloadType)
                .map_err(Self::Error::PayloadType)?,
            TlvTag::PrivacyIndicator => Decode::decode(src)
                .map_decoded(Self::PrivacyIndicator)
                .map_err(Self::Error::PrivacyIndicator)?,
            TlvTag::QosTimeToLive => Decode::decode(src)
                .map_decoded(Self::QosTimeToLive)
                .map_err(Self::Error::QosTimeToLive)?,
            TlvTag::ReceiptedMessageId => Decode::decode(src)
                .map_decoded(Self::ReceiptedMessageId)
                .map_err(Self::Error::ReceiptedMessageId)?,
            TlvTag::SarMsgRefNum => Decode::decode(src)
                .map_decoded(Self::SarMsgRefNum)
                .map_err(Self::Error::SarMsgRefNum)?,
            TlvTag::SarSegmentSeqnum => Decode::decode(src)
                .map_decoded(Self::SarSegmentSeqnum)
                .map_err(Self::Error::SarSegmentSeqnum)?,
            TlvTag::SarTotalSegments => Decode::decode(src)
                .map_decoded(Self::SarTotalSegments)
                .map_err(Self::Error::SarTotalSegments)?,
            TlvTag::ScInterfaceVersion => Decode::decode(src)
                .map_decoded(Self::ScInterfaceVersion)
                .map_err(Self::Error::ScInterfaceVersion)?,
            TlvTag::SetDpf => Decode::decode(src)
                .map_decoded(Self::SetDpf)
                .map_err(Self::Error::SetDpf)?,
            TlvTag::SmsSignal => Decode::decode(src)
                .map_decoded(Self::SmsSignal)
                .map_err(Self::Error::SmsSignal)?,
            TlvTag::SourceAddrSubunit => Decode::decode(src)
                .map_decoded(Self::SourceAddrSubunit)
                .map_err(Self::Error::SourceAddrSubunit)?,
            TlvTag::SourceBearerType => Decode::decode(src)
                .map_decoded(Self::SourceBearerType)
                .map_err(Self::Error::SourceBearerType)?,
            TlvTag::SourceNetworkId => Decode::decode(src)
                .map_decoded(Self::SourceNetworkId)
                .map_err(Self::Error::SourceNetworkId)?,
            TlvTag::SourceNetworkType => Decode::decode(src)
                .map_decoded(Self::SourceNetworkType)
                .map_err(Self::Error::SourceNetworkType)?,
            TlvTag::SourceNodeId => DecodeWithLength::decode(src, length)
                .map_decoded(Self::SourceNodeId)
                .map_err(Self::Error::SourceNodeId)?,
            TlvTag::SourcePort => Decode::decode(src)
                .map_decoded(Self::SourcePort)
                .map_err(Self::Error::SourcePort)?,
            TlvTag::SourceSubaddress => DecodeWithLength::decode(src, length)
                .map_decoded(Self::SourceSubaddress)
                .map_err(Self::Error::SourceSubaddress)?,
            TlvTag::SourceTelematicsId => Decode::decode(src)
                .map_decoded(Self::SourceTelematicsId)
                .map_err(Self::Error::SourceTelematicsId)?,
            TlvTag::UserMessageReference => Decode::decode(src)
                .map_decoded(Self::UserMessageReference)
                .map_err(Self::Error::UserMessageReference)?,
            TlvTag::UserResponseCode => Decode::decode(src)
                .map_decoded(Self::UserResponseCode)
                .map_err(Self::Error::UserResponseCode)?,
            TlvTag::UssdServiceOp => Decode::decode(src)
                .map_decoded(Self::UssdServiceOp)
                .map_err(Self::Error::UssdServiceOp)?,
            TlvTag::Other(other) => DecodeWithLength::decode(src, length)
                .map_decoded(|value| TlvValue::Other {
                    tag: TlvTag::Other(other),
                    value,
                })
                .map_err(Self::Error::Other)?,
        };

        Ok((value, size))
    }
}
