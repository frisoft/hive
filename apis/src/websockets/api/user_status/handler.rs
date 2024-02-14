use crate::websockets::internal_server_message::InternalServerMessage;
use anyhow::Result;

pub struct UserStatusHandler {}

impl UserStatusHandler {
    pub async fn new() -> Result<Self> {
        Ok(Self {})
    }

    pub async fn handle(&self) -> Result<Vec<InternalServerMessage>> {
        let messages = Vec::new();
        Ok(messages)
    }
}
