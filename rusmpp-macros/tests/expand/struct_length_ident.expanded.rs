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
#[non_exhaustive]
pub struct SubmitSmDecodeErrorContext {
    pub other: ::core::option::Option<
        ::core::result::Result<u8, <u8 as crate::decode::owned::DecodeErrorType>::Error>,
    >,
    pub sm_length: ::core::option::Option<
        ::core::result::Result<u8, <u8 as crate::decode::owned::DecodeErrorType>::Error>,
    >,
    pub short_message: ::core::option::Option<
        ::core::result::Result<
            OctetString<0, 255>,
            <OctetString<0, 255> as crate::decode::owned::DecodeErrorType>::Error,
        >,
    >,
}
#[automatically_derived]
impl ::core::fmt::Debug for SubmitSmDecodeErrorContext {
    #[inline]
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        ::core::fmt::Formatter::debug_struct_field3_finish(
            f,
            "SubmitSmDecodeErrorContext",
            "other",
            &self.other,
            "sm_length",
            &self.sm_length,
            "short_message",
            &&self.short_message,
        )
    }
}
#[automatically_derived]
impl ::core::clone::Clone for SubmitSmDecodeErrorContext {
    #[inline]
    fn clone(&self) -> SubmitSmDecodeErrorContext {
        SubmitSmDecodeErrorContext {
            other: ::core::clone::Clone::clone(&self.other),
            sm_length: ::core::clone::Clone::clone(&self.sm_length),
            short_message: ::core::clone::Clone::clone(&self.short_message),
        }
    }
}
#[non_exhaustive]
pub struct SubmitSmDecodeError {
    pub context: SubmitSmDecodeErrorContext,
}
#[automatically_derived]
impl ::core::fmt::Debug for SubmitSmDecodeError {
    #[inline]
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        ::core::fmt::Formatter::debug_struct_field1_finish(
            f,
            "SubmitSmDecodeError",
            "context",
            &&self.context,
        )
    }
}
#[automatically_derived]
impl ::core::clone::Clone for SubmitSmDecodeError {
    #[inline]
    fn clone(&self) -> SubmitSmDecodeError {
        SubmitSmDecodeError {
            context: ::core::clone::Clone::clone(&self.context),
        }
    }
}
impl ::core::fmt::Display for SubmitSmDecodeError {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.write_fmt(format_args!("Failed to decode {0} {{ ", "SubmitSm"))?;
        if let ::core::option::Option::Some(::core::result::Result::Err(err)) = &self
            .context
            .other
        {
            f.write_fmt(format_args!("{0}: {1}", "other", err))?;
            f.write_fmt(format_args!(" }}"))?;
            return Ok(());
        }
        if let ::core::option::Option::Some(::core::result::Result::Err(err)) = &self
            .context
            .sm_length
        {
            f.write_fmt(format_args!("{0}: {1}", "sm_length", err))?;
            f.write_fmt(format_args!(" }}"))?;
            return Ok(());
        }
        if let ::core::option::Option::Some(::core::result::Result::Err(err)) = &self
            .context
            .short_message
        {
            f.write_fmt(format_args!("{0}: {1}", "short_message", err))?;
            f.write_fmt(format_args!(" }}"))?;
            return Ok(());
        }
        f.write_fmt(format_args!(" }}"))
    }
}
impl ::core::error::Error for SubmitSmDecodeError {
    fn source(&self) -> Option<&(dyn ::core::error::Error + 'static)> {
        if let ::core::option::Option::Some(::core::result::Result::Err(err)) = &self
            .context
            .other
        {
            return ::core::option::Option::Some(
                err as &(dyn ::core::error::Error + 'static),
            );
        }
        if let ::core::option::Option::Some(::core::result::Result::Err(err)) = &self
            .context
            .sm_length
        {
            return ::core::option::Option::Some(
                err as &(dyn ::core::error::Error + 'static),
            );
        }
        if let ::core::option::Option::Some(::core::result::Result::Err(err)) = &self
            .context
            .short_message
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
impl crate::decode::owned::DecodeErrorType for SubmitSm {
    type Error = SubmitSmDecodeError;
}
impl crate::decode::owned::DecodeWithLength for SubmitSm {
    fn decode(
        src: &mut ::bytes::BytesMut,
        length: usize,
    ) -> Result<(Self, usize), Self::Error> {
        let size = 0;
        let (other, size) = match crate::decode::owned::DecodeExt::decode_move(
            src,
            size,
        ) {
            Ok(ok) => ok,
            Err(err) => {
                let context = SubmitSmDecodeErrorContext {
                    other: ::core::option::Option::Some(
                        ::core::result::Result::Err(err),
                    ),
                    sm_length: ::core::option::Option::None,
                    short_message: ::core::option::Option::None,
                };
                return Err(Self::Error { context });
            }
        };
        let (sm_length, size) = match crate::decode::owned::DecodeExt::decode_move(
            src,
            size,
        ) {
            Ok(ok) => ok,
            Err(err) => {
                let context = SubmitSmDecodeErrorContext {
                    other: ::core::option::Option::Some(
                        ::core::result::Result::Ok(other),
                    ),
                    sm_length: ::core::option::Option::Some(
                        ::core::result::Result::Err(err),
                    ),
                    short_message: ::core::option::Option::None,
                };
                return Err(Self::Error { context });
            }
        };
        let (short_message, size) = match crate::decode::owned::DecodeWithLengthExt::decode_move(
            src,
            sm_length as usize,
            size,
        ) {
            Ok(ok) => ok,
            Err(err) => {
                let context = SubmitSmDecodeErrorContext {
                    other: ::core::option::Option::Some(
                        ::core::result::Result::Ok(other),
                    ),
                    sm_length: ::core::option::Option::Some(
                        ::core::result::Result::Ok(sm_length),
                    ),
                    short_message: ::core::option::Option::Some(
                        ::core::result::Result::Err(err),
                    ),
                };
                return Err(Self::Error { context });
            }
        };
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
        let (other, size) = crate::decode::borrowed::DecodeExt::decode_move(src, size)?;
        let (sm_length, size) = crate::decode::borrowed::DecodeExt::decode_move(
            src,
            size,
        )?;
        let (short_message, size) = crate::decode::borrowed::DecodeWithLengthExt::decode_move(
            src,
            sm_length as usize,
            size,
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
