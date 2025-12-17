use rusmpp_macros::Rusmpp;

use crate::{
    decode::{
        DecodeError, DecodeResultExt,
        owned::{Decode, DecodeWithKey, DecodeWithLength},
    },
    encode::Length,
    types::owned::AnyOctetString,
    udhs::{
        UdhId,
        concatenation::{ConcatenatedShortMessage8Bit, ConcatenatedShortMessage16Bit},
    },
};

/// User Data Header (UDH).
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Rusmpp)]
#[rusmpp(decode = skip, test = skip)]
pub struct Udh {
    /// UDH length (excluding the length field itself).
    length: u8,
    /// UDH identifier.
    id: UdhId,
    /// UDH value.
    value: Option<UdhValue>,
}

impl Udh {
    /// Creates a new [`Udh`] from the given [`UdhValue`].
    pub fn new(value: impl Into<UdhValue>) -> Self {
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
    pub const fn value(&self) -> Option<&UdhValue> {
        self.value.as_ref()
    }
}

impl From<UdhValue> for Udh {
    fn from(value: UdhValue) -> Self {
        Self::new(value)
    }
}

/// User Data Header (UDH) value.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum UdhValue {
    /// 8-bit Concatenated Short Message UDH.
    ConcatenatedShortMessage8Bit(ConcatenatedShortMessage8Bit),
    /// 16-bit Concatenated Short Message UDH.
    ConcatenatedShortMessage16Bit(ConcatenatedShortMessage16Bit),
    /// Other UDH types.
    Other {
        udh_id: UdhId,
        value: AnyOctetString,
    },
}

impl UdhValue {
    /// Returns the UDH identifier.
    pub const fn id(&self) -> UdhId {
        match self {
            UdhValue::ConcatenatedShortMessage8Bit(_) => UdhId::ConcatenatedShortMessages8Bit,
            UdhValue::ConcatenatedShortMessage16Bit(_) => UdhId::ConcatenatedShortMessages16Bit,
            UdhValue::Other { udh_id, .. } => *udh_id,
        }
    }
}

impl Length for UdhValue {
    fn length(&self) -> usize {
        match self {
            UdhValue::ConcatenatedShortMessage8Bit(udh) => udh.length(),
            UdhValue::ConcatenatedShortMessage16Bit(udh) => udh.length(),
            UdhValue::Other { value, .. } => value.length(),
        }
    }
}

impl crate::encode::Encode for UdhValue {
    fn encode(&self, dst: &mut [u8]) -> usize {
        match self {
            UdhValue::ConcatenatedShortMessage8Bit(udh) => udh.encode(dst),
            UdhValue::ConcatenatedShortMessage16Bit(udh) => udh.encode(dst),
            UdhValue::Other { value, .. } => value.encode(dst),
        }
    }
}

impl crate::encode::owned::Encode for UdhValue {
    fn encode(&self, dst: &mut bytes::BytesMut) {
        match self {
            UdhValue::ConcatenatedShortMessage8Bit(udh) => udh.encode(dst),
            UdhValue::ConcatenatedShortMessage16Bit(udh) => udh.encode(dst),
            UdhValue::Other { value, .. } => value.encode(dst),
        }
    }
}

impl DecodeWithKey for UdhValue {
    type Key = UdhId;

    fn decode(
        key: Self::Key,
        src: &mut bytes::BytesMut,
        length: usize,
    ) -> Result<(Self, usize), DecodeError> {
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

impl Decode for Udh {
    fn decode(src: &mut bytes::BytesMut) -> Result<(Self, usize), DecodeError> {
        let size = 0;
        let (length, size) = crate::decode::DecodeErrorExt::map_as_source(
            crate::decode::owned::DecodeExt::decode_move(src, size),
            crate::fields::SmppField::udh_length,
        )?;
        let (id, size): (UdhId, usize) = crate::decode::DecodeErrorExt::map_as_source(
            crate::decode::owned::DecodeExt::decode_move(src, size),
            crate::fields::SmppField::udh_id,
        )?;

        let value_length = (length as usize).saturating_sub(id.length());

        let (value, size) = crate::decode::DecodeErrorExt::map_as_source(
            crate::decode::owned::DecodeWithKeyExt::optional_length_checked_decode_move(
                id,
                src,
                value_length,
                size,
            ),
            crate::fields::SmppField::udh_value,
        )?
        .map(|(this, size)| (Some(this), size))
        .unwrap_or((None, size));

        Ok((Self { length, id, value }, size))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod encode {
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

    mod decode {
        use bytes::BytesMut;

        use crate::decode::owned::Decode;

        use super::*;

        #[test]
        fn ok() {
            let mut buf = BytesMut::from(
                &[
                    0x06, // UDH length (following bytes = 6)
                    0x08, // UDH ID: Concatenated Short Messages, 16-bit reference number
                    0x04, // IE Data Length = 4 bytes
                    0x12, // Ref high
                    0x34, // Ref low
                    0x03, // Total parts
                    0x01, // Part number
                    0x00, // Extra bytes
                    0x00,
                ][..],
            );

            let (udh, size) = <Udh as Decode>::decode(&mut buf).unwrap();

            assert_eq!(size, 7);
            assert_eq!(
                udh,
                Udh::new(ConcatenatedShortMessage16Bit::new(0x1234, 3, 1).unwrap())
            );

            let mut buf = BytesMut::from(
                &[
                    0x05, // UDH length (following bytes = 5)
                    0x00, // UDH ID: Concatenated Short Messages, 8-bit reference number
                    0x03, // IE Data Length = 3 bytes
                    0x12, // Ref
                    0x03, // Total parts
                    0x01, // Part number
                    0x00, // Extra bytes
                    0x00,
                ][..],
            );

            let (udh, size) = <Udh as Decode>::decode(&mut buf).unwrap();
            assert_eq!(size, 6);
            assert_eq!(
                udh,
                Udh::new(ConcatenatedShortMessage8Bit::new(0x12, 3, 1).unwrap())
            );
        }
    }
}
