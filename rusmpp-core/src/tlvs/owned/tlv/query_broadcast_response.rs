use rusmpp_macros::TlvValue;

use crate::{
    tlvs::{
        TlvTag,
        owned::{Tlv, TlvValue},
    },
    types::owned::{AnyOctetString, OctetString},
    values::{owned::*, *},
};

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, TlvValue)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]

pub enum QueryBroadcastResponseTlvValue {
    MessageState(MessageState),
    BroadcastAreaIdentifier(BroadcastAreaIdentifier),
    BroadcastAreaSuccess(BroadcastAreaSuccess),
    BroadcastEndTime(OctetString<0, 17>),
    UserMessageReference(UserMessageReference),
    Other { tag: TlvTag, value: AnyOctetString },
}
