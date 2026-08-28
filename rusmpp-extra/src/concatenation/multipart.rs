use rusmpp_core::decode::{
    ConcatenatedShortMessageDecodeError, DecodeError, DecodeErrorKind, UdhDecodeError,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum MultipartType {
    Udh { size: usize },
    Sar,
}

#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct MultipartSegment {
    pub r#type: MultipartType,
    pub reference: u16,
    pub part_number: u8,
    pub total_parts: u8,
}

impl MultipartSegment {
    /// Creates a new [`MultipartSegment`].
    ///
    /// # Returns
    ///
    /// - `Ok(Self)` if the invariants are satisfied.
    /// - `Err(MultipartSegmentError)` if any invariant is violated.
    pub const fn new(
        r#type: MultipartType,
        reference: u16,
        total_parts: u8,
        part_number: u8,
    ) -> Result<Self, MultipartSegmentError> {
        Self::new_unchecked(r#type, reference, total_parts, part_number).assert()
    }

    /// Creates a new [`MultipartSegment`] without checking invariants.
    pub const fn new_unchecked(
        r#type: MultipartType,
        reference: u16,
        total_parts: u8,
        part_number: u8,
    ) -> Self {
        Self {
            r#type,
            reference,
            total_parts,
            part_number,
        }
    }

    /// Asserts the invariants of the [`MultipartSegment`].
    ///
    /// # Returns
    ///
    /// - `Ok(Self)` if the invariants are satisfied.
    /// - `Err(MultipartSegmentError)` if any invariant is violated.
    const fn assert(self) -> Result<Self, MultipartSegmentError> {
        if self.total_parts == 0 {
            return Err(MultipartSegmentError::TotalPartsZero);
        }

        if self.part_number == 0 {
            return Err(MultipartSegmentError::PartNumberZero);
        }

        if self.part_number > self.total_parts {
            return Err(MultipartSegmentError::PartNumberExceedsTotalParts {
                part_number: self.part_number,
                total_parts: self.total_parts,
            });
        }

        Ok(self)
    }

    /// Returns `true` if the [`MultipartSegment`] is the first part of the multipart message.
    pub const fn is_first(&self) -> bool {
        self.part_number == 1
    }

    /// Returns `true` if the [`MultipartSegment`] is the last part of the multipart message.
    pub const fn is_last(&self) -> bool {
        self.part_number == self.total_parts
    }

    /// Returns the UDH size if the [`MultipartSegment`] is of type [`MultipartType::Udh`].
    pub const fn udh_size(&self) -> Option<usize> {
        match self.r#type {
            MultipartType::Udh { size } => Some(size),
            MultipartType::Sar => None,
        }
    }

    /// Returns the header size of the `short_message`.
    ///
    /// `header size` is the size of the UDH if the [`MultipartSegment`] is of type [`MultipartType::Udh`], otherwise it is 0.
    pub const fn header_size(&self) -> usize {
        match self.udh_size() {
            Some(size) => size,
            None => 0,
        }
    }
}

#[derive(Debug, thiserror::Error)]
pub enum MultipartSegmentError {
    /// The underlying decode error.
    #[error("Decode error: {0}")]
    Decode(#[source] DecodeError),
    /// The total number of parts is zero.
    #[error("Total parts cannot be zero")]
    TotalPartsZero,
    /// The part number is zero.
    #[error("Part number cannot be zero")]
    PartNumberZero,
    /// The part number exceeds the total number of parts.
    #[error("Part number {part_number} exceeds total parts {total_parts}")]
    PartNumberExceedsTotalParts { part_number: u8, total_parts: u8 },
}

impl From<DecodeError> for MultipartSegmentError {
    fn from(err: DecodeError) -> Self {
        match err.kind {
            DecodeErrorKind::UdhDecodeError(
                UdhDecodeError::ConcatenatedShortMessageDecodeError(concatenation_err),
            ) => match concatenation_err {
                ConcatenatedShortMessageDecodeError::TotalPartsZero => Self::TotalPartsZero,
                ConcatenatedShortMessageDecodeError::PartNumberZero => Self::PartNumberZero,
                ConcatenatedShortMessageDecodeError::PartNumberExceedsTotalParts {
                    part_number,
                    total_parts,
                } => Self::PartNumberExceedsTotalParts {
                    part_number,
                    total_parts,
                },
                _ => Self::Decode(err),
            },
            _ => Self::Decode(err),
        }
    }
}
