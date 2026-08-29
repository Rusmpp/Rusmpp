use alloc::vec::Vec;
use rusmpp_core::{
    pdus::owned::SubmitSm, types::owned::OctetString,
    udhs::concatenation::ConcatenatedShortMessageType,
};

use crate::{
    concatenation::{
        MAX_PARTS, MIN_PARTS,
        errors::MultipartError,
        owned::{Concatenation, Concatenator},
    },
    encoding::{
        gsm7bit::{Gsm7BitPackedEncoder, Gsm7BitUnpackedEncoder},
        latin1::Latin1Encoder,
        ucs2::Ucs2Encoder,
    },
    fallback::Fallback,
    traits::owned::UdhMultipart,
};

/// Builder for creating multipart [`SubmitSm`](rusmpp_core::pdus::owned::SubmitSm), [`DeliverSm`](rusmpp_core::pdus::owned::DeliverSm) and [`SubmitMulti`](rusmpp_core::pdus::owned::SubmitMulti) messages.
///
/// Created using [`Multipart::udh_multipart`](super::Multipart::udh_multipart).
#[derive(Debug)]
pub struct UdhMultipartBuilder<'a, Sm, E> {
    short_message: &'a str,
    max_short_message_size: usize,
    sm: Sm,
    encoder: E,
    concatenation_type: ConcatenatedShortMessageType,
}

impl<'a, Sm, E> UdhMultipartBuilder<'a, Sm, E> {
    /// Creates a new [`UdhMultipartBuilder`].
    pub(super) const fn new(
        short_message: &'a str,
        sm: Sm,
        encoder: E,
    ) -> UdhMultipartBuilder<'a, Sm, E> {
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
    pub fn encoder<U>(self, encoder: U) -> UdhMultipartBuilder<'a, Sm, U> {
        UdhMultipartBuilder {
            short_message: self.short_message,
            max_short_message_size: self.max_short_message_size,
            sm: self.sm,
            encoder,
            concatenation_type: self.concatenation_type,
        }
    }

    /// Sets the [`Gsm7BitUnpackedEncoder`] encoder.
    pub fn gsm7bit_unpacked(self) -> UdhMultipartBuilder<'a, Sm, Gsm7BitUnpackedEncoder> {
        self.encoder(Gsm7BitUnpackedEncoder::new())
    }

    /// Sets the [`Gsm7BitPackedEncoder`] encoder.
    pub fn gsm7bit_packed(self) -> UdhMultipartBuilder<'a, Sm, Gsm7BitPackedEncoder> {
        self.encoder(Gsm7BitPackedEncoder::new())
    }

    /// Sets the [`Ucs2Encoder`] encoder.
    pub fn ucs2(self) -> UdhMultipartBuilder<'a, Sm, Ucs2Encoder> {
        self.encoder(Ucs2Encoder::new())
    }

    /// Sets the [`Latin1Encoder`] encoder.
    pub fn latin1(self) -> UdhMultipartBuilder<'a, Sm, Latin1Encoder> {
        self.encoder(Latin1Encoder::new())
    }

    /// Sets a fallback encoder.
    pub fn fallback<U>(self, encoder: U) -> UdhMultipartBuilder<'a, Sm, Fallback<E, U>> {
        UdhMultipartBuilder {
            short_message: self.short_message,
            max_short_message_size: self.max_short_message_size,
            sm: self.sm,
            encoder: Fallback::new(self.encoder, encoder),
            concatenation_type: self.concatenation_type,
        }
    }
}

impl<'a, Sm, E> UdhMultipartBuilder<'a, Sm, E>
where
    Sm: UdhMultipart + Clone,
    E: Concatenator + 'a,
{
    /// Builds the multipart messages.
    pub fn build(self) -> Result<Vec<Sm>, MultipartError<E::Error>> {
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
                            .with_udh_indicator()
                            .with_short_message(short_message)
                            .with_data_coding(data_coding);

                        Ok(sm)
                    })
                    .collect()
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use bytes::BytesMut;
    use rusmpp_core::{
        decode::owned::Decode,
        pdus::owned::SubmitSm,
        udhs::owned::{Udh, UdhValue},
        values::{DataCoding, GsmFeatures},
    };

    use crate::concatenation::owned::{
        Multipart, multipart::tests::GSM_7_BIT_UNPACKED_3_PARTS_MESSAGE,
    };

    #[test]
    fn gsm7bit_unpacked_3_parts() {
        let multipart = SubmitSm::default()
            .udh_multipart(GSM_7_BIT_UNPACKED_3_PARTS_MESSAGE)
            .reference_u16(1)
            .gsm7bit_unpacked()
            .build()
            .expect("Failed to build multipart SubmitSm messages");

        assert_eq!(multipart.len(), 3);

        for (i, sm) in multipart.into_iter().enumerate() {
            assert!(matches!(
                sm.esm_class.gsm_features,
                GsmFeatures::UdhIndicator
            ));

            assert!(matches!(sm.data_coding, DataCoding::McSpecific));

            let mut buf = BytesMut::from(sm.into_parts().short_message.into_bytes());

            let (udh, _) = <Udh as Decode>::decode(&mut buf).expect("Failed to decode udh");

            let Some(UdhValue::ConcatenatedShortMessage16Bit(concatenated)) =
                udh.into_parts().value
            else {
                panic!("UDH is not Concatenated Short Message 16-bit");
            };

            assert_eq!(concatenated.reference(), 1);
            assert_eq!(concatenated.part_number(), i as u8 + 1);
            assert_eq!(concatenated.total_parts(), 3);
        }
    }
}
