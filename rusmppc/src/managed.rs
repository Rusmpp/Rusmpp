use std::fmt::Debug;

use bb8::{ManageConnection, Pool};
use futures::{Stream, StreamExt};
use rusmpp::pdus::BindTransceiver;
use tokio::sync::mpsc::UnboundedSender;
use tokio_stream::wrappers::UnboundedReceiverStream;

use crate::{Client, ConnectionBuilder, event::EventChannel};

#[derive(Debug, Clone)]
pub struct ManagedClient<E: EventChannel + Clone + Send + Sync + 'static>
where
    E::Event: Send + Sync + 'static,
{
    pool: Pool<ClientConnectionManger<E>>,
}

impl<E: EventChannel + Clone + Send + Sync + 'static> ManagedClient<E>
where
    E::Event: Send + Sync + 'static,
{
    fn new(pool: Pool<ClientConnectionManger<E>>) -> Self {
        Self { pool }
    }

    pub async fn get(&self) -> Option<Client> {
        self.pool.get_owned().await.ok().map(|conn| conn.clone())
    }
}

#[derive(Debug)]
pub struct ManagedConnectionBuilder<E: EventChannel + Clone + Send + Sync + 'static> {
    builder: ConnectionBuilder<E>,
}

impl<E: EventChannel + Clone + Send + Sync + 'static> ManagedConnectionBuilder<E> {
    pub(crate) fn new(builder: ConnectionBuilder<E>) -> Self {
        Self { builder }
    }

    // TODO: this one takes an async function that returns a Result<AsyncRead + AsyncWrite, std::io::Error>
    pub async fn connected(self) {}

    pub async fn connect(
        self,
        url: impl Into<String>,
    ) -> Result<
        (
            ManagedClient<E>,
            impl Stream<Item = E::Event> + Unpin + 'static,
        ),
        crate::error::Error,
    >
    where
        E::Event: Send + Sync + 'static,
    {
        let (tx, rx) = tokio::sync::mpsc::unbounded_channel();
        let rx = UnboundedReceiverStream::new(rx);

        let manager = ClientConnectionManger::new(self.builder, url.into(), tx);

        let pool = bb8::Pool::builder()
            .max_lifetime(None)
            .max_size(1)
            .build(manager)
            .await
            .unwrap();

        Ok((ManagedClient::new(pool), rx))
    }
}

#[derive(Debug)]
pub struct ClientConnectionManger<E: EventChannel + Clone + Send + Sync + 'static> {
    builder: ConnectionBuilder<E>,
    url: String,
    tx: UnboundedSender<E::Event>,
}

impl<E: EventChannel + Clone + Send + Sync + 'static> ClientConnectionManger<E> {
    fn new(builder: ConnectionBuilder<E>, url: String, tx: UnboundedSender<E::Event>) -> Self {
        Self { builder, url, tx }
    }
}

impl<E: EventChannel + Clone + Send + Sync + 'static> ManageConnection for ClientConnectionManger<E>
where
    E::Event: Send + Sync + 'static,
{
    type Connection = Client;

    type Error = crate::error::Error;

    async fn connect(&self) -> Result<Self::Connection, Self::Error> {
        let (client, mut events) = self.builder.clone().connect(&self.url).await?;

        // TODO: we should move this out and let the user define what should happen.
        // and use it in the customizer maybe
        client
            .bind_transceiver(BindTransceiver::builder().build())
            .await?;

        let tx = self.tx.clone();

        tokio::spawn(async move {
            while let Some(event) = events.next().await {
                let _ = tx.send(event);
            }
        });

        Ok(client)
    }

    async fn is_valid(&self, conn: &mut Self::Connection) -> Result<(), Self::Error> {
        conn.is_active()
            .then_some(())
            .ok_or(crate::error::Error::ConnectionClosed)
    }

    fn has_broken(&self, conn: &mut Self::Connection) -> bool {
        conn.is_closed()
    }
}
