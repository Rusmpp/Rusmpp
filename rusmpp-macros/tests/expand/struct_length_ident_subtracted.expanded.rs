/// Docs
///
/// More docs
#[rusmpp(decode = owned)]
pub struct Udh {
    /// Docs
    ///
    /// More docs
    pub length: u8,
    id: UdhId,
    /// Docs
    ///
    /// More docs
    #[rusmpp(key = id, length = length-1)]
    value: Option<UdhValue>,
}
#[automatically_derived]
impl ::core::fmt::Debug for Udh {
    #[inline]
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        ::core::fmt::Formatter::debug_struct_field3_finish(
            f,
            "Udh",
            "length",
            &self.length,
            "id",
            &self.id,
            "value",
            &&self.value,
        )
    }
}
pub struct UdhParts {
    pub length: u8,
    pub id: UdhId,
    pub value: Option<UdhValue>,
}
#[automatically_derived]
impl ::core::fmt::Debug for UdhParts {
    #[inline]
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        ::core::fmt::Formatter::debug_struct_field3_finish(
            f,
            "UdhParts",
            "length",
            &self.length,
            "id",
            &self.id,
            "value",
            &&self.value,
        )
    }
}
impl UdhParts {
    #[inline]
    #[allow(clippy::too_many_arguments)]
    pub const fn new(length: u8, id: UdhId, value: Option<UdhValue>) -> Self {
        Self { length, id, value }
    }
    #[inline]
    #[allow(unused_parens)]
    pub fn raw(self) -> (u8, UdhId, Option<UdhValue>) {
        (self.length, self.id, self.value)
    }
}
impl Udh {
    /// Converts [`Self`] into its parts.
    #[inline]
    pub fn into_parts(self) -> UdhParts {
        UdhParts {
            length: self.length,
            id: self.id,
            value: self.value,
        }
    }
    /// Creates a new instance of [`Self`] from its parts.
    ///
    /// # Note
    ///
    /// This may create invalid instances. It's up to the caller to ensure that the parts are valid.
    #[inline]
    pub fn from_parts(parts: UdhParts) -> Self {
        Self {
            length: parts.length,
            id: parts.id,
            value: parts.value,
        }
    }
}
impl crate::encode::Length for Udh {
    fn length(&self) -> usize {
        let mut length = 0;
        length += crate::encode::Length::length(&self.length);
        length += crate::encode::Length::length(&self.id);
        length += crate::encode::Length::length(&self.value);
        length
    }
}
impl crate::encode::Encode for Udh {
    fn encode(&self, dst: &mut [u8]) -> usize {
        let size = 0;
        let size = crate::encode::EncodeExt::encode_move(&self.length, dst, size);
        let size = crate::encode::EncodeExt::encode_move(&self.id, dst, size);
        let size = crate::encode::EncodeExt::encode_move(&self.value, dst, size);
        size
    }
}
#[non_exhaustive]
pub struct UdhDecodeErrorContext {
    pub length: ::core::option::Option<
        ::core::result::Result<u8, <u8 as crate::decode::owned::DecodeErrorType>::Error>,
    >,
    pub id: ::core::option::Option<
        ::core::result::Result<
            UdhId,
            <UdhId as crate::decode::owned::DecodeErrorType>::Error,
        >,
    >,
    pub value: ::core::option::Option<
        ::core::result::Result<
            Option<UdhValue>,
            <Option<UdhValue> as crate::decode::owned::DecodeErrorType>::Error,
        >,
    >,
}
#[automatically_derived]
impl ::core::fmt::Debug for UdhDecodeErrorContext {
    #[inline]
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        ::core::fmt::Formatter::debug_struct_field3_finish(
            f,
            "UdhDecodeErrorContext",
            "length",
            &self.length,
            "id",
            &self.id,
            "value",
            &&self.value,
        )
    }
}
#[automatically_derived]
impl ::core::clone::Clone for UdhDecodeErrorContext {
    #[inline]
    fn clone(&self) -> UdhDecodeErrorContext {
        UdhDecodeErrorContext {
            length: ::core::clone::Clone::clone(&self.length),
            id: ::core::clone::Clone::clone(&self.id),
            value: ::core::clone::Clone::clone(&self.value),
        }
    }
}
#[non_exhaustive]
pub struct UdhDecodeError {
    pub context: UdhDecodeErrorContext,
}
#[automatically_derived]
impl ::core::fmt::Debug for UdhDecodeError {
    #[inline]
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        ::core::fmt::Formatter::debug_struct_field1_finish(
            f,
            "UdhDecodeError",
            "context",
            &&self.context,
        )
    }
}
#[automatically_derived]
impl ::core::clone::Clone for UdhDecodeError {
    #[inline]
    fn clone(&self) -> UdhDecodeError {
        UdhDecodeError {
            context: ::core::clone::Clone::clone(&self.context),
        }
    }
}
impl ::core::fmt::Display for UdhDecodeError {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.write_fmt(format_args!("Failed to decode {0} {{ ", "Udh"))?;
        if let ::core::option::Option::Some(::core::result::Result::Err(err)) = &self
            .context
            .length
        {
            f.write_fmt(format_args!("{0}: {1}", "length", err))?;
            f.write_fmt(format_args!(" }}"))?;
            return Ok(());
        }
        if let ::core::option::Option::Some(::core::result::Result::Err(err)) = &self
            .context
            .id
        {
            f.write_fmt(format_args!("{0}: {1}", "id", err))?;
            f.write_fmt(format_args!(" }}"))?;
            return Ok(());
        }
        if let ::core::option::Option::Some(::core::result::Result::Err(err)) = &self
            .context
            .value
        {
            f.write_fmt(format_args!("{0}: {1}", "value", err))?;
            f.write_fmt(format_args!(" }}"))?;
            return Ok(());
        }
        f.write_fmt(format_args!(" }}"))
    }
}
impl ::core::error::Error for UdhDecodeError {
    fn source(&self) -> Option<&(dyn ::core::error::Error + 'static)> {
        if let ::core::option::Option::Some(::core::result::Result::Err(err)) = &self
            .context
            .length
        {
            return ::core::option::Option::Some(
                err as &(dyn ::core::error::Error + 'static),
            );
        }
        if let ::core::option::Option::Some(::core::result::Result::Err(err)) = &self
            .context
            .id
        {
            return ::core::option::Option::Some(
                err as &(dyn ::core::error::Error + 'static),
            );
        }
        if let ::core::option::Option::Some(::core::result::Result::Err(err)) = &self
            .context
            .value
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
impl crate::decode::owned::DecodeErrorType for Udh {
    type Error = UdhDecodeError;
}
impl crate::decode::owned::Decode for Udh {
    fn decode(src: &mut ::bytes::BytesMut) -> Result<(Self, usize), Self::Error> {
        let size = 0;
        let (length, size) = match crate::decode::owned::DecodeExt::decode_move(
            src,
            size,
        ) {
            Ok(ok) => ok,
            Err(err) => {
                let context = UdhDecodeErrorContext {
                    length: ::core::option::Option::Some(
                        ::core::result::Result::Err(err),
                    ),
                    id: ::core::option::Option::None,
                    value: ::core::option::Option::None,
                };
                return Err(Self::Error { context });
            }
        };
        let (id, size) = match crate::decode::owned::DecodeExt::decode_move(src, size) {
            Ok(ok) => ok,
            Err(err) => {
                let context = UdhDecodeErrorContext {
                    length: ::core::option::Option::Some(
                        ::core::result::Result::Ok(length),
                    ),
                    id: ::core::option::Option::Some(::core::result::Result::Err(err)),
                    value: ::core::option::Option::None,
                };
                return Err(Self::Error { context });
            }
        };
        let _length = (length as usize).saturating_sub(1usize);
        let opt = match crate::decode::owned::DecodeWithKeyExt::optional_length_checked_decode_move(
            id,
            src,
            _length,
            size,
        ) {
            Ok(ok) => ok,
            Err(err) => {
                let context = UdhDecodeErrorContext {
                    length: ::core::option::Option::Some(
                        ::core::result::Result::Ok(length),
                    ),
                    id: ::core::option::Option::Some(::core::result::Result::Ok(id)),
                    value: ::core::option::Option::Some(::core::result::Result::Err(err)),
                };
                return Err(Self::Error { context });
            }
        };
        let (value, size) = opt
            .map(|(this, size)| (Some(this), size))
            .unwrap_or((None, size));
        Ok((Self { length, id, value }, size))
    }
}
/// Docs
///
/// More docs
#[rusmpp(decode = borrowed)]
pub struct Udh<'a, const N: usize> {
    /// Docs
    ///
    /// More docs
    pub length: u8,
    id: UdhId,
    /// Docs
    ///
    /// More docs
    #[rusmpp(key = id, length = length-1)]
    value: Option<UdhValue<'a, N>>,
}
#[automatically_derived]
impl<'a, const N: usize> ::core::fmt::Debug for Udh<'a, N> {
    #[inline]
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        ::core::fmt::Formatter::debug_struct_field3_finish(
            f,
            "Udh",
            "length",
            &self.length,
            "id",
            &self.id,
            "value",
            &&self.value,
        )
    }
}
pub struct UdhParts<'a, const N: usize> {
    pub length: u8,
    pub id: UdhId,
    pub value: Option<UdhValue<'a, N>>,
}
#[automatically_derived]
impl<'a, const N: usize> ::core::fmt::Debug for UdhParts<'a, N> {
    #[inline]
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        ::core::fmt::Formatter::debug_struct_field3_finish(
            f,
            "UdhParts",
            "length",
            &self.length,
            "id",
            &self.id,
            "value",
            &&self.value,
        )
    }
}
impl<'a, const N: usize> UdhParts<'a, N> {
    #[inline]
    #[allow(clippy::too_many_arguments)]
    pub const fn new(length: u8, id: UdhId, value: Option<UdhValue<'a, N>>) -> Self {
        Self { length, id, value }
    }
    #[inline]
    #[allow(unused_parens)]
    pub fn raw(self) -> (u8, UdhId, Option<UdhValue<'a, N>>) {
        (self.length, self.id, self.value)
    }
}
impl<'a, const N: usize> Udh<'a, N> {
    /// Converts [`Self`] into its parts.
    #[inline]
    pub fn into_parts(self) -> UdhParts<'a, N> {
        UdhParts {
            length: self.length,
            id: self.id,
            value: self.value,
        }
    }
    /// Creates a new instance of [`Self`] from its parts.
    ///
    /// # Note
    ///
    /// This may create invalid instances. It's up to the caller to ensure that the parts are valid.
    #[inline]
    pub fn from_parts(parts: UdhParts<'a, N>) -> Self {
        Self {
            length: parts.length,
            id: parts.id,
            value: parts.value,
        }
    }
}
impl<'a, const N: usize> crate::encode::Length for Udh<'a, N> {
    fn length(&self) -> usize {
        let mut length = 0;
        length += crate::encode::Length::length(&self.length);
        length += crate::encode::Length::length(&self.id);
        length += crate::encode::Length::length(&self.value);
        length
    }
}
impl<'a, const N: usize> crate::encode::Encode for Udh<'a, N> {
    fn encode(&self, dst: &mut [u8]) -> usize {
        let size = 0;
        let size = crate::encode::EncodeExt::encode_move(&self.length, dst, size);
        let size = crate::encode::EncodeExt::encode_move(&self.id, dst, size);
        let size = crate::encode::EncodeExt::encode_move(&self.value, dst, size);
        size
    }
}
impl<'a, const N: usize> crate::decode::borrowed::Decode<'a> for Udh<'a, N> {
    fn decode(src: &'a [u8]) -> Result<(Self, usize), crate::decode::DecodeError> {
        let size = 0;
        let (length, size) = crate::decode::borrowed::DecodeExt::decode_move(src, size)?;
        let (id, size) = crate::decode::borrowed::DecodeExt::decode_move(src, size)?;
        let _length = (length as usize).saturating_sub(1usize);
        let (value, size) = crate::decode::borrowed::DecodeWithKeyExt::optional_length_checked_decode_move(
                id,
                src,
                _length,
                size,
            )?
            .map(|(this, size)| (Some(this), size))
            .unwrap_or((None, size));
        Ok((Self { length, id, value }, size))
    }
}
