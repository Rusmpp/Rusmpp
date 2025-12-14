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
        let (id, size) = crate::decode::DecodeErrorExt::map_as_source(
            crate::decode::borrowed::DecodeExt::decode_move(src, size),
            crate::fields::SmppField::id,
        )?;
        let (command_status, size) = crate::decode::DecodeErrorExt::map_as_source(
            crate::decode::borrowed::DecodeExt::decode_move(src, size),
            crate::fields::SmppField::command_status,
        )?;
        let (sequence_number, size) = crate::decode::DecodeErrorExt::map_as_source(
            crate::decode::borrowed::DecodeExt::decode_move(src, size),
            crate::fields::SmppField::sequence_number,
        )?;
        let (pdu, size) = crate::decode::DecodeErrorExt::map_as_source(
                crate::decode::borrowed::DecodeWithKeyOptionalExt::decode_move(
                    id,
                    src,
                    length.saturating_sub(size),
                    size,
                ),
                crate::fields::SmppField::pdu,
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
