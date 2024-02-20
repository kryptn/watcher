use serde::{Deserialize, Serialize};

use crate::types::node::Signal;

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "snake_case", rename = "discord")]
pub struct Config {
    pub webhook: String,
}

impl Config {
    pub async fn send(&self, signal: Signal) -> Result<(), String> {
        let payload = WebhookPayload {
            content: signal.contents.clone(),
        };

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
