use async_trait::async_trait;
use clap::Parser;
use rusmpp::pdus::SubmitSmResp;
use rusmpp::{CommandStatus, Pdu};
use rusmpps::bind_mode::BindMode;
use rusmpps::client::Action;
use rusmpps::handler::{BindHandler, Handler};
use rusmpps::{
    args::Args,
    config::Config,
    server::{Server, ServerParameters},
};
use std::net::SocketAddr;
use tokio::sync::mpsc::Sender;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt()
        .with_env_filter("rusmpps=trace")
        .init();

    dotenvy::dotenv().ok();

    let args = Args::parse();

    let config = Config::from_yaml_file(args.config_file).unwrap_or_else(|err| {
        tracing::error!("Failed to load config: {}", err);
        tracing::warn!("Using default configuration");

        Config::default()
    });

    tracing::info!(?config);

    let parameters = ServerParameters {
        clients: vec![],
        enquire_link_interval: config.enquire_link_interval,
        enquire_link_response_timeout: config.enquire_link_response_timeout,
        enquire_link_response_delay: config.enquire_link_response_delay,
        session_timeout: config.session_timeout,
        bind_delay: config.bind_delay,
        response_delay: config.response_delay,
        socket_addr: config.socket_addr,
        bind_handler: DefaultBindHandler,
    };

    let server = Server::new(parameters);

    tracing::info!("Starting server");

    tokio::select! {
        result = server.run() => {
            result?;
        }
        _ = shutdown_signal() => {

        }
    }

    Ok(())
}

async fn shutdown_signal() {
    let ctrl_c = async {
        tokio::signal::ctrl_c()
            .await
            .expect("Failed to install CTRL+C signal handler");

        tracing::info!("CTRL+C received");
    };

    #[cfg(unix)]
    let terminate = async {
        tokio::signal::unix::signal(tokio::signal::unix::SignalKind::terminate())
            .expect("Failed to install SIGTERM signal handler")
            .recv()
            .await;

        tracing::info!("SIGTERM received");
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => {},
        _ = terminate => {},
    }

    tracing::info!("Shutting down");
}

#[derive(Debug)]
pub struct DefaultHandler;

#[async_trait]
impl Handler for DefaultHandler {
    /// ```rust
    ///     /**
    ///      * Asynchronously handles a Protocol Data Unit (PDU) along with its sequence number.
    ///      *
    ///      * This method is used to process an incoming PDU and its associated sequence number.
    ///      * The implementation logs the sequence number and PDU for debugging purposes.
    ///      * Currently, the method does not perform additional logic and always returns `None`.
    ///      *
    ///      * # Parameters
    ///      * - `sequence_number`: The sequence number associated with the incoming PDU.
    ///      * - `pdu`: The Protocol Data Unit (PDU) to be processed.
    ///      *
    ///      * # Returns
    ///      * - An `Option<(Pdu, CommandStatus)>` where:
    ///      *   - `Some((Pdu, CommandStatus))`: Represents a response PDU along with a status indicating the outcome of the processing.
    ///      *   - `None`: Indicates no response is generated (default in this implementation).
    ///      *
    ///      * # Side Effects
    ///      * - Logs the sequence number and PDU to the console using `println!`.
    ///      *
    ///      * # Example
    ///      * ```
    ///      * let result = handler.handle_pdu(42, pdu).await;
    ///      * assert!(result.is_none());
    ///      * ```
    ///      *
    ///      * # Notes
    ///      * - This is a placeholder function and may need further implementation to handle specific PDU types and business logic.
    ///      */
    ///     async fn handle_pdu(&self, sequence_number: u32, pdu: Pdu) -> Option<(Pdu, CommandStatus)> {
    ///         println!("{sequence_number}-{:?}", pdu);
    ///         None
    ///     }
    /// ```
    async fn handle_pdu(&self, sequence_number: u32, pdu: Pdu) -> Option<(Pdu, CommandStatus)> {
        match pdu {
            Pdu::SubmitSm(_) => Some((
                SubmitSmResp::builder().build().into(),
                CommandStatus::EsmeRok,
            )),
            _ => {
                tracing::warn!("Received unsupported PDU: {sequence_number} {pdu:?}");
                None
            }
        }
    }
}

#[derive(Debug)]
pub struct DefaultBindHandler;

#[async_trait]
impl BindHandler for DefaultBindHandler {
    type Handler = DefaultHandler;

    async fn bind(
        &self,
        _addr: SocketAddr,
        _bind_mode: BindMode,
        _system_id: &str,
        _password: &str,
        _tx: Sender<Action>,
    ) -> Result<Self::Handler, CommandStatus> {
        Ok(DefaultHandler)
    }
}
