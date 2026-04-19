//! Traits for decoding `SMPP` values.

pub use rusmpp_core::decode::{
    AnyOctetStringDecodeError, COctetStringDecodeError, ConcatenatedShortMessageDecodeError,
    IntegerDecodeError, OctetStringDecodeError, UdhDecodeError, VecDecodeError, owned::*,
};
