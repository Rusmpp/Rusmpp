/// Docs
///
/// More docs
#[rusmpp(decode = owned)]
pub struct SubmitMulti {
    /// Docs
    ///
    /// More docs
    pub other: u8,
    number_of_dests: u8,
    /// Docs
    ///
    /// More docs
    #[rusmpp(count = number_of_dests)]
    dest_address: ::alloc::vec::Vec<DestAddress>,
}
#[automatically_derived]
impl ::core::fmt::Debug for SubmitMulti {
    #[inline]
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        ::core::fmt::Formatter::debug_struct_field3_finish(
            f,
            "SubmitMulti",
            "other",
            &self.other,
            "number_of_dests",
            &self.number_of_dests,
            "dest_address",
            &&self.dest_address,
        )
    }
}
pub struct SubmitMultiParts {
    pub other: u8,
    pub number_of_dests: u8,
    pub dest_address: ::alloc::vec::Vec<DestAddress>,
}
#[automatically_derived]
impl ::core::fmt::Debug for SubmitMultiParts {
    #[inline]
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        ::core::fmt::Formatter::debug_struct_field3_finish(
            f,
            "SubmitMultiParts",
            "other",
            &self.other,
            "number_of_dests",
            &self.number_of_dests,
            "dest_address",
            &&self.dest_address,
        )
    }
}
impl SubmitMultiParts {
    #[inline]
    #[allow(clippy::too_many_arguments)]
    pub const fn new(
        other: u8,
        number_of_dests: u8,
        dest_address: ::alloc::vec::Vec<DestAddress>,
    ) -> Self {
        Self {
            other,
            number_of_dests,
            dest_address,
        }
    }
    #[inline]
    #[allow(unused_parens)]
    pub fn raw(self) -> (u8, u8, ::alloc::vec::Vec<DestAddress>) {
        (self.other, self.number_of_dests, self.dest_address)
    }
}
impl SubmitMulti {
    /// Converts [`Self`] into its parts.
    #[inline]
    pub fn into_parts(self) -> SubmitMultiParts {
        SubmitMultiParts {
            other: self.other,
            number_of_dests: self.number_of_dests,
            dest_address: self.dest_address,
        }
    }
    /// Creates a new instance of [`Self`] from its parts.
    ///
    /// # Note
    ///
    /// This may create invalid instances. It's up to the caller to ensure that the parts are valid.
    #[inline]
    pub fn from_parts(parts: SubmitMultiParts) -> Self {
        Self {
            other: parts.other,
            number_of_dests: parts.number_of_dests,
            dest_address: parts.dest_address,
        }
    }
}
impl crate::encode::Length for SubmitMulti {
    fn length(&self) -> usize {
        let mut length = 0;
        length += crate::encode::Length::length(&self.other);
        length += crate::encode::Length::length(&self.number_of_dests);
        length += crate::encode::Length::length(&self.dest_address);
        length
    }
}
impl crate::encode::Encode for SubmitMulti {
    fn encode(&self, dst: &mut [u8]) -> usize {
        let size = 0;
        let size = crate::encode::EncodeExt::encode_move(&self.other, dst, size);
        let size = crate::encode::EncodeExt::encode_move(
            &self.number_of_dests,
            dst,
            size,
        );
        let size = crate::encode::EncodeExt::encode_move(&self.dest_address, dst, size);
        size
    }
}
#[non_exhaustive]
pub struct SubmitMultiDecodeErrorContext {
    pub other: ::core::option::Option<
        ::core::result::Result<u8, <u8 as crate::decode::owned::DecodeErrorType>::Error>,
    >,
    pub number_of_dests: ::core::option::Option<
        ::core::result::Result<u8, <u8 as crate::decode::owned::DecodeErrorType>::Error>,
    >,
    pub dest_address: ::core::option::Option<
        ::core::result::Result<
            ::alloc::vec::Vec<DestAddress>,
            <DestAddress as crate::decode::owned::DecodeErrorType>::Error,
        >,
    >,
}
#[automatically_derived]
impl ::core::fmt::Debug for SubmitMultiDecodeErrorContext {
    #[inline]
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        ::core::fmt::Formatter::debug_struct_field3_finish(
            f,
            "SubmitMultiDecodeErrorContext",
            "other",
            &self.other,
            "number_of_dests",
            &self.number_of_dests,
            "dest_address",
            &&self.dest_address,
        )
    }
}
#[automatically_derived]
impl ::core::clone::Clone for SubmitMultiDecodeErrorContext {
    #[inline]
    fn clone(&self) -> SubmitMultiDecodeErrorContext {
        SubmitMultiDecodeErrorContext {
            other: ::core::clone::Clone::clone(&self.other),
            number_of_dests: ::core::clone::Clone::clone(&self.number_of_dests),
            dest_address: ::core::clone::Clone::clone(&self.dest_address),
        }
    }
}
#[non_exhaustive]
pub struct SubmitMultiDecodeError {
    pub context: SubmitMultiDecodeErrorContext,
}
#[automatically_derived]
impl ::core::fmt::Debug for SubmitMultiDecodeError {
    #[inline]
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        ::core::fmt::Formatter::debug_struct_field1_finish(
            f,
            "SubmitMultiDecodeError",
            "context",
            &&self.context,
        )
    }
}
#[automatically_derived]
impl ::core::clone::Clone for SubmitMultiDecodeError {
    #[inline]
    fn clone(&self) -> SubmitMultiDecodeError {
        SubmitMultiDecodeError {
            context: ::core::clone::Clone::clone(&self.context),
        }
    }
}
impl ::core::fmt::Display for SubmitMultiDecodeError {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.write_fmt(format_args!("Failed to decode {0} {{ ", "SubmitMulti"))?;
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
            .number_of_dests
        {
            f.write_fmt(format_args!("{0}: {1}", "number_of_dests", err))?;
            f.write_fmt(format_args!(" }}"))?;
            return Ok(());
        }
        if let ::core::option::Option::Some(::core::result::Result::Err(err)) = &self
            .context
            .dest_address
        {
            f.write_fmt(format_args!("{0}: {1}", "dest_address", err))?;
            f.write_fmt(format_args!(" }}"))?;
            return Ok(());
        }
        f.write_fmt(format_args!(" }}"))
    }
}
impl ::core::error::Error for SubmitMultiDecodeError {
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
            .number_of_dests
        {
            return ::core::option::Option::Some(
                err as &(dyn ::core::error::Error + 'static),
            );
        }
        if let ::core::option::Option::Some(::core::result::Result::Err(err)) = &self
            .context
            .dest_address
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
impl crate::decode::owned::DecodeErrorType for SubmitMulti {
    type Error = SubmitMultiDecodeError;
}
impl crate::decode::owned::DecodeWithLength for SubmitMulti {
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
                let context = SubmitMultiDecodeErrorContext {
                    other: ::core::option::Option::Some(
                        ::core::result::Result::Err(err),
                    ),
                    number_of_dests: ::core::option::Option::None,
                    dest_address: ::core::option::Option::None,
                };
                return Err(Self::Error { context });
            }
        };
        let (number_of_dests, size) = match crate::decode::owned::DecodeExt::decode_move(
            src,
            size,
        ) {
            Ok(ok) => ok,
            Err(err) => {
                let context = SubmitMultiDecodeErrorContext {
                    other: ::core::option::Option::Some(
                        ::core::result::Result::Ok(other),
                    ),
                    number_of_dests: ::core::option::Option::Some(
                        ::core::result::Result::Err(err),
                    ),
                    dest_address: ::core::option::Option::None,
                };
                return Err(Self::Error { context });
            }
        };
        let (dest_address, size) = match crate::decode::owned::DecodeExt::counted_move(
            src,
            number_of_dests as usize,
            size,
        ) {
            Ok(ok) => ok,
            Err(err) => {
                let context = SubmitMultiDecodeErrorContext {
                    other: ::core::option::Option::Some(
                        ::core::result::Result::Ok(other),
                    ),
                    number_of_dests: ::core::option::Option::Some(
                        ::core::result::Result::Ok(number_of_dests),
                    ),
                    dest_address: ::core::option::Option::Some(
                        ::core::result::Result::Err(err),
                    ),
                };
                return Err(Self::Error { context });
            }
        };
        Ok((
            Self {
                other,
                number_of_dests,
                dest_address,
            },
            size,
        ))
    }
}
/// Docs
///
/// More docs
#[rusmpp(decode = borrowed)]
pub struct SubmitMulti<'a, const N: usize> {
    /// Docs
    ///
    /// More docs
    pub other: u8,
    number_of_dests: u8,
    /// Docs
    ///
    /// More docs
    #[rusmpp(count = number_of_dests)]
    dest_address: ::heapless::vec::Vec<DestAddress<'a>, N>,
}
#[automatically_derived]
impl<'a, const N: usize> ::core::fmt::Debug for SubmitMulti<'a, N> {
    #[inline]
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        ::core::fmt::Formatter::debug_struct_field3_finish(
            f,
            "SubmitMulti",
            "other",
            &self.other,
            "number_of_dests",
            &self.number_of_dests,
            "dest_address",
            &&self.dest_address,
        )
    }
}
pub struct SubmitMultiParts<'a, const N: usize> {
    pub other: u8,
    pub number_of_dests: u8,
    pub dest_address: ::heapless::vec::Vec<DestAddress<'a>, N>,
}
#[automatically_derived]
impl<'a, const N: usize> ::core::fmt::Debug for SubmitMultiParts<'a, N> {
    #[inline]
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        ::core::fmt::Formatter::debug_struct_field3_finish(
            f,
            "SubmitMultiParts",
            "other",
            &self.other,
            "number_of_dests",
            &self.number_of_dests,
            "dest_address",
            &&self.dest_address,
        )
    }
}
impl<'a, const N: usize> SubmitMultiParts<'a, N> {
    #[inline]
    #[allow(clippy::too_many_arguments)]
    pub const fn new(
        other: u8,
        number_of_dests: u8,
        dest_address: ::heapless::vec::Vec<DestAddress<'a>, N>,
    ) -> Self {
        Self {
            other,
            number_of_dests,
            dest_address,
        }
    }
    #[inline]
    #[allow(unused_parens)]
    pub fn raw(self) -> (u8, u8, ::heapless::vec::Vec<DestAddress<'a>, N>) {
        (self.other, self.number_of_dests, self.dest_address)
    }
}
impl<'a, const N: usize> SubmitMulti<'a, N> {
    /// Converts [`Self`] into its parts.
    #[inline]
    pub fn into_parts(self) -> SubmitMultiParts<'a, N> {
        SubmitMultiParts {
            other: self.other,
            number_of_dests: self.number_of_dests,
            dest_address: self.dest_address,
        }
    }
    /// Creates a new instance of [`Self`] from its parts.
    ///
    /// # Note
    ///
    /// This may create invalid instances. It's up to the caller to ensure that the parts are valid.
    #[inline]
    pub fn from_parts(parts: SubmitMultiParts<'a, N>) -> Self {
        Self {
            other: parts.other,
            number_of_dests: parts.number_of_dests,
            dest_address: parts.dest_address,
        }
    }
}
impl<'a, const N: usize> crate::encode::Length for SubmitMulti<'a, N> {
    fn length(&self) -> usize {
        let mut length = 0;
        length += crate::encode::Length::length(&self.other);
        length += crate::encode::Length::length(&self.number_of_dests);
        length += crate::encode::Length::length(&self.dest_address);
        length
    }
}
impl<'a, const N: usize> crate::encode::Encode for SubmitMulti<'a, N> {
    fn encode(&self, dst: &mut [u8]) -> usize {
        let size = 0;
        let size = crate::encode::EncodeExt::encode_move(&self.other, dst, size);
        let size = crate::encode::EncodeExt::encode_move(
            &self.number_of_dests,
            dst,
            size,
        );
        let size = crate::encode::EncodeExt::encode_move(&self.dest_address, dst, size);
        size
    }
}
impl<'a, const N: usize> crate::decode::borrowed::DecodeWithLength<'a>
for SubmitMulti<'a, N> {
    fn decode(
        src: &'a [u8],
        length: usize,
    ) -> Result<(Self, usize), crate::decode::DecodeError> {
        let size = 0;
        let (other, size) = crate::decode::borrowed::DecodeExt::decode_move(src, size)?;
        let (number_of_dests, size) = crate::decode::borrowed::DecodeExt::decode_move(
            src,
            size,
        )?;
        let (dest_address, size) = crate::decode::borrowed::DecodeExt::counted_move(
            src,
            number_of_dests as usize,
            size,
        )?;
        Ok((
            Self {
                other,
                number_of_dests,
                dest_address,
            },
            size,
        ))
    }
}
