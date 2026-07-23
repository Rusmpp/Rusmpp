use rusmpp_macros::Rusmpp;

use crate::{
    pdus::borrowed::Pdu,
    types::borrowed::{COctetString, EmptyOrFullCOctetString},
    values::*,
};

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Default, Rusmpp)]
#[rusmpp(decode = borrowed, test = skip)]
#[cfg_attr(feature = "arbitrary", derive(::arbitrary::Arbitrary))]
#[cfg_attr(feature = "serde", derive(::serde::Serialize))]
pub struct QuerySmResp<'a> {
    /// MC Message ID of the message whose
    /// state is being queried.
    pub message_id: COctetString<'a, 1, 65>,
    /// Date and time when the queried
    /// message reached a final state. For
    /// messages, which have not yet reached
    /// a final state, this field will contain a
    /// single NULL octet.
    pub final_date: EmptyOrFullCOctetString<'a, 17>,
    /// Specifies the status of the queried short
    /// message.
    pub message_state: MessageState,
    /// Where appropriate this holds a network
    /// error code defining the reason for failure
    /// of message delivery.
    ///
    /// The range of values returned depends
    /// on the underlying telecommunications
    /// network.
    pub error_code: u8,
}

impl<'a> QuerySmResp<'a> {
    pub fn new(
        message_id: COctetString<'a, 1, 65>,
        final_date: EmptyOrFullCOctetString<'a, 17>,
        message_state: MessageState,
        error_code: u8,
    ) -> Self {
        Self {
            message_id,
            final_date,
            message_state,
            error_code,
        }
    }

    pub fn builder() -> QuerySmRespBuilder<'a> {
        QuerySmRespBuilder::new()
    }
}

impl<'a, const N: usize> From<QuerySmResp<'a>> for Pdu<'a, N> {
    fn from(value: QuerySmResp<'a>) -> Self {
        Self::QuerySmResp(value)
    }
}

#[derive(Debug, Default)]
pub struct QuerySmRespBuilder<'a> {
    inner: QuerySmResp<'a>,
}

impl<'a> QuerySmRespBuilder<'a> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn message_id(mut self, message_id: COctetString<'a, 1, 65>) -> Self {
        self.inner.message_id = message_id;
        self
    }

    pub fn final_date(mut self, final_date: EmptyOrFullCOctetString<'a, 17>) -> Self {
        self.inner.final_date = final_date;
        self
    }

    pub fn message_state(mut self, message_state: MessageState) -> Self {
        self.inner.message_state = message_state;
        self
    }

    pub fn error_code(mut self, error_code: u8) -> Self {
        self.inner.error_code = error_code;
        self
    }

    pub fn build(self) -> QuerySmResp<'a> {
        self.inner
    }
}

#[cfg(any(test, feature = "test"))]
mod tests {
    use crate::tests::TestInstance;

    use super::*;

    impl TestInstance for QuerySmResp<'_> {
        fn instances() -> alloc::vec::Vec<Self> {
            alloc::vec![
                Self::default(),
                Self::builder()
                    .message_id(COctetString::new(b"123456789012345678901234\0").unwrap())
                    .final_date(EmptyOrFullCOctetString::new(b"2023-10-01T12:00\0").unwrap())
                    .message_state(MessageState::Delivered)
                    .error_code(0)
                    .build(),
            ]
        }
    }

    #[test]
    fn encode_decode() {
        crate::tests::borrowed::encode_decode_test_instances::<QuerySmResp>();
    }
}
