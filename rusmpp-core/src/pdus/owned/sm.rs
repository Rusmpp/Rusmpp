use crate::{
    types::owned::OctetString,
    udhs::owned::{Udh, UdhDecodeError},
    values::DataCoding,
};

pub trait Sm: crate::Sealed {
    fn with_udh_indicator(self) -> Self;
    fn with_data_coding(self, data_coding: DataCoding) -> Self;
    fn with_sar_msg_ref_num(self, sar_msg_ref_num: u16) -> Self;
    fn with_sar_segment_seqnum(self, sar_segment_seqnum: u8) -> Self;
    fn with_sar_total_segments(self, sar_total_segments: u8) -> Self;
    fn with_short_message(self, short_message: OctetString<0, 255>) -> Self;
    fn udh_indicator_exists(&self) -> bool;
    fn sar_msg_ref_num(&self) -> Option<u16>;
    fn sar_segment_seqnum(&self) -> Option<u8>;
    fn sar_total_segments(&self) -> Option<u8>;
    fn short_message(&self) -> &OctetString<0, 255>;
    fn udh(&self) -> Option<Result<Udh, UdhDecodeError>> {
        if self.udh_indicator_exists() {
            // TODO: how the fuck do we decode the UDH from Bytes without creating BytesMut?
            todo!()
        }

        None
    }
}

impl Sm for super::SubmitSm {
    fn with_udh_indicator(self) -> Self {
        self.with_udh_indicator()
    }

    fn with_data_coding(self, data_coding: DataCoding) -> Self {
        self.with_data_coding(data_coding)
    }

    fn with_sar_msg_ref_num(self, sar_msg_ref_num: u16) -> Self {
        self.with_sar_msg_ref_num(sar_msg_ref_num)
    }

    fn with_sar_segment_seqnum(self, sar_segment_seqnum: u8) -> Self {
        self.with_sar_segment_seqnum(sar_segment_seqnum)
    }

    fn with_sar_total_segments(self, sar_total_segments: u8) -> Self {
        self.with_sar_total_segments(sar_total_segments)
    }

    fn with_short_message(self, short_message: OctetString<0, 255>) -> Self {
        self.with_short_message(short_message)
    }

    fn udh_indicator_exists(&self) -> bool {
        self.is_udh_indicator_set()
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

    fn short_message(&self) -> &OctetString<0, 255> {
        self.short_message()
    }
}

impl Sm for super::SubmitMulti {
    fn with_udh_indicator(self) -> Self {
        self.with_udh_indicator()
    }

    fn with_data_coding(self, data_coding: DataCoding) -> Self {
        self.with_data_coding(data_coding)
    }

    fn with_sar_msg_ref_num(self, sar_msg_ref_num: u16) -> Self {
        self.with_sar_msg_ref_num(sar_msg_ref_num)
    }

    fn with_sar_segment_seqnum(self, sar_segment_seqnum: u8) -> Self {
        self.with_sar_segment_seqnum(sar_segment_seqnum)
    }

    fn with_sar_total_segments(self, sar_total_segments: u8) -> Self {
        self.with_sar_total_segments(sar_total_segments)
    }

    fn with_short_message(self, short_message: OctetString<0, 255>) -> Self {
        self.with_short_message(short_message)
    }

    fn udh_indicator_exists(&self) -> bool {
        self.is_udh_indicator_set()
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

    fn short_message(&self) -> &OctetString<0, 255> {
        self.short_message()
    }
}

impl Sm for super::DeliverSm {
    fn with_udh_indicator(self) -> Self {
        self.with_udh_indicator()
    }

    fn with_data_coding(self, data_coding: DataCoding) -> Self {
        self.with_data_coding(data_coding)
    }

    fn with_sar_msg_ref_num(self, sar_msg_ref_num: u16) -> Self {
        self.with_sar_msg_ref_num(sar_msg_ref_num)
    }

    fn with_sar_segment_seqnum(self, sar_segment_seqnum: u8) -> Self {
        self.with_sar_segment_seqnum(sar_segment_seqnum)
    }

    fn with_sar_total_segments(self, sar_total_segments: u8) -> Self {
        self.with_sar_total_segments(sar_total_segments)
    }

    fn with_short_message(self, short_message: OctetString<0, 255>) -> Self {
        self.with_short_message(short_message)
    }

    fn udh_indicator_exists(&self) -> bool {
        self.is_udh_indicator_set()
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

    fn short_message(&self) -> &OctetString<0, 255> {
        self.short_message()
    }
}
