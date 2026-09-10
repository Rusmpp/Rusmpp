#[cfg(any(test, feature = "alloc"))]
pub mod owned {
    use rusmpp_core::{
        decode::owned::DecodeWithLength,
        pdus::owned::{DeliverSm, SubmitMulti, SubmitSm},
        types::owned::OctetString,
        udhs::owned::{Udh, UdhDecodeError},
        values::{DataCoding, owned::MessagePayload},
    };

    impl crate::Sealed for SubmitSm {}
    impl crate::Sealed for DeliverSm {}
    impl crate::Sealed for SubmitMulti {}

    pub trait Sm: crate::Sealed {
        fn with_udh_indicator(self) -> Self;
        fn with_data_coding(self, data_coding: DataCoding) -> Self;
        fn with_sar_msg_ref_num(self, sar_msg_ref_num: u16) -> Self;
        fn with_sar_segment_seqnum(self, sar_segment_seqnum: u8) -> Self;
        fn with_sar_total_segments(self, sar_total_segments: u8) -> Self;
        fn with_short_message(self, short_message: OctetString<0, 255>) -> Self;
        fn with_message_payload(self, message_payload: MessagePayload) -> Self;

        fn udh_indicator_exists(&self) -> bool;

        fn sar_msg_ref_num(&self) -> Option<u16>;
        fn sar_segment_seqnum(&self) -> Option<u8>;
        fn sar_total_segments(&self) -> Option<u8>;
        fn short_message(&self) -> &OctetString<0, 255>;
        fn message_payload(&self) -> Option<&MessagePayload>;

        fn udh(&self) -> Option<Result<(Udh, &[u8]), UdhDecodeError>> {
            if !self.udh_indicator_exists() {
                return None;
            }

            let (mut slice, length) = match self.message_payload() {
                Some(payload) => {
                    let length = payload.len();
                    let slice = payload.iter().as_slice();

                    (slice, length)
                }
                None => {
                    let length = self.short_message().len();
                    let slice = self.short_message().iter().as_slice();

                    (slice, length)
                }
            };

            match Udh::decode(&mut slice, length) {
                Ok((udh, size)) => Some(Ok((udh, &self.short_message()[size..]))),
                Err(err) => Some(Err(err)),
            }
        }
    }

    impl Sm for SubmitSm {
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

        fn with_message_payload(self, message_payload: MessagePayload) -> Self {
            self.with_message_payload(message_payload)
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

        fn message_payload(&self) -> Option<&MessagePayload> {
            self.message_payload()
        }
    }

    impl Sm for SubmitMulti {
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

        fn with_message_payload(self, message_payload: MessagePayload) -> Self {
            self.with_message_payload(message_payload)
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

        fn message_payload(&self) -> Option<&MessagePayload> {
            self.message_payload()
        }
    }

    impl Sm for DeliverSm {
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

        fn with_message_payload(self, message_payload: MessagePayload) -> Self {
            self.with_message_payload(message_payload)
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

        fn message_payload(&self) -> Option<&MessagePayload> {
            self.message_payload()
        }
    }
}
