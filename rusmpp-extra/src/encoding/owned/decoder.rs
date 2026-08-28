use crate::encoding::gsm7bit::{Gsm7BitDecodeError, Gsm7BitPackedDecoder, Gsm7BitUnpackedDecoder};

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

#[derive(Debug)]
pub enum SupportedDecoder {
    Gsm7BitUnpacked(Gsm7BitUnpackedDecoder),
    Gsm7BitPacked(Gsm7BitPackedDecoder),
}

/// Errors that can occur when decoding messages using a [`SupportedDecoder`].
#[derive(Debug, thiserror::Error)]
pub enum SupportedDecodeError {
    #[error(transparent)]
    Gsm7BitUnpacked(Gsm7BitDecodeError),
    #[error(transparent)]
    Gsm7BitPacked(Gsm7BitDecodeError),
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
        }
    }

    fn peek(&self) -> &str {
        match self {
            SupportedDecoder::Gsm7BitUnpacked(decoder) => decoder.peek(),
            SupportedDecoder::Gsm7BitPacked(decoder) => decoder.peek(),
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
        }
    }
}
