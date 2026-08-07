use rusmpp_macros::TlvValue;

use crate::{
    CommandStatus,
    tlvs::{
        TlvTag,
        owned::{Tlv, TlvValue},
    },
    types::owned::AnyOctetString,
    values::owned::*,
};

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, TlvValue)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]

pub enum BroadcastResponseTlvValue {
    BroadcastErrorStatus(CommandStatus),
    BroadcastAreaIdentifier(BroadcastAreaIdentifier),
    Other { tag: TlvTag, value: AnyOctetString },
}
