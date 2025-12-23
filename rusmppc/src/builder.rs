use std::{net::SocketAddr, time::Duration};

use futures::Stream;
use rusmpp::tokio_codec::CommandCodec;
use tokio::{
    io::{AsyncRead, AsyncWrite},
    net::TcpStream,
};
use tokio_util::codec::Framed;

use crate::{
    Client, MaybeTlsStream,
    delay::TokioDelay,
    error::Error,
    event::{DefaultEventChannel, DiscardEventChannel, EventChannel, InsightEventChannel},
};

/// Builder for creating a new `SMPP` connection.
#[derive(Debug)]
pub struct ConnectionBuilder<E = DefaultEventChannel> {
    pub(crate) max_command_length: usize,
    pub(crate) enquire_link_interval: Option<Duration>,
    /// Timeout for waiting for a an enquire link response from the server.
    pub(crate) enquire_link_response_timeout: Duration,
    /// Whether to automatically respond to enquire link requests from the server.
    pub(crate) auto_enquire_link_response: bool,
    /// Timeout for waiting for a response from the server.
    pub(crate) response_timeout: Option<Duration>,
    pub(crate) check_interface_version: bool,
    /// TLS configurations provided by the user. If None, default configurations will be used.
    #[cfg(feature = "rustls")]
    rustls_config: Option<rustls::ClientConfig>,
    /// Native TLS connector provided by the user. If None, default connector will be used.
    #[cfg(feature = "native-tls")]
    native_tls_connector: Option<native_tls::TlsConnector>,
    _phantom: std::marker::PhantomData<E>,
}

impl Default for ConnectionBuilder<DefaultEventChannel> {
    fn default() -> Self {
        Self::new()
    }
}

impl ConnectionBuilder<DefaultEventChannel> {
    /// Creates a new [`ConnectionBuilder`] with default configurations.
    ///
    /// # Defaults
    /// - `max_command_length`: 4096 bytes
    /// - `enquire_link_interval`: 30 seconds
    /// - `enquire_link_response_timeout`: 5 seconds
    /// - `auto_enquire_link_response`: true
    /// - `response_timeout`: 5 seconds
    /// - `check_interface_version`: true
    /// - `rustls_config`: default configuration will be used if TLS is enabled. See [`rustls_config`](Self::rustls_config) for more details.
    /// - `native_tls_connector`: default connector will be used if TLS is enabled. See [`native_tls_connector`](Self::native_tls_connector) for more details.
    pub fn new() -> Self {
        Self {
            max_command_length: 4096,
            enquire_link_interval: Some(Duration::from_secs(30)),
            enquire_link_response_timeout: Duration::from_secs(5),
            auto_enquire_link_response: true,
            response_timeout: Some(Duration::from_secs(5)),
            check_interface_version: true,
            #[cfg(feature = "rustls")]
            rustls_config: None,
            #[cfg(feature = "native-tls")]
            native_tls_connector: None,
            _phantom: std::marker::PhantomData,
        }
    }
}

impl<E> ConnectionBuilder<E> {
    /// Does not spawn the connection in the background.
    ///
    /// It is your responsibility to run the connection to completion.
    pub fn no_spawn(self) -> NoSpawnConnectionBuilder<E> {
        NoSpawnConnectionBuilder { builder: self }
    }

    /// Configures the connection event stream.
    pub fn events(self) -> EventsConnectionBuilder<E> {
        EventsConnectionBuilder { builder: self }
    }
}

impl<E: EventChannel> ConnectionBuilder<E> {
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
    /// - `smpp`: Connect using plain TCP.
    /// - `ssmpp` or `smpps`: Connect using TLS. Requires the `rustls` or `native-tls` features to be enabled.
    ///
    /// # Notes
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
    pub async fn connect(
        self,
        url: impl AsRef<str>,
    ) -> Result<(Client, impl Stream<Item = E::Event> + Unpin + 'static), Error> {
        let (client, events, connection) = self.no_spawn().connect(url).await?;

        tokio::spawn(connection);

        Ok((client, events))
    }

    /// Creates a client from an existing connection.
    ///
    /// Manages a connection in the background and returns a client and an event stream.
    ///
    /// - The client is used as a handle to communicate with the server through the managed connection.
    /// - The event stream is used to receive events from the server, such as incoming messages or errors.
    pub fn connected<S>(self, stream: S) -> (Client, impl Stream<Item = E::Event> + Unpin + 'static)
    where
        S: AsyncRead + AsyncWrite + Send + 'static,
    {
        let (client, events, connection) = self.no_spawn().connected(stream);

        tokio::spawn(connection);

        (client, events)
    }
}

impl ConnectionBuilder {
    /// Sets the maximum command length for incoming commands.
    pub const fn max_command_length(mut self, max_command_length: usize) -> Self {
        self.max_command_length = max_command_length;
        self
    }

    /// Sets the enquire link interval.
    ///
    /// Used to determine how often an enquire link command should be sent to the server.
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

    /// Sets a custom `rustls` client configuration.
    ///
    /// If not set, a default configuration will be used.
    ///  - If the `rustls-tls-native-roots` feature is enabled, native root certificates are used.
    ///  - If the `rustls-tls-webpki-roots` feature is enabled, webpki root certificates are used.
    #[cfg(feature = "rustls")]
    pub fn rustls_config(mut self, config: rustls::ClientConfig) -> Self {
        self.rustls_config = Some(config);
        self
    }

    /// Sets a custom `native-tls` connector.
    ///
    /// If not set, a default connector will be used.
    #[cfg(feature = "native-tls")]
    pub fn native_tls_connector(mut self, connector: native_tls::TlsConnector) -> Self {
        self.native_tls_connector = Some(connector);
        self
    }
}

/// Builder for creating a new `SMPP` connection without spawning it in the background.
#[derive(Debug)]
pub struct NoSpawnConnectionBuilder<E = DefaultEventChannel> {
    pub(crate) builder: ConnectionBuilder<E>,
}

impl<E: EventChannel> NoSpawnConnectionBuilder<E> {
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
            Client,
            impl Stream<Item = E::Event> + Unpin + 'static,
            impl Future<Output = ()> + 'static,
        ),
        Error,
    > {
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
            .map_err(|err| {
                Error::Connect(std::io::Error::other(format!(
                    "Failed to create DNS resolver: {err}"
                )))
            })?
            .build();

        let ip_addr = resolver
            .lookup_ip(domain)
            .await
            .map_err(|err| {
                Error::Connect(std::io::Error::other(format!("Failed to lookup IP: {err}")))
            })?
            .into_iter()
            .next()
            .ok_or_else(|| {
                Error::Connect(std::io::Error::new(
                    std::io::ErrorKind::NotFound,
                    "No addresses found for the given host",
                ))
            })?;

        let socket_addr = SocketAddr::new(ip_addr, port);

        tracing::debug!(target: "rusmppc::connection::tcp", %socket_addr, "Connecting");

        let stream = TcpStream::connect(socket_addr)
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

    /// Creates a client from an existing connection without spawning the connection in the background.
    pub fn connected<S>(
        self,
        stream: S,
    ) -> (
        Client,
        impl Stream<Item = E::Event> + Unpin + 'static,
        impl Future<Output = ()>,
    )
    where
        S: AsyncRead + AsyncWrite + Send + 'static,
    {
        let framed = Framed::new(
            stream,
            CommandCodec::new().with_max_length(self.builder.max_command_length),
        );

        self.raw(framed, TokioDelay::new(), TokioDelay::new())
    }
}

/// Builder for configuring the event stream
#[derive(Debug)]
pub struct EventsConnectionBuilder<E = DefaultEventChannel> {
    builder: ConnectionBuilder<E>,
}

impl<E> EventsConnectionBuilder<E> {
    /// Discards all events from the background connection.
    pub fn discard(self) -> ConnectionBuilder<DiscardEventChannel> {
        ConnectionBuilder {
            max_command_length: self.builder.max_command_length,
            enquire_link_interval: self.builder.enquire_link_interval,
            enquire_link_response_timeout: self.builder.enquire_link_response_timeout,
            auto_enquire_link_response: self.builder.auto_enquire_link_response,
            response_timeout: self.builder.response_timeout,
            check_interface_version: self.builder.check_interface_version,
            #[cfg(feature = "rustls")]
            rustls_config: self.builder.rustls_config,
            #[cfg(feature = "native-tls")]
            native_tls_connector: self.builder.native_tls_connector,
            _phantom: std::marker::PhantomData,
        }
    }

    /// Enables insight events from the background connection.
    pub fn insights(self) -> ConnectionBuilder<InsightEventChannel> {
        ConnectionBuilder {
            max_command_length: self.builder.max_command_length,
            enquire_link_interval: self.builder.enquire_link_interval,
            enquire_link_response_timeout: self.builder.enquire_link_response_timeout,
            auto_enquire_link_response: self.builder.auto_enquire_link_response,
            response_timeout: self.builder.response_timeout,
            check_interface_version: self.builder.check_interface_version,
            #[cfg(feature = "rustls")]
            rustls_config: self.builder.rustls_config,
            #[cfg(feature = "native-tls")]
            native_tls_connector: self.builder.native_tls_connector,
            _phantom: std::marker::PhantomData,
        }
    }
}
