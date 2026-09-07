//! Traits for decoding `SMPP` values.

pub use rusmpp_core::decode::{
    AnyOctetStringDecodeError, COctetStringDecodeError, DecodeError, DecodeErrorKind,
    HeaplessVecDecodeError, IntegerDecodeError, OctetStringDecodeError, borrowed::*,
};
