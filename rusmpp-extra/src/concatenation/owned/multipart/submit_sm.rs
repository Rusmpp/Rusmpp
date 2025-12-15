use alloc::vec::Vec;
use rusmpp_core::{
    pdus::owned::SubmitSm, types::owned::OctetString,
    udhs::owned::concatenation::ConcatenatedShortMessageType,
};

use crate::{
    concatenation::{
        MAX_PARTS, MIN_PARTS,
        errors::MultipartError,
        owned::{Concatenation, Concatenator},
    },
    encoding::{gsm7bit::Gsm7BitUnpacked, latin1::Latin1, ucs2::Ucs2},
    fallback::Fallback,
};

/// Builder for creating multipart [`SubmitSm`] messages.
///
/// Created using [`SubmitSmMultipartExt::multipart`].
#[derive(Debug)]
pub struct SubmitSmMultipartBuilder<'a, E> {
    short_message: &'a str,
    max_short_message_size: usize,
    sm: SubmitSm,
    encoder: E,
    concatenation_type: ConcatenatedShortMessageType,
}

impl<'a, E> SubmitSmMultipartBuilder<'a, E> {
    /// Creates a new [`SubmitSmMultipartBuilder`].
    const fn new(
        short_message: &'a str,
        sm: SubmitSm,
        encoder: E,
    ) -> SubmitSmMultipartBuilder<'a, E> {
        Self {
            short_message,
            max_short_message_size: SubmitSm::default_max_short_message_size(),
            sm,
            encoder,
            concatenation_type: ConcatenatedShortMessageType::u8(0),
        }
    }

    /// Override the default max short message size.
    ///
    /// See [`SubmitSm::default_max_short_message_size`].
    pub const fn max_short_message_size(mut self, size: usize) -> Self {
        self.max_short_message_size = size;
        self
    }

    /// Sets the reference number for the concatenated short message as [`u8`].
    pub const fn reference_u8(mut self, reference: u8) -> Self {
        self.concatenation_type = ConcatenatedShortMessageType::u8(reference);
        self
    }

    /// Sets the reference number for the concatenated short message as [`u16`].
    pub const fn reference_u16(mut self, reference: u16) -> Self {
        self.concatenation_type = ConcatenatedShortMessageType::u16(reference);
        self
    }

    /// Sets a custom encoder.
    pub fn encoder<U>(self, encoder: U) -> SubmitSmMultipartBuilder<'a, U> {
        SubmitSmMultipartBuilder {
            short_message: self.short_message,
            max_short_message_size: self.max_short_message_size,
            sm: self.sm,
            encoder,
            concatenation_type: self.concatenation_type,
        }
    }

    /// Sets the [`Gsm7BitUnpacked`] encoder.
    pub fn gsm7bit_unpacked(self) -> SubmitSmMultipartBuilder<'a, Gsm7BitUnpacked> {
        self.encoder(Gsm7BitUnpacked::new())
    }

    /// Sets the [`Ucs2`] encoder.
    pub fn ucs2(self) -> SubmitSmMultipartBuilder<'a, Ucs2> {
        self.encoder(Ucs2::new())
    }

    /// Sets the [`Latin1`] encoder.
    pub fn latin1(self) -> SubmitSmMultipartBuilder<'a, Latin1> {
        self.encoder(Latin1::new())
    }

    /// Sets a fallback encoder.
    pub fn fallback<U>(self, encoder: U) -> SubmitSmMultipartBuilder<'a, Fallback<E, U>> {
        SubmitSmMultipartBuilder {
            short_message: self.short_message,
            max_short_message_size: self.max_short_message_size,
            sm: self.sm,
            encoder: Fallback::new(self.encoder, encoder),
            concatenation_type: self.concatenation_type,
        }
    }
}

impl<'a, E> SubmitSmMultipartBuilder<'a, E>
where
    E: Concatenator + 'a,
{
    /// Builds the multipart [`SubmitSm`] messages.
    pub fn build(self) -> Result<Vec<SubmitSm>, MultipartError<E::Error>> {
        let (concatenation, data_coding) = self
            .encoder
            .concatenate(
                self.short_message,
                self.max_short_message_size,
                self.concatenation_type.udh_length(),
            )
            .map_err(MultipartError::concatenation)?;

        match concatenation {
            Concatenation::Single(bytes) => {
                let short_message = OctetString::from_vec(bytes)?;

                let sm = self
                    .sm
                    .with_short_message(short_message)
                    .with_data_coding(data_coding);

                Ok(alloc::vec![sm])
            }
            Concatenation::Concatenated(parts) => {
                if parts.len() < MIN_PARTS {
                    return Err(MultipartError::min_part_count(parts.len()));
                }

                if parts.len() > MAX_PARTS {
                    return Err(MultipartError::max_parts_count(parts.len()));
                }

                let total_parts = parts.len().min(MAX_PARTS) as u8;

                parts
                    .into_iter()
                    .enumerate()
                    .map(|(index, part)| {
                        let udh = self
                            .concatenation_type
                            /*
                               Correctness:
                               - total_parts is at least 2 due to the earlier check.
                               - total_parts is at most 255 due to the earlier check.
                               - part_number (index + 1) is at least 1.
                               - part_number (index + 1) is at most total_parts due to the earlier check.
                            */
                            .concatenated_short_message_unchecked(total_parts, index as u8 + 1);

                        let mut payload = Vec::with_capacity(udh.udh_length() + part.len());

                        payload.extend_from_slice(udh.udh_bytes().as_bytes());
                        payload.extend_from_slice(&part);

                        let short_message = OctetString::from_vec(payload)?;

                        let sm = self
                            .sm
                            .clone()
                            .with_udhi_indicator()
                            .with_short_message(short_message)
                            .with_data_coding(data_coding);

                        Ok(sm)
                    })
                    .collect()
            }
        }
    }
}

/// Extension trait for [`SubmitSm`] to create multipart messages.
pub trait SubmitSmMultipartExt {
    /// Creates a new [`SubmitSmMultipartBuilder`] with the default [`Gsm7BitUnpacked`] encoder.
    ///
    /// # Notes
    ///
    /// - [`SubmitSm::esm_class`] will be updated with UDHI indicator by the multipart builder.
    /// - [`SubmitSm::data_coding`] will be overridden by the multipart builder to match the encoder.
    /// - [`SubmitSm::short_message`] will be overridden by `short_message` of the multipart builder.
    fn multipart<'a>(self, short_message: &'a str)
    -> SubmitSmMultipartBuilder<'a, Gsm7BitUnpacked>;
}

impl SubmitSmMultipartExt for SubmitSm {
    fn multipart<'a>(
        self,
        short_message: &'a str,
    ) -> SubmitSmMultipartBuilder<'a, Gsm7BitUnpacked> {
        SubmitSmMultipartBuilder::new(short_message, self, Gsm7BitUnpacked::new())
    }
}
