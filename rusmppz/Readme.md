# Rusmppz

![Build Status](https://github.com/Rusmpp/Rusmpp/actions/workflows/build-and-test.yml/badge.svg)
[![crates.io](https://img.shields.io/crates/v/rusmppz.svg)](https://crates.io/crates/rusmppz)
[![Crates.io (MSRV)](https://img.shields.io/crates/msrv/rusmppz)](https://crates.io/crates/rusmppz)
[![docs.rs](https://docs.rs/rusmpp/badge.svg)](https://docs.rs/rusmppz)
[![Crates.io (Downloads)](https://img.shields.io/crates/d/rusmppz)](https://crates.io/crates/rusmppz)
[![Crates.io (License)](https://img.shields.io/crates/l/rusmppz)](https://crates.io/crates/rusmppz)

A `zerocopy` Rust implementation of the [SMPP v5](https://smpp.org/SMPP_v5.pdf) protocol.

## Features

- `framez`: Implements [`framez`](https://docs.rs/framez/latest/framez/index.html) [`Encoder`](https://docs.rs/framez/latest/framez/encode/trait.Encoder.html) and [`Decoder`](https://docs.rs/framez/latest/framez/decode/trait.Decoder.html) traits.
- `tracing`: Enables logging using [`tracing`](https://docs.rs/tracing/latest/tracing/).

## License

Licensed under either of

- Apache License, Version 2.0. [LICENSE-APACHE](../LICENSE-APACHE) or [Apache-2.0 license](http://apache.org/licenses/LICENSE-2.0)
- MIT license. [LICENSE-MIT](../LICENSE-MIT) or [MIT license](http://opensource.org/licenses/MIT)

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall
be dual licensed as above, without any additional terms or conditions.
