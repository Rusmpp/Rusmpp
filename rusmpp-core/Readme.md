# Rusmpp-Core

![Build Status](https://github.com/Rusmpp/Rusmpp/actions/workflows/build-and-test.yml/badge.svg)
[![crates.io](https://img.shields.io/crates/v/rusmpp-core.svg)](https://crates.io/crates/rusmpp-core)
[![Crates.io (MSRV)](https://img.shields.io/crates/msrv/rusmpp-core)](https://crates.io/crates/rusmpp-core)
[![docs.rs](https://docs.rs/rusmpp/badge.svg)](https://docs.rs/rusmpp-core)
[![Crates.io (Downloads)](https://img.shields.io/crates/d/rusmpp-core)](https://crates.io/crates/rusmpp-core)
[![Crates.io (License)](https://img.shields.io/crates/l/rusmpp-core)](https://crates.io/crates/rusmpp-core)

Rust implementation of the [SMPP v5](https://smpp.org/SMPP_v5.pdf) protocol.

You may not want to depend on this crate directly as it may be hard to choose the right features. A set of pre defined features are made available for `std` and `no_std` environments in the [`rusmpp`](https://crates.io/crates/rusmpp) and [`rusmppz`](https://crates.io/crates/rusmppz)crates respectively.

## Features

- `alloc`:  Enables the `alloc` crate.
- `arbitrary`: Implements [`Arbitrary`](https://docs.rs/arbitrary/latest/arbitrary/trait.Arbitrary.html) trait for all SMPP types.
- `serde`: Implements [`Serialize`](https://docs.rs/serde/latest/serde/trait.Serialize.html) trait for all SMPP types.
- `serde-deserialize-unchecked`: Implements [`Deserialize`](https://docs.rs/serde/latest/serde/trait.Deserialize.html) trait for owned SMPP types, but does not check the validity of the data. Use with caution.
- `tokio-codec`: Implements [`tokio-util`](https://docs.rs/tokio-util/latest/tokio_util/index.html) [`Encoder`](https://docs.rs/tokio-util/latest/tokio_util/codec/trait.Encoder.html) and [`Decoder`](https://docs.rs/tokio-util/latest/tokio_util/codec/trait.Decoder.html) traits.
- `framez`: Implements [`framez`](https://docs.rs/framez/latest/framez/index.html) [`Encoder`](https://docs.rs/framez/latest/framez/encode/trait.Encoder.html) and [`Decoder`](https://docs.rs/framez/latest/framez/decode/trait.Decoder.html) traits.
- `tracing`: Enables logging using [`tracing`](https://docs.rs/tracing/latest/tracing/).
- `pretty-hex-fmt`: Logs byte slices like `[0x00, 0x00, 0x00, 0x6F]` instead of `[00, 00, 00, 6F]`, if `tracing` feature is enabled.
- `char-fmt`: Logs byte slices as characters, if `tracing` feature is enabled.

## License

Licensed under either of

- Apache License, Version 2.0. [LICENSE-APACHE](../LICENSE-APACHE) or [Apache-2.0 license](http://apache.org/licenses/LICENSE-2.0)
- MIT license. [LICENSE-MIT](../LICENSE-MIT) or [MIT license](http://opensource.org/licenses/MIT)

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall
be dual licensed as above, without any additional terms or conditions.
