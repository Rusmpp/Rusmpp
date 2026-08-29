use crate::encoding::{
    ascii::{AsciiDecodeError, AsciiDecoder},
    gsm7bit::{Gsm7BitDecodeError, Gsm7BitPackedDecoder, Gsm7BitUnpackedDecoder},
    latin1::{Latin1DecodeError, Latin1Decoder},
    ucs2::{Ucs2DecodeError, Ucs2Decoder},
};

/// A trait for decoding messages from byte vectors.
pub trait Decoder {
    /// The type of errors that can occur during decoding.
    type Error;

    /// Feeds the given input bytes into the decoder.
    fn feed(&mut self, input: &[u8], header_size: usize) -> Result<(), Self::Error>;

    /// Peeks at the current state of the decoder without consuming it.
    fn peek(&self) -> &str;

    /// Finishes the decoding process and returns the final decoded string.
    fn finish(self) -> Result<alloc::string::String, Self::Error>;
}

/// A decoder that can handle multiple encoding types.
#[derive(Debug)]
pub enum SupportedDecoder {
    /// A decoder for GSM 7-bit unpacked encoding.
    Gsm7BitUnpacked(Gsm7BitUnpackedDecoder),
    /// A decoder for GSM 7-bit packed encoding.
    Gsm7BitPacked(Gsm7BitPackedDecoder),
    /// A decoder for UCS2 encoding.
    Ucs2(Ucs2Decoder),
    /// A decoder for ASCII encoding.
    Ascii(AsciiDecoder),
    /// A decoder for Latin1 encoding.
    Latin1(Latin1Decoder),
}

/// Errors that can occur when decoding messages using a [`SupportedDecoder`].
#[derive(Debug, thiserror::Error)]
pub enum SupportedDecodeError {
    /// GSM 7-bit unpacked decoding error.
    #[error(transparent)]
    Gsm7BitUnpacked(Gsm7BitDecodeError),
    /// GSM 7-bit packed decoding error.
    #[error(transparent)]
    Gsm7BitPacked(Gsm7BitDecodeError),
    /// UCS2 decoding error.
    #[error(transparent)]
    Ucs2(Ucs2DecodeError),
    /// ASCII decoding error.
    #[error(transparent)]
    Ascii(AsciiDecodeError),
    /// Latin1 decoding error.
    #[error(transparent)]
    Latin1(Latin1DecodeError),
}

impl Decoder for SupportedDecoder {
    type Error = SupportedDecodeError;

    fn feed(&mut self, input: &[u8], header_size: usize) -> Result<(), Self::Error> {
        match self {
            SupportedDecoder::Gsm7BitUnpacked(decoder) => decoder
                .feed(input, header_size)
                .map_err(SupportedDecodeError::Gsm7BitUnpacked),
            SupportedDecoder::Gsm7BitPacked(decoder) => decoder
                .feed(input, header_size)
                .map_err(SupportedDecodeError::Gsm7BitPacked),
            SupportedDecoder::Ucs2(decoder) => decoder
                .feed(input, header_size)
                .map_err(SupportedDecodeError::Ucs2),
            SupportedDecoder::Ascii(decoder) => decoder
                .feed(input, header_size)
                .map_err(SupportedDecodeError::Ascii),
            SupportedDecoder::Latin1(decoder) => decoder
                .feed(input, header_size)
                .map_err(SupportedDecodeError::Latin1),
        }
    }

    fn peek(&self) -> &str {
        match self {
            SupportedDecoder::Gsm7BitUnpacked(decoder) => decoder.peek(),
            SupportedDecoder::Gsm7BitPacked(decoder) => decoder.peek(),
            SupportedDecoder::Ucs2(decoder) => decoder.peek(),
            SupportedDecoder::Ascii(decoder) => decoder.peek(),
            SupportedDecoder::Latin1(decoder) => decoder.peek(),
        }
    }

    fn finish(self) -> Result<alloc::string::String, Self::Error> {
        match self {
            SupportedDecoder::Gsm7BitUnpacked(decoder) => decoder
                .finish()
                .map_err(SupportedDecodeError::Gsm7BitUnpacked),
            SupportedDecoder::Gsm7BitPacked(decoder) => decoder
                .finish()
                .map_err(SupportedDecodeError::Gsm7BitPacked),
            SupportedDecoder::Ucs2(decoder) => decoder.finish().map_err(SupportedDecodeError::Ucs2),
            SupportedDecoder::Ascii(decoder) => {
                decoder.finish().map_err(SupportedDecodeError::Ascii)
            }
            SupportedDecoder::Latin1(decoder) => {
                decoder.finish().map_err(SupportedDecodeError::Latin1)
            }
        }
    }
}
