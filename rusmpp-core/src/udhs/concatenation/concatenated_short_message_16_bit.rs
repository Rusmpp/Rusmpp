use crate::{encode::Length, udhs::errors::ConcatenatedShortMessageError};

/// 16-bit Concatenated Short Message UDH.
///
/// 16-bit reference number (IEI = 0x08)
///
/// # Format
///
/// This format shows the [`ConcatenatedShortMessage16Bit`] encoded as a full UDH.
///
/// ```txt
/// 06 08 04 RRH RRL TP PN
/// │  │  │  │   │   │  └─ Part number
/// │  │  │  │   │   └──── Total parts
/// │  │  │  │   └──────── Low byte of 16-bit reference
/// │  │  │  └──────────── High byte of 16-bit reference
/// │  │  └─────────────── IE Data Length = 4
/// │  └────────────────── IEI = 08 (16-bit reference)
/// └───────────────────── UDH length = 6
/// ```
/// # Note
///
/// The first 3 bytes `(06 08 04)` are part of the UDH header and are not stored in the struct.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct ConcatenatedShortMessage16Bit {
    /// Reference number for the concatenated message.
    reference: u16,
    /// Total number of parts in the concatenated message.
    total_parts: u8,
    /// Part number of this message.
    part_number: u8,
}

impl ConcatenatedShortMessage16Bit {
    /// The length of [`ConcatenatedShortMessage16Bit`].
    const LENGTH: usize = 5;

    /// The length of [`ConcatenatedShortMessage16Bit`] encoded as a full UDH.
    pub const UDH_LENGTH: usize = Self::LENGTH + 2;

    /// Creates a new [`ConcatenatedShortMessage16Bit`].
    ///
    /// # Returns
    ///
    /// - `Ok(Self)` if the invariants are satisfied.
    /// - `Err(ConcatenatedShortMessageError)` if any invariant is violated.
    pub const fn new(
        reference: u16,
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

    /// Creates a new [`ConcatenatedShortMessage16Bit`] without checking invariants.
    pub const fn new_unchecked(reference: u16, total_parts: u8, part_number: u8) -> Self {
        Self {
            reference,
            total_parts,
            part_number,
        }
    }

    /// Returns the reference number.
    pub const fn reference(&self) -> u16 {
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

    /// The byte representation of [`ConcatenatedShortMessage16Bit`].
    const fn bytes(&self) -> [u8; Self::LENGTH] {
        [
            0x04,                          // IE Data Length = 4 bytes
            (self.reference >> 8) as u8,   // Ref high
            (self.reference & 0xFF) as u8, // Ref low
            self.total_parts,
            self.part_number,
        ]
    }

    /// The bytes representation of [`ConcatenatedShortMessage16Bit`] encoded as a full UDH.
    pub const fn udh_bytes(&self) -> [u8; Self::LENGTH + 2] {
        let bytes = self.bytes();
        [
            0x06, // UDH length = 6
            0x08, // IEI = 08 (16-bit reference)
            bytes[0], bytes[1], bytes[2], bytes[3], bytes[4],
        ]
    }

    /// Consumes the [`ConcatenatedShortMessage16Bit`] and returns its parts.
    pub const fn into_parts(self) -> ConcatenatedShortMessage16BitParts {
        ConcatenatedShortMessage16BitParts {
            reference: self.reference,
            total_parts: self.total_parts,
            part_number: self.part_number,
        }
    }
}

/// Parts of a [`ConcatenatedShortMessage16Bit`].
#[derive(Debug, Clone)]
pub struct ConcatenatedShortMessage16BitParts {
    /// Reference number for the concatenated message.
    pub reference: u16,
    /// Total number of parts in the concatenated message.
    pub total_parts: u8,
    /// Part number of this message.
    pub part_number: u8,
}

impl Length for ConcatenatedShortMessage16Bit {
    fn length(&self) -> usize {
        Self::LENGTH
    }
}

impl crate::encode::Encode for ConcatenatedShortMessage16Bit {
    fn encode(&self, dst: &mut [u8]) -> usize {
        let bytes = self.bytes();

        dst[..Self::LENGTH].copy_from_slice(&bytes);

        Self::LENGTH
    }
}

#[cfg(feature = "alloc")]
impl crate::encode::owned::Encode for ConcatenatedShortMessage16Bit {
    fn encode(&self, dst: &mut bytes::BytesMut) {
        use bytes::BufMut;

        let bytes = self.bytes();

        dst.put(&bytes[..]);
    }
}

#[cfg(feature = "alloc")]
impl crate::decode::owned::Decode for ConcatenatedShortMessage16Bit {
    fn decode(src: &mut bytes::BytesMut) -> Result<(Self, usize), crate::decode::DecodeError> {
        if src.len() < Self::LENGTH {
            return Err(
                crate::decode::DecodeError::concatenated_short_message_decode_error(
                    crate::decode::ConcatenatedShortMessageDecodeError::TooFewBytes {
                        actual: src.len(),
                        min: Self::LENGTH,
                    },
                ),
            );
        }

        let length = src[0];

        if length != 0x04_u8 {
            return Err(crate::decode::DecodeError::concatenated_short_message_decode_error(
                crate::decode::ConcatenatedShortMessageDecodeError::InvalidInformationElementLength {
                    actual: length,
                    expected: 0x04_u8,
                },
            ));
        }

        let reference = ((src[1] as u16) << 8) | (src[2] as u16);
        let total_parts = src[3];
        let part_number = src[4];

        let decoded = Self::new(reference, total_parts, part_number)?;

        Ok((decoded, Self::LENGTH))
    }
}

#[cfg(feature = "alloc")]
impl From<ConcatenatedShortMessage16Bit> for crate::udhs::owned::UdhValue {
    fn from(udh: ConcatenatedShortMessage16Bit) -> Self {
        crate::udhs::owned::UdhValue::ConcatenatedShortMessage16Bit(udh)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod new {
        use super::*;

        #[test]
        fn ok() {
            let udh = ConcatenatedShortMessage16Bit::new(0x1234, 3, 2).unwrap();
            assert_eq!(udh.reference, 0x1234);
            assert_eq!(udh.total_parts, 3);
            assert_eq!(udh.part_number, 2);
        }

        #[test]
        fn part_number_zero() {
            let err = ConcatenatedShortMessage16Bit::new(0x1234, 3, 0).unwrap_err();
            assert!(matches!(err, ConcatenatedShortMessageError::PartNumberZero));
        }

        #[test]
        fn part_number_exceeds_total_parts() {
            let err = ConcatenatedShortMessage16Bit::new(0x1234, 2, 3).unwrap_err();
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
            let err = ConcatenatedShortMessage16Bit::new(0x1234, 0, 1).unwrap_err();
            assert!(matches!(err, ConcatenatedShortMessageError::TotalPartsZero));
        }
    }

    mod decode {
        use super::*;

        #[cfg(feature = "alloc")]
        mod owned {
            use bytes::BytesMut;

            use crate::decode::{
                ConcatenatedShortMessageDecodeError, DecodeErrorKind, UdhDecodeError, owned::Decode,
            };

            use super::*;

            #[test]
            fn ok() {
                let mut buf = BytesMut::from(&[0x04, 0x12, 0x34, 0x03, 0x02][..]);
                let (udh, size) = ConcatenatedShortMessage16Bit::decode(&mut buf).unwrap();
                assert_eq!(size, 5);
                assert_eq!(udh.reference, 0x1234);
                assert_eq!(udh.total_parts, 3);
                assert_eq!(udh.part_number, 2);
            }

            #[test]
            fn too_few_bytes() {
                let mut buf = BytesMut::from(&[0x04, 0x12, 0x34][..]);
                let err = ConcatenatedShortMessage16Bit::decode(&mut buf).unwrap_err();
                assert!(matches!(
                    err.kind(),
                    DecodeErrorKind::UdhDecodeError(
                        UdhDecodeError::ConcatenatedShortMessageDecodeError(
                            ConcatenatedShortMessageDecodeError::TooFewBytes { actual: 3, min: 5 }
                        )
                    )
                ));
            }

            #[test]
            fn invalid_information_element_length() {
                let mut buf = BytesMut::from(&[0x03, 0x12, 0x34, 0x03, 0x02][..]);
                let err = ConcatenatedShortMessage16Bit::decode(&mut buf).unwrap_err();
                assert!(matches!(
                    err.kind(),
                    DecodeErrorKind::UdhDecodeError(
                        UdhDecodeError::ConcatenatedShortMessageDecodeError(
                            ConcatenatedShortMessageDecodeError::InvalidInformationElementLength {
                                actual: 3,
                                expected: 4
                            }
                        )
                    )
                ));
            }

            #[test]
            fn part_number_exceeds_total_parts() {
                let mut buf = BytesMut::from(&[0x04, 0x12, 0x34, 0x02, 0x03][..]);
                let err = ConcatenatedShortMessage16Bit::decode(&mut buf).unwrap_err();
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
                let mut buf = BytesMut::from(&[0x04, 0x12, 0x34, 0x00, 0x01][..]);
                let err = ConcatenatedShortMessage16Bit::decode(&mut buf).unwrap_err();
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
                let mut buf = BytesMut::from(&[0x04, 0x12, 0x34, 0x03, 0x00][..]);
                let err = ConcatenatedShortMessage16Bit::decode(&mut buf).unwrap_err();
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

        // TODO: borrowed decode
    }

    mod encode {
        use crate::encode::Encode;

        use super::*;

        #[test]
        fn ok() {
            let udh = ConcatenatedShortMessage16Bit::new(0x1234, 3, 2).unwrap();
            let mut buf = [0u8; 5];
            let size = udh.encode(&mut buf);
            assert_eq!(size, 5);
            assert_eq!(buf, [0x04, 0x12, 0x34, 0x03, 0x02]);
        }

        #[cfg(feature = "alloc")]
        mod owned {
            use bytes::BytesMut;

            use crate::encode::owned::Encode;

            use super::ConcatenatedShortMessage16Bit;

            #[test]
            fn ok() {
                let udh = ConcatenatedShortMessage16Bit::new(0x1234, 3, 2).unwrap();
                let mut buf = BytesMut::with_capacity(5);
                udh.encode(&mut buf);
                assert_eq!(&buf[..], [0x04, 0x12, 0x34, 0x03, 0x02]);
            }
        }
    }
}
