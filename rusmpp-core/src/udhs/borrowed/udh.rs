use rusmpp_macros::Rusmpp;

use crate::{
    Sealed,
    decode::{
        DecodeError, DecodeResultExt,
        borrowed::{Decode, DecodeWithKey, DecodeWithLength},
    },
    encode::Length,
    types::borrowed::AnyOctetString,
    udhs::{
        UdhId,
        concatenation::{ConcatenatedShortMessage8Bit, ConcatenatedShortMessage16Bit},
    },
};

// TODO: impl serde,
// TODO: tests for Udh like the owned version.

/// User Data Header (UDH).
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Rusmpp)]
#[rusmpp(decode = borrowed, test = skip)]
pub struct Udh<'a> {
    /// UDH length (excluding the length field itself).
    length: u8,
    /// UDH identifier.
    id: UdhId,
    /// UDH value.
    // XXX: the length of the value is `self.length` - `self.id.length()`
    // `self.id.length()` is always `1`
    #[rusmpp(key = id, length = length - 1)]
    value: Option<UdhValue<'a>>,
}

impl<'a> Udh<'a> {
    /// Creates a new [`Udh`] from the given [`UdhValue`].
    pub fn new(value: impl Into<UdhValue<'a>>) -> Self {
        let value = value.into();
        let id = value.id();
        let length = value.length() as u8 + id.length() as u8;

        Self {
            id,
            length,
            value: Some(value),
        }
    }

    /// Returns the UDH identifier.
    pub const fn id(&self) -> UdhId {
        self.id
    }

    /// Returns the UDH length (excluding the length field itself).
    pub const fn length(&self) -> u8 {
        self.length
    }

    /// Returns a reference to the UDH value.
    pub const fn value(&self) -> Option<&UdhValue<'_>> {
        self.value.as_ref()
    }
}

impl<'a> From<UdhValue<'a>> for Udh<'a> {
    fn from(value: UdhValue<'a>) -> Self {
        Self::new(value)
    }
}

/// User Data Header (UDH) value.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(bound(deserialize = "'de: 'a")))]
pub enum UdhValue<'a> {
    /// 8-bit Concatenated Short Message UDH.
    ConcatenatedShortMessage8Bit(ConcatenatedShortMessage8Bit),
    /// 16-bit Concatenated Short Message UDH.
    ConcatenatedShortMessage16Bit(ConcatenatedShortMessage16Bit),
    /// Other UDH types.
    Other {
        udh_id: UdhId,
        value: AnyOctetString<'a>,
    },
}

impl<'a> UdhValue<'a> {
    /// Returns the UDH identifier.
    pub const fn id(&self) -> UdhId {
        match self {
            UdhValue::ConcatenatedShortMessage8Bit(_) => UdhId::ConcatenatedShortMessages8Bit,
            UdhValue::ConcatenatedShortMessage16Bit(_) => UdhId::ConcatenatedShortMessages16Bit,
            UdhValue::Other { udh_id, .. } => *udh_id,
        }
    }
}

impl<'a> Sealed for UdhValue<'a> {}

impl<'a> Length for UdhValue<'a> {
    fn length(&self) -> usize {
        match self {
            UdhValue::ConcatenatedShortMessage8Bit(udh) => udh.length(),
            UdhValue::ConcatenatedShortMessage16Bit(udh) => udh.length(),
            UdhValue::Other { value, .. } => value.length(),
        }
    }
}

impl<'a> crate::encode::Encode for UdhValue<'a> {
    fn encode(&self, dst: &mut [u8]) -> usize {
        match self {
            UdhValue::ConcatenatedShortMessage8Bit(udh) => udh.encode(dst),
            UdhValue::ConcatenatedShortMessage16Bit(udh) => udh.encode(dst),
            UdhValue::Other { value, .. } => value.encode(dst),
        }
    }
}

#[cfg(feature = "alloc")]
impl crate::encode::owned::Encode for UdhValue<'_> {
    fn encode(&self, dst: &mut bytes::BytesMut) {
        match self {
            UdhValue::ConcatenatedShortMessage8Bit(udh) => udh.encode(dst),
            UdhValue::ConcatenatedShortMessage16Bit(udh) => udh.encode(dst),
            UdhValue::Other { value, .. } => value.encode(dst),
        }
    }
}

impl<'a> DecodeWithKey<'a> for UdhValue<'a> {
    type Key = UdhId;

    fn decode(key: Self::Key, src: &'a [u8], length: usize) -> Result<(Self, usize), DecodeError> {
        let (value, size) = match key {
            UdhId::ConcatenatedShortMessages8Bit => {
                Decode::decode(src).map_decoded(Self::ConcatenatedShortMessage8Bit)?
            }
            UdhId::ConcatenatedShortMessages16Bit => {
                Decode::decode(src).map_decoded(Self::ConcatenatedShortMessage16Bit)?
            }
            other => {
                DecodeWithLength::decode(src, length).map_decoded(|value| UdhValue::Other {
                    udh_id: other,
                    value,
                })?
            }
        };

        Ok((value, size))
    }
}
