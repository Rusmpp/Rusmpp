use std::time::Duration;

use futures::Stream;
use rusmpp::tokio_codec::CommandCodec;
use tokio::io::{AsyncRead, AsyncWrite};
use tokio_util::codec::Framed;

use crate::{
    Client, SequenceNumber,
    connection::Connection,
    event_::{DefaultEventChannel, DiscardEventChannel, EventChannel, InsightEventChannel},
    runtime_::{Delay, Timeout, tokio::Tokio, wasm::Wasm},
};

/// Default [`tokio`] connection builder that sends [`Event`](crate::event::Event)s through the event stream.
pub type DefaultTokioConnectionBuilder = ConnectionBuilder<DefaultEventChannel, Tokio>;

/// Default `wasm` connection builder that sends [`Event`](crate::event::Event)s through the event stream.
pub type DefaultWasmConnectionBuilder = ConnectionBuilder<DefaultEventChannel, Wasm>;

/// Builder for creating a new `SMPP` connection.
#[derive(Debug, Clone)]
pub struct ConnectionBuilder<E, R> {
    max_command_length: usize,
    min_sequence_number: u32,
    max_sequence_number: u32,
    enquire_link_interval: Option<Duration>,
    /// Timeout for waiting for a an enquire link response from the server.
    enquire_link_response_timeout: Duration,
    /// Whether to automatically respond to enquire link requests from the server.
    auto_enquire_link_response: bool,
    /// Timeout for waiting for a command to be sent to the server.
    request_timeout: Option<Duration>,
    /// Timeout for waiting for a response from the server.
    response_timeout: Option<Duration>,
    check_interface_version: bool,
    /// Capacity of the channel used to send events to the event stream.
    events_channel_capacity: Option<usize>,
    /// Capacity of the channel used to send actions to the connection task.
    actions_channel_capacity: Option<usize>,
    /// TLS configurations provided by the user. If None, default configurations will be used.
    #[cfg(all(feature = "tokio", feature = "rustls"))]
    rustls_config: Option<rustls::ClientConfig>,
    /// Native TLS connector provided by the user. If None, default connector will be used.
    #[cfg(all(feature = "tokio", feature = "native-tls"))]
    native_tls_connector: Option<native_tls::TlsConnector>,
    _e: std::marker::PhantomData<E>,
    _r: std::marker::PhantomData<R>,
}

impl<E, R> ConnectionBuilder<E, R> {
    /// Creates a new [`ConnectionBuilder`] with default configurations.
    ///
    #[doc = include_str!("../docs/defaults.md")]
    fn with_defaults() -> Self {
        Self {
            max_command_length: 4096,
            min_sequence_number: crate::DEFAULT_MIN_SEQUENCE_NUMBER,
            max_sequence_number: crate::DEFAULT_MAX_SEQUENCE_NUMBER,
            enquire_link_interval: Some(Duration::from_secs(30)),
            enquire_link_response_timeout: Duration::from_secs(5),
            auto_enquire_link_response: true,
            request_timeout: None,
            response_timeout: Some(Duration::from_secs(5)),
            check_interface_version: true,
            events_channel_capacity: Some(1024),
            actions_channel_capacity: Some(1024),
            #[cfg(all(feature = "tokio", feature = "rustls"))]
            rustls_config: None,
            #[cfg(all(feature = "tokio", feature = "native-tls"))]
            native_tls_connector: None,
            _e: std::marker::PhantomData,
            _r: std::marker::PhantomData,
        }
    }

    /// Changes the event channel and/or runtime type, keeping all settings.
    fn cast<E2, R2>(self) -> ConnectionBuilder<E2, R2> {
        ConnectionBuilder {
            max_command_length: self.max_command_length,
            min_sequence_number: self.min_sequence_number,
            max_sequence_number: self.max_sequence_number,
            enquire_link_interval: self.enquire_link_interval,
            enquire_link_response_timeout: self.enquire_link_response_timeout,
            auto_enquire_link_response: self.auto_enquire_link_response,
            request_timeout: self.request_timeout,
            response_timeout: self.response_timeout,
            check_interface_version: self.check_interface_version,
            events_channel_capacity: self.events_channel_capacity,
            actions_channel_capacity: self.actions_channel_capacity,
            #[cfg(all(feature = "tokio", feature = "rustls"))]
            rustls_config: self.rustls_config,
            #[cfg(all(feature = "tokio", feature = "native-tls"))]
            native_tls_connector: self.native_tls_connector,
            _e: std::marker::PhantomData,
            _r: std::marker::PhantomData,
        }
    }
}

impl Default for DefaultTokioConnectionBuilder {
    fn default() -> Self {
        Self::new()
    }
}

impl DefaultTokioConnectionBuilder {
    /// Creates a new [`DefaultTokioConnectionBuilder`] with default configurations.
    ///
    #[doc = include_str!("../docs/defaults.md")]
    /// - `rustls_config`: default configuration will be used if TLS is enabled. See [`rustls_config`](Self::rustls_config) for more details.
    /// - `native_tls_connector`: default connector will be used if TLS is enabled. See [`native_tls_connector`](Self::native_tls_connector) for more details.
    pub fn new() -> Self {
        Self::with_defaults()
    }

    /// Creates a new [`DefaultTokioConnectionBuilder`] with default configurations.
    ///
    #[doc = include_str!("../docs/defaults.md")]
    pub fn new_tokio() -> Self {
        Self::new()
    }
}

impl Default for DefaultWasmConnectionBuilder {
    fn default() -> Self {
        Self::new_wasm()
    }
}

impl DefaultWasmConnectionBuilder {
    /// Creates a new [`DefaultWasmConnectionBuilder`] with default configurations.
    ///
    #[doc = include_str!("../docs/defaults.md")]
    pub fn new_wasm() -> Self {
        Self::with_defaults()
    }
}

#[cfg(feature = "tokio")]
impl<E: EventChannel> ConnectionBuilder<E, Tokio> {
    /// Connects to the `SMPP` server.
    ///
    /// Opens and manages a connection in the background and returns a client and an event stream.
    ///
    /// - The client is used as a handle to communicate with the server through the managed connection.
    /// - The event stream is used to receive events from the server, such as incoming messages or errors.
    ///
    /// # Example
    ///
    /// Connect to an `SMPP` server running on localhost at port 2775.
    ///
    /// ```
    /// # use rusmppc::ConnectionBuilder;
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let (client, events) = ConnectionBuilder::new()
    ///     .connect("smpp://localhost:2775")
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// Connect to an `SMPP` server running on localhost at port 2775 using TLS.
    ///
    /// ```
    /// # use rusmppc::ConnectionBuilder;
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let (client, events) = ConnectionBuilder::new()
    ///     .connect("smpps://localhost:2775")
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # Supported URL schemes
    ///
    /// - `smpp`: Connect using plain TCP.
    /// - `ssmpp` or `smpps`: Connect using TLS. Requires the `rustls` or `native-tls` features to be enabled.
    ///
    /// # Notes
    ///
    /// - If no port is specified in the URL, the default port `2775` will be used.
    /// - Path and query parameters in the URL are ignored silently.
    ///
    /// # Errors
    ///
    /// This function will return an error in the following cases:
    ///
    /// - If the URL is invalid. See [`url::Url::parse`] for more details.
    /// - If the URL scheme is not supported. Supported schemes are `smpp`, `ssmpp`, and `smpps`.
    /// - If the URL does not have a host.
    /// - If DNS resolution fails.
    /// - If the connection to the server fails.
    /// - If TLS is enabled (when using `ssmpp` or `smpps` schemes) but the `rustls` or `native-tls` features are not enabled.
    /// - If TLS handshake fails.
    ///
    #[doc = include_str!("../docs/events.md")]
    pub async fn connect(
        self,
        url: impl AsRef<str>,
    ) -> Result<
        (
            Client<Tokio>,
            impl Stream<Item = E::Event> + Unpin + 'static,
        ),
        crate::error::Error,
    > {
        let (client, events, connection) = self.no_spawn().connect(url).await?;

        Tokio::spawn(connection);

        Ok((client, events))
    }

    /// Creates a client from an existing connection.
    ///
    /// Manages a connection in the background and returns a client and an event stream.
    ///
    /// - The client is used as a handle to communicate with the server through the managed connection.
    /// - The event stream is used to receive events from the server, such as incoming messages or errors.
    ///
    #[doc = include_str!("../docs/events.md")]
    pub fn connected<S>(
        self,
        stream: S,
    ) -> (
        Client<Tokio>,
        impl Stream<Item = E::Event> + Unpin + 'static,
    )
    where
        S: AsyncRead + AsyncWrite + Send + 'static,
    {
        let (client, events, connection) = self.no_spawn().connected(stream);

        Tokio::spawn(connection);

        (client, events)
    }
}

#[cfg(feature = "wasm")]
impl<E: EventChannel> ConnectionBuilder<E, Wasm> {
    /// Creates a client from an existing connection.
    ///
    /// Manages a connection in the background and returns a client and an event stream.
    ///
    /// - The client is used as a handle to communicate with the server through the managed connection.
    /// - The event stream is used to receive events from the server, such as incoming messages or errors.
    ///
    #[doc = include_str!("../docs/events.md")]
    pub fn connected<S>(
        self,
        stream: S,
    ) -> (Client<Wasm>, impl Stream<Item = E::Event> + Unpin + 'static)
    where
        S: AsyncRead + AsyncWrite + 'static,
    {
        let (client, events, connection) = self.no_spawn().connected(stream);

        Wasm::spawn(connection);

        (client, events)
    }
}

#[cfg(feature = "tokio")]
impl<E: EventChannel> ConnectionBuilder<E, Tokio> {
    /// Creates a managed connection builder that handles reconnection automatically.
    pub fn managed(self) -> crate::managed_::UnboundManagedConnectionBuilder<E>
    where
        E: EventChannel + Clone + Send + Sync + 'static,
        E::Event: Send + Sync + 'static,
    {
        crate::managed_::UnboundManagedConnectionBuilder::new(self)
    }
}

impl<E, R> ConnectionBuilder<E, R> {
    /// Sets the maximum command length for incoming commands.
    pub const fn max_command_length(mut self, max_command_length: usize) -> Self {
        self.max_command_length = max_command_length;
        self
    }

    /// Sets the minimum sequence number for outgoing commands.
    ///
    /// # Panics
    ///
    /// Panics if `min_sequence_number` is greater than `max_sequence_number`.
    pub fn min_sequence_number(mut self, min_sequence_number: u32) -> Self {
        crate::assertion::assert_min_less_or_equal_to!(
            min_sequence_number,
            self.max_sequence_number
        );

        self.min_sequence_number = min_sequence_number;
        self
    }

    /// Sets the maximum sequence number for outgoing commands.
    ///
    /// # Panics
    ///
    /// Panics if `max_sequence_number` is less than `min_sequence_number`.
    pub fn max_sequence_number(mut self, max_sequence_number: u32) -> Self {
        crate::assertion::assert_min_less_or_equal_to!(
            self.min_sequence_number,
            max_sequence_number
        );

        self.max_sequence_number = max_sequence_number;
        self
    }

    /// Sets the enquire link interval.
    ///
    /// Used to determine how often an enquire link command should be sent to the server.
    ///
    /// The timer is reset each time a command is read from the server. If the timer expires, an enquire link command is sent to the server.
    pub fn enquire_link_interval(mut self, enquire_link_interval: Duration) -> Self {
        self.enquire_link_interval = Some(enquire_link_interval);
        self
    }

    /// Disables the enquire link interval.
    ///
    /// When disabled, no enquire link commands will be sent to the server.
    pub fn no_enquire_link_interval(mut self) -> Self {
        self.enquire_link_interval = None;
        self
    }

    /// Sets the enquire link interval.
    ///
    /// If set to `None`, no enquire link commands will be sent to the server.
    pub fn with_enquire_link_interval(mut self, enquire_link_interval: Option<Duration>) -> Self {
        self.enquire_link_interval = enquire_link_interval;
        self
    }

    /// Sets the enquire link response timeout.
    ///
    /// This timeout is used to determine how long the connection should wait for an enquire link response from the server.
    pub fn enquire_link_response_timeout(
        mut self,
        enquire_link_response_timeout: Duration,
    ) -> Self {
        self.enquire_link_response_timeout = enquire_link_response_timeout;
        self
    }

    /// Enables automatic responses to enquire link requests from the server.
    ///
    /// See [`with_auto_enquire_link_response`](Self::with_auto_enquire_link_response) for more details.
    pub fn enable_auto_enquire_link_response(mut self) -> Self {
        self.auto_enquire_link_response = true;
        self
    }

    /// Disables automatic responses to enquire link requests from the server.
    ///
    /// See [`with_auto_enquire_link_response`](Self::with_auto_enquire_link_response) for more details.
    pub fn disable_auto_enquire_link_response(mut self) -> Self {
        self.auto_enquire_link_response = false;
        self
    }

    /// Sets whether to automatically respond to enquire link requests from the server.
    ///
    /// By default, this is set to `true`.
    ///
    /// When enabled, the connection will automatically respond to any enquire link requests received from the server.
    ///
    /// When disabled, the client will need to handle enquire link requests manually. The [`EnquireLink`](rusmpp::Pdu::EnquireLink) command will be received as an event in the event stream.
    pub fn with_auto_enquire_link_response(mut self, auto: bool) -> Self {
        self.auto_enquire_link_response = auto;
        self
    }

    /// Sets the request timeout.
    ///
    /// This timeout is used to determine how long the client should wait for a request to be sent to the server.
    ///
    /// If the timeout fired with [`Error::RequestTimeout`](crate::error::Error::RequestTimeout), the request will be dropped.
    pub fn request_timeout(mut self, request_timeout: Duration) -> Self {
        self.request_timeout = Some(request_timeout);
        self
    }

    /// Disables the request timeout.
    ///
    /// When disabled, the client will wait indefinitely for the request to be sent to the server.
    pub fn no_request_timeout(mut self) -> Self {
        self.request_timeout = None;
        self
    }

    /// Sets the request timeout.
    ///
    /// If set to `None`, no timeout will be used and the client will wait indefinitely for the request to be sent to the server.
    pub fn with_request_timeout(mut self, request_timeout: Option<Duration>) -> Self {
        self.request_timeout = request_timeout;
        self
    }

    /// Sets the response timeout.
    ///
    /// This timeout is used to determine how long the client should wait for a response from the server.
    ///
    /// The timer is started after the command has been sent to the server.
    pub fn response_timeout(mut self, response_timeout: Duration) -> Self {
        self.response_timeout = Some(response_timeout);
        self
    }

    /// Disables the response timeout.
    ///
    /// When disabled, the client will wait indefinitely for a response from the server.
    pub fn no_response_timeout(mut self) -> Self {
        self.response_timeout = None;
        self
    }

    /// Sets the response timeout.
    ///
    /// If set to `None`, no timeout will be used and the client will wait indefinitely for a response from the server.
    pub fn with_response_timeout(mut self, response_timeout: Option<Duration>) -> Self {
        self.response_timeout = response_timeout;
        self
    }

    /// Enables the interface version check.
    ///
    /// See [`with_interface_version_check`](Self::with_interface_version_check) for more details.
    pub fn enable_interface_version_check(mut self) -> Self {
        self.check_interface_version = true;
        self
    }

    /// Disables the interface version check.
    ///
    /// See [`with_interface_version_check`](Self::with_interface_version_check) for more details.
    pub fn disable_interface_version_check(mut self) -> Self {
        self.check_interface_version = false;
        self
    }

    /// Enables or disables the interface version check.
    ///
    /// By default, the interface version check is enabled.
    ///
    /// This library uses `SMPP v5` implementation to encode and decode commands.
    ///
    /// Binding to a server with another `SMPP` version may cause issues encoding and decoding commands.
    /// Disable interface version check to allow binding to servers with any `SMPP` version.
    pub fn with_interface_version_check(mut self, check: bool) -> Self {
        self.check_interface_version = check;
        self
    }

    /// Sets the capacity of the channel used to send actions to the connection task.
    ///
    /// # Panics
    ///
    /// Panics if `capacity` is 0.
    pub fn channel_capacity(mut self, capacity: usize) -> Self {
        assert!(capacity > 0, "Channel capacity must be greater than 0");

        self.actions_channel_capacity = Some(capacity);
        self
    }

    /// Removes the capacity of the channel used to send actions to the connection task.
    ///
    /// The channel will be unbounded.
    pub fn no_channel_capacity(mut self) -> Self {
        self.actions_channel_capacity = None;
        self
    }

    /// Sets the capacity of the channel used to send actions to the connection task.
    ///
    /// If set to `None`, the channel will be unbounded.
    ///
    /// # Panics
    ///
    /// Panics if `capacity` is `Some(0)`.
    pub fn with_channel_capacity(mut self, capacity: Option<usize>) -> Self {
        if let Some(capacity) = capacity {
            assert!(capacity > 0, "Channel capacity must be greater than 0");
        }

        self.actions_channel_capacity = capacity;
        self
    }

    /// Sets a custom `rustls` client configuration.
    ///
    /// If not set, a default configuration will be used.
    ///  - If the `rustls-tls-native-roots` feature is enabled, native root certificates are used.
    ///  - If the `rustls-tls-webpki-roots` feature is enabled, webpki root certificates are used.
    #[cfg(all(feature = "tokio", feature = "rustls"))]
    pub fn rustls_config(mut self, config: rustls::ClientConfig) -> Self {
        self.rustls_config = Some(config);
        self
    }

    /// Sets a custom `native-tls` connector.
    ///
    /// If not set, a default connector will be used.
    #[cfg(all(feature = "tokio", feature = "native-tls"))]
    pub fn native_tls_connector(mut self, connector: native_tls::TlsConnector) -> Self {
        self.native_tls_connector = Some(connector);
        self
    }

    /// Does not spawn the connection in the background.
    ///
    /// It is your responsibility to run the connection to completion.
    pub fn no_spawn(self) -> NoSpawnConnectionBuilder<E, R> {
        NoSpawnConnectionBuilder { builder: self }
    }

    /// Configures the connection event stream.
    pub fn events(self) -> EventsConnectionBuilder<E, R> {
        EventsConnectionBuilder { builder: self }
    }

    /// Configures the connection to use the [`tokio`] runtime.
    pub fn tokio(self) -> ConnectionBuilder<E, Tokio> {
        self.cast()
    }

    /// Configures the connection to use the `wasm` runtime.
    pub fn wasm(self) -> ConnectionBuilder<E, Wasm> {
        self.cast()
    }
}

/// Builder for creating a new `SMPP` connection without spawning it in the background.
#[derive(Debug)]
pub struct NoSpawnConnectionBuilder<E, R> {
    pub(crate) builder: ConnectionBuilder<E, R>,
}

#[cfg(feature = "tokio")]
impl<E: EventChannel> NoSpawnConnectionBuilder<E, Tokio> {
    /// Connects to the `SMPP` server without spawning the connection in the background.
    ///
    /// # Errors
    ///
    /// This function will return an error in the following cases:
    ///
    /// - If the URL is invalid. See [`url::Url::parse`] for more details.
    /// - If the URL scheme is not supported. Supported schemes are `smpp`, `ssmpp`, and `smpps`.
    /// - If the URL does not have a host.
    /// - If DNS resolution fails.
    /// - If the connection to the server fails.
    /// - If TLS is enabled (when using `ssmpp` or `smpps` schemes) but the `rustls` or `native-tls` features are not enabled.
    /// - If TLS handshake fails.
    #[allow(unused_mut)]
    pub async fn connect(
        mut self,
        url: impl AsRef<str>,
    ) -> Result<
        (
            Client<Tokio>,
            impl Stream<Item = E::Event> + Unpin + 'static,
            impl Future<Output = ()> + 'static,
        ),
        crate::error::Error,
    > {
        use crate::{MaybeTlsStream, error::Error};

        enum Scheme {
            Smpp,
            Ssmpp,
        }

        let url = url::Url::parse(url.as_ref()).map_err(|err| {
            Error::Connect(std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                format!("Invalid URL: {err}"),
            ))
        })?;

        let scheme = match url.scheme() {
            "smpp" => Scheme::Smpp,
            "ssmpp" | "smpps" => Scheme::Ssmpp,
            scheme => {
                return Err(Error::Connect(std::io::Error::new(
                    std::io::ErrorKind::InvalidInput,
                    format!(
                        "Unsupported URL scheme: {scheme}, supported schemes are smpp and ssmpp/smpps"
                    ),
                )));
            }
        };

        let domain = url.host_str().ok_or_else(|| {
            Error::Connect(std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                "URL must have a host",
            ))
        })?;

        let port = url.port().unwrap_or(2775);

        tracing::debug!(target: "rusmppc::connection::dns", domain, "Resolving domain");

        let resolver = hickory_resolver::TokioResolver::builder_tokio()
            .and_then(|builder| builder.build())
            .map_err(|err| {
                Error::Connect(std::io::Error::other(format!(
                    "Failed to create DNS resolver: {err}"
                )))
            })?;

        let ip_addr = resolver
            .lookup_ip(domain)
            .await
            .map_err(|err| {
                Error::Connect(std::io::Error::other(format!("Failed to lookup IP: {err}")))
            })?
            .iter()
            .next()
            .ok_or_else(|| {
                Error::Connect(std::io::Error::new(
                    std::io::ErrorKind::NotFound,
                    "No addresses found for the given host",
                ))
            })?;

        let socket_addr = std::net::SocketAddr::new(ip_addr, port);

        tracing::debug!(target: "rusmppc::connection::tcp", %socket_addr, "Connecting");

        let stream = tokio::net::TcpStream::connect(socket_addr)
            .await
            .map_err(Error::Connect)?;

        tracing::debug!(target: "rusmppc::connection::tcp", %socket_addr, "Connected");

        let stream = match scheme {
            Scheme::Smpp => MaybeTlsStream::plain(stream),
            Scheme::Ssmpp => {
                #[cfg(all(feature = "rustls", not(feature = "native-tls")))]
                {
                    MaybeTlsStream::rustls(stream, domain, self.builder.rustls_config.take())
                        .await?
                }
                // If both features are enabled, prefer rustls.
                #[cfg(all(feature = "rustls", feature = "native-tls"))]
                {
                    tracing::warn!(target: "rusmppc::connection::tls", "Both `rustls` and `native-tls` features are enabled, preferring `rustls` for TLS connections");

                    MaybeTlsStream::rustls(stream, domain, self.builder.rustls_config.take())
                        .await?
                }
                #[cfg(all(not(feature = "rustls"), feature = "native-tls"))]
                {
                    MaybeTlsStream::native_tls(
                        stream,
                        domain,
                        self.builder.native_tls_connector.take(),
                    )
                    .await?
                }
                #[cfg(not(any(feature = "rustls", feature = "native-tls")))]
                {
                    return Err(Error::Connect(std::io::Error::new(
                        std::io::ErrorKind::InvalidInput,
                        "TLS support is not enabled, enable the `rustls` or `native-tls` feature to use ssmpp/smpps",
                    )));
                }
            }
        };

        Ok(self.connected(stream))
    }
}

impl<E: EventChannel, R: Delay + Timeout> NoSpawnConnectionBuilder<E, R> {
    /// Creates a client from an existing connection without spawning the connection in the background.
    pub fn connected<S>(
        self,
        stream: S,
    ) -> (
        Client<R>,
        impl Stream<Item = E::Event> + Unpin + 'static,
        impl Future<Output = ()>,
    )
    where
        S: AsyncRead + AsyncWrite + 'static,
    {
        let framed = Framed::new(
            stream,
            CommandCodec::new().with_max_length(self.builder.max_command_length),
        );

        let (client, events, connection) = self.connection(framed);

        (client, events, connection.run())
    }

    /// Consumes `self` and creates a new [`Client`] along with the connection and event stream.
    pub(crate) fn connection<F>(
        self,
        framed: F,
    ) -> (
        Client<R>,
        impl Stream<Item = E::Event> + Unpin + 'static,
        Connection<F, E, R>,
    ) {
        let sequence_number = SequenceNumber::new(
            self.builder.min_sequence_number,
            self.builder.max_sequence_number,
        );

        let (connection, watch, urgent_actions, actions, events) = Connection::<F, E, R>::new(
            sequence_number.clone(),
            framed,
            self.builder.enquire_link_interval,
            self.builder.enquire_link_response_timeout,
            self.builder.auto_enquire_link_response,
            self.builder.events_channel_capacity,
            self.builder.actions_channel_capacity,
        );

        let client = Client::new(
            sequence_number,
            urgent_actions,
            actions,
            self.builder.request_timeout,
            self.builder.response_timeout,
            self.builder.check_interface_version,
            watch,
        );

        (client, events, connection)
    }
}

/// Builder for configuring the event stream.
#[derive(Debug)]
pub struct EventsConnectionBuilder<E, R> {
    builder: ConnectionBuilder<E, R>,
}

impl<E, R> EventsConnectionBuilder<E, R> {
    /// Sets the capacity of the channel used to send events to the event stream.
    ///
    /// # Panics
    ///
    /// Panics if `capacity` is 0.
    pub fn channel_capacity(mut self, capacity: usize) -> Self {
        assert!(capacity > 0, "Channel capacity must be greater than 0");

        self.builder.events_channel_capacity = Some(capacity);
        self
    }

    /// Removes the capacity of the channel used to send events to the event stream.
    ///
    /// The channel will be unbounded.
    pub fn no_channel_capacity(mut self) -> Self {
        self.builder.events_channel_capacity = None;
        self
    }

    /// Sets the capacity of the channel used to send events to the event stream.
    ///
    /// If set to `None`, the channel will be unbounded.
    ///
    /// # Panics
    ///
    /// Panics if `capacity` is `Some(0)`.
    pub fn with_channel_capacity(mut self, capacity: Option<usize>) -> Self {
        if let Some(capacity) = capacity {
            assert!(capacity > 0, "Channel capacity must be greater than 0");
        }

        self.builder.events_channel_capacity = capacity;
        self
    }

    /// Uses the default event channel that sends events through the event stream.
    pub fn default(self) -> ConnectionBuilder<DefaultEventChannel, R> {
        self.builder.cast()
    }

    /// Discards all events from the background connection.
    pub fn discard(self) -> ConnectionBuilder<DiscardEventChannel, R> {
        self.builder.cast()
    }

    /// Enables insight events from the background connection.
    pub fn insights(self) -> ConnectionBuilder<InsightEventChannel, R> {
        self.builder.cast()
    }
}

#[cfg(test)]
impl<E, R> ConnectionBuilder<E, R> {
    pub(crate) fn mock_delay(
        self,
    ) -> ConnectionBuilder<E, crate::mock::runtime::MockRuntime<crate::mock::delay::MockDelay, Tokio>>
    {
        self.cast()
    }
}
