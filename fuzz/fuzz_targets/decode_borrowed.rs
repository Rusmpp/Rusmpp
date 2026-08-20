//! If we panic!, we lose.
//!
//! ```not_rust
//! cargo +nightly fuzz run decode_borrowed
//! ```

#![no_main]

extern crate alloc;

use arbitrary::Unstructured;
use bytes::BytesMut;
use framez::decode::Decoder;
use libfuzzer_sys::fuzz_target;
use rusmpp_core::{
    command::borrowed::Command,
    decode::borrowed::DecodeWithLength,
    encode::{Encode, Length},
};
use rusmppz::framez::CommandCodec;

fuzz_target!(|data: &[u8]| {
    let mut codec = CommandCodec::<1024>::new();

    // Garbage
    let _ = Command::<1024>::decode(data, data.len());

    let mut bytes = BytesMut::new();
    bytes.extend_from_slice(data);

    // Garbage with framez' Decoder
    let _ = codec.decode(&mut bytes);

    // Unstructured garbage
    let mut u = Unstructured::new(data);

    let command = u
        .arbitrary::<Command<1024>>()
        .expect("Failed to generate Command");

    let mut buf = ::alloc::vec![0u8; command.length()];

    let mut bytes = BytesMut::new();
    bytes.extend_from_slice(&buf);

    // Encode the garbage
    let size = command.encode(&mut buf);

    // Decode the garbage
    let _ = Command::<1024>::decode(&buf[..size], command.length());

    // Decode the garbage with framez' Decoder
    let _ = codec.decode(&mut bytes);
});
