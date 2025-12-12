use rusmpp_core::{pdus::owned::SubmitSm, types::owned::OctetString};

use crate::{
    codecs::{
        errors::EncodeError, gsm7bit::Gsm7BitUnpacked, latin1::Latin1, owned::Encoder, ucs2::Ucs2,
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

impl<E> EncodedSubmitSmBuilder<'static, E> {
    /// Creates a new [`EncodedSubmitSmBuilder`].
    const fn new(sm: SubmitSm, encoder: E) -> EncodedSubmitSmBuilder<'static, E> {
        Self {
            short_message: "",
            sm,
            encoder,
        }
    }
}

impl<'a, E> EncodedSubmitSmBuilder<'a, E> {
    /// Sets the short message.
    pub fn short_message<'b>(self, short_message: &'b str) -> EncodedSubmitSmBuilder<'b, E> {
        EncodedSubmitSmBuilder {
            short_message,
            sm: self.sm,
            encoder: self.encoder,
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

    /// Sets the [`Gsm7BitUnpacked`] encoder.
    pub fn gsm7bit_unpacked(self) -> EncodedSubmitSmBuilder<'a, Gsm7BitUnpacked> {
        self.encoder(Gsm7BitUnpacked::new())
    }

    /// Sets the [`Ucs2`] encoder.
    pub fn ucs2(self) -> EncodedSubmitSmBuilder<'a, Ucs2> {
        self.encoder(Ucs2::new())
    }

    /// Sets the [`Latin1`] encoder.
    pub fn latin1(self) -> EncodedSubmitSmBuilder<'a, Latin1> {
        self.encoder(Latin1::new())
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
    /// Creates a new [`EncodedSubmitSmBuilder`] with the default [`Gsm7BitUnpacked`] encoder.
    ///
    /// # Notes
    ///
    /// - [`SubmitSm::data_coding`] will be overridden by the multipart builder to match the encoder.
    /// - [`SubmitSm::short_message`] will be overridden by `short_message` of the multipart builder.
    fn encode(self) -> EncodedSubmitSmBuilder<'static, Gsm7BitUnpacked>;
}

impl EncodedSubmitSmExt for SubmitSm {
    fn encode(self) -> EncodedSubmitSmBuilder<'static, Gsm7BitUnpacked> {
        EncodedSubmitSmBuilder::new(self, Gsm7BitUnpacked::new())
    }
}
