/// Docs
///
/// More docs
#[rusmpp(decode = owned)]
pub struct BroadcastAreaIdentifier {
    /// Docs
    ///
    /// More docs
    pub format: BroadcastAreaFormat,
    /// Docs
    ///
    /// More docs
    #[rusmpp(length = "unchecked")]
    pub area: AnyOctetString,
}
#[automatically_derived]
impl ::core::fmt::Debug for BroadcastAreaIdentifier {
    #[inline]
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        ::core::fmt::Formatter::debug_struct_field2_finish(
            f,
            "BroadcastAreaIdentifier",
            "format",
            &self.format,
            "area",
            &&self.area,
        )
    }
}
pub struct BroadcastAreaIdentifierParts {
    pub format: BroadcastAreaFormat,
    pub area: AnyOctetString,
}
#[automatically_derived]
impl ::core::fmt::Debug for BroadcastAreaIdentifierParts {
    #[inline]
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        ::core::fmt::Formatter::debug_struct_field2_finish(
            f,
            "BroadcastAreaIdentifierParts",
            "format",
            &self.format,
            "area",
            &&self.area,
        )
    }
}
impl BroadcastAreaIdentifierParts {
    #[inline]
    #[allow(clippy::too_many_arguments)]
    pub const fn new(format: BroadcastAreaFormat, area: AnyOctetString) -> Self {
        Self { format, area }
    }
    #[inline]
    #[allow(unused_parens)]
    pub fn raw(self) -> (BroadcastAreaFormat, AnyOctetString) {
        (self.format, self.area)
    }
}
impl BroadcastAreaIdentifier {
    /// Converts [`Self`] into its parts.
    #[inline]
    pub fn into_parts(self) -> BroadcastAreaIdentifierParts {
        BroadcastAreaIdentifierParts {
            format: self.format,
            area: self.area,
        }
    }
    /// Creates a new instance of [`Self`] from its parts.
    ///
    /// # Note
    ///
    /// This may create invalid instances. It's up to the caller to ensure that the parts are valid.
    #[inline]
    pub fn from_parts(parts: BroadcastAreaIdentifierParts) -> Self {
        Self {
            format: parts.format,
            area: parts.area,
        }
    }
}
impl crate::encode::Length for BroadcastAreaIdentifier {
    fn length(&self) -> usize {
        let mut length = 0;
        length += crate::encode::Length::length(&self.format);
        length += crate::encode::Length::length(&self.area);
        length
    }
}
impl crate::encode::Encode for BroadcastAreaIdentifier {
    fn encode(&self, dst: &mut [u8]) -> usize {
        let size = 0;
        let size = crate::encode::EncodeExt::encode_move(&self.format, dst, size);
        let size = crate::encode::EncodeExt::encode_move(&self.area, dst, size);
        size
    }
}
/// Docs
///
/// More docs
#[rusmpp(decode = borrowed)]
pub struct BroadcastAreaIdentifier<'a> {
    /// Docs
    ///
    /// More docs
    pub format: BroadcastAreaFormat,
    /// Docs
    ///
    /// More docs
    #[rusmpp(length = "unchecked")]
    pub area: AnyOctetString<'a>,
}
#[automatically_derived]
impl<'a> ::core::fmt::Debug for BroadcastAreaIdentifier<'a> {
    #[inline]
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        ::core::fmt::Formatter::debug_struct_field2_finish(
            f,
            "BroadcastAreaIdentifier",
            "format",
            &self.format,
            "area",
            &&self.area,
        )
    }
}
pub struct BroadcastAreaIdentifierParts<'a> {
    pub format: BroadcastAreaFormat,
    pub area: AnyOctetString<'a>,
}
#[automatically_derived]
impl<'a> ::core::fmt::Debug for BroadcastAreaIdentifierParts<'a> {
    #[inline]
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        ::core::fmt::Formatter::debug_struct_field2_finish(
            f,
            "BroadcastAreaIdentifierParts",
            "format",
            &self.format,
            "area",
            &&self.area,
        )
    }
}
impl<'a> BroadcastAreaIdentifierParts<'a> {
    #[inline]
    #[allow(clippy::too_many_arguments)]
    pub const fn new(format: BroadcastAreaFormat, area: AnyOctetString<'a>) -> Self {
        Self { format, area }
    }
    #[inline]
    #[allow(unused_parens)]
    pub fn raw(self) -> (BroadcastAreaFormat, AnyOctetString<'a>) {
        (self.format, self.area)
    }
}
impl<'a> BroadcastAreaIdentifier<'a> {
    /// Converts [`Self`] into its parts.
    #[inline]
    pub fn into_parts(self) -> BroadcastAreaIdentifierParts<'a> {
        BroadcastAreaIdentifierParts {
            format: self.format,
            area: self.area,
        }
    }
    /// Creates a new instance of [`Self`] from its parts.
    ///
    /// # Note
    ///
    /// This may create invalid instances. It's up to the caller to ensure that the parts are valid.
    #[inline]
    pub fn from_parts(parts: BroadcastAreaIdentifierParts<'a>) -> Self {
        Self {
            format: parts.format,
            area: parts.area,
        }
    }
}
impl<'a> crate::encode::Length for BroadcastAreaIdentifier<'a> {
    fn length(&self) -> usize {
        let mut length = 0;
        length += crate::encode::Length::length(&self.format);
        length += crate::encode::Length::length(&self.area);
        length
    }
}
impl<'a> crate::encode::Encode for BroadcastAreaIdentifier<'a> {
    fn encode(&self, dst: &mut [u8]) -> usize {
        let size = 0;
        let size = crate::encode::EncodeExt::encode_move(&self.format, dst, size);
        let size = crate::encode::EncodeExt::encode_move(&self.area, dst, size);
        size
    }
}
impl<'a> crate::decode::borrowed::DecodeWithLength<'a> for BroadcastAreaIdentifier<'a> {
    fn decode(
        src: &'a [u8],
        length: usize,
    ) -> Result<(Self, usize), crate::decode::DecodeError> {
        let size = 0;
        let (format, size) = crate::decode::DecodeErrorExt::map_as_source(
            crate::decode::borrowed::DecodeExt::decode_move(src, size),
            crate::fields::SmppField::format,
        )?;
        let (area, size) = crate::decode::DecodeErrorExt::map_as_source(
            crate::decode::borrowed::DecodeWithLengthExt::decode_move(
                src,
                length.saturating_sub(size),
                size,
            ),
            crate::fields::SmppField::area,
        )?;
        Ok((Self { format, area }, size))
    }
}
