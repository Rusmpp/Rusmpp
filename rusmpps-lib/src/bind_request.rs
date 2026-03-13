use crate::bind_requet_kind::BindRequestKind;
use futures::Stream;
use rusmpp::pdus::builders::BindTransmitterRespBuilder;
use rusmpp::pdus::{BindReceiver, BindTransceiver, BindTransmitter};
use rusmpp::types::COctetString;
use rusmpp::{Command, CommandStatus, Pdu};
use rusmppc::Client;
use rusmppc::error::Error;

#[derive(Debug)]
pub struct BindRequest {
    client: Client,
    pub bind: Bind,
    kind: BindRequestKind,
    pub sequence_id: u32,
}

impl BindRequest {
    pub fn kind(&self) -> BindRequestKind {
        self.kind
    }

    pub async fn accept(
        self,
    ) -> Result<(Client, impl Stream<Item = Command> + Unpin + 'static), Error> {
        let stream = futures::stream::empty::<Command>();

        Ok((self.client, stream))
    }

    pub async fn reject(self, status: CommandStatus) {
        let pdu: Pdu = match self.kind {
            BindRequestKind::Tx => BindTransmitterRespBuilder::default().build().into(),
            BindRequestKind::Rx => BindTransmitterRespBuilder::default().build().into(),
            BindRequestKind::Trx => BindTransmitterRespBuilder::default().build().into(),
        };
        let command = Command::builder()
            .status(status)
            .sequence_number(self.sequence_id)
            .pdu(pdu);
    }
}

#[derive(Debug)]
pub struct Bind {
    pub system_id: COctetString<1, 16>,
}

macro_rules! frombind {
    ($bind_type:ty, $kind:expr) => {
        impl From<$bind_type> for Bind {
            fn from(bind: $bind_type) -> Self {
                Self {
                    system_id: bind.system_id,
                }
            }
        }

        impl From<(Client, u32, $bind_type)> for BindRequest {
            fn from((client, sequence_id, bind): (Client, u32, $bind_type)) -> Self {
                Self {
                    client,
                    bind: bind.into(),
                    sequence_id,
                    kind: $kind,
                }
            }
        }
    };
}

frombind!(BindReceiver, BindRequestKind::Rx);
frombind!(BindTransmitter, BindRequestKind::Tx);
frombind!(BindTransceiver, BindRequestKind::Trx);
