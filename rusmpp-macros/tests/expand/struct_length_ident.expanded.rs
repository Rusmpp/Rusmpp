/// Docs
///
/// More docs
#[rusmpp(decode = owned)]
pub struct SubmitSm {
    /// Docs
    ///
    /// More docs
    pub other: u8,
    sm_length: u8,
    /// Docs
    ///
    /// More docs
    #[rusmpp(length = sm_length)]
    short_message: OctetString<0, 255>,
}
#[automatically_derived]
impl ::core::fmt::Debug for SubmitSm {
    #[inline]
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        ::core::fmt::Formatter::debug_struct_field3_finish(
            f,
            "SubmitSm",
            "other",
            &self.other,
            "sm_length",
            &self.sm_length,
            "short_message",
            &&self.short_message,
        )
    }
}
pub struct SubmitSmParts {
    pub other: u8,
    pub sm_length: u8,
    pub short_message: OctetString<0, 255>,
}
#[automatically_derived]
impl ::core::fmt::Debug for SubmitSmParts {
    #[inline]
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        ::core::fmt::Formatter::debug_struct_field3_finish(
            f,
            "SubmitSmParts",
            "other",
            &self.other,
            "sm_length",
            &self.sm_length,
            "short_message",
            &&self.short_message,
        )
    }
}
impl SubmitSmParts {
    #[inline]
    #[allow(clippy::too_many_arguments)]
    pub const fn new(
        other: u8,
        sm_length: u8,
        short_message: OctetString<0, 255>,
    ) -> Self {
        Self {
            other,
            sm_length,
            short_message,
        }
    }
    #[inline]
    #[allow(unused_parens)]
    pub fn raw(self) -> (u8, u8, OctetString<0, 255>) {
        (self.other, self.sm_length, self.short_message)
    }
}
impl SubmitSm {
    /// Converts [`Self`] into its parts.
    #[inline]
    pub fn into_parts(self) -> SubmitSmParts {
        SubmitSmParts {
            other: self.other,
            sm_length: self.sm_length,
            short_message: self.short_message,
        }
    }
    /// Creates a new instance of [`Self`] from its parts.
    ///
    /// # Note
    ///
    /// This may create invalid instances. It's up to the caller to ensure that the parts are valid.
    #[inline]
    pub fn from_parts(parts: SubmitSmParts) -> Self {
        Self {
            other: parts.other,
            sm_length: parts.sm_length,
            short_message: parts.short_message,
        }
    }
}
impl crate::encode::Length for SubmitSm {
    fn length(&self) -> usize {
        let mut length = 0;
        length += crate::encode::Length::length(&self.other);
        length += crate::encode::Length::length(&self.sm_length);
        length += crate::encode::Length::length(&self.short_message);
        length
    }
}
impl crate::encode::Encode for SubmitSm {
    fn encode(&self, dst: &mut [u8]) -> usize {
        let size = 0;
        let size = crate::encode::EncodeExt::encode_move(&self.other, dst, size);
        let size = crate::encode::EncodeExt::encode_move(&self.sm_length, dst, size);
        let size = crate::encode::EncodeExt::encode_move(&self.short_message, dst, size);
        size
    }
}
/// Docs
///
/// More docs
#[rusmpp(decode = borrowed)]
pub struct SubmitSm<'a, const N: usize> {
    /// Docs
    ///
    /// More docs
    pub other: u8,
    sm_length: u8,
    /// Docs
    ///
    /// More docs
    #[rusmpp(length = sm_length)]
    short_message: OctetString<'a, 0, 255>,
}
#[automatically_derived]
impl<'a, const N: usize> ::core::fmt::Debug for SubmitSm<'a, N> {
    #[inline]
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        ::core::fmt::Formatter::debug_struct_field3_finish(
            f,
            "SubmitSm",
            "other",
            &self.other,
            "sm_length",
            &self.sm_length,
            "short_message",
            &&self.short_message,
        )
    }
}
pub struct SubmitSmParts<'a, const N: usize> {
    pub other: u8,
    pub sm_length: u8,
    pub short_message: OctetString<'a, 0, 255>,
}
#[automatically_derived]
impl<'a, const N: usize> ::core::fmt::Debug for SubmitSmParts<'a, N> {
    #[inline]
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        ::core::fmt::Formatter::debug_struct_field3_finish(
            f,
            "SubmitSmParts",
            "other",
            &self.other,
            "sm_length",
            &self.sm_length,
            "short_message",
            &&self.short_message,
        )
    }
}
impl<'a, const N: usize> SubmitSmParts<'a, N> {
    #[inline]
    #[allow(clippy::too_many_arguments)]
    pub const fn new(
        other: u8,
        sm_length: u8,
        short_message: OctetString<'a, 0, 255>,
    ) -> Self {
        Self {
            other,
            sm_length,
            short_message,
        }
    }
    #[inline]
    #[allow(unused_parens)]
    pub fn raw(self) -> (u8, u8, OctetString<'a, 0, 255>) {
        (self.other, self.sm_length, self.short_message)
    }
}
impl<'a, const N: usize> SubmitSm<'a, N> {
    /// Converts [`Self`] into its parts.
    #[inline]
    pub fn into_parts(self) -> SubmitSmParts<'a, N> {
        SubmitSmParts {
            other: self.other,
            sm_length: self.sm_length,
            short_message: self.short_message,
        }
    }
    /// Creates a new instance of [`Self`] from its parts.
    ///
    /// # Note
    ///
    /// This may create invalid instances. It's up to the caller to ensure that the parts are valid.
    #[inline]
    pub fn from_parts(parts: SubmitSmParts<'a, N>) -> Self {
        Self {
            other: parts.other,
            sm_length: parts.sm_length,
            short_message: parts.short_message,
        }
    }
}
impl<'a, const N: usize> crate::encode::Length for SubmitSm<'a, N> {
    fn length(&self) -> usize {
        let mut length = 0;
        length += crate::encode::Length::length(&self.other);
        length += crate::encode::Length::length(&self.sm_length);
        length += crate::encode::Length::length(&self.short_message);
        length
    }
}
impl<'a, const N: usize> crate::encode::Encode for SubmitSm<'a, N> {
    fn encode(&self, dst: &mut [u8]) -> usize {
        let size = 0;
        let size = crate::encode::EncodeExt::encode_move(&self.other, dst, size);
        let size = crate::encode::EncodeExt::encode_move(&self.sm_length, dst, size);
        let size = crate::encode::EncodeExt::encode_move(&self.short_message, dst, size);
        size
    }
}
impl<'a, const N: usize> crate::decode::borrowed::DecodeWithLength<'a>
for SubmitSm<'a, N> {
    fn decode(
        src: &'a [u8],
        length: usize,
    ) -> Result<(Self, usize), crate::decode::DecodeError> {
        let size = 0;
        let (other, size) = crate::decode::DecodeErrorExt::map_as_source(
            crate::decode::borrowed::DecodeExt::decode_move(src, size),
            crate::fields::SmppField::other,
        )?;
        let (sm_length, size) = crate::decode::DecodeErrorExt::map_as_source(
            crate::decode::borrowed::DecodeExt::decode_move(src, size),
            crate::fields::SmppField::sm_length,
        )?;
        let (short_message, size) = crate::decode::DecodeErrorExt::map_as_source(
            crate::decode::borrowed::DecodeWithLengthExt::decode_move(
                src,
                sm_length as usize,
                size,
            ),
            crate::fields::SmppField::short_message,
        )?;
        Ok((
            Self {
                other,
                sm_length,
                short_message,
            },
            size,
        ))
    }
}
