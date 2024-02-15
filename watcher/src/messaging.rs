use aws_sdk_sqs::{client, Client};

struct SqsProvider {
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
}
