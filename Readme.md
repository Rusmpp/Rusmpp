# Rusmpp

![Build Status](https://github.com/Rusmpp/Rusmpp/actions/workflows/build-and-test.yml/badge.svg)
[![crates.io](https://img.shields.io/crates/v/rusmpp.svg)](https://crates.io/crates/rusmpp)
[![Crates.io (MSRV)](https://img.shields.io/crates/msrv/rusmpp)](https://crates.io/crates/rusmpp)
[![docs.rs](https://docs.rs/rusmpp/badge.svg)](https://docs.rs/rusmpp)
[![Crates.io (Downloads)](https://img.shields.io/crates/d/rusmpp)](https://crates.io/crates/rusmpp)
[![Crates.io (License)](https://img.shields.io/crates/l/rusmpp)](https://crates.io/crates/rusmpp)

Rust implementation of the [SMPP v5](https://smpp.org/SMPP_v5.pdf) protocol.

This is a low level library for implementing clients and servers. If you are looking for a client, check out [rusmppc](https://crates.io/crates/rusmppc).

```rust
use core::error::Error;
use futures::{SinkExt, StreamExt};
use rusmpp::{
    tokio_codec::{EncodeError, CommandCodec},
    Command, CommandId, CommandStatus, Pdu,
};
use tokio::io::DuplexStream;
use tokio_util::codec::Framed;
use tracing::info;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Rusmpp produces a lot of logs while decoding and encoding PDUs.
    // You can filter them out by setting the `rusmpp` target to `off`,
    // or by disabling the `tracing` feature.
    tracing_subscriber::fmt()
        .with_env_filter("client=info,server=info,rusmpp=trace")
        .init();

    // In-memory duplex stream to simulate a server and client.
    let (server_stream, client_stream) = tokio::io::duplex(4096);

    launch_server(server_stream).await?;

    // The CommandCodec encodes/decodes SMPP commands into/from bytes.
    let mut framed = Framed::new(client_stream, CommandCodec::new());

    // Rusmpp takes care of setting the correct command ID.
    let command = Command::new(CommandStatus::EsmeRok, 1, Pdu::EnquireLink);

    info!(target: "client", "EnquireLink sent");

    framed.send(command).await?;

    while let Some(Ok(command)) = framed.next().await {
        if let CommandId::EnquireLinkResp = command.id() {
            info!(target: "client", "EnquireLink response received");

            break;
        }
    }

    Ok(())
}

async fn launch_server(stream: DuplexStream) -> Result<(), Box<dyn Error>> {
    tokio::spawn(async move {
        let mut framed = Framed::new(stream, CommandCodec::new());

        while let Some(Ok(command)) = framed.next().await {
            if let CommandId::EnquireLink = command.id() {
                info!(target: "server", "EnquireLink received");

                // We can also use the Command::builder() to create commands.
                let response = Command::builder()
                    .status(CommandStatus::EsmeRok)
                    .sequence_number(command.sequence_number())
                    .pdu(Pdu::EnquireLinkResp);

                framed.send(response).await?;

                info!(target: "server", "EnquireLink response sent");

                break;
            }
        }

        Ok::<(), EncodeError>(())
    });

    Ok(())
}
```

See the [examples](https://github.com/Rusmpp/Rusmpp/tree/main/rusmpp/examples) directory for more examples.

## Features

- `tokio-codec`: Implements [`Encoder`](https://docs.rs/tokio-util/latest/tokio_util/codec/trait.Encoder.html) and [`Decoder`](https://docs.rs/tokio-util/latest/tokio_util/codec/trait.Decoder.html) traits.
- `extra`: Enables encoding/decoding and concatenation support for `SubmitSm`.
- `serde`: Implements [`Serialize`](https://docs.rs/serde/latest/serde/trait.Serialize.html) trait for all SMPP types.
- `serde-deserialize-unchecked`: Implements [`Deserialize`](https://docs.rs/serde/latest/serde/trait.Deserialize.html) trait for all SMPP types, but does not check the validity of the data. Use with caution.
- `tracing`: Enables logging using [`tracing`](https://docs.rs/tracing/latest/tracing/).
- `pretty-hex-fmt`: Logs byte slices like `[0x00, 0x00, 0x00, 0x6F]` instead of `[00, 00, 00, 6F]`, if `tracing` feature is enabled.
- `char-fmt`: Logs byte slices as characters, if `tracing` feature is enabled.

## License

Licensed under either of

- Apache License, Version 2.0. [LICENSE-APACHE](LICENSE-APACHE) or [Apache-2.0 license](http://apache.org/licenses/LICENSE-2.0)
- MIT license. [LICENSE-MIT](LICENSE-MIT) or [MIT license](http://opensource.org/licenses/MIT)

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall
be dual licensed as above, without any additional terms or conditions.
