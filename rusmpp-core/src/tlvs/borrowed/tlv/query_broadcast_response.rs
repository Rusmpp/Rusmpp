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
pub enum QueryBroadcastResponseTlvValue<'a> {
    MessageState(MessageState),
    BroadcastAreaIdentifier(BroadcastAreaIdentifier<'a>),
    BroadcastAreaSuccess(BroadcastAreaSuccess),
    BroadcastEndTime(OctetString<'a, 0, 17>),
    UserMessageReference(UserMessageReference),
    Other {
        tag: TlvTag,
        value: AnyOctetString<'a>,
    },
}
