//! If we panic!, we lose.
//!
//! ```not_rust
//! cargo +nightly fuzz run decode_owned
//! ```

#![no_main]

extern crate alloc;

use arbitrary::Unstructured;
use bytes::BytesMut;
use libfuzzer_sys::fuzz_target;
use rusmpp_core::{
    command::owned::Command,
    decode::owned::DecodeWithLength,
    encode::{owned::Encode, Length},
    tokio_codec::CommandCodec,
};
use tokio_util::codec::Decoder;

fuzz_target!(|data: &[u8]| {
    let mut codec = CommandCodec::new().with_max_length(1024);

    // Garbage
    let mut src = BytesMut::from(data);
    let _ = Command::decode(&mut src, data.len());

    // Garbage with tokio's Decoder
    let mut src = BytesMut::from(data);
    let _ = codec.decode(&mut src);

    // Unstructured garbage
    let mut u = Unstructured::new(data);

    let command = u
        .arbitrary::<Command>()
        .expect("Failed to generate Command");

    let mut buf = BytesMut::with_capacity(command.length());

    // Encode the garbage
    command.encode(&mut buf);

    let mut buf = buf.split_to(command.length());

    // Decode the garbage
    let _ = Command::decode(&mut buf, command.length());

    // Decode the garbage with tokio's Decoder
    let mut src = BytesMut::from(data);
    let _ = codec.decode(&mut src);
});
