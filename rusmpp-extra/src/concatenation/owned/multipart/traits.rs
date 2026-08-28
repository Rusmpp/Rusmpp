use rusmpp_core::{
    decode::borrowed::Decode,
    pdus::owned::{DeliverSm, SubmitSm},
    types::owned::OctetString,
    udhs::borrowed::{Udh, UdhValue},
    values::DataCoding,
};

use crate::{
    concatenation::multipart::{MultipartSegment, MultipartSegmentError, MultipartType},
    encoding::gsm7bit::Gsm7BitUnpackedEncoder,
};

use super::{SarMultipartBuilder, UdhMultipartBuilder};

/// Trait for [`SubmitSm`] and [`DeliverSm`] to create multipart messages.
pub trait Multipart: Sized {
    /// Creates a new [`UdhMultipartBuilder`] with the default [`Gsm7BitUnpackedEncoder`] encoder.
    ///
    /// # Notes
    ///
    /// - [`EsmClass`](rusmpp_core::values::EsmClass) will be updated with UDH indicator by the multipart builder.
    /// - [`DataCoding`] will be overridden by the multipart builder to match the encoder.
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

impl crate::Sealed for SubmitSm {}
impl crate::Sealed for DeliverSm {}

/// Something that holds a `short message` and a `data coding`.
///
/// Implemented for [`SubmitSm`] and [`DeliverSm`].
pub trait ShortMessage: crate::Sealed {
    fn with_data_coding(self, data_coding: DataCoding) -> Self;
    fn with_short_message(self, short_message: OctetString<0, 255>) -> Self;
    fn short_message(&self) -> &OctetString<0, 255>;
}

/// Something that can be concatenated using UDH.
///
/// Implemented for [`SubmitSm`] and [`DeliverSm`].
pub trait UdhMultipart: ShortMessage {
    fn with_udh_indicator(self) -> Self;
    fn is_udh_indicator_set(&self) -> bool;
}

/// Something that can be concatenated using SAR TLVs.
///
/// Implemented for [`SubmitSm`] and [`DeliverSm`].
pub trait SarMultipart: ShortMessage {
    fn with_sar_msg_ref_num(self, sar_msg_ref_num: u16) -> Self;
    fn with_sar_segment_seqnum(self, sar_segment_seqnum: u8) -> Self;
    fn with_sar_total_segments(self, sar_total_segments: u8) -> Self;
    fn sar_msg_ref_num(&self) -> Option<u16>;
    fn sar_segment_seqnum(&self) -> Option<u8>;
    fn sar_total_segments(&self) -> Option<u8>;
}

impl ShortMessage for SubmitSm {
    fn with_data_coding(self, data_coding: DataCoding) -> Self {
        self.with_data_coding(data_coding)
    }

    fn with_short_message(self, short_message: OctetString<0, 255>) -> Self {
        self.with_short_message(short_message)
    }

    fn short_message(&self) -> &OctetString<0, 255> {
        self.short_message()
    }
}

impl ShortMessage for DeliverSm {
    fn with_data_coding(self, data_coding: DataCoding) -> Self {
        self.with_data_coding(data_coding)
    }

    fn with_short_message(self, short_message: OctetString<0, 255>) -> Self {
        self.with_short_message(short_message)
    }

    fn short_message(&self) -> &OctetString<0, 255> {
        self.short_message()
    }
}

impl UdhMultipart for SubmitSm {
    fn with_udh_indicator(self) -> Self {
        self.with_udh_indicator()
    }

    fn is_udh_indicator_set(&self) -> bool {
        self.is_udh_indicator_set()
    }
}

impl UdhMultipart for DeliverSm {
    fn with_udh_indicator(self) -> Self {
        self.with_udh_indicator()
    }

    fn is_udh_indicator_set(&self) -> bool {
        self.is_udh_indicator_set()
    }
}

impl SarMultipart for SubmitSm {
    fn with_sar_msg_ref_num(self, sar_msg_ref_num: u16) -> Self {
        self.with_sar_msg_ref_num(sar_msg_ref_num)
    }

    fn with_sar_segment_seqnum(self, sar_segment_seqnum: u8) -> Self {
        self.with_sar_segment_seqnum(sar_segment_seqnum)
    }

    fn with_sar_total_segments(self, sar_total_segments: u8) -> Self {
        self.with_sar_total_segments(sar_total_segments)
    }

    fn sar_msg_ref_num(&self) -> Option<u16> {
        self.sar_msg_ref_num()
    }

    fn sar_segment_seqnum(&self) -> Option<u8> {
        self.sar_segment_seqnum()
    }

    fn sar_total_segments(&self) -> Option<u8> {
        self.sar_total_segments()
    }
}

impl SarMultipart for DeliverSm {
    fn with_sar_msg_ref_num(self, sar_msg_ref_num: u16) -> Self {
        self.with_sar_msg_ref_num(sar_msg_ref_num)
    }

    fn with_sar_segment_seqnum(self, sar_segment_seqnum: u8) -> Self {
        self.with_sar_segment_seqnum(sar_segment_seqnum)
    }

    fn with_sar_total_segments(self, sar_total_segments: u8) -> Self {
        self.with_sar_total_segments(sar_total_segments)
    }

    fn sar_msg_ref_num(&self) -> Option<u16> {
        self.sar_msg_ref_num()
    }

    fn sar_segment_seqnum(&self) -> Option<u8> {
        self.sar_segment_seqnum()
    }

    fn sar_total_segments(&self) -> Option<u8> {
        self.sar_total_segments()
    }
}
