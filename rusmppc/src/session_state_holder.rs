use rusmpp::session::SessionState;
use std::sync::{Arc, RwLock};

#[derive(Debug, Clone)]
pub(crate) struct SessionStateHolder {
    inner: Arc<RwLock<SessionState>>,
}

impl SessionStateHolder {
    pub(crate) fn new(state: SessionState) -> Self {
        Self {
            inner: Arc::new(RwLock::new(state)),
        }
    }

    pub(crate) fn get(&self) -> SessionState {
        let state = *self.inner.read().unwrap();
        tracing::debug!("Session state.get: {state:?}");
        state
    }

    pub(crate) fn set(&self, new_state: SessionState) {
        tracing::debug!("Session state.set: {new_state:?}");
        *self.inner.write().unwrap() = new_state;
    }
}
