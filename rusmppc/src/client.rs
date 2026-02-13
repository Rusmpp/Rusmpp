use std::{
    sync::{
        Arc,
        atomic::{AtomicU32, Ordering},
    },
    time::Duration,
};

use futures::TryFutureExt;
use rusmpp::{
    Command, CommandId, CommandStatus, Pdu,
    command::CommandParts,
    pdus::{
        BindReceiver, BindReceiverResp, BindTransceiver, BindTransceiverResp, BindTransmitter,
        BindTransmitterResp, BroadcastSm, BroadcastSmResp, CancelBroadcastSm, CancelSm, DataSm,
        DataSmResp, DeliverSmResp, QueryBroadcastSm, QueryBroadcastSmResp, QuerySm, QuerySmResp,
        ReplaceSm, SubmitMulti, SubmitMultiResp, SubmitSm, SubmitSmResp,
    },
    values::InterfaceVersion,
};
use tokio::sync::{mpsc::UnboundedSender, oneshot, watch};

use crate::{
    Action, CloseRequest, CommandExt, ConnectionBuilder, PendingResponses, RegisteredRequest,
    RequestFutureGuard, UnregisteredRequest, error::Error,
};

const TARGET: &str = "rusmppc::client";

/// `SMPP` Client.
///
/// The client is a handle to communicate with the `SMPP` server through a managed connection in the background.
#[derive(Debug)]
pub struct Client {
    inner: Arc<ClientInner>,
}

impl Clone for Client {
    fn clone(&self) -> Self {
        Self {
            inner: self.inner.clone(),
        }
    }
}

impl Client {
    pub(crate) fn new(
        actions: UnboundedSender<Action>,
        response_timeout: Option<Duration>,
        check_interface_version: bool,
        watch: watch::Sender<()>,
    ) -> Self {
        Self {
            inner: Arc::new(ClientInner::new(
                actions,
                response_timeout,
                check_interface_version,
                watch,
            )),
        }
    }

    /// Creates a new `SMPP` connection builder.
    ///
    /// See [`ConnectionBuilder::new`] for more details.
    pub fn builder() -> ConnectionBuilder {
        ConnectionBuilder::new()
    }

    /// Sends a [`BindTransmitter`] command to the server and waits for a successful [`BindTransmitterResp`].
    pub async fn bind_transmitter(
        &self,
        bind: impl Into<BindTransmitter>,
    ) -> Result<BindTransmitterResp, Error> {
        self.registered_request().bind_transmitter(bind).await
    }

    /// Sends a [`BindReceiver`] command to the server and waits for a successful [`BindReceiverResp`].
    pub async fn bind_receiver(
        &self,
        bind: impl Into<BindReceiver>,
    ) -> Result<BindReceiverResp, Error> {
        self.registered_request().bind_receiver(bind).await
    }

    /// Sends a [`BindTransceiver`] command to the server and waits for a successful [`BindTransceiverResp`].
    pub async fn bind_transceiver(
        &self,
        bind: impl Into<BindTransceiver>,
    ) -> Result<BindTransceiverResp, Error> {
        self.registered_request().bind_transceiver(bind).await
    }

    /// Sends a [`BroadcastSm`] command to the server and waits for a successful [`BroadcastSmResp`].
    pub async fn broadcast_sm(
        &self,
        broadcast_sm: impl Into<BroadcastSm>,
    ) -> Result<BroadcastSmResp, Error> {
        self.registered_request().broadcast_sm(broadcast_sm).await
    }

    /// Sends a [`CancelBroadcastSm`] command to the server and waits for a successful [`CancelBroadcastSmResp`](Pdu::CancelBroadcastSmResp).
    pub async fn cancel_broadcast_sm(
        &self,
        cancel_broadcast_sm: impl Into<CancelBroadcastSm>,
    ) -> Result<(), Error> {
        self.registered_request()
            .cancel_broadcast_sm(cancel_broadcast_sm)
            .await
    }

    /// Sends a [`CancelSm`] command to the server and waits for a successful [`CancelSmResp`](Pdu::CancelSmResp).
    pub async fn cancel_sm(&self, cancel_sm: impl Into<CancelSm>) -> Result<(), Error> {
        self.registered_request().cancel_sm(cancel_sm).await
    }

    /// Sends a [`DataSm`] command to the server and waits for a successful [`DataSmResp`].
    pub async fn data_sm(&self, data_sm: impl Into<DataSm>) -> Result<DataSmResp, Error> {
        self.registered_request().data_sm(data_sm).await
    }

    /// Sends a [`DataSmResp`] command to the server.
    pub async fn data_sm_resp(
        &self,
        sequence_number: u32,
        data_sm_resp: impl Into<DataSmResp>,
    ) -> Result<(), Error> {
        self.unregistered_request()
            .data_sm_resp(sequence_number, data_sm_resp)
            .await
    }

    /// Sends a [`DeliverSmResp`] command to the server.
    pub async fn deliver_sm_resp(
        &self,
        sequence_number: u32,
        deliver_sm_resp: impl Into<DeliverSmResp>,
    ) -> Result<(), Error> {
        self.unregistered_request()
            .deliver_sm_resp(sequence_number, deliver_sm_resp)
            .await
    }

    /// Sends a [`QueryBroadcastSm`] command to the server and waits for a successful [`QueryBroadcastSmResp`].
    pub async fn query_broadcast_sm(
        &self,
        query_broadcast_sm: impl Into<QueryBroadcastSm>,
    ) -> Result<QueryBroadcastSmResp, Error> {
        self.registered_request()
            .query_broadcast_sm(query_broadcast_sm)
            .await
    }

    /// Sends a [`QuerySm`] command to the server and waits for a successful [`QuerySmResp`].
    pub async fn query_sm(&self, query_sm: impl Into<QuerySm>) -> Result<QuerySmResp, Error> {
        self.registered_request().query_sm(query_sm).await
    }

    /// Sends a [`ReplaceSm`] command to the server and waits for a successful [`ReplaceSmResp`](Pdu::ReplaceSmResp).
    pub async fn replace_sm(&self, replace_sm: impl Into<ReplaceSm>) -> Result<(), Error> {
        self.registered_request().replace_sm(replace_sm).await
    }

    /// Sends a [`SubmitMulti`] command to the server and waits for a successful [`SubmitMultiResp`].
    pub async fn submit_multi(
        &self,
        submit_multi: impl Into<SubmitMulti>,
    ) -> Result<SubmitMultiResp, Error> {
        self.registered_request().submit_multi(submit_multi).await
    }

    /// Sends a [`SubmitSm`] command to the server and waits for a successful [`SubmitSmResp`].
    pub async fn submit_sm(&self, submit_sm: impl Into<SubmitSm>) -> Result<SubmitSmResp, Error> {
        self.registered_request().submit_sm(submit_sm).await
    }

    /// Sends an [`Unbind`](Pdu::Unbind) command to the server and waits for a successful [`UnbindResp`](Pdu::UnbindResp).
    pub async fn unbind(&self) -> Result<(), Error> {
        self.registered_request().unbind().await
    }

    /// Sends an [`UnbindResp`](Pdu::UnbindResp) command to the server.
    pub async fn unbind_resp(&self, sequence_number: u32) -> Result<(), Error> {
        self.unregistered_request()
            .unbind_resp(sequence_number)
            .await
    }

    /// Sends an [`EnquireLink`](Pdu::EnquireLink) command to the server and waits for a successful [`EnquireLinkResp`](Pdu::EnquireLinkResp).
    pub async fn enquire_link(&self) -> Result<(), Error> {
        self.registered_request().enquire_link().await
    }

    /// Sends an [`EnquireLinkResp`](Pdu::EnquireLinkResp) command to the server.
    pub async fn enquire_link_resp(&self, sequence_number: u32) -> Result<(), Error> {
        self.unregistered_request()
            .enquire_link_resp(sequence_number)
            .await
    }

    /// Sends a [`GenericNack`](Pdu::GenericNack) command to the server.
    pub async fn generic_nack(&self, sequence_number: u32) -> Result<(), Error> {
        self.unregistered_request()
            .generic_nack(sequence_number)
            .await
    }

    /// Closes the connection.
    ///
    /// This method completes, when the connection has registered the close request.
    /// The connection will stop reading from the server, stop time keeping, close the requests channel, flush pending requests and terminate.
    ///
    /// After calling this method, clients can no longer send requests to the server.
    pub async fn close(&self) -> Result<(), Error> {
        self.inner.close().await
    }

    /// Checks if the connection is closed.
    ///
    /// # Note
    ///
    /// If the connection is not closed, this does not mean that it is active.
    /// The connection may be in the process of closing.
    ///
    /// To check if the connection is active, use [`Client::is_active()`].
    pub fn is_closed(&self) -> bool {
        self.inner.watch.is_closed()
    }

    /// Completes when the connection is closed.
    pub async fn closed(&self) {
        self.inner.watch.closed().await
    }

    /// Closes the connection and waits for it to terminate.
    pub async fn close_and_wait(&self) -> Result<(), Error> {
        self.close().await?;
        self.closed().await;

        Ok(())
    }

    /// Checks if the connection is active.
    ///
    /// The connection is considered active if:
    ///  - [`Client::close()`] was never called.
    ///  - The connection did not encounter an error.
    ///  - The connection can receive requests form the client.
    ///
    /// # Note
    ///
    /// If the connection is not active, this does not mean that it is closed.
    /// The connection may be in the process of closing.
    ///
    /// To check if the connection is closed, use [`Client::is_closed()`].
    pub fn is_active(&self) -> bool {
        // If the connection is not active, closing or errored,
        // it will close the actions channel and stop receiving actions, this call would fail.
        self.inner.actions.send(Action::Ping).is_ok()
    }

    /// Returns a vector of pending responses.
    pub async fn pending_responses(&self) -> Result<Vec<u32>, Error> {
        let (pending_responses, ack) = PendingResponses::new();

        self.inner
            .actions
            .send(Action::PendingResponses(pending_responses))
            .map_err(|_| Error::ConnectionClosed)?;

        ack.await.map_err(|_| Error::ConnectionClosed)?
    }

    /// Sets the command status for the next request.
    pub const fn status(&'_ self, status: CommandStatus) -> UnregisteredRequestBuilder<'_> {
        self.unregistered_request().status(status)
    }

    /// Sets the response timeout for the next request.
    pub fn response_timeout(&'_ self, timeout: Duration) -> RegisteredRequestBuilder<'_> {
        self.registered_request().response_timeout(timeout)
    }

    /// Disables the response timeout for the next request.
    pub fn no_response_timeout(&'_ self) -> RegisteredRequestBuilder<'_> {
        self.registered_request().no_response_timeout()
    }

    /// Sends a request without waiting for a response.
    pub const fn no_wait(&'_ self) -> NoWaitRequestBuilder<'_> {
        self.no_wait_request()
    }

    /// Sends a raw request to the server.
    pub fn raw(&'_ self) -> RawRegisteredRequestBuilder<'_> {
        self.raw_request()
    }

    const fn unregistered_request(&'_ self) -> UnregisteredRequestBuilder<'_> {
        UnregisteredRequestBuilder::new(self, CommandStatus::EsmeRok)
    }

    fn registered_request(&'_ self) -> RegisteredRequestBuilder<'_> {
        RegisteredRequestBuilder::new(self, CommandStatus::EsmeRok)
    }

    const fn no_wait_request(&'_ self) -> NoWaitRequestBuilder<'_> {
        NoWaitRequestBuilder::new(self, CommandStatus::EsmeRok)
    }

    fn raw_request(&'_ self) -> RawRegisteredRequestBuilder<'_> {
        RawRegisteredRequestBuilder::new(self, CommandStatus::EsmeRok)
    }
}

#[derive(Debug)]
struct ClientInner {
    actions: UnboundedSender<Action>,
    response_timeout: Option<Duration>,
    sequence_number: AtomicU32,
    check_interface_version: bool,
    watch: watch::Sender<()>,
}

impl ClientInner {
    const fn new(
        actions: UnboundedSender<Action>,
        response_timeout: Option<Duration>,
        check_interface_version: bool,
        watch: watch::Sender<()>,
    ) -> Self {
        Self {
            actions,
            response_timeout,
            sequence_number: AtomicU32::new(1),
            check_interface_version,
            watch,
        }
    }

    fn next_sequence_number(&self) -> u32 {
        self.sequence_number.fetch_add(2, Ordering::Relaxed)
    }

    async fn close(&self) -> Result<(), Error> {
        let (request, ack) = CloseRequest::new();

        self.actions
            .send(Action::Close(request))
            .map_err(|_| Error::ConnectionClosed)?;

        ack.await.map_err(|_| Error::ConnectionClosed)
    }

    /// Sends a [`Command`] to the server and returns a receiver for the response.
    ///
    /// # Note
    ///
    /// This function does not use the [`RequestFutureGuard`] to remove the pending response on drop.
    /// The caller is responsible for handling that.
    async fn send_registered(&self, command: Command) -> Result<oneshot::Receiver<Command>, Error> {
        let sequence_number = command.sequence_number();
        let status = command.status();
        let id = command.id();

        tracing::trace!(target: TARGET, sequence_number, ?status, ?id, "Sending request");

        let (request, ack, response) = RegisteredRequest::new(command);

        self.actions
            .send(Action::registered_request(request))
            .map_err(|_| Error::ConnectionClosed)?;

        tracing::trace!(target: TARGET, sequence_number, ?status, ?id, "Waiting for ack");

        ack.await.map_err(|_| Error::ConnectionClosed)??;

        Ok(response)
    }

    /// Awaits a response for a sent command.
    ///
    /// # Note
    ///
    /// This function does not perform any validation on the response.
    async fn await_response(
        &self,
        response: oneshot::Receiver<Command>,
        sequence_number: u32,
        response_timeout: Option<Duration>,
    ) -> Result<Command, Error> {
        tracing::trace!(target: TARGET, sequence_number, timeout = ?response_timeout, "Waiting for response");

        match response_timeout {
            None => response.await.map_err(|_| Error::ConnectionClosed),
            Some(timeout) => tokio::time::timeout(timeout, response)
                .await
                .inspect_err(|_| {
                    self.actions.send(Action::Remove(sequence_number)).ok();
                })
                .map_err(|_| Error::response_timeout(sequence_number, timeout))?
                .map_err(|_| Error::ConnectionClosed),
        }
    }

    /// See [`Self::send_registered`] and [`Self::await_response`].
    async fn send_registered_and_await_response(
        &self,
        command: Command,
        response_timeout: Option<Duration>,
    ) -> Result<Command, Error> {
        let sequence_number = command.sequence_number();
        let status = command.status();
        let id = command.id();

        let response = self.send_registered(command).await?;

        tracing::trace!(target: TARGET, sequence_number, ?status, ?id, timeout = ?response_timeout, "Starting response timer");

        self.await_response(response, sequence_number, response_timeout)
            .await
    }
}

#[derive(Debug)]
pub struct UnregisteredRequestBuilder<'a> {
    client: &'a Client,
    status: CommandStatus,
}

impl<'a> UnregisteredRequestBuilder<'a> {
    const fn new(client: &'a Client, status: CommandStatus) -> Self {
        Self { client, status }
    }

    fn registered_request(&'_ self) -> RegisteredRequestBuilder<'_> {
        RegisteredRequestBuilder::new(self.client, self.status)
    }

    const fn no_wait_request(&'_ self) -> NoWaitRequestBuilder<'_> {
        NoWaitRequestBuilder::new(self.client, self.status)
    }

    pub const fn status(mut self, status: CommandStatus) -> Self {
        self.status = status;
        self
    }

    pub fn response_timeout(&'_ self, timeout: Duration) -> RegisteredRequestBuilder<'_> {
        self.registered_request().response_timeout(timeout)
    }

    pub fn no_response_timeout(&'_ self) -> RegisteredRequestBuilder<'_> {
        self.registered_request().no_response_timeout()
    }

    pub const fn no_wait(&'_ self) -> NoWaitRequestBuilder<'_> {
        self.no_wait_request()
    }

    async fn unregistered_request(
        self,
        pdu: impl Into<Pdu>,
        sequence_number: u32,
    ) -> Result<(), Error> {
        let command = Command::builder()
            .status(self.status)
            .sequence_number(sequence_number)
            .pdu(pdu.into());

        let sequence_number = command.sequence_number();
        let status = command.status();
        let id = command.id();

        tracing::trace!(target: TARGET, sequence_number, ?status, ?id, "Sending request");

        let (request, ack) = UnregisteredRequest::new(command);

        self.client
            .inner
            .actions
            .send(Action::unregistered_request(request))
            .map_err(|_| Error::ConnectionClosed)?;

        tracing::trace!(target: TARGET, sequence_number, ?status, ?id, "Waiting for ack");

        // No need to timeout here, since we are not waiting for a response from the server.
        ack.await.map_err(|_| Error::ConnectionClosed)?
    }

    /// Sends a [`DataSmResp`] command to the server.
    pub async fn data_sm_resp(
        self,
        sequence_number: u32,
        data_sm_resp: impl Into<DataSmResp>,
    ) -> Result<(), Error> {
        self.unregistered_request(data_sm_resp.into(), sequence_number)
            .await
    }

    /// Sends a [`DeliverSmResp`] command to the server.
    pub async fn deliver_sm_resp(
        self,
        sequence_number: u32,
        deliver_sm_resp: impl Into<DeliverSmResp>,
    ) -> Result<(), Error> {
        self.unregistered_request(deliver_sm_resp.into(), sequence_number)
            .await
    }

    /// Sends an [`UnbindResp`](Pdu::UnbindResp) command to the server.
    pub async fn unbind_resp(self, sequence_number: u32) -> Result<(), Error> {
        self.unregistered_request(Pdu::UnbindResp, sequence_number)
            .await
    }

    /// Sends an [`EnquireLinkResp`](Pdu::EnquireLinkResp) command to the server.
    pub async fn enquire_link_resp(self, sequence_number: u32) -> Result<(), Error> {
        self.unregistered_request(Pdu::EnquireLinkResp, sequence_number)
            .await
    }

    /// Sends a [`GenericNack`](Pdu::GenericNack) command to the server.
    pub async fn generic_nack(self, sequence_number: u32) -> Result<(), Error> {
        self.unregistered_request(Pdu::GenericNack, sequence_number)
            .await
    }

    /// Sends a [`BindTransmitter`] command to the server and waits for a successful [`BindTransmitterResp`].
    pub async fn bind_transmitter(
        &self,
        bind: impl Into<BindTransmitter>,
    ) -> Result<BindTransmitterResp, Error> {
        self.registered_request().bind_transmitter(bind).await
    }

    /// Sends a [`BindReceiver`] command to the server and waits for a successful [`BindReceiverResp`].
    pub async fn bind_receiver(
        &self,
        bind: impl Into<BindReceiver>,
    ) -> Result<BindReceiverResp, Error> {
        self.registered_request().bind_receiver(bind).await
    }

    /// Sends a [`BindTransceiver`] command to the server and waits for a successful [`BindTransceiverResp`].
    pub async fn bind_transceiver(
        &self,
        bind: impl Into<BindTransceiver>,
    ) -> Result<BindTransceiverResp, Error> {
        self.registered_request().bind_transceiver(bind).await
    }

    /// Sends a [`BroadcastSm`] command to the server and waits for a successful [`BroadcastSmResp`].
    pub async fn broadcast_sm(
        &self,
        broadcast_sm: impl Into<BroadcastSm>,
    ) -> Result<BroadcastSmResp, Error> {
        self.registered_request().broadcast_sm(broadcast_sm).await
    }

    /// Sends a [`CancelBroadcastSm`] command to the server and waits for a successful [`CancelBroadcastSmResp`](Pdu::CancelBroadcastSmResp).
    pub async fn cancel_broadcast_sm(
        &self,
        cancel_broadcast_sm: impl Into<CancelBroadcastSm>,
    ) -> Result<(), Error> {
        self.registered_request()
            .cancel_broadcast_sm(cancel_broadcast_sm)
            .await
    }

    /// Sends a [`CancelSm`] command to the server and waits for a successful [`CancelSmResp`](Pdu::CancelSmResp).
    pub async fn cancel_sm(&self, cancel_sm: impl Into<CancelSm>) -> Result<(), Error> {
        self.registered_request().cancel_sm(cancel_sm).await
    }

    /// Sends a [`DataSm`] command to the server and waits for a successful [`DataSmResp`].
    pub async fn data_sm(&self, data_sm: impl Into<DataSm>) -> Result<DataSmResp, Error> {
        self.registered_request().data_sm(data_sm).await
    }

    /// Sends a [`QueryBroadcastSm`] command to the server and waits for a successful [`QueryBroadcastSmResp`].
    pub async fn query_broadcast_sm(
        &self,
        query_broadcast_sm: impl Into<QueryBroadcastSm>,
    ) -> Result<QueryBroadcastSmResp, Error> {
        self.registered_request()
            .query_broadcast_sm(query_broadcast_sm)
            .await
    }

    /// Sends a [`QuerySm`] command to the server and waits for a successful [`QuerySmResp`].
    pub async fn query_sm(&self, query_sm: impl Into<QuerySm>) -> Result<QuerySmResp, Error> {
        self.registered_request().query_sm(query_sm).await
    }

    /// Sends a [`ReplaceSm`] command to the server and waits for a successful [`ReplaceSmResp`](Pdu::ReplaceSmResp).
    pub async fn replace_sm(&self, replace_sm: impl Into<ReplaceSm>) -> Result<(), Error> {
        self.registered_request().replace_sm(replace_sm).await
    }

    /// Sends a [`SubmitMulti`] command to the server and waits for a successful [`SubmitMultiResp`].
    pub async fn submit_multi(
        &self,
        submit_multi: impl Into<SubmitMulti>,
    ) -> Result<SubmitMultiResp, Error> {
        self.registered_request().submit_multi(submit_multi).await
    }

    /// Sends a [`SubmitSm`] command to the server and waits for a successful [`SubmitSmResp`].
    pub async fn submit_sm(&self, submit_sm: impl Into<SubmitSm>) -> Result<SubmitSmResp, Error> {
        self.registered_request().submit_sm(submit_sm).await
    }

    /// Sends an [`Unbind`](Pdu::Unbind) command to the server and waits for a successful [`UnbindResp`](Pdu::UnbindResp).
    pub async fn unbind(&self) -> Result<(), Error> {
        self.registered_request().unbind().await
    }

    /// Sends an [`EnquireLink`](Pdu::EnquireLink) command to the server and waits for a successful [`EnquireLinkResp`](Pdu::EnquireLinkResp).
    pub async fn enquire_link(&self) -> Result<(), Error> {
        self.registered_request().enquire_link().await
    }
}

#[derive(Debug)]
pub struct RegisteredRequestBuilder<'a> {
    client: &'a Client,
    status: CommandStatus,
    response_timeout: Option<Duration>,
}

/// Extracts a specific [`Pdu`] from a generic [`Pdu`].
macro_rules! extract {
    ($pdu:ident) => {
        |pdu| match pdu {
            Pdu::$pdu(response) => Ok(response),
            _ => Err(pdu),
        }
    };
}

impl<'a> RegisteredRequestBuilder<'a> {
    fn new(client: &'a Client, status: CommandStatus) -> Self {
        Self {
            client,
            status,
            response_timeout: client.inner.response_timeout,
        }
    }

    pub const fn status(mut self, status: CommandStatus) -> Self {
        self.status = status;
        self
    }

    pub fn response_timeout(mut self, timeout: Duration) -> Self {
        self.response_timeout = Some(timeout);
        self
    }

    pub fn no_response_timeout(mut self) -> Self {
        self.response_timeout = None;
        self
    }

    fn check_interface_version(&self, interface_version: InterfaceVersion) -> Result<(), Error> {
        if self.client.inner.check_interface_version
            && !matches!(interface_version, InterfaceVersion::Smpp5_0)
        {
            return Err(Error::unsupported_interface_version(interface_version));
        }

        Ok(())
    }

    fn request(&self, pdu: impl Into<Pdu>) -> impl Future<Output = Result<Command, Error>> {
        let sequence_number = self.client.inner.next_sequence_number();

        let command = Command::builder()
            .status(self.status)
            .sequence_number(sequence_number)
            .pdu(pdu.into());

        let future = self
            .client
            .inner
            .send_registered_and_await_response(command, self.response_timeout);

        RequestFutureGuard::new(&self.client.inner.actions, sequence_number, future)
    }

    async fn request_extract<R>(
        &self,
        pdu: impl Into<Pdu>,
        extract: fn(Pdu) -> Result<R, Pdu>,
    ) -> Result<R, Error> {
        self.request(pdu.into())
            .await?
            .ok()
            .map_err(Error::unexpected_response)
            .map(Command::into_parts)
            .map(CommandParts::raw)
            .map(|(id, status, sequence_number, pdu)| {
                pdu.ok_or(CommandParts::new(id, status, sequence_number, None))
                    .and_then(|pdu| {
                        extract(pdu).map_err(|pdu| {
                            CommandParts::new(id, status, sequence_number, Some(pdu))
                        })
                    })
                    .map_err(Command::from_parts)
            })?
            .map_err(Error::unexpected_response)
    }

    /// Sends a [`Pdu`] to the server and waits for a successful response matching the given [`CommandId`].
    async fn request_ok_and_matches(
        &self,
        pdu: impl Into<Pdu>,
        id: CommandId,
    ) -> Result<(), Error> {
        self.request(pdu.into())
            .await?
            .ok_and_matches(id)
            .map(|_| ())
            .map_err(Error::unexpected_response)
    }

    /// Sends a [`BindTransmitter`] command to the server and waits for a successful [`BindTransmitterResp`].
    pub async fn bind_transmitter(
        &self,
        bind: impl Into<BindTransmitter>,
    ) -> Result<BindTransmitterResp, Error> {
        let bind: BindTransmitter = bind.into();

        self.check_interface_version(bind.interface_version)?;

        self.request_extract(bind, extract!(BindTransmitterResp))
            .await
    }

    /// Sends a [`BindReceiver`] command to the server and waits for a successful [`BindReceiverResp`].
    pub async fn bind_receiver(
        &self,
        bind: impl Into<BindReceiver>,
    ) -> Result<BindReceiverResp, Error> {
        let bind: BindReceiver = bind.into();

        self.check_interface_version(bind.interface_version)?;

        self.request_extract(bind, extract!(BindReceiverResp)).await
    }

    /// Sends a [`BindTransceiver`] command to the server and waits for a successful [`BindTransceiverResp`].
    pub async fn bind_transceiver(
        &self,
        bind: impl Into<BindTransceiver>,
    ) -> Result<BindTransceiverResp, Error> {
        let bind: BindTransceiver = bind.into();

        self.check_interface_version(bind.interface_version)?;

        self.request_extract(bind, extract!(BindTransceiverResp))
            .await
    }

    /// Sends a [`BroadcastSm`] command to the server and waits for a successful [`BroadcastSmResp`].
    pub async fn broadcast_sm(
        &self,
        broadcast_sm: impl Into<BroadcastSm>,
    ) -> Result<BroadcastSmResp, Error> {
        self.request_extract(broadcast_sm.into(), extract!(BroadcastSmResp))
            .await
    }

    /// Sends a [`CancelBroadcastSm`] command to the server and waits for a successful [`CancelBroadcastSmResp`](Pdu::CancelBroadcastSmResp).
    pub async fn cancel_broadcast_sm(
        &self,
        cancel_broadcast_sm: impl Into<CancelBroadcastSm>,
    ) -> Result<(), Error> {
        self.request_ok_and_matches(cancel_broadcast_sm.into(), CommandId::CancelBroadcastSmResp)
            .await
    }

    /// Sends a [`CancelSm`] command to the server and waits for a successful [`CancelSmResp`](Pdu::CancelSmResp).
    pub async fn cancel_sm(&self, cancel_sm: impl Into<CancelSm>) -> Result<(), Error> {
        self.request_ok_and_matches(cancel_sm.into(), CommandId::CancelSmResp)
            .await
    }

    /// Sends a [`DataSm`] command to the server and waits for a successful [`DataSmResp`].
    pub async fn data_sm(&self, data_sm: impl Into<DataSm>) -> Result<DataSmResp, Error> {
        self.request_extract(data_sm.into(), extract!(DataSmResp))
            .await
    }

    /// Sends a [`QueryBroadcastSm`] command to the server and waits for a successful [`QueryBroadcastSmResp`].
    pub async fn query_broadcast_sm(
        &self,
        query_broadcast_sm: impl Into<QueryBroadcastSm>,
    ) -> Result<QueryBroadcastSmResp, Error> {
        self.request_extract(query_broadcast_sm.into(), extract!(QueryBroadcastSmResp))
            .await
    }

    /// Sends a [`QuerySm`] command to the server and waits for a successful [`QuerySmResp`].
    pub async fn query_sm(&self, query_sm: impl Into<QuerySm>) -> Result<QuerySmResp, Error> {
        self.request_extract(query_sm.into(), extract!(QuerySmResp))
            .await
    }

    /// Sends a [`ReplaceSm`] command to the server and waits for a successful [`ReplaceSmResp`](Pdu::ReplaceSmResp).
    pub async fn replace_sm(&self, replace_sm: impl Into<ReplaceSm>) -> Result<(), Error> {
        self.request_ok_and_matches(replace_sm.into(), CommandId::ReplaceSmResp)
            .await
    }

    /// Sends a [`SubmitMulti`] command to the server and waits for a successful [`SubmitMultiResp`].
    pub async fn submit_multi(
        &self,
        submit_multi: impl Into<SubmitMulti>,
    ) -> Result<SubmitMultiResp, Error> {
        self.request_extract(submit_multi.into(), extract!(SubmitMultiResp))
            .await
    }

    /// Sends a [`SubmitSm`] command to the server and waits for a successful [`SubmitSmResp`].
    pub async fn submit_sm(&self, submit_sm: impl Into<SubmitSm>) -> Result<SubmitSmResp, Error> {
        self.request_extract(submit_sm.into(), extract!(SubmitSmResp))
            .await
    }

    /// Sends an [`Unbind`](Pdu::Unbind) command to the server and waits for a successful [`UnbindResp`](Pdu::UnbindResp).
    pub async fn unbind(&self) -> Result<(), Error> {
        self.request_ok_and_matches(Pdu::Unbind, CommandId::UnbindResp)
            .await
    }

    /// Sends an [`EnquireLink`](Pdu::EnquireLink) command to the server and waits for a successful [`EnquireLinkResp`](Pdu::EnquireLinkResp).
    pub async fn enquire_link(&self) -> Result<(), Error> {
        self.request_ok_and_matches(Pdu::EnquireLink, CommandId::EnquireLinkResp)
            .await
    }
}

#[derive(Debug)]
pub struct NoWaitRequestBuilder<'a> {
    client: &'a Client,
    status: CommandStatus,
}

impl<'a> NoWaitRequestBuilder<'a> {
    const fn new(client: &'a Client, status: CommandStatus) -> Self {
        Self { client, status }
    }

    const fn unregistered_request(&'_ self) -> UnregisteredRequestBuilder<'_> {
        UnregisteredRequestBuilder::new(self.client, self.status)
    }

    pub const fn status(mut self, status: CommandStatus) -> Self {
        self.status = status;
        self
    }

    /// Sends a [`Pdu`] to the server without waiting for the response.
    async fn send(&self, pdu: impl Into<Pdu>) -> Result<u32, Error> {
        let sequence_number = self.client.inner.next_sequence_number();

        self.unregistered_request()
            .unregistered_request(pdu.into(), sequence_number)
            .await?;

        Ok(sequence_number)
    }

    /// Sends a [`BroadcastSm`] command to the server without waiting for the response.
    pub async fn broadcast_sm(&self, broadcast_sm: impl Into<BroadcastSm>) -> Result<u32, Error> {
        self.send(broadcast_sm.into()).await
    }

    /// Sends a [`CancelBroadcastSm`] command to the server without waiting for the response.
    pub async fn cancel_broadcast_sm(
        &self,
        cancel_broadcast_sm: impl Into<CancelBroadcastSm>,
    ) -> Result<u32, Error> {
        self.send(cancel_broadcast_sm.into()).await
    }

    /// Sends a [`CancelSm`] command to the server without waiting for the response.
    pub async fn cancel_sm(&self, cancel_sm: impl Into<CancelSm>) -> Result<u32, Error> {
        self.send(cancel_sm.into()).await
    }

    /// Sends a [`DataSm`] command to the server without waiting for the response.
    pub async fn data_sm(&self, data_sm: impl Into<DataSm>) -> Result<u32, Error> {
        self.send(data_sm.into()).await
    }

    /// Sends a [`QueryBroadcastSm`] command to the server without waiting for the response.
    pub async fn query_broadcast_sm(
        &self,
        query_broadcast_sm: impl Into<QueryBroadcastSm>,
    ) -> Result<u32, Error> {
        self.send(query_broadcast_sm.into()).await
    }

    /// Sends a [`QuerySm`] command to the server without waiting for the response.
    pub async fn query_sm(&self, query_sm: impl Into<QuerySm>) -> Result<u32, Error> {
        self.send(query_sm.into()).await
    }

    /// Sends a [`ReplaceSm`] command to the server without waiting for the response.
    pub async fn replace_sm(&self, replace_sm: impl Into<ReplaceSm>) -> Result<u32, Error> {
        self.send(replace_sm.into()).await
    }

    /// Sends a [`SubmitMulti`] command to the server without waiting for the response.
    pub async fn submit_multi(&self, submit_multi: impl Into<SubmitMulti>) -> Result<u32, Error> {
        self.send(submit_multi.into()).await
    }

    /// Sends a [`SubmitSm`] command to the server without waiting for the response.
    pub async fn submit_sm(&self, submit_sm: impl Into<SubmitSm>) -> Result<u32, Error> {
        self.send(submit_sm.into()).await
    }

    /// Sends an [`Unbind`](Pdu::Unbind) command to the server without waiting for the response.
    pub async fn unbind(&self) -> Result<u32, Error> {
        self.send(Pdu::Unbind).await
    }

    /// Sends an [`EnquireLink`](Pdu::EnquireLink) command to the server without waiting for the response.
    pub async fn enquire_link(&self) -> Result<u32, Error> {
        self.send(Pdu::EnquireLink).await
    }
}

#[derive(Debug)]
pub struct RawRegisteredRequestBuilder<'a> {
    client: &'a Client,
    status: CommandStatus,
    response_timeout: Option<Duration>,
}

impl<'a> RawRegisteredRequestBuilder<'a> {
    fn new(client: &'a Client, status: CommandStatus) -> Self {
        Self {
            client,
            status,
            response_timeout: client.inner.response_timeout,
        }
    }

    pub const fn status(mut self, status: CommandStatus) -> Self {
        self.status = status;
        self
    }

    pub fn response_timeout(mut self, timeout: Duration) -> Self {
        self.response_timeout = Some(timeout);
        self
    }

    pub fn no_response_timeout(mut self) -> Self {
        self.response_timeout = None;
        self
    }

    /// Sends a raw [`Pdu`] to the server and returns the sent `sequence number` and a future resolving to a successful response [`Command`].
    ///
    /// # Notes
    ///
    /// - If the sent command is not an operation expecting a response, and the response timeout is unset, the response future will never resolve and should be dropped.
    /// - The response timeout is started when the response future is awaited.
    /// - No interface version check is performed.
    pub fn send(
        self,
        pdu: impl Into<Pdu>,
    ) -> impl Future<Output = Result<(u32, impl Future<Output = Result<Command, Error>>), Error>>
    {
        let sequence_number = self.client.inner.next_sequence_number();

        let command = Command::builder()
            .status(self.status)
            .sequence_number(sequence_number)
            .pdu(pdu.into());

        let id = command.id();

        let future = self
            .client
            .inner
            .send_registered(command)
            .and_then(move |response| futures::future::ok((sequence_number, response)));

        async move {
            let (sequence_number, response) =
                RequestFutureGuard::new(&self.client.inner.actions, sequence_number, future)
                    .await?;

            let future = self
                .client
                .inner
                .await_response(response, sequence_number, self.response_timeout)
                .and_then(move |command| async move {
                    // XXX: it is ok to match against responses only, as this is a registered request
                    // If the request does not have a matching response, the user should not be awaiting it here anyway
                    command
                        .ok_and_matches(id.matching_response())
                        .map_err(Error::unexpected_response)
                });

            Ok((
                sequence_number,
                RequestFutureGuard::new(&self.client.inner.actions, sequence_number, future),
            ))
        }
    }
}
