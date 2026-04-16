/// Docs
///
/// More docs
#[rusmpp(decode = owned)]
pub struct Command {
    /// Docs
    ///
    /// More docs
    id: CommandId,
    pub command_status: CommandStatus,
    pub sequence_number: u32,
    /// Docs
    ///
    /// More docs
    #[rusmpp(key = id, length = "unchecked")]
    pdu: Option<Pdu>,
}
#[automatically_derived]
impl ::core::fmt::Debug for Command {
    #[inline]
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        ::core::fmt::Formatter::debug_struct_field4_finish(
            f,
            "Command",
            "id",
            &self.id,
            "command_status",
            &self.command_status,
            "sequence_number",
            &self.sequence_number,
            "pdu",
            &&self.pdu,
        )
    }
}
pub struct CommandParts {
    pub id: CommandId,
    pub command_status: CommandStatus,
    pub sequence_number: u32,
    pub pdu: Option<Pdu>,
}
#[automatically_derived]
impl ::core::fmt::Debug for CommandParts {
    #[inline]
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        ::core::fmt::Formatter::debug_struct_field4_finish(
            f,
            "CommandParts",
            "id",
            &self.id,
            "command_status",
            &self.command_status,
            "sequence_number",
            &self.sequence_number,
            "pdu",
            &&self.pdu,
        )
    }
}
impl CommandParts {
    #[inline]
    #[allow(clippy::too_many_arguments)]
    pub const fn new(
        id: CommandId,
        command_status: CommandStatus,
        sequence_number: u32,
        pdu: Option<Pdu>,
    ) -> Self {
        Self {
            id,
            command_status,
            sequence_number,
            pdu,
        }
    }
    #[inline]
    #[allow(unused_parens)]
    pub fn raw(self) -> (CommandId, CommandStatus, u32, Option<Pdu>) {
        (self.id, self.command_status, self.sequence_number, self.pdu)
    }
}
impl Command {
    /// Converts [`Self`] into its parts.
    #[inline]
    pub fn into_parts(self) -> CommandParts {
        CommandParts {
            id: self.id,
            command_status: self.command_status,
            sequence_number: self.sequence_number,
            pdu: self.pdu,
        }
    }
    /// Creates a new instance of [`Self`] from its parts.
    ///
    /// # Note
    ///
    /// This may create invalid instances. It's up to the caller to ensure that the parts are valid.
    #[inline]
    pub fn from_parts(parts: CommandParts) -> Self {
        Self {
            id: parts.id,
            command_status: parts.command_status,
            sequence_number: parts.sequence_number,
            pdu: parts.pdu,
        }
    }
}
impl crate::encode::Length for Command {
    fn length(&self) -> usize {
        let mut length = 0;
        length += crate::encode::Length::length(&self.id);
        length += crate::encode::Length::length(&self.command_status);
        length += crate::encode::Length::length(&self.sequence_number);
        length += crate::encode::Length::length(&self.pdu);
        length
    }
}
impl crate::encode::Encode for Command {
    fn encode(&self, dst: &mut [u8]) -> usize {
        let size = 0;
        let size = crate::encode::EncodeExt::encode_move(&self.id, dst, size);
        let size = crate::encode::EncodeExt::encode_move(
            &self.command_status,
            dst,
            size,
        );
        let size = crate::encode::EncodeExt::encode_move(
            &self.sequence_number,
            dst,
            size,
        );
        let size = crate::encode::EncodeExt::encode_move(&self.pdu, dst, size);
        size
    }
}
#[non_exhaustive]
pub struct CommandDecodeErrorContext {
    pub id: ::core::option::Option<
        ::core::result::Result<
            CommandId,
            <CommandId as crate::decode::owned::DecodeErrorType>::Error,
        >,
    >,
    pub command_status: ::core::option::Option<
        ::core::result::Result<
            CommandStatus,
            <CommandStatus as crate::decode::owned::DecodeErrorType>::Error,
        >,
    >,
    pub sequence_number: ::core::option::Option<
        ::core::result::Result<
            u32,
            <u32 as crate::decode::owned::DecodeErrorType>::Error,
        >,
    >,
    pub pdu: ::core::option::Option<
        ::core::result::Result<
            Option<Pdu>,
            <Option<Pdu> as crate::decode::owned::DecodeErrorType>::Error,
        >,
    >,
}
#[automatically_derived]
impl ::core::fmt::Debug for CommandDecodeErrorContext {
    #[inline]
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        ::core::fmt::Formatter::debug_struct_field4_finish(
            f,
            "CommandDecodeErrorContext",
            "id",
            &self.id,
            "command_status",
            &self.command_status,
            "sequence_number",
            &self.sequence_number,
            "pdu",
            &&self.pdu,
        )
    }
}
#[automatically_derived]
impl ::core::clone::Clone for CommandDecodeErrorContext {
    #[inline]
    fn clone(&self) -> CommandDecodeErrorContext {
        CommandDecodeErrorContext {
            id: ::core::clone::Clone::clone(&self.id),
            command_status: ::core::clone::Clone::clone(&self.command_status),
            sequence_number: ::core::clone::Clone::clone(&self.sequence_number),
            pdu: ::core::clone::Clone::clone(&self.pdu),
        }
    }
}
#[non_exhaustive]
pub struct CommandDecodeError {
    pub context: CommandDecodeErrorContext,
}
#[automatically_derived]
impl ::core::fmt::Debug for CommandDecodeError {
    #[inline]
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        ::core::fmt::Formatter::debug_struct_field1_finish(
            f,
            "CommandDecodeError",
            "context",
            &&self.context,
        )
    }
}
#[automatically_derived]
impl ::core::clone::Clone for CommandDecodeError {
    #[inline]
    fn clone(&self) -> CommandDecodeError {
        CommandDecodeError {
            context: ::core::clone::Clone::clone(&self.context),
        }
    }
}
impl ::core::fmt::Display for CommandDecodeError {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.write_fmt(format_args!("Failed to decode {0} {{ ", "Command"))?;
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
            .command_status
        {
            f.write_fmt(format_args!("{0}: {1}", "command_status", err))?;
            f.write_fmt(format_args!(" }}"))?;
            return Ok(());
        }
        if let ::core::option::Option::Some(::core::result::Result::Err(err)) = &self
            .context
            .sequence_number
        {
            f.write_fmt(format_args!("{0}: {1}", "sequence_number", err))?;
            f.write_fmt(format_args!(" }}"))?;
            return Ok(());
        }
        if let ::core::option::Option::Some(::core::result::Result::Err(err)) = &self
            .context
            .pdu
        {
            f.write_fmt(format_args!("{0}: {1}", "pdu", err))?;
            f.write_fmt(format_args!(" }}"))?;
            return Ok(());
        }
        f.write_fmt(format_args!(" }}"))
    }
}
impl ::core::error::Error for CommandDecodeError {
    fn source(&self) -> Option<&(dyn ::core::error::Error + 'static)> {
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
            .command_status
        {
            return ::core::option::Option::Some(
                err as &(dyn ::core::error::Error + 'static),
            );
        }
        if let ::core::option::Option::Some(::core::result::Result::Err(err)) = &self
            .context
            .sequence_number
        {
            return ::core::option::Option::Some(
                err as &(dyn ::core::error::Error + 'static),
            );
        }
        if let ::core::option::Option::Some(::core::result::Result::Err(err)) = &self
            .context
            .pdu
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
impl crate::decode::owned::DecodeErrorType for Command {
    type Error = CommandDecodeError;
}
impl crate::decode::owned::DecodeWithLength for Command {
    fn decode(
        src: &mut ::bytes::BytesMut,
        length: usize,
    ) -> Result<(Self, usize), Self::Error> {
        let size = 0;
        let (id, size) = match crate::decode::owned::DecodeExt::decode_move(src, size) {
            Ok(ok) => ok,
            Err(err) => {
                let context = CommandDecodeErrorContext {
                    id: ::core::option::Option::Some(::core::result::Result::Err(err)),
                    command_status: ::core::option::Option::None,
                    sequence_number: ::core::option::Option::None,
                    pdu: ::core::option::Option::None,
                };
                return Err(Self::Error { context });
            }
        };
        let (command_status, size) = match crate::decode::owned::DecodeExt::decode_move(
            src,
            size,
        ) {
            Ok(ok) => ok,
            Err(err) => {
                let context = CommandDecodeErrorContext {
                    id: ::core::option::Option::Some(::core::result::Result::Ok(id)),
                    command_status: ::core::option::Option::Some(
                        ::core::result::Result::Err(err),
                    ),
                    sequence_number: ::core::option::Option::None,
                    pdu: ::core::option::Option::None,
                };
                return Err(Self::Error { context });
            }
        };
        let (sequence_number, size) = match crate::decode::owned::DecodeExt::decode_move(
            src,
            size,
        ) {
            Ok(ok) => ok,
            Err(err) => {
                let context = CommandDecodeErrorContext {
                    id: ::core::option::Option::Some(::core::result::Result::Ok(id)),
                    command_status: ::core::option::Option::Some(
                        ::core::result::Result::Ok(command_status),
                    ),
                    sequence_number: ::core::option::Option::Some(
                        ::core::result::Result::Err(err),
                    ),
                    pdu: ::core::option::Option::None,
                };
                return Err(Self::Error { context });
            }
        };
        let opt = match crate::decode::owned::DecodeWithKeyOptionalExt::decode_move(
            id,
            src,
            length.saturating_sub(size),
            size,
        ) {
            Ok(ok) => ok,
            Err(err) => {
                let context = CommandDecodeErrorContext {
                    id: ::core::option::Option::Some(::core::result::Result::Ok(id)),
                    command_status: ::core::option::Option::Some(
                        ::core::result::Result::Ok(command_status),
                    ),
                    sequence_number: ::core::option::Option::Some(
                        ::core::result::Result::Ok(sequence_number),
                    ),
                    pdu: ::core::option::Option::Some(::core::result::Result::Err(err)),
                };
                return Err(Self::Error { context });
            }
        };
        let (pdu, size) = opt
            .map(|(this, size)| (Some(this), size))
            .unwrap_or((None, size));
        Ok((
            Self {
                id,
                command_status,
                sequence_number,
                pdu,
            },
            size,
        ))
    }
}
/// Docs
///
/// More docs
#[rusmpp(decode = borrowed)]
pub struct Command<'a, const N: usize> {
    /// Docs
    ///
    /// More docs
    id: CommandId,
    pub command_status: CommandStatus,
    pub sequence_number: u32,
    /// Docs
    ///
    /// More docs
    #[rusmpp(key = id, length = "unchecked")]
    pdu: Option<Pdu<'a, N>>,
}
#[automatically_derived]
impl<'a, const N: usize> ::core::fmt::Debug for Command<'a, N> {
    #[inline]
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        ::core::fmt::Formatter::debug_struct_field4_finish(
            f,
            "Command",
            "id",
            &self.id,
            "command_status",
            &self.command_status,
            "sequence_number",
            &self.sequence_number,
            "pdu",
            &&self.pdu,
        )
    }
}
pub struct CommandParts<'a, const N: usize> {
    pub id: CommandId,
    pub command_status: CommandStatus,
    pub sequence_number: u32,
    pub pdu: Option<Pdu<'a, N>>,
}
#[automatically_derived]
impl<'a, const N: usize> ::core::fmt::Debug for CommandParts<'a, N> {
    #[inline]
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        ::core::fmt::Formatter::debug_struct_field4_finish(
            f,
            "CommandParts",
            "id",
            &self.id,
            "command_status",
            &self.command_status,
            "sequence_number",
            &self.sequence_number,
            "pdu",
            &&self.pdu,
        )
    }
}
impl<'a, const N: usize> CommandParts<'a, N> {
    #[inline]
    #[allow(clippy::too_many_arguments)]
    pub const fn new(
        id: CommandId,
        command_status: CommandStatus,
        sequence_number: u32,
        pdu: Option<Pdu<'a, N>>,
    ) -> Self {
        Self {
            id,
            command_status,
            sequence_number,
            pdu,
        }
    }
    #[inline]
    #[allow(unused_parens)]
    pub fn raw(self) -> (CommandId, CommandStatus, u32, Option<Pdu<'a, N>>) {
        (self.id, self.command_status, self.sequence_number, self.pdu)
    }
}
impl<'a, const N: usize> Command<'a, N> {
    /// Converts [`Self`] into its parts.
    #[inline]
    pub fn into_parts(self) -> CommandParts<'a, N> {
        CommandParts {
            id: self.id,
            command_status: self.command_status,
            sequence_number: self.sequence_number,
            pdu: self.pdu,
        }
    }
    /// Creates a new instance of [`Self`] from its parts.
    ///
    /// # Note
    ///
    /// This may create invalid instances. It's up to the caller to ensure that the parts are valid.
    #[inline]
    pub fn from_parts(parts: CommandParts<'a, N>) -> Self {
        Self {
            id: parts.id,
            command_status: parts.command_status,
            sequence_number: parts.sequence_number,
            pdu: parts.pdu,
        }
    }
}
impl<'a, const N: usize> crate::encode::Length for Command<'a, N> {
    fn length(&self) -> usize {
        let mut length = 0;
        length += crate::encode::Length::length(&self.id);
        length += crate::encode::Length::length(&self.command_status);
        length += crate::encode::Length::length(&self.sequence_number);
        length += crate::encode::Length::length(&self.pdu);
        length
    }
}
impl<'a, const N: usize> crate::encode::Encode for Command<'a, N> {
    fn encode(&self, dst: &mut [u8]) -> usize {
        let size = 0;
        let size = crate::encode::EncodeExt::encode_move(&self.id, dst, size);
        let size = crate::encode::EncodeExt::encode_move(
            &self.command_status,
            dst,
            size,
        );
        let size = crate::encode::EncodeExt::encode_move(
            &self.sequence_number,
            dst,
            size,
        );
        let size = crate::encode::EncodeExt::encode_move(&self.pdu, dst, size);
        size
    }
}
impl<'a, const N: usize> crate::decode::borrowed::DecodeWithLength<'a>
for Command<'a, N> {
    fn decode(
        src: &'a [u8],
        length: usize,
    ) -> Result<(Self, usize), crate::decode::DecodeError> {
        let size = 0;
        let (id, size) = crate::decode::borrowed::DecodeExt::decode_move(src, size)?;
        let (command_status, size) = crate::decode::borrowed::DecodeExt::decode_move(
            src,
            size,
        )?;
        let (sequence_number, size) = crate::decode::borrowed::DecodeExt::decode_move(
            src,
            size,
        )?;
        let (pdu, size) = crate::decode::borrowed::DecodeWithKeyOptionalExt::decode_move(
                id,
                src,
                length.saturating_sub(size),
                size,
            )?
            .map(|(this, size)| (Some(this), size))
            .unwrap_or((None, size));
        Ok((
            Self {
                id,
                command_status,
                sequence_number,
                pdu,
            },
            size,
        ))
    }
}
