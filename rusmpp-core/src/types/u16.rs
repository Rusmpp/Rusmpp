//! An unsigned integer value, which can be 1, 2 or 4 octets
//! in size. The octets are always encoded in Most Significant
//! Byte (MSB) first order, otherwise known as Big Endian
//! Encoding.
//!
//! A 2-octet integer with the decimal value of 41746 would
//! be encoded as 2 octets with the value 0xA312

use crate::{
    decode::{DecodeError, borrowed},
    encode::{Encode, Length},
};

impl Length for u16 {
    fn length(&self) -> usize {
        2
    }
}

impl Encode for u16 {
    fn encode(&self, dst: &mut [u8]) -> usize {
        let bytes = self.to_be_bytes();

        dst[0] = bytes[0];
        dst[1] = bytes[1];

        2
    }
}

#[cfg(feature = "alloc")]
impl crate::encode::owned::Encode for u16 {
    fn encode(&self, dst: &mut bytes::BytesMut) {
        use bytes::BufMut;

        dst.put_u16(*self);
    }
}

#[cfg(feature = "alloc")]
impl crate::decode::owned::Decode for u16 {
    fn decode(src: &mut bytes::BytesMut) -> Result<(Self, usize), DecodeError> {
        use bytes::Buf;

        if src.len() < 2 {
            return Err(DecodeError::unexpected_eof());
        }

        Ok((src.get_u16(), 2))
    }
}

impl borrowed::Decode<'_> for u16 {
    fn decode(src: &[u8]) -> Result<(Self, usize), DecodeError> {
        if src.len() < 2 {
            return Err(DecodeError::unexpected_eof());
        }

        let mut bytes = [0; 2];

        bytes.copy_from_slice(&src[..2]);

        let value = u16::from_be_bytes(bytes);

        Ok((value, 2))
    }
}
