use rusmpp_macros::TlvValue;

use crate::{
    tlvs::{
        TlvTag,
        borrowed::{Tlv, TlvValue},
    },
    types::borrowed::{AnyOctetString, OctetString},
    values::{borrowed::*, *},
};

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, TlvValue)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(bound(deserialize = "'de: 'a")))]
pub enum BroadcastRequestTlvValue<'a> {
    BroadcastAreaIdentifier(BroadcastAreaIdentifier<'a>),
    BroadcastContentType(BroadcastContentType),
    BroadcastFrequencyInterval(BroadcastFrequencyInterval),
    BroadcastRepNum(BroadcastRepNum),
    AlertOnMessageDelivery(AlertOnMessageDelivery),
    BroadcastChannelIndicator(BroadcastChannelIndicator),
    BroadcastContentTypeInfo(OctetString<'a, 0, 255>),
    BroadcastMessageClass(BroadcastMessageClass),
    BroadcastServiceGroup(OctetString<'a, 1, 255>),
    CallbackNum(OctetString<'a, 4, 19>),
    CallbackNumAtag(OctetString<'a, 0, 65>),
    CallbackNumPresInd(CallbackNumPresInd),
    DestAddrSubunit(AddrSubunit),
    DestSubaddress(Subaddress<'a>),
    DestPort(u16),
    DisplayTime(DisplayTime),
    LanguageIndicator(LanguageIndicator),
    MessagePayload(MessagePayload<'a>),
    MsValidity(MsValidity),
    PayloadType(PayloadType),
    PrivacyIndicator(PrivacyIndicator),
    SmsSignal(u16),
    SourceAddrSubunit(AddrSubunit),
    SourcePort(u16),
    SourceSubaddress(Subaddress<'a>),
    UserMessageReference(UserMessageReference),
    Other {
        tag: TlvTag,
        value: AnyOctetString<'a>,
    },
}
