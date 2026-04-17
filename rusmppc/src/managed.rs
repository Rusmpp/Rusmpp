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
pub struct ClientConnectionManger<E: EventChannel + Clone + Send + Sync + 'static> {
    builder: ConnectionBuilder<E>,
    url: String,
    tx: Option<UnboundedSender<E::Event>>,
}

impl<E: EventChannel + Clone + Send + Sync + 'static> ClientConnectionManger<E> {
    pub(crate) fn new(builder: ConnectionBuilder<E>, url: String) -> Self {
        Self {
            builder,
            url,
            tx: None,
        }
    }

    fn with_tx(mut self, tx: UnboundedSender<E::Event>) -> Self {
        self.tx = Some(tx);
        self
    }

    pub async fn connect(
        self,
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

        let manager = self.with_tx(tx);

        let pool = bb8::Pool::builder()
            .max_lifetime(None)
            .max_size(1)
            .build(manager)
            .await
            .unwrap();

        Ok((ManagedClient::new(pool), rx))
    }
}

impl<E: EventChannel + Clone + Send + Sync + 'static> ManageConnection for ClientConnectionManger<E>
where
    E::Event: Send + Sync + 'static,
{
    type Connection = Client;

    type Error = crate::error::Error;

    async fn connect(&self) -> Result<Self::Connection, Self::Error> {
        let builder = self.builder.clone();
        let url = self.url.clone();

        let (client, mut events) = builder.connect(url).await?;

        // TODO: we should move this out and let the user define what should happen.
        // and use it in the customizer maybe
        client
            .bind_transceiver(BindTransceiver::builder().build())
            .await?;

        let tx = self.tx.clone();

        tokio::spawn(async move {
            while let Some(event) = events.next().await {
                if let Some(tx) = &tx {
                    let _ = tx.send(event);
                }
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
