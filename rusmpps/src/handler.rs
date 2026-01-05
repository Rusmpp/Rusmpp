use crate::bind_mode::BindMode;
use crate::client::Action;
use async_trait::async_trait;
use rusmpp::{CommandStatus, Pdu};
use std::fmt::Debug;
use std::net::SocketAddr;
use tokio::sync::mpsc::Sender;

#[async_trait]
pub trait Handler: Debug + Send {
    async fn handle_pdu(&self, sequence_number: u32, pdu: Pdu) -> Option<(Pdu, CommandStatus)>;
}

#[async_trait]
pub trait BindHandler: Debug + Send + Sync + 'static {
    type Handler: Handler;

    /// A bind was received. We will process it.
    /// If it is successful, then a handler managing received requests is returned.
    /// If it is not, then the CommandStatus is returned, often a CommandStatus::EsmeRbindfail
    ///
    /// # Arguments
    ///
    /// - `addr` - A `SocketAddr` specifying the IP address and port to bind to.
    /// - `bind_mode` - A `BindMode` indicating the type of bind operation to perform
    ///   (e.g., Transmitter, Receiver, or Transceiver modes).
    /// - `system_id` - A `&str` representing the identifier of the system connecting
    ///   to the external service.
    /// - `password` - A `&str` representing the password to authenticate the bind
    ///   operation with the external system.
    /// - `tx` - A `Sender<Action>` channel used to send actions or events relevant
    ///   to this binding session.
    ///
    /// # Returns
    ///
    /// A `Result<Self::Handler, CommandStatus>` where:
    /// - `Ok(Self::Handler)` contains the handler instance responsible for managing
    ///   the bound connection. In that case a CommandStatus::EsmeRok is returned
    /// - `Err(CommandStatus)` represents a failure during the bind operation,
    ///   encapsulating the reason for the failure. This command status will be used to respond to
    ///  the bind
    async fn bind(
        &self,
        addr: SocketAddr,
        bind_mode: BindMode,
        system_id: &str,
        password: &str,
        tx: Sender<Action>,
    ) -> Result<Self::Handler, CommandStatus>;
}
