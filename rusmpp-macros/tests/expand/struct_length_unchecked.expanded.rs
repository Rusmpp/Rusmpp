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
#[non_exhaustive]
pub struct BroadcastAreaIdentifierDecodeErrorContext {
    pub format: ::core::option::Option<
        ::core::result::Result<
            BroadcastAreaFormat,
            <BroadcastAreaFormat as crate::decode::owned::DecodeErrorType>::Error,
        >,
    >,
    pub area: ::core::option::Option<
        ::core::result::Result<
            AnyOctetString,
            <AnyOctetString as crate::decode::owned::DecodeErrorType>::Error,
        >,
    >,
}
#[automatically_derived]
impl ::core::fmt::Debug for BroadcastAreaIdentifierDecodeErrorContext {
    #[inline]
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        ::core::fmt::Formatter::debug_struct_field2_finish(
            f,
            "BroadcastAreaIdentifierDecodeErrorContext",
            "format",
            &self.format,
            "area",
            &&self.area,
        )
    }
}
#[automatically_derived]
impl ::core::clone::Clone for BroadcastAreaIdentifierDecodeErrorContext {
    #[inline]
    fn clone(&self) -> BroadcastAreaIdentifierDecodeErrorContext {
        BroadcastAreaIdentifierDecodeErrorContext {
            format: ::core::clone::Clone::clone(&self.format),
            area: ::core::clone::Clone::clone(&self.area),
        }
    }
}
#[non_exhaustive]
pub struct BroadcastAreaIdentifierDecodeError {
    pub context: BroadcastAreaIdentifierDecodeErrorContext,
}
#[automatically_derived]
impl ::core::fmt::Debug for BroadcastAreaIdentifierDecodeError {
    #[inline]
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        ::core::fmt::Formatter::debug_struct_field1_finish(
            f,
            "BroadcastAreaIdentifierDecodeError",
            "context",
            &&self.context,
        )
    }
}
#[automatically_derived]
impl ::core::clone::Clone for BroadcastAreaIdentifierDecodeError {
    #[inline]
    fn clone(&self) -> BroadcastAreaIdentifierDecodeError {
        BroadcastAreaIdentifierDecodeError {
            context: ::core::clone::Clone::clone(&self.context),
        }
    }
}
impl ::core::fmt::Display for BroadcastAreaIdentifierDecodeError {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.write_fmt(
            format_args!("Failed to decode {0} {{ ", "BroadcastAreaIdentifier"),
        )?;
        if let ::core::option::Option::Some(::core::result::Result::Err(err)) = &self
            .context
            .format
        {
            f.write_fmt(format_args!("{0}: {1}", "format", err))?;
            f.write_fmt(format_args!(" }}"))?;
            return Ok(());
        }
        if let ::core::option::Option::Some(::core::result::Result::Err(err)) = &self
            .context
            .area
        {
            f.write_fmt(format_args!("{0}: {1}", "area", err))?;
            f.write_fmt(format_args!(" }}"))?;
            return Ok(());
        }
        f.write_fmt(format_args!(" }}"))
    }
}
impl ::core::error::Error for BroadcastAreaIdentifierDecodeError {
    fn source(&self) -> Option<&(dyn ::core::error::Error + 'static)> {
        if let ::core::option::Option::Some(::core::result::Result::Err(err)) = &self
            .context
            .format
        {
            return ::core::option::Option::Some(
                err as &(dyn ::core::error::Error + 'static),
            );
        }
        if let ::core::option::Option::Some(::core::result::Result::Err(err)) = &self
            .context
            .area
        {
            return ::core::option::Option::Some(
                err as &(dyn ::core::error::Error + 'static),
            );
        }
        ::core::option::Option::None
    }
    fn cause(&self) -> Option<&dyn ::core::error::Error> {
        self.source()
    }
}
impl crate::decode::owned::DecodeErrorType for BroadcastAreaIdentifier {
    type Error = BroadcastAreaIdentifierDecodeError;
}
impl crate::decode::owned::DecodeWithLength for BroadcastAreaIdentifier {
    fn decode(
        src: &mut ::bytes::BytesMut,
        length: usize,
    ) -> Result<(Self, usize), Self::Error> {
        let size = 0;
        let (format, size) = match crate::decode::owned::DecodeExt::decode_move(
            src,
            size,
        ) {
            Ok(ok) => ok,
            Err(err) => {
                let context = BroadcastAreaIdentifierDecodeErrorContext {
                    format: ::core::option::Option::Some(
                        ::core::result::Result::Err(err),
                    ),
                    area: ::core::option::Option::None,
                };
                return Err(Self::Error { context });
            }
        };
        let (area, size) = match crate::decode::owned::DecodeWithLengthExt::decode_move(
            src,
            length.saturating_sub(size),
            size,
        ) {
            Ok(ok) => ok,
            Err(err) => {
                let context = BroadcastAreaIdentifierDecodeErrorContext {
                    format: ::core::option::Option::Some(
                        ::core::result::Result::Ok(format),
                    ),
                    area: ::core::option::Option::Some(::core::result::Result::Err(err)),
                };
                return Err(Self::Error { context });
            }
        };
        Ok((Self { format, area }, size))
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
        let (format, size) = crate::decode::borrowed::DecodeExt::decode_move(src, size)?;
        let (area, size) = crate::decode::borrowed::DecodeWithLengthExt::decode_move(
            src,
            length.saturating_sub(size),
            size,
        )?;
        Ok((Self { format, area }, size))
    }
}
