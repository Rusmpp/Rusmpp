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

// TODO: impl serde

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

#[cfg(test)]
mod tests {
    use super::*;

    mod encode {
        use super::*;

        mod borrowed {
            use super::*;

            #[test]
            fn ok() {
                use crate::encode::Encode;

                let udh = Udh::new(ConcatenatedShortMessage16Bit::new(0x1234, 3, 1).unwrap());

                let expected = [
                    0x06, // UDH length (following bytes = 6)
                    0x08, // UDH ID: Concatenated Short Messages, 16-bit reference number
                    0x04, // IE Data Length = 4 bytes
                    0x12, // Ref high
                    0x34, // Ref low
                    0x03, // Total parts
                    0x01, // Part number
                ];

                let mut buf = [0u8; 24];
                let size = udh.encode(&mut buf);

                assert_eq!(size, 7);
                assert_eq!(&buf[..size], &expected);

                let udh = Udh::new(ConcatenatedShortMessage8Bit::new(0x12, 3, 1).unwrap());
                let expected = [
                    0x05, // UDH length (following bytes = 5)
                    0x00, // UDH ID: Concatenated Short Messages, 8-bit reference number
                    0x03, // IE Data Length = 3 bytes
                    0x12, // Ref
                    0x03, // Total parts
                    0x01, // Part number
                ];

                let mut buf = [0u8; 24];
                let size = udh.encode(&mut buf);

                assert_eq!(size, 6);
                assert_eq!(&buf[..size], &expected);
            }
        }

        #[cfg(feature = "alloc")]
        mod owned {
            use super::*;

            #[test]
            fn ok() {
                use bytes::BytesMut;

                use crate::encode::{Length, owned::Encode};

                let udh = Udh::new(ConcatenatedShortMessage16Bit::new(0x1234, 3, 1).unwrap());

                let expected = [
                    0x06, // UDH length (following bytes = 6)
                    0x08, // UDH ID: Concatenated Short Messages, 16-bit reference number
                    0x04, // IE Data Length = 4 bytes
                    0x12, // Ref high
                    0x34, // Ref low
                    0x03, // Total parts
                    0x01, // Part number
                ];

                let mut buf = BytesMut::with_capacity(Length::length(&udh));

                udh.encode(&mut buf);

                let encoded = buf.split_to(Length::length(&udh));

                assert_eq!(encoded.len(), 7);
                assert_eq!(&encoded[..], &expected);

                let udh = Udh::new(ConcatenatedShortMessage8Bit::new(0x12, 3, 1).unwrap());
                let expected = [
                    0x05, // UDH length (following bytes = 5)
                    0x00, // UDH ID: Concatenated Short Messages, 8-bit reference number
                    0x03, // IE Data Length = 3 bytes
                    0x12, // Ref
                    0x03, // Total parts
                    0x01, // Part number
                ];

                let mut buf = BytesMut::with_capacity(Length::length(&udh));

                udh.encode(&mut buf);

                let encoded = buf.split_to(Length::length(&udh));

                assert_eq!(encoded.len(), 6);
                assert_eq!(&encoded[..], &expected);
            }
        }
    }

    mod decode {
        use crate::decode::borrowed::Decode;

        use super::*;

        #[test]
        fn ok() {
            let bytes = &[
                0x06, // UDH length (following bytes = 6)
                0x08, // UDH ID: Concatenated Short Messages, 16-bit reference number
                0x04, // IE Data Length = 4 bytes
                0x12, // Ref high
                0x34, // Ref low
                0x03, // Total parts
                0x01, // Part number
                0x00, // Extra bytes
                0x00,
            ];

            let (udh, size) = <Udh as Decode>::decode(bytes).unwrap();

            assert_eq!(size, 7);
            assert_eq!(
                udh,
                Udh::new(ConcatenatedShortMessage16Bit::new(0x1234, 3, 1).unwrap())
            );
            assert_eq!(&bytes[size..], &[0x00, 0x00][..]);

            let bytes = &[
                0x05, // UDH length (following bytes = 5)
                0x00, // UDH ID: Concatenated Short Messages, 8-bit reference number
                0x03, // IE Data Length = 3 bytes
                0x12, // Ref
                0x03, // Total parts
                0x01, // Part number
                0x00, // Extra bytes
                0x00,
            ];

            let (udh, size) = <Udh as Decode>::decode(bytes).unwrap();
            assert_eq!(size, 6);
            assert_eq!(
                udh,
                Udh::new(ConcatenatedShortMessage8Bit::new(0x12, 3, 1).unwrap())
            );
            assert_eq!(&bytes[size..], &[0x00, 0x00][..]);
        }
    }
}
