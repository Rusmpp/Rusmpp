use rusmpp_macros::Rusmpp;

use crate::udhs::errors::ConcatenatedShortMessageError;

/// 8-bit Concatenated Short Message UDH.
///
/// 8-bit reference number (IEI = 0x00)
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Rusmpp)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize))]
pub struct ConcatenatedShortMessage8Bit {
    /// Reference number for the concatenated message.
    reference: u8,
    /// Total number of parts in the concatenated message.
    total_parts: u8,
    /// Part number of this message.
    part_number: u8,
}

impl Default for ConcatenatedShortMessage8Bit {
    fn default() -> Self {
        Self {
            reference: 0,
            total_parts: 1,
            part_number: 1,
        }
    }
}

#[cfg(feature = "serde")]
const _: () = {
    use serde::{Deserialize, Deserializer};

    impl<'de> Deserialize<'de> for ConcatenatedShortMessage8Bit {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let de = ConcatenatedShortMessage8BitParts::deserialize(deserializer)?;

            Self::from_parts(de)
                .assert()
                .map_err(serde::de::Error::custom)
        }
    }
};

impl ConcatenatedShortMessage8Bit {
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
}

#[cfg(feature = "alloc")]
impl From<ConcatenatedShortMessage8Bit> for crate::udhs::owned::UdhValue {
    fn from(udh: ConcatenatedShortMessage8Bit) -> Self {
        crate::udhs::owned::UdhValue::ConcatenatedShortMessage8Bit(udh)
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
}
