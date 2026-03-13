use crate::acceptor::Acceptor;
use rusmpp::types::COctetString;
use std::time::Duration;
use tokio::net::TcpListener;

#[derive(Debug)]
pub struct Server {
    system_id: COctetString<1, 16>,
    session_timeout: Duration,
    enquire_link_interval: Duration,
    response_timeout: Option<Duration>,
    check_interface_version: bool,
}

impl Server {
    pub fn acceptor(self, tcp_listener: TcpListener) -> Acceptor {
        Acceptor {
            session_timeout: self.session_timeout,
            response_timeout: self.response_timeout,
            tcp_listener,
            enquire_link_interval: self.enquire_link_interval,
            check_interface_version: self.check_interface_version,
        }
    }
}

impl Server {
    pub fn builder() -> ServerBuilder {
        ServerBuilder::default()
    }
}

#[derive(Debug, Default)]
pub struct ServerBuilder {
    system_id: COctetString<1, 16>,
    session_timeout: Duration,
}

impl ServerBuilder {
    pub fn system_id(mut self, system_id: COctetString<1, 16>) -> Self {
        self.system_id = system_id;
        self
    }

    pub fn build(self) -> Server {
        Server {
            system_id: self.system_id,
            session_timeout: self.session_timeout,
            enquire_link_interval: Duration::from_secs(5),
            response_timeout: Some(Duration::from_secs(2)),
            check_interface_version: true,
        }
    }
}

impl ServerBuilder {
    pub fn new() -> Self {
        Default::default()
    }
}
