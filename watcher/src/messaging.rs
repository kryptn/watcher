use aws_sdk_sqs::{client, operation::send_message_batch::SendMessageBatch, types, Client};
use itertools::Itertools;

pub struct SqsProvider {
    client: Client,
    queue_url: String,
}

impl SqsProvider {
    pub async fn new(queue_url: String) -> Self {
        let aws_config = aws_config::load_from_env().await;
        let client = aws_sdk_sqs::Client::new(&aws_config);
        Self { client, queue_url }
    }

    pub async fn send<T>(&self, message: T) -> Result<(), Box<dyn std::error::Error>>
    where
        T: serde::Serialize,
    {
        let value: serde_json::Value = serde_json::to_value(message)?;

        self.client
            .send_message()
            .queue_url(self.queue_url.clone())
            .message_body(value.to_string())
            .send()
            .await?;

        Ok(())
    }

    pub async fn send_many<T>(
        &self,
        messages: impl Iterator<Item = T>,
    ) -> Result<(), Box<dyn std::error::Error>>
    where
        T: serde::Serialize,
    {
        for chunk in &messages.enumerate().chunks(10) {
            let mut request = self
                .client
                .send_message_batch()
                .queue_url(self.queue_url.clone());

            for (id, message) in chunk {
                let value: serde_json::Value = serde_json::to_value(message)?;
                let entry = types::SendMessageBatchRequestEntry::builder()
                    .id(id.to_string())
                    .message_body(value.to_string())
                    .build()?;
                request = request.entries(entry);
            }

            request.send().await?;
        }

        Ok(())
    }
}
