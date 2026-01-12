use futures::StreamExt;
use rusmpp::pdus::builders::{DeliverSmBuilder, SubmitSmRespBuilder};
use rusmpp::types::COctetString;
use rusmpp::{CommandStatus, Pdu};
use rusmpps_lib::bind_requet_kind::BindRequestKind;
use rusmpps_lib::server::Server;
use std::str::FromStr;
use tokio::net::TcpListener;
use tracing::warn;

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    // Configure the server
    let server = Server::builder()
        // system_id, will be used in the BindResp
        .system_id(COctetString::from_static_slice(b"rusmpps\0")?)
        // Other config
        // .other_config(())
        // Close all connections gracefully and stop accepting connections and shutdown
        // .graceful_shutdown(async { _ = signal::ctrl_c().await })
        .build();

    let listener = TcpListener::bind("127.0.0.1:2345").await?;

    // The acceptor consumes the server and is `impl Stream<Item = Result<BindRequest, Error>> + Unpin + 'static`
    let mut acceptor = server.acceptor(listener);

    // The acceptor waits for an incoming tcp connection,
    //  then waits with a timeout for a bind command from the client.
    //  When the command arrives, we get it as an incoming `BindRequest`.
    while let Ok(request) = acceptor.next().await {
        tokio::spawn(async move {
            // Accept or reject the request here
            if matches!(request.kind(), BindRequestKind::Trx)
                && request.bind.system_id
                    == COctetString::from_static_slice(b"system_id\0").expect("Must be valid")
            {
                match request.accept().await {
                    Err(e) => {
                        tracing::error!("Error accepting connection: {e}");
                        return;
                    }
                    Ok((mut client, mut events)) => {
                        let client2 = client.clone();
                        tokio::spawn(async move {
                            while let Some(command) = events.next().await {
                                match command.pdu() {
                                    Some(Pdu::SubmitSm(submit_sm)) => {
                                        tracing::info!("Received submit_sm: {:?}", submit_sm);
                                        client
                                            .submit_sm_resp(
                                                command.sequence_number,
                                                SubmitSmRespBuilder::new()
                                                    .message_id(
                                                        COctetString::<1, 65>::from_str(
                                                            "123123123",
                                                        )
                                                        .unwrap(),
                                                    )
                                                    .build(),
                                            )
                                            .await;
                                    }
                                    Some(_) => {
                                        client.generic_nack(command.sequence_number).await;
                                    }
                                    None => {
                                        client.generic_nack(command.sequence_number).await;
                                    }
                                }
                            }
                            // If the event stream is closed, this means that the connection is closed.
                        });

                        tokio::spawn(async move {
                            match client2
                                .deliver_sm(
                                    DeliverSmBuilder::new()
                                        .source_addr(
                                            COctetString::<1, 21>::from_str("123456").unwrap(),
                                        )
                                        .destination_addr(
                                            COctetString::<1, 21>::from_str("123456").unwrap(),
                                        )
                                        .build(),
                                )
                                .await
                            {
                                Ok(resp) => {}
                                Err(e) => warn!("Error sending deliver_sm: {e:?} "),
                            }
                        });
                    }
                }
                // Drop the client to close the connection.
            } else {
                // Reject
                request.reject(CommandStatus::EsmeRbindfail).await;
            }
            // If the request is dropped, the tcp connection is closed. (no response would be sent to the client).
        });
    }

    Ok(())
}
