use aws_sdk_scheduler as scheduler;
use aws_sdk_scheduler::types;

use crate::types::ScheduledObservation;

async fn new() -> scheduler::Client {
    let config = aws_config::load_from_env().await;
    scheduler::Client::new(&config)
}

pub struct TargetConfig {
    pub function_name: String,
    pub region: String,
    pub account_id: String,
}

impl TargetConfig {
    pub fn arn(&self) -> String {
        format!(
            "arn:aws:lambda:{}:{}:function:{}",
            self.region, self.account_id, self.function_name
        )
    }
}

async fn create_schedule(
    client: &scheduler::Client,
    schedule_name: &str,
    target_config: TargetConfig,
    target_input: &ScheduledObservation,
) -> Result<(), Box<dyn std::error::Error>> {
    let input = serde_json::json! {
        {
            "FunctionName": target_config.arn(),
            "InvocationType": "Event",
            "Payload": target_input,
        }
    };

    client
        .create_schedule()
        .name(schedule_name)
        .schedule_expression("rate(1 minute)")
        .target(
            types::Target::builder()
                .arn("arn:aws:scheduler:::aws-sdk:lambda:invoke")
                .input(input.to_string())
                .role_arn("")
                .build()?,
        )
        .send()
        .await?;

    todo!()
}
