use bytes::BufMut;

use crate::{
    decode::{ConcatenatedShortMessageDecodeError, DecodeError},
    encode::Length,
    udhs::{
        errors::ConcatenatedShortMessageError,
        owned::{UdhValue, concatenation::ConcatenatedShortMessage16Bit},
    },
};

/// 8-bit Concatenated Short Message UDH.
///
/// 8-bit reference number (IEI = 0x00)
///
/// # Format
///
/// This format shows the [`ConcatenatedShortMessage8Bit`] encoded as a full UDH.
///
/// ```txt
/// 05 00 03 RR TP PN
/// │  │  │  │  │  └─ Part number
/// │  │  │  │  └──── Total parts
/// │  │  │  └─────── 8-bit reference number (1 byte)
/// │  │  └────────── IE Data Length = 3
/// │  └───────────── IEI = 00 (8-bit reference)
/// └──────────────── UDH length = 5
/// ```
/// # Note
///
/// The first 3 bytes `(05 00 03)` are part of the UDH header and are not stored in the struct.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct ConcatenatedShortMessage8Bit {
    /// Reference number for the concatenated message.
    reference: u8,
    /// Total number of parts in the concatenated message.
    total_parts: u8,
    /// Part number of this message.
    part_number: u8,
}

/// Parts of a [`ConcatenatedShortMessage8Bit`].
#[derive(Debug, Clone)]
pub struct ConcatenatedShortMessage8BitParts {
    /// Reference number for the concatenated message.
    pub reference: u8,
    /// Total number of parts in the concatenated message.
    pub total_parts: u8,
    /// Part number of this message.
    pub part_number: u8,
}

impl ConcatenatedShortMessage8Bit {
    /// The length of [`ConcatenatedShortMessage8Bit`].
    const LENGTH: usize = 4;

    /// The length of [`ConcatenatedShortMessage8Bit`] encoded as a full UDH.
    pub const UDH_LENGTH: usize = Self::LENGTH + 2;

    /// Creates a new [`ConcatenatedShortMessage8Bit`].
    ///
    /// # Returns
    ///
    /// - `Ok(Self)` if the invariants are satisfied.
    /// - `Err(ConcatenatedShortMessageError)` if any invariant is violated.
    pub const fn new(
        reference: u8,
        total_parts: u8,
        part_number: u8,
    ) -> Result<Self, ConcatenatedShortMessageError> {
        Self::new_unchecked(reference, total_parts, part_number).assert()
    }

    /// Asserts the invariants of the UDH.
    ///
    /// # Returns
    ///
    /// - `Ok(Self)` if the invariants are satisfied.
    /// - `Err(ConcatenatedShortMessageError)` if any invariant is violated.
    const fn assert(self) -> Result<Self, ConcatenatedShortMessageError> {
        if self.total_parts == 0 {
            return Err(ConcatenatedShortMessageError::TotalPartsZero);
        }

        if self.part_number == 0 {
            return Err(ConcatenatedShortMessageError::PartNumberZero);
        }

        if self.part_number > self.total_parts {
            return Err(ConcatenatedShortMessageError::PartNumberExceedsTotalParts {
                part_number: self.part_number,
                total_parts: self.total_parts,
            });
        }

        Ok(self)
    }

    /// Creates a new [`ConcatenatedShortMessage8Bit`] without checking invariants.
    pub const fn new_unchecked(reference: u8, total_parts: u8, part_number: u8) -> Self {
        Self {
            reference,
            total_parts,
            part_number,
        }
    }

    /// Returns the reference number.
    pub const fn reference(&self) -> u8 {
        self.reference
    }

    /// Returns the total number of parts.
    pub const fn total_parts(&self) -> u8 {
        self.total_parts
    }

    /// Returns the part number.
    pub const fn part_number(&self) -> u8 {
        self.part_number
    }

    /// The bytes representation of [`ConcatenatedShortMessage8Bit`].
    const fn bytes(&self) -> [u8; Self::LENGTH] {
        [
            0x03, // IE Data Length = 3 bytes
            self.reference,
            self.total_parts,
            self.part_number,
        ]
    }

    /// The bytes representation of [`ConcatenatedShortMessage8Bit`] encoded as a full UDH.
    pub const fn udh_bytes(&self) -> [u8; Self::LENGTH + 2] {
        let bytes = self.bytes();
        [
            0x05, // UDH Length = 5 bytes
            0x00, // IEI = 00 (8-bit reference)
            bytes[0], bytes[1], bytes[2], bytes[3],
        ]
    }

    /// Consumes the [`ConcatenatedShortMessage8Bit`] and returns its parts.
    pub const fn into_parts(self) -> ConcatenatedShortMessage8BitParts {
        ConcatenatedShortMessage8BitParts {
            reference: self.reference,
            total_parts: self.total_parts,
            part_number: self.part_number,
        }
    }
}

impl Length for ConcatenatedShortMessage8Bit {
    fn length(&self) -> usize {
        Self::LENGTH
    }
}

impl crate::encode::Encode for ConcatenatedShortMessage8Bit {
    fn encode(&self, dst: &mut [u8]) -> usize {
        let bytes = self.bytes();

        dst[..Self::LENGTH].copy_from_slice(&bytes);

        Self::LENGTH
    }
}

impl crate::encode::owned::Encode for ConcatenatedShortMessage8Bit {
    fn encode(&self, dst: &mut bytes::BytesMut) {
        let bytes = self.bytes();

        dst.put(&bytes[..]);
    }
}

impl crate::decode::owned::Decode for ConcatenatedShortMessage8Bit {
    fn decode(src: &mut bytes::BytesMut) -> Result<(Self, usize), DecodeError> {
        if src.len() < Self::LENGTH {
            return Err(DecodeError::concatenated_short_message_decode_error(
                ConcatenatedShortMessageDecodeError::TooFewBytes {
                    actual: src.len(),
                    min: Self::LENGTH,
                },
            ));
        }

        let length = src[0];

        if length != 0x03_u8 {
            return Err(DecodeError::concatenated_short_message_decode_error(
                ConcatenatedShortMessageDecodeError::InvalidInformationElementLength {
                    actual: length,
                    expected: 0x03_u8,
                },
            ));
        }

        let decoded = Self::new(src[1], src[2], src[3])?;

        Ok((decoded, Self::LENGTH))
    }
}

impl From<ConcatenatedShortMessage8Bit> for UdhValue {
    fn from(udh: ConcatenatedShortMessage8Bit) -> Self {
        UdhValue::ConcatenatedShortMessage8Bit(udh)
    }
}

/// Converts a [`ConcatenatedShortMessage8Bit`] into a [`ConcatenatedShortMessage16Bit`].
///
/// # Note
///
/// Conversion from [`ConcatenatedShortMessage16Bit`] to [`ConcatenatedShortMessage8Bit`] does not exist
/// due to potential loss of information (the 16-bit reference number may not fit into 8 bits).
impl From<ConcatenatedShortMessage8Bit> for ConcatenatedShortMessage16Bit {
    fn from(udh: ConcatenatedShortMessage8Bit) -> Self {
        ConcatenatedShortMessage16Bit::new_unchecked(
            udh.reference as u16,
            udh.total_parts,
            udh.part_number,
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod new {
        use super::*;

        #[test]
        fn ok() {
            let udh = ConcatenatedShortMessage8Bit::new(1, 3, 2).unwrap();
            assert_eq!(udh.reference, 1);
            assert_eq!(udh.total_parts, 3);
            assert_eq!(udh.part_number, 2);
        }

        #[test]
        fn part_number_zero() {
            let err = ConcatenatedShortMessage8Bit::new(1, 3, 0).unwrap_err();
            assert!(matches!(err, ConcatenatedShortMessageError::PartNumberZero));
        }

        #[test]
        fn part_number_exceeds_total_parts() {
            let err = ConcatenatedShortMessage8Bit::new(1, 2, 3).unwrap_err();
            assert!(matches!(
                err,
                ConcatenatedShortMessageError::PartNumberExceedsTotalParts {
                    part_number: 3,
                    total_parts: 2
                }
            ));
        }

        #[test]
        fn total_parts_zero() {
            let err = ConcatenatedShortMessage8Bit::new(1, 0, 1).unwrap_err();
            assert!(matches!(err, ConcatenatedShortMessageError::TotalPartsZero));
        }
    }

    mod decode {
        use bytes::BytesMut;

        use crate::decode::{DecodeErrorKind, UdhDecodeError, owned::Decode};

        use super::*;

        #[test]
        fn ok() {
            let mut buf = BytesMut::from(&[0x03, 0x12, 0x34, 0x02][..]);
            let (udh, size) = ConcatenatedShortMessage8Bit::decode(&mut buf).unwrap();
            assert_eq!(size, 4);
            assert_eq!(udh.reference, 0x12);
            assert_eq!(udh.total_parts, 0x34);
            assert_eq!(udh.part_number, 0x02);
        }

        #[test]
        fn too_few_bytes() {
            let mut buf = BytesMut::from(&[0x03, 0x12, 0x34][..]);
            let err = ConcatenatedShortMessage8Bit::decode(&mut buf).unwrap_err();
            assert!(matches!(
                err.kind(),
                DecodeErrorKind::UdhDecodeError(
                    UdhDecodeError::ConcatenatedShortMessageDecodeError(
                        ConcatenatedShortMessageDecodeError::TooFewBytes { actual: 3, min: 4 }
                    )
                )
            ));
        }

        #[test]
        fn invalid_information_element_length() {
            let mut buf = BytesMut::from(&[0x04, 0x12, 0x34, 0x02][..]);
            let err = ConcatenatedShortMessage8Bit::decode(&mut buf).unwrap_err();
            assert!(matches!(
                err.kind(),
                DecodeErrorKind::UdhDecodeError(
                    UdhDecodeError::ConcatenatedShortMessageDecodeError(
                        ConcatenatedShortMessageDecodeError::InvalidInformationElementLength {
                            actual: 4,
                            expected: 3
                        }
                    )
                )
            ));
        }

        #[test]
        fn part_number_exceeds_total_parts() {
            let mut buf = BytesMut::from(&[0x03, 0x12, 2, 3][..]);
            let err = ConcatenatedShortMessage8Bit::decode(&mut buf).unwrap_err();
            assert!(matches!(
                err.kind(),
                DecodeErrorKind::UdhDecodeError(
                    UdhDecodeError::ConcatenatedShortMessageDecodeError(
                        ConcatenatedShortMessageDecodeError::PartNumberExceedsTotalParts {
                            part_number: 3,
                            total_parts: 2
                        }
                    )
                )
            ));
        }

        #[test]
        fn total_parts_zero() {
            let mut buf = BytesMut::from(&[0x03, 0x12, 0x00, 0x01][..]);
            let err = ConcatenatedShortMessage8Bit::decode(&mut buf).unwrap_err();
            assert!(matches!(
                err.kind(),
                DecodeErrorKind::UdhDecodeError(
                    UdhDecodeError::ConcatenatedShortMessageDecodeError(
                        ConcatenatedShortMessageDecodeError::TotalPartsZero
                    )
                )
            ));
        }

        #[test]
        fn part_number_zero() {
            let mut buf = BytesMut::from(&[0x03, 0x12, 0x03, 0x00][..]);
            let err = ConcatenatedShortMessage8Bit::decode(&mut buf).unwrap_err();
            assert!(matches!(
                err.kind(),
                DecodeErrorKind::UdhDecodeError(
                    UdhDecodeError::ConcatenatedShortMessageDecodeError(
                        ConcatenatedShortMessageDecodeError::PartNumberZero
                    )
                )
            ));
        }
    }

    mod encode {
        use crate::encode::Encode;

        use super::*;

        #[test]
        fn ok() {
            let udh = ConcatenatedShortMessage8Bit::new(0x12, 0x34, 0x02).unwrap();
            let mut buf = [0u8; 4];
            let size = udh.encode(&mut buf);
            assert_eq!(size, 4);
            assert_eq!(buf, [0x03, 0x12, 0x34, 0x02]);
        }
    }
}
