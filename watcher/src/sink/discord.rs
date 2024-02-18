use serde::{Deserialize, Serialize};

use crate::types::{node, Node, SinkSignalCreated};

use super::Sink;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Config {
    pub webhook: String,
}

impl From<node::Sink> for Config {
    fn from(node: node::Sink) -> Self {
        match node.sink {
            node::SinkType::Discord(discord) => Config {
                webhook: discord.url,
            },
        }
    }
}

impl Config {
    pub async fn send(&self, payload: WebhookPayload) -> Result<(), String> {
        let client = reqwest::Client::new();
        let response = client
            .post(&self.webhook)
            .json(&payload)
            .send()
            .await
            .map_err(|e| e.to_string())?;

        Ok(())
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct WebhookPayload {
    pub content: String,
}

impl From<SinkSignalCreated> for WebhookPayload {
    fn from(edge: SinkSignalCreated) -> Self {
        WebhookPayload {
            content: format!("Signal created: {}", edge.signal_id),
        }
    }
}
