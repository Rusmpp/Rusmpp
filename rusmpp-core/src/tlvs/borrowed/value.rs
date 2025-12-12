use crate::{
    CommandStatus,
    decode::{
        DecodeError, DecodeResultExt,
        borrowed::{Decode, DecodeWithKey, DecodeWithLength},
    },
    encode::Length,
    tlvs::TlvTag,
    types::borrowed::{AnyOctetString, COctetString, OctetString},
    values::{borrowed::*, *},
};

/// See module level documentation.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[cfg_attr(feature = "arbitrary", derive(::arbitrary::Arbitrary))]
#[cfg_attr(feature = "serde", derive(::serde::Serialize))]
pub enum TlvValue<'a> {
    AdditionalStatusInfoText(COctetString<'a, 1, 256>),
    AlertOnMessageDelivery(AlertOnMessageDelivery),
    BillingIdentification(OctetString<'a, 0, 1024>),
    /// Identifies one or more target Broadcast Area(s) for which the
    /// status information applies.
    ///
    /// The number of instances of this parameter will be exactly equal
    /// to the number of occurrences of the broadcast_area_identifiers
    /// parameter in the corresponding broadcast_sm.
    BroadcastAreaIdentifier(BroadcastAreaIdentifier<'a>),
    /// The success rate indicator, defined as the ratio of the
    /// number of BTSs that accepted the message and the total
    /// number of BTSs that should have accepted the message, for
    /// a particular broadcast_area_identifier.
    BroadcastAreaSuccess(BroadcastAreaSuccess),
    BroadcastContentTypeInfo(OctetString<'a, 0, 255>),
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
    BroadcastEndTime(OctetString<'a, 0, 17>),
    BroadcastErrorStatus(CommandStatus),
    /// This field indicates the frequency interval at which
    /// the broadcasts of a message should be repeated.
    BroadcastFrequencyInterval(BroadcastFrequencyInterval),
    BroadcastMessageClass(BroadcastMessageClass),
    /// This field indicates the number of repeated
    /// broadcasts of a message requested by the submitter.
    BroadcastRepNum(BroadcastRepNum),
    BroadcastServiceGroup(OctetString<'a, 1, 255>),
    CallbackNum(OctetString<'a, 4, 19>),
    CallbackNumAtag(OctetString<'a, 0, 65>),
    CallbackNumPresInd(CallbackNumPresInd),
    CongestionState(CongestionState),
    DeliveryFailureReason(DeliveryFailureReason),
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
    DpfResult(DpfResult),
    ItsReplyType(ItsReplyType),
    ItsSessionInfo(ItsSessionInfo),
    LanguageIndicator(LanguageIndicator),
    MessagePayload(MessagePayload<'a>),
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
    ReceiptedMessageId(COctetString<'a, 1, 65>),
    SarMsgRefNum(u16),
    SarSegmentSeqnum(u8),
    SarTotalSegments(u8),
    ScInterfaceVersion(InterfaceVersion),
    SetDpf(SetDpf),
    /// Encoded as per [CMT-136]
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

impl TlvValue<'_> {
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

impl Length for TlvValue<'_> {
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

impl crate::encode::Encode for TlvValue<'_> {
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

#[cfg(feature = "alloc")]
impl crate::encode::owned::Encode for TlvValue<'_> {
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

impl<'a> DecodeWithKey<'a> for TlvValue<'a> {
    type Key = TlvTag;

    fn decode(key: Self::Key, src: &'a [u8], length: usize) -> Result<(Self, usize), DecodeError> {
        let (value, size) = match key {
            TlvTag::AdditionalStatusInfoText => {
                Decode::decode(src).map_decoded(Self::AdditionalStatusInfoText)?
            }
            TlvTag::AlertOnMessageDelivery => {
                Decode::decode(src).map_decoded(Self::AlertOnMessageDelivery)?
            }
            TlvTag::BillingIdentification => {
                DecodeWithLength::decode(src, length).map_decoded(Self::BillingIdentification)?
            }
            TlvTag::BroadcastAreaIdentifier => {
                DecodeWithLength::decode(src, length).map_decoded(Self::BroadcastAreaIdentifier)?
            }
            TlvTag::BroadcastAreaSuccess => {
                Decode::decode(src).map_decoded(Self::BroadcastAreaSuccess)?
            }
            TlvTag::BroadcastContentTypeInfo => {
                DecodeWithLength::decode(src, length).map_decoded(Self::BroadcastContentTypeInfo)?
            }
            TlvTag::BroadcastChannelIndicator => {
                Decode::decode(src).map_decoded(Self::BroadcastChannelIndicator)?
            }
            TlvTag::BroadcastContentType => {
                Decode::decode(src).map_decoded(Self::BroadcastContentType)?
            }
            TlvTag::BroadcastEndTime => {
                DecodeWithLength::decode(src, length).map_decoded(Self::BroadcastEndTime)?
            }
            TlvTag::BroadcastErrorStatus => {
                Decode::decode(src).map_decoded(Self::BroadcastErrorStatus)?
            }
            TlvTag::BroadcastFrequencyInterval => {
                Decode::decode(src).map_decoded(Self::BroadcastFrequencyInterval)?
            }
            TlvTag::BroadcastMessageClass => {
                Decode::decode(src).map_decoded(Self::BroadcastMessageClass)?
            }
            TlvTag::BroadcastRepNum => Decode::decode(src).map_decoded(Self::BroadcastRepNum)?,
            TlvTag::BroadcastServiceGroup => {
                DecodeWithLength::decode(src, length).map_decoded(Self::BroadcastServiceGroup)?
            }
            TlvTag::CallbackNum => {
                DecodeWithLength::decode(src, length).map_decoded(Self::CallbackNum)?
            }
            TlvTag::CallbackNumAtag => {
                DecodeWithLength::decode(src, length).map_decoded(Self::CallbackNumAtag)?
            }
            TlvTag::CallbackNumPresInd => {
                Decode::decode(src).map_decoded(Self::CallbackNumPresInd)?
            }
            TlvTag::CongestionState => Decode::decode(src).map_decoded(Self::CongestionState)?,
            TlvTag::DeliveryFailureReason => {
                Decode::decode(src).map_decoded(Self::DeliveryFailureReason)?
            }
            TlvTag::DestAddrNpCountry => {
                DecodeWithLength::decode(src, length).map_decoded(Self::DestAddrNpCountry)?
            }
            TlvTag::DestAddrNpInformation => {
                DecodeWithLength::decode(src, length).map_decoded(Self::DestAddrNpInformation)?
            }
            TlvTag::DestAddrNpResolution => {
                Decode::decode(src).map_decoded(Self::DestAddrNpResolution)?
            }
            TlvTag::DestAddrSubunit => Decode::decode(src).map_decoded(Self::DestAddrSubunit)?,
            TlvTag::DestBearerType => Decode::decode(src).map_decoded(Self::DestBearerType)?,
            TlvTag::DestNetworkId => Decode::decode(src).map_decoded(Self::DestNetworkId)?,
            TlvTag::DestNetworkType => Decode::decode(src).map_decoded(Self::DestNetworkType)?,
            TlvTag::DestNodeId => {
                DecodeWithLength::decode(src, length).map_decoded(Self::DestNodeId)?
            }
            TlvTag::DestSubaddress => {
                DecodeWithLength::decode(src, length).map_decoded(Self::DestSubaddress)?
            }
            TlvTag::DestTelematicsId => Decode::decode(src).map_decoded(Self::DestTelematicsId)?,
            TlvTag::DestPort => Decode::decode(src).map_decoded(Self::DestPort)?,
            TlvTag::DisplayTime => Decode::decode(src).map_decoded(Self::DisplayTime)?,
            TlvTag::DpfResult => Decode::decode(src).map_decoded(Self::DpfResult)?,
            TlvTag::ItsReplyType => Decode::decode(src).map_decoded(Self::ItsReplyType)?,
            TlvTag::ItsSessionInfo => Decode::decode(src).map_decoded(Self::ItsSessionInfo)?,
            TlvTag::LanguageIndicator => {
                Decode::decode(src).map_decoded(Self::LanguageIndicator)?
            }
            TlvTag::MessagePayload => {
                DecodeWithLength::decode(src, length).map_decoded(Self::MessagePayload)?
            }
            TlvTag::MessageState => Decode::decode(src).map_decoded(Self::MessageState)?,
            TlvTag::MoreMessagesToSend => {
                Decode::decode(src).map_decoded(Self::MoreMessagesToSend)?
            }
            TlvTag::MsAvailabilityStatus => {
                Decode::decode(src).map_decoded(Self::MsAvailabilityStatus)?
            }
            TlvTag::MsMsgWaitFacilities => {
                Decode::decode(src).map_decoded(Self::MsMsgWaitFacilities)?
            }
            TlvTag::MsValidity => {
                DecodeWithLength::decode(src, length).map_decoded(Self::MsValidity)?
            }
            TlvTag::NetworkErrorCode => Decode::decode(src).map_decoded(Self::NetworkErrorCode)?,
            TlvTag::NumberOfMessages => Decode::decode(src).map_decoded(Self::NumberOfMessages)?,
            TlvTag::PayloadType => Decode::decode(src).map_decoded(Self::PayloadType)?,
            TlvTag::PrivacyIndicator => Decode::decode(src).map_decoded(Self::PrivacyIndicator)?,
            TlvTag::QosTimeToLive => Decode::decode(src).map_decoded(Self::QosTimeToLive)?,
            TlvTag::ReceiptedMessageId => {
                Decode::decode(src).map_decoded(Self::ReceiptedMessageId)?
            }
            TlvTag::SarMsgRefNum => Decode::decode(src).map_decoded(Self::SarMsgRefNum)?,
            TlvTag::SarSegmentSeqnum => Decode::decode(src).map_decoded(Self::SarSegmentSeqnum)?,
            TlvTag::SarTotalSegments => Decode::decode(src).map_decoded(Self::SarTotalSegments)?,
            TlvTag::ScInterfaceVersion => {
                Decode::decode(src).map_decoded(Self::ScInterfaceVersion)?
            }
            TlvTag::SetDpf => Decode::decode(src).map_decoded(Self::SetDpf)?,
            TlvTag::SmsSignal => Decode::decode(src).map_decoded(Self::SmsSignal)?,
            TlvTag::SourceAddrSubunit => {
                Decode::decode(src).map_decoded(Self::SourceAddrSubunit)?
            }
            TlvTag::SourceBearerType => Decode::decode(src).map_decoded(Self::SourceBearerType)?,
            TlvTag::SourceNetworkId => Decode::decode(src).map_decoded(Self::SourceNetworkId)?,
            TlvTag::SourceNetworkType => {
                Decode::decode(src).map_decoded(Self::SourceNetworkType)?
            }
            TlvTag::SourceNodeId => {
                DecodeWithLength::decode(src, length).map_decoded(Self::SourceNodeId)?
            }
            TlvTag::SourcePort => Decode::decode(src).map_decoded(Self::SourcePort)?,
            TlvTag::SourceSubaddress => {
                DecodeWithLength::decode(src, length).map_decoded(Self::SourceSubaddress)?
            }
            TlvTag::SourceTelematicsId => {
                Decode::decode(src).map_decoded(Self::SourceTelematicsId)?
            }
            TlvTag::UserMessageReference => {
                Decode::decode(src).map_decoded(Self::UserMessageReference)?
            }
            TlvTag::UserResponseCode => Decode::decode(src).map_decoded(Self::UserResponseCode)?,
            TlvTag::UssdServiceOp => Decode::decode(src).map_decoded(Self::UssdServiceOp)?,
            TlvTag::Other(other) => {
                DecodeWithLength::decode(src, length).map_decoded(|value| TlvValue::Other {
                    tag: TlvTag::Other(other),
                    value,
                })?
            }
        };

        Ok((value, size))
    }
}
