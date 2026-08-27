use rusmpp_core::{
    decode::borrowed::Decode,
    pdus::owned::SubmitSm,
    udhs::borrowed::{Udh, UdhValue},
};

use crate::{
    concatenation::multipart::{MultipartSegment, MultipartSegmentError, MultipartType},
    encoding::gsm7bit::Gsm7BitUnpacked,
};

use super::{SubmitSmMultipartBuilder, SubmitSmSarMultipartBuilder};

/// Extension trait for [`SubmitSm`] to create multipart messages.
pub trait SubmitSmMultipartExt {
    /// Creates a new [`SubmitSmMultipartBuilder`] with the default [`Gsm7BitUnpacked`] encoder.
    ///
    /// # Notes
    ///
    /// - [`SubmitSm::esm_class`] will be updated with UDH indicator by the multipart builder.
    /// - [`SubmitSm::data_coding`] will be overridden by the multipart builder to match the encoder.
    /// - [`SubmitSm::short_message`] will be overridden by `short_message` of the multipart builder.
    fn multipart<'a>(self, short_message: &'a str)
    -> SubmitSmMultipartBuilder<'a, Gsm7BitUnpacked>;

    /// Creates a new [`SubmitSmSarMultipartBuilder`] with the default [`Gsm7BitUnpacked`] encoder.
    fn sar_multipart<'a>(
        self,
        short_message: &'a str,
    ) -> SubmitSmSarMultipartBuilder<'a, Gsm7BitUnpacked>;

    fn multipart_segment(&self) -> Option<Result<MultipartSegment, MultipartSegmentError>>;
}

impl SubmitSmMultipartExt for SubmitSm {
    fn multipart<'a>(
        self,
        short_message: &'a str,
    ) -> SubmitSmMultipartBuilder<'a, Gsm7BitUnpacked> {
        SubmitSmMultipartBuilder::new(short_message, self, Gsm7BitUnpacked::new())
    }

    fn sar_multipart<'a>(
        self,
        short_message: &'a str,
    ) -> SubmitSmSarMultipartBuilder<'a, Gsm7BitUnpacked> {
        SubmitSmSarMultipartBuilder::new(short_message, self, Gsm7BitUnpacked::new())
    }

    fn multipart_segment(&self) -> Option<Result<MultipartSegment, MultipartSegmentError>> {
        if self.is_udh_indicator_set() {
            match Udh::decode(self.short_message()) {
                Ok((udh, size)) => match udh.value() {
                    Some(UdhValue::ConcatenatedShortMessage8Bit(concatenation)) => {
                        return Some(Ok(MultipartSegment {
                            r#type: MultipartType::Udh { size },
                            reference: concatenation.reference() as u16,
                            part_number: concatenation.part_number(),
                            total_parts: concatenation.total_parts(),
                        }));
                    }
                    Some(UdhValue::ConcatenatedShortMessage16Bit(concatenation)) => {
                        return Some(Ok(MultipartSegment {
                            r#type: MultipartType::Udh { size },
                            reference: concatenation.reference(),
                            part_number: concatenation.part_number(),
                            total_parts: concatenation.total_parts(),
                        }));
                    }
                    _ => {
                        return None;
                    }
                },
                Err(err) => {
                    // if the decode error is a concatenated short message error, map it, otherwise return a udh decode error.
                    todo!()
                }
            }
        };

        match (
            self.sar_msg_ref_num(),
            self.sar_segment_seqnum(),
            self.sar_total_segments(),
        ) {
            (Some(reference), Some(part_number), Some(total_parts)) => {
                // Validate the parts like concatenated short message does.
                Some(Ok(MultipartSegment {
                    r#type: MultipartType::Sar,
                    reference,
                    part_number,
                    total_parts,
                }))
            }
            _ => None,
        }
    }
}
