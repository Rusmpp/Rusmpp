//! Tokio's util [`Encoder`] and [`Decoder`] implementations for [`CommandCodec`].

use core::num::TryFromIntError;

use bytes::Buf;
use tokio_util::{
    bytes::{BufMut, BytesMut},
    codec::{Decoder, Encoder},
};

use crate::{
    command::owned::Command,
    decode::owned::DecodeWithLength,
    encode::{Length, owned::Encode},
    logging::{debug, error, trace},
};

#[cfg(test)]
mod tests;

#[derive(Debug)]
enum DecodeState {
    /// Decoding the command length.
    Length,
    /// Decoding the command (Header without length + Body).
    Command { command_length: usize },
}

/// Codec for encoding and decoding `SMPP` PDUs.
#[derive(Debug)]
pub struct CommandCodec {
    max_length: Option<usize>,
    state: DecodeState,
}

impl CommandCodec {
    /// Creates a new [`CommandCodec`] with a default maximum length of `8192` bytes.
    #[inline]
    pub const fn new() -> Self {
        Self {
            max_length: Some(8192),
            state: DecodeState::Length,
        }
    }

    #[inline]
    pub const fn max_length(&self) -> Option<usize> {
        self.max_length
    }

    #[inline]
    pub fn with_max_length(mut self, max_length: usize) -> Self {
        self.max_length = Some(max_length);
        self
    }

    #[inline]
    pub fn without_max_length(mut self) -> Self {
        self.max_length = None;
        self
    }

    /// Sets the decoder state to decode the command length.
    #[inline]
    const fn decode_length(&mut self) {
        self.state = DecodeState::Length;
    }

    /// Sets the decoder state to decode the rest of the command.
    #[inline]
    const fn decode_command(&mut self, command_length: usize) {
        self.state = DecodeState::Command { command_length };
    }
}

impl Default for CommandCodec {
    fn default() -> Self {
        Self::new()
    }
}

/// An error that can occur when encoding a `Command`.
#[derive(Debug)]
#[non_exhaustive]
pub enum EncodeError {
    /// I/O error.
    Io(std::io::Error),
}

impl From<std::io::Error> for EncodeError {
    fn from(e: std::io::Error) -> Self {
        EncodeError::Io(e)
    }
}

impl core::fmt::Display for EncodeError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            EncodeError::Io(e) => write!(f, "I/O error: {e}"),
        }
    }
}

impl core::error::Error for EncodeError {
    fn source(&self) -> Option<&(dyn core::error::Error + 'static)> {
        match self {
            EncodeError::Io(e) => Some(e),
        }
    }

    fn cause(&self) -> Option<&dyn core::error::Error> {
        self.source()
    }
}

impl Encoder<&Command> for CommandCodec {
    type Error = EncodeError;

    fn encode(&mut self, command: &Command, dst: &mut BytesMut) -> Result<(), Self::Error> {
        let command_length = 4 + command.length();

        dst.reserve(command_length);
        dst.put_u32(command_length as u32);
        command.encode(dst);

        debug!(target: "rusmpp::codec::encode", command=?command, "Encoding");
        debug!(target: "rusmpp::codec::encode", encoded=?crate::formatter::Formatter(&dst[..command_length]), encoded_length=command.length(), command_length, "Encoded");

        Ok(())
    }
}

impl Encoder<Command> for CommandCodec {
    type Error = EncodeError;

    fn encode(&mut self, command: Command, dst: &mut BytesMut) -> Result<(), Self::Error> {
        self.encode(&command, dst)
    }
}

/// An error that can occur when decoding a `Command`.
#[derive(Debug)]
#[non_exhaustive]
pub enum DecodeError {
    /// I/O error.
    Io(std::io::Error),
    /// Decode error.
    Decode(crate::decode::DecodeError),
    /// Minimum command length not met.
    MinLength { actual: usize, min: usize },
    /// Maximum command length exceeded.
    MaxLength { actual: usize, max: usize },
    /// Integral type conversion failed.
    InvalidLength(TryFromIntError),
}

impl From<std::io::Error> for DecodeError {
    fn from(e: std::io::Error) -> Self {
        DecodeError::Io(e)
    }
}

impl core::fmt::Display for DecodeError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            DecodeError::Io(e) => write!(f, "I/O error: {e}"),
            DecodeError::Decode(e) => write!(f, "Decode error: {e}"),
            DecodeError::MinLength { actual, min } => {
                write!(
                    f,
                    "Minimum command length not met. actual: {actual}, min: {min}"
                )
            }
            DecodeError::MaxLength { actual, max } => {
                write!(
                    f,
                    "Maximum command length exceeded. actual: {actual}, max: {max}"
                )
            }
            DecodeError::InvalidLength(e) => {
                write!(f, "Integral type conversion failed: {e}")
            }
        }
    }
}

impl core::error::Error for DecodeError {
    fn source(&self) -> Option<&(dyn core::error::Error + 'static)> {
        match self {
            DecodeError::Io(e) => Some(e),
            DecodeError::Decode(e) => Some(e),
            DecodeError::MinLength { .. } => None,
            DecodeError::MaxLength { .. } => None,
            DecodeError::InvalidLength(e) => Some(e),
        }
    }

    fn cause(&self) -> Option<&dyn core::error::Error> {
        self.source()
    }
}

impl Decoder for CommandCodec {
    type Item = Command;
    type Error = DecodeError;

    fn decode(&mut self, src: &mut BytesMut) -> Result<Option<Self::Item>, Self::Error> {
        const HEADER_LENGTH: usize = 16;

        loop {
            match self.state {
                DecodeState::Length => {
                    if src.len() < HEADER_LENGTH {
                        trace!(target: "rusmpp::codec::decode", source_length=src.len(), "Not enough bytes to read the header");

                        return Ok(None);
                    }

                    let command_length = usize::try_from(src.get_u32()).map_err(|err|
                        {
                            error!(target: "rusmpp::codec::decode", ?err, "Failed to convert command length to usize");

                            DecodeError::InvalidLength(err)
                        })?;

                    trace!(target: "rusmpp::codec::decode", command_length);

                    if command_length < HEADER_LENGTH {
                        error!(target: "rusmpp::codec::decode", command_length, min_command_length=HEADER_LENGTH, "Minimum command length not met");

                        return Err(DecodeError::MinLength {
                            actual: command_length,
                            min: HEADER_LENGTH,
                        });
                    }

                    // XXX: keep msrv lower than 1.90
                    #[allow(clippy::collapsible_if)]
                    if let Some(max_command_length) = self.max_length {
                        if command_length > max_command_length {
                            error!(target: "rusmpp::codec::decode", command_length, max_command_length, "Maximum command length exceeded");

                            return Err(DecodeError::MaxLength {
                                actual: command_length,
                                max: max_command_length,
                            });
                        }
                    }

                    self.decode_command(command_length);
                }
                DecodeState::Command { command_length } => {
                    // command_length is at least 16 (bytes)
                    let pdu_length = command_length - 4;

                    if src.len() < pdu_length {
                        // Reserve enough space to read the entire command
                        src.reserve(pdu_length - src.len());

                        trace!(target: "rusmpp::codec::decode", command_length, decode_length=pdu_length, "Not enough bytes to read the entire command");

                        return Ok(None);
                    }

                    debug!(target: "rusmpp::codec::decode", command_length, decode_length=pdu_length, decoding=?crate::formatter::Formatter(&src[..pdu_length]), "Decoding");

                    let (command, _size) = match Command::decode(src, pdu_length) {
                        Ok((command, size)) => {
                            debug!(target: "rusmpp::codec::decode", command=?command, command_length, decoded_length=size, "Decoded");

                            (command, size)
                        }
                        Err(err) => {
                            error!(target: "rusmpp::codec::decode", ?err);

                            self.decode_length();

                            return Err(DecodeError::Decode(err));
                        }
                    };

                    self.decode_length();

                    return Ok(Some(command));
                }
            }
        }
    }
}
