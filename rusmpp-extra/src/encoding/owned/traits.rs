use rusmpp_core::types::owned::OctetString;

use crate::{
    encoding::{
        errors::EncodeError,
        gsm7bit::{Gsm7BitPackedEncoder, Gsm7BitUnpackedEncoder},
        latin1::Latin1Encoder,
        owned::Encoder,
        ucs2::Ucs2Encoder,
    },
    fallback::Fallback,
    traits::owned::ShortMessage,
};

/// Builder for creating encoded [`SubmitSm`](rusmpp_core::pdus::owned::SubmitSm) and [`DeliverSm`](rusmpp_core::pdus::owned::DeliverSm) messages.
///
/// Created using [`Encoded::encoded`].
#[derive(Debug)]
pub struct EncodedBuilder<'a, Sm, E> {
    short_message: &'a str,
    sm: Sm,
    encoder: E,
}

impl<'a, Sm, E> EncodedBuilder<'a, Sm, E> {
    /// Creates a new [`EncodedBuilder`].
    const fn new(short_message: &'a str, sm: Sm, encoder: E) -> EncodedBuilder<'a, Sm, E> {
        Self {
            short_message,
            sm,
            encoder,
        }
    }

    /// Sets a custom encoder.
    pub fn encoder<U>(self, encoder: U) -> EncodedBuilder<'a, Sm, U> {
        EncodedBuilder {
            short_message: self.short_message,
            sm: self.sm,
            encoder,
        }
    }

    /// Sets the [`Gsm7BitUnpackedEncoder`] encoder.
    pub fn gsm7bit_unpacked(self) -> EncodedBuilder<'a, Sm, Gsm7BitUnpackedEncoder> {
        self.encoder(Gsm7BitUnpackedEncoder::new())
    }

    /// Sets the [`Gsm7BitPackedEncoder`] encoder.
    pub fn gsm7bit_packed(self) -> EncodedBuilder<'a, Sm, Gsm7BitPackedEncoder> {
        self.encoder(Gsm7BitPackedEncoder::new())
    }

    /// Sets the [`Ucs2Encoder`] encoder.
    pub fn ucs2(self) -> EncodedBuilder<'a, Sm, Ucs2Encoder> {
        self.encoder(Ucs2Encoder::new())
    }

    /// Sets the [`Latin1Encoder`] encoder.
    pub fn latin1(self) -> EncodedBuilder<'a, Sm, Latin1Encoder> {
        self.encoder(Latin1Encoder::new())
    }

    /// Sets a fallback encoder.
    pub fn fallback<U>(self, encoder: U) -> EncodedBuilder<'a, Sm, Fallback<E, U>> {
        EncodedBuilder {
            short_message: self.short_message,
            sm: self.sm,
            encoder: Fallback::new(self.encoder, encoder),
        }
    }
}

impl<'a, Sm, E> EncodedBuilder<'a, Sm, E>
where
    Sm: ShortMessage,
    E: Encoder + 'a,
{
    /// Builds the encoded message.
    pub fn build(self) -> Result<Sm, EncodeError<E::Error>> {
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

/// Trait for [`SubmitSm`](rusmpp_core::pdus::owned::SubmitSm) and [`DeliverSm`](rusmpp_core::pdus::owned::DeliverSm) to create encoded messages.
pub trait Encoded: Sized {
    /// Creates a new [`EncodedBuilder`] with the default [`Gsm7BitUnpackedEncoder`] encoder.
    ///
    /// # Notes
    ///
    /// - [`DataCoding`](rusmpp_core::values::DataCoding) will be overridden by the multipart builder to match the encoder.
    /// - `short_message` will be overridden by `short_message` of the multipart builder.
    ///
    /// # Warning
    ///
    /// - This will not automatically handle multipart messages. Use [`Multipart::udh_multipart`](crate::concatenation::owned::Multipart::udh_multipart) or [`Multipart::sar_multipart`](crate::concatenation::owned::Multipart::sar_multipart) for automatic multipart handling.
    fn encoded<'a>(
        self,
        short_message: &'a str,
    ) -> EncodedBuilder<'a, Self, Gsm7BitUnpackedEncoder>
    where
        Self: ShortMessage;
}

impl<T> Encoded for T {
    fn encoded<'a>(self, short_message: &'a str) -> EncodedBuilder<'a, Self, Gsm7BitUnpackedEncoder>
    where
        Self: ShortMessage,
    {
        EncodedBuilder::new(short_message, self, Gsm7BitUnpackedEncoder::new())
    }
}
