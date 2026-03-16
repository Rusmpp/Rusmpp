use rusmpp::session::SessionState;

#[derive(Debug, Copy, Clone)]
pub enum BindMode {
    Tx,
    Rx,
    Trx,
}

impl From<BindMode> for SessionState {
    fn from(bind_mode: BindMode) -> Self {
        match bind_mode {
            BindMode::Tx => SessionState::BoundTx,
            BindMode::Rx => SessionState::BoundRx,
            BindMode::Trx => SessionState::BoundTrx,
        }
    }
}
