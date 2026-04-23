use alloc::vec::Vec;
use rusmpp_core::{
    pdus::owned::SubmitSm, tlvs::owned::MessageSubmissionRequestTlvValue, types::owned::OctetString,
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

/// Builder for creating multipart [`SubmitSm`] messages using SAR TLVs.
///
/// Created using [`SubmitSmMultipartExt::sar_multipart`](super::SubmitSmMultipartExt::sar_multipart).
#[derive(Debug)]
pub struct SubmitSmSarMultipartBuilder<'a, E> {
    short_message: &'a str,
    max_short_message_size: usize,
    sm: SubmitSm,
    encoder: E,
    reference: u16,
}

impl<'a, E> SubmitSmSarMultipartBuilder<'a, E> {
    /// Creates a new [`SubmitSmSarMultipartBuilder`].
    pub(super) const fn new(
        short_message: &'a str,
        sm: SubmitSm,
        encoder: E,
    ) -> SubmitSmSarMultipartBuilder<'a, E> {
        Self {
            short_message,
            max_short_message_size: SubmitSm::default_max_short_message_size(),
            sm,
            encoder,
            reference: 0,
        }
    }

    /// Override the default max short message size.
    ///
    /// See [`SubmitSm::default_max_short_message_size`].
    pub const fn max_short_message_size(mut self, size: usize) -> Self {
        self.max_short_message_size = size;
        self
    }

    /// Sets the reference number for the concatenated short message.
    pub const fn reference(mut self, reference: u16) -> Self {
        self.reference = reference;
        self
    }

    /// Sets a custom encoder.
    pub fn encoder<U>(self, encoder: U) -> SubmitSmSarMultipartBuilder<'a, U> {
        SubmitSmSarMultipartBuilder {
            short_message: self.short_message,
            max_short_message_size: self.max_short_message_size,
            sm: self.sm,
            encoder,
            reference: self.reference,
        }
    }

    /// Sets the [`Gsm7BitUnpacked`] encoder.
    pub fn gsm7bit_unpacked(self) -> SubmitSmSarMultipartBuilder<'a, Gsm7BitUnpacked> {
        self.encoder(Gsm7BitUnpacked::new())
    }

    /// Sets the [`Ucs2`] encoder.
    pub fn ucs2(self) -> SubmitSmSarMultipartBuilder<'a, Ucs2> {
        self.encoder(Ucs2::new())
    }

    /// Sets the [`Latin1`] encoder.
    pub fn latin1(self) -> SubmitSmSarMultipartBuilder<'a, Latin1> {
        self.encoder(Latin1::new())
    }

    /// Sets a fallback encoder.
    pub fn fallback<U>(self, encoder: U) -> SubmitSmSarMultipartBuilder<'a, Fallback<E, U>> {
        SubmitSmSarMultipartBuilder {
            short_message: self.short_message,
            max_short_message_size: self.max_short_message_size,
            sm: self.sm,
            encoder: Fallback::new(self.encoder, encoder),
            reference: self.reference,
        }
    }
}

impl<'a, E> SubmitSmSarMultipartBuilder<'a, E>
where
    E: Concatenator + 'a,
{
    /// Builds the multipart [`SubmitSm`] messages.
    pub fn build(self) -> Result<Vec<SubmitSm>, MultipartError<E::Error>> {
        let (concatenation, data_coding) = self
            .encoder
            .concatenate(self.short_message, self.max_short_message_size, 0)
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

                let sar_total_segments = parts.len().min(MAX_PARTS) as u8;

                parts
                    .into_iter()
                    .enumerate()
                    .map(|(index, part)| {
                        let sar_segment_seq_num = index as u8 + 1;

                        let short_message = OctetString::from_vec(part)?;

                        let mut sm = self
                            .sm
                            .clone()
                            .with_short_message(short_message)
                            .with_data_coding(data_coding);

                        sm.push_tlv(MessageSubmissionRequestTlvValue::SarMsgRefNum(
                            self.reference,
                        ));
                        sm.push_tlv(MessageSubmissionRequestTlvValue::SarTotalSegments(
                            sar_total_segments,
                        ));
                        sm.push_tlv(MessageSubmissionRequestTlvValue::SarSegmentSeqnum(
                            sar_segment_seq_num,
                        ));

                        Ok(sm)
                    })
                    .collect()
            }
        }
    }
}
