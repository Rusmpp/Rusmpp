use std::{collections::HashMap, sync::Arc};

use rusmpp::{Pdu, session::SessionState};
use tokio::sync::{RwLock, RwLockReadGuard, mpsc::Sender};

#[derive(Debug)]
pub struct Client {
    pub system_id: String,
    pub password: String,
}

#[derive(Debug)]
pub enum Action {
    Send(Pdu),
}

#[derive(Debug)]
pub struct SequenceNumber {
    current: u32,
}

impl Default for SequenceNumber {
    fn default() -> Self {
        Self::new()
    }
}

impl SequenceNumber {
    pub fn new() -> Self {
        Self { current: 1 }
    }

    pub fn current_and_increment(&mut self) -> u32 {
        let seq = self.current;

        self.current += 1;

        seq
    }
}

#[derive(Debug, Default)]
pub struct ConnectedClient {
    sessions: HashMap<u64, ClientSession>,
}

impl ConnectedClient {
    pub fn new() -> Self {
        Self {
            sessions: HashMap::new(),
        }
    }

    pub fn session(&self, session_id: u64) -> Option<&ClientSession> {
        self.sessions.get(&session_id)
    }

    fn insert_session(&mut self, session_id: u64, session: ClientSession) {
        self.sessions.insert(session_id, session);
    }

    fn remove_session(&mut self, session_id: u64) -> Option<ClientSession> {
        self.sessions.remove(&session_id)
    }
}

#[derive(Debug)]
pub struct ClientSession {
    pub tx: Sender<Action>,
    pub session_state: SessionState,
}

impl ClientSession {
    pub fn new(sender: Sender<Action>, session_state: SessionState) -> Self {
        Self {
            tx: sender,
            session_state,
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct ConnectedClients {
    clients: Arc<RwLock<HashMap<String, ConnectedClient>>>,
}

impl ConnectedClients {
    pub fn new() -> Self {
        Default::default()
    }

    pub async fn insert_session(&self, system_id: String, session_id: u64, session: ClientSession) {
        let mut clients = self.clients.write().await;

        let client = clients
            .entry(system_id)
            .or_insert_with(ConnectedClient::new);

        client.insert_session(session_id, session);
    }

    pub async fn remove_session(&self, system_id: &str, session_id: u64) -> Option<ClientSession> {
        tracing::debug!(system_id, session_id, "Removing session");

        let mut clients = self.clients.write().await;

        match clients.get_mut(system_id) {
            Some(client) => {
                let session = client.remove_session(session_id);

                if client.sessions.is_empty() {
                    tracing::debug!(system_id, "Removing client");

                    clients.remove(system_id);
                }

                session
            }
            None => None,
        }
    }

    pub async fn clients(&self) -> RwLockReadGuard<'_, HashMap<String, ConnectedClient>> {
        self.clients.read().await
    }
}
