/// Docs
///
/// More docs
#[rusmpp(decode = owned, test = skip)]
pub struct SubmitSmResp {
    /// Docs
    ///
    /// More docs
    message_id: COctetString<1, 65>,
}
#[automatically_derived]
impl ::core::fmt::Debug for SubmitSmResp {
    #[inline]
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        ::core::fmt::Formatter::debug_struct_field1_finish(
            f,
            "SubmitSmResp",
            "message_id",
            &&self.message_id,
        )
    }
}
pub struct SubmitSmRespParts {
    pub message_id: COctetString<1, 65>,
}
#[automatically_derived]
impl ::core::fmt::Debug for SubmitSmRespParts {
    #[inline]
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        ::core::fmt::Formatter::debug_struct_field1_finish(
            f,
            "SubmitSmRespParts",
            "message_id",
            &&self.message_id,
        )
    }
}
impl SubmitSmRespParts {
    #[inline]
    #[allow(clippy::too_many_arguments)]
    pub const fn new(message_id: COctetString<1, 65>) -> Self {
        Self { message_id }
    }
    #[inline]
    #[allow(unused_parens)]
    pub fn raw(self) -> (COctetString<1, 65>) {
        (self.message_id)
    }
}
impl SubmitSmResp {
    /// Converts [`Self`] into its parts.
    #[inline]
    pub fn into_parts(self) -> SubmitSmRespParts {
        SubmitSmRespParts {
            message_id: self.message_id,
        }
    }
    /// Creates a new instance of [`Self`] from its parts.
    ///
    /// # Note
    ///
    /// This may create invalid instances. It's up to the caller to ensure that the parts are valid.
    #[inline]
    pub fn from_parts(parts: SubmitSmRespParts) -> Self {
        Self {
            message_id: parts.message_id,
        }
    }
}
impl crate::encode::Length for SubmitSmResp {
    fn length(&self) -> usize {
        let mut length = 0;
        length += crate::encode::Length::length(&self.message_id);
        length
    }
}
impl crate::encode::Encode for SubmitSmResp {
    fn encode(&self, dst: &mut [u8]) -> usize {
        let size = 0;
        let size = crate::encode::EncodeExt::encode_move(&self.message_id, dst, size);
        size
    }
}
#[non_exhaustive]
pub struct SubmitSmRespDecodeErrorContext {
    pub message_id: ::core::option::Option<
        ::core::result::Result<
            COctetString<1, 65>,
            <COctetString<1, 65> as crate::decode::owned::DecodeErrorType>::Error,
        >,
    >,
}
#[automatically_derived]
impl ::core::fmt::Debug for SubmitSmRespDecodeErrorContext {
    #[inline]
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        ::core::fmt::Formatter::debug_struct_field1_finish(
            f,
            "SubmitSmRespDecodeErrorContext",
            "message_id",
            &&self.message_id,
        )
    }
}
#[automatically_derived]
impl ::core::clone::Clone for SubmitSmRespDecodeErrorContext {
    #[inline]
    fn clone(&self) -> SubmitSmRespDecodeErrorContext {
        SubmitSmRespDecodeErrorContext {
            message_id: ::core::clone::Clone::clone(&self.message_id),
        }
    }
}
#[non_exhaustive]
pub struct SubmitSmRespDecodeError {
    pub context: SubmitSmRespDecodeErrorContext,
}
#[automatically_derived]
impl ::core::fmt::Debug for SubmitSmRespDecodeError {
    #[inline]
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        ::core::fmt::Formatter::debug_struct_field1_finish(
            f,
            "SubmitSmRespDecodeError",
            "context",
            &&self.context,
        )
    }
}
#[automatically_derived]
impl ::core::clone::Clone for SubmitSmRespDecodeError {
    #[inline]
    fn clone(&self) -> SubmitSmRespDecodeError {
        SubmitSmRespDecodeError {
            context: ::core::clone::Clone::clone(&self.context),
        }
    }
}
impl ::core::fmt::Display for SubmitSmRespDecodeError {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.write_fmt(format_args!("Failed to decode {0} {{ ", "SubmitSmResp"))?;
        if let ::core::option::Option::Some(::core::result::Result::Err(err)) = &self
            .context
            .message_id
        {
            f.write_fmt(format_args!("{0}: {1}", "message_id", err))?;
            f.write_fmt(format_args!(" }}"))?;
            return Ok(());
        }
        f.write_fmt(format_args!(" }}"))
    }
}
impl ::core::error::Error for SubmitSmRespDecodeError {
    fn source(&self) -> Option<&(dyn ::core::error::Error + 'static)> {
        if let ::core::option::Option::Some(::core::result::Result::Err(err)) = &self
            .context
            .message_id
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
impl crate::decode::owned::DecodeErrorType for SubmitSmResp {
    type Error = SubmitSmRespDecodeError;
}
impl crate::decode::owned::Decode for SubmitSmResp {
    fn decode(src: &mut ::bytes::BytesMut) -> Result<(Self, usize), Self::Error> {
        let size = 0;
        let (message_id, size) = match crate::decode::owned::DecodeExt::decode_move(
            src,
            size,
        ) {
            Ok(ok) => ok,
            Err(err) => {
                let context = SubmitSmRespDecodeErrorContext {
                    message_id: ::core::option::Option::Some(
                        ::core::result::Result::Err(err),
                    ),
                };
                return Err(Self::Error { context });
            }
        };
        Ok((Self { message_id }, size))
    }
}
/// Docs
///
/// More docs
#[rusmpp(decode = borrowed, test = skip)]
pub struct SubmitSmResp<'a> {
    /// Docs
    ///
    /// More docs
    message_id: COctetString<'a, 1, 65>,
}
#[automatically_derived]
impl<'a> ::core::fmt::Debug for SubmitSmResp<'a> {
    #[inline]
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        ::core::fmt::Formatter::debug_struct_field1_finish(
            f,
            "SubmitSmResp",
            "message_id",
            &&self.message_id,
        )
    }
}
pub struct SubmitSmRespParts<'a> {
    pub message_id: COctetString<'a, 1, 65>,
}
#[automatically_derived]
impl<'a> ::core::fmt::Debug for SubmitSmRespParts<'a> {
    #[inline]
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        ::core::fmt::Formatter::debug_struct_field1_finish(
            f,
            "SubmitSmRespParts",
            "message_id",
            &&self.message_id,
        )
    }
}
impl<'a> SubmitSmRespParts<'a> {
    #[inline]
    #[allow(clippy::too_many_arguments)]
    pub const fn new(message_id: COctetString<'a, 1, 65>) -> Self {
        Self { message_id }
    }
    #[inline]
    #[allow(unused_parens)]
    pub fn raw(self) -> (COctetString<'a, 1, 65>) {
        (self.message_id)
    }
}
impl<'a> SubmitSmResp<'a> {
    /// Converts [`Self`] into its parts.
    #[inline]
    pub fn into_parts(self) -> SubmitSmRespParts<'a> {
        SubmitSmRespParts {
            message_id: self.message_id,
        }
    }
    /// Creates a new instance of [`Self`] from its parts.
    ///
    /// # Note
    ///
    /// This may create invalid instances. It's up to the caller to ensure that the parts are valid.
    #[inline]
    pub fn from_parts(parts: SubmitSmRespParts<'a>) -> Self {
        Self {
            message_id: parts.message_id,
        }
    }
}
impl<'a> crate::encode::Length for SubmitSmResp<'a> {
    fn length(&self) -> usize {
        let mut length = 0;
        length += crate::encode::Length::length(&self.message_id);
        length
    }
}
impl<'a> crate::encode::Encode for SubmitSmResp<'a> {
    fn encode(&self, dst: &mut [u8]) -> usize {
        let size = 0;
        let size = crate::encode::EncodeExt::encode_move(&self.message_id, dst, size);
        size
    }
}
impl<'a> crate::decode::borrowed::Decode<'a> for SubmitSmResp<'a> {
    fn decode(src: &'a [u8]) -> Result<(Self, usize), crate::decode::DecodeError> {
        let size = 0;
        let (message_id, size) = crate::decode::borrowed::DecodeExt::decode_move(
            src,
            size,
        )?;
        Ok((Self { message_id }, size))
    }
}
