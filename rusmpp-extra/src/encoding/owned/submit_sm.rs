use rusmpp_core::{pdus::owned::SubmitSm, types::owned::OctetString};

use crate::{
    encoding::{
        errors::EncodeError, gsm7bit::Gsm7BitUnpackedEncoder, latin1::Latin1Encoder,
        owned::Encoder, ucs2::Ucs2Encoder,
    },
    fallback::Fallback,
};

/// Builder for creating encoded [`SubmitSm`] messages.
///
/// Created using [`EncodedSubmitSmExt::encode`].
#[derive(Debug)]
pub struct EncodedSubmitSmBuilder<'a, E> {
    short_message: &'a str,
    sm: SubmitSm,
    encoder: E,
}

impl<E> EncodedSubmitSmBuilder<'static, E> {}

impl<'a, E> EncodedSubmitSmBuilder<'a, E> {
    /// Creates a new [`EncodedSubmitSmBuilder`].
    const fn new(
        short_message: &'a str,
        sm: SubmitSm,
        encoder: E,
    ) -> EncodedSubmitSmBuilder<'a, E> {
        Self {
            short_message,
            sm,
            encoder,
        }
    }

    /// Sets a custom encoder.
    pub fn encoder<U>(self, encoder: U) -> EncodedSubmitSmBuilder<'a, U> {
        EncodedSubmitSmBuilder {
            short_message: self.short_message,
            sm: self.sm,
            encoder,
        }
    }

    /// Sets the [`Gsm7BitUnpackedEncoder`] encoder.
    pub fn gsm7bit_unpacked(self) -> EncodedSubmitSmBuilder<'a, Gsm7BitUnpackedEncoder> {
        self.encoder(Gsm7BitUnpackedEncoder::new())
    }

    /// Sets the [`Ucs2Encoder`] encoder.
    pub fn ucs2(self) -> EncodedSubmitSmBuilder<'a, Ucs2Encoder> {
        self.encoder(Ucs2Encoder::new())
    }

    /// Sets the [`Latin1Encoder`] encoder.
    pub fn latin1(self) -> EncodedSubmitSmBuilder<'a, Latin1Encoder> {
        self.encoder(Latin1Encoder::new())
    }

    /// Sets a fallback encoder.
    pub fn fallback<U>(self, encoder: U) -> EncodedSubmitSmBuilder<'a, Fallback<E, U>> {
        EncodedSubmitSmBuilder {
            short_message: self.short_message,
            sm: self.sm,
            encoder: Fallback::new(self.encoder, encoder),
        }
    }
}

impl<'a, E> EncodedSubmitSmBuilder<'a, E>
where
    E: Encoder + 'a,
{
    /// Builds the encoded [`SubmitSm`] message.
    pub fn build(self) -> Result<SubmitSm, EncodeError<E::Error>> {
        let (encoded, data_coding) = self
            .encoder
            .encode(self.short_message)
            .map_err(EncodeError::encode)?;

        let short_message = OctetString::from_vec(encoded)?;

        let sm = self
            .sm
            .with_short_message(short_message)
            .with_data_coding(data_coding);

        Ok(sm)
    }
}

/// Extension trait for [`SubmitSm`] to create encoded messages.
pub trait EncodedSubmitSmExt {
    /// Creates a new [`EncodedSubmitSmBuilder`] with the default [`Gsm7BitUnpackedEncoder`] encoder.
    ///
    /// # Notes
    ///
    /// - [`SubmitSm::data_coding`] will be overridden by the multipart builder to match the encoder.
    /// - [`SubmitSm::short_message`] will be overridden by `short_message` of the multipart builder.
    fn encode<'a>(
        self,
        short_message: &'a str,
    ) -> EncodedSubmitSmBuilder<'a, Gsm7BitUnpackedEncoder>;
}

impl EncodedSubmitSmExt for SubmitSm {
    fn encode<'a>(
        self,
        short_message: &'a str,
    ) -> EncodedSubmitSmBuilder<'a, Gsm7BitUnpackedEncoder> {
        EncodedSubmitSmBuilder::new(short_message, self, Gsm7BitUnpackedEncoder::new())
    }
}
