use rusmpp_core::{
    decode::borrowed::Decode,
    udhs::borrowed::{Udh, UdhValue},
};

use crate::{
    concatenation::multipart::{MultipartSegment, MultipartSegmentError, MultipartType},
    encoding::gsm7bit::Gsm7BitUnpackedEncoder,
    traits::owned::{SarMultipart, UdhMultipart},
};

use super::{SarMultipartBuilder, UdhMultipartBuilder};

/// A trait for [`SubmitSm`](rusmpp_core::pdus::owned::SubmitSm), [`DeliverSm`](rusmpp_core::pdus::owned::DeliverSm) and [`SubmitMulti`](rusmpp_core::pdus::owned::SubmitMulti) to create multipart messages.
pub trait Multipart: Sized {
    /// Creates a new [`UdhMultipartBuilder`] with the default [`Gsm7BitUnpackedEncoder`] encoder.
    ///
    /// # Notes
    ///
    /// - [`EsmClass`](rusmpp_core::values::EsmClass) will be updated with UDH indicator by the multipart builder.
    /// - [`DataCoding`](rusmpp_core::values::DataCoding) will be overridden by the multipart builder to match the encoder.
    /// - `short_message` will be overridden by `short_message` of the multipart builder.
    fn udh_multipart<'a>(
        self,
        short_message: &'a str,
    ) -> UdhMultipartBuilder<'a, Self, Gsm7BitUnpackedEncoder>
    where
        Self: UdhMultipart;

    /// Creates a new [`SarMultipartBuilder`] with the default [`Gsm7BitUnpackedEncoder`] encoder.
    fn sar_multipart<'a>(
        self,
        short_message: &'a str,
    ) -> SarMultipartBuilder<'a, Self, Gsm7BitUnpackedEncoder>
    where
        Self: SarMultipart;

    /// Checks wether this is a multipart message and returns the [`MultipartSegment`] and the remaining `short message`.
    fn multipart_segment(&self) -> Option<Result<(MultipartSegment, &[u8]), MultipartSegmentError>>
    where
        Self: SarMultipart + UdhMultipart;
}

impl<T> Multipart for T {
    fn udh_multipart<'a>(
        self,
        short_message: &'a str,
    ) -> UdhMultipartBuilder<'a, Self, Gsm7BitUnpackedEncoder>
    where
        Self: UdhMultipart,
    {
        UdhMultipartBuilder::new(short_message, self, Gsm7BitUnpackedEncoder::new())
    }

    fn sar_multipart<'a>(
        self,
        short_message: &'a str,
    ) -> SarMultipartBuilder<'a, Self, Gsm7BitUnpackedEncoder>
    where
        Self: SarMultipart,
    {
        SarMultipartBuilder::new(short_message, self, Gsm7BitUnpackedEncoder::new())
    }

    fn multipart_segment(&self) -> Option<Result<(MultipartSegment, &[u8]), MultipartSegmentError>>
    where
        Self: SarMultipart + UdhMultipart,
    {
        if self.is_udh_indicator_set() {
            let (udh, size) = match Udh::decode(self.short_message()) {
                Ok(value) => value,
                Err(err) => return Some(Err(MultipartSegmentError::from(err))),
            };

            match udh.value() {
                Some(UdhValue::ConcatenatedShortMessage8Bit(concatenation)) => {
                    return Some(Ok((
                        // XXX: unchecked because `ConcatenatedShortMessage8Bit` already checks the invariants while decoding.
                        // See: `impl crate::decode::borrowed::Decode` for `ConcatenatedShortMessage8Bit`.
                        MultipartSegment::new_unchecked(
                            MultipartType::Udh { size },
                            concatenation.reference() as u16,
                            concatenation.total_parts(),
                            concatenation.part_number(),
                        ),
                        &self.short_message()[size..],
                    )));
                }
                Some(UdhValue::ConcatenatedShortMessage16Bit(concatenation)) => {
                    return Some(Ok((
                        // XXX: unchecked because `ConcatenatedShortMessage16Bit` already checks the invariants while decoding.
                        // See: `impl crate::decode::borrowed::Decode` for `ConcatenatedShortMessage16Bit`.
                        MultipartSegment::new_unchecked(
                            MultipartType::Udh { size },
                            concatenation.reference(),
                            concatenation.total_parts(),
                            concatenation.part_number(),
                        ),
                        &self.short_message()[size..],
                    )));
                }
                // The UDH is not a concatenated short message.
                _ => return None,
            }
        };

        match (
            self.sar_msg_ref_num(),
            self.sar_segment_seqnum(),
            self.sar_total_segments(),
        ) {
            (Some(reference), Some(part_number), Some(total_parts)) => {
                let segment = match MultipartSegment::new(
                    MultipartType::Sar,
                    reference,
                    total_parts,
                    part_number,
                ) {
                    Ok(value) => value,
                    Err(err) => return Some(Err(err)),
                };

                Some(Ok((segment, &self.short_message()[..])))
            }
            _ => None,
        }
    }
}
