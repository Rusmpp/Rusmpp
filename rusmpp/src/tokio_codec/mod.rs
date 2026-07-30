//! Codec for encoding and decoding `SMPP` PDUs.

pub use rusmpp_tokio_codec::{
    CommandCodec,
    error::{DecodeError, EncodeError},
};
