#[cfg(any(test, feature = "alloc"))]
pub mod owned {
    use rusmpp_core::{
        pdus::owned::{DeliverSm, SubmitMulti, SubmitSm},
        types::owned::OctetString,
        values::DataCoding,
    };

    impl crate::Sealed for SubmitSm {}
    impl crate::Sealed for DeliverSm {}
    impl crate::Sealed for SubmitMulti {}

    /// Something that holds a `short message` and a `data coding`.
    ///
    /// Implemented for [`SubmitSm`], [`DeliverSm`] and [`SubmitMulti`].
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

    impl ShortMessage for SubmitMulti {
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

    impl UdhMultipart for SubmitMulti {
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

    impl SarMultipart for SubmitMulti {
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
}
