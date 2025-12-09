//! An unsigned integer value, which can be 1, 2 or 4 octets
//! in size. The octets are always encoded in Most Significant
//! Byte (MSB) first order, otherwise known as Big Endian
//! Encoding.
//!
//! A 1-octet Integer with a value 5, would be encoded in a
//! single octet with the value 0x05

use crate::{
    decode::{DecodeError, borrowed},
    encode::{Encode, Length},
};

impl Length for u8 {
    fn length(&self) -> usize {
        1
    }
}

impl Encode for u8 {
    fn encode(&self, dst: &mut [u8]) -> usize {
        dst[0] = *self;

        1
    }
}

#[cfg(feature = "alloc")]
impl crate::decode::owned::Decode for u8 {
    fn decode(src: &[u8]) -> Result<(Self, usize), DecodeError> {
        if src.is_empty() {
            return Err(DecodeError::unexpected_eof());
        }

        Ok((src[0], 1))
    }
}

impl borrowed::Decode<'_> for u8 {
    fn decode(src: &[u8]) -> Result<(Self, usize), DecodeError> {
        if src.is_empty() {
            return Err(DecodeError::unexpected_eof());
        }

        Ok((src[0], 1))
    }
}
