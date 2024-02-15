use std::env;

use lambda_runtime::{run, service_fn, Error, LambdaEvent};
use tracing_subscriber::filter::{EnvFilter, LevelFilter};

use serde::{Deserialize, Serialize};
use watcher::{
    repository::Repository,
    types::{Source, SourceSchedule},
};

/// This is a made-up example of what a response structure may look like.
/// There is no restriction on what it can be. The runtime requires responses
/// to be serialized into json. The runtime pays no attention
/// to the contents of the response payload.
#[derive(Serialize)]
struct Response {
    req_id: String,
    msg: String,
}

async fn make_repo(table_name: &str) -> Result<Repository, Box<dyn std::error::Error>> {
    let aws_config = aws_config::load_from_env().await;
    let client = aws_sdk_dynamodb::Client::new(&aws_config);
    let repo = Repository::new(table_name.to_string(), client);

    Ok(repo)
}

/// This is the main body for the function.
/// Write your code inside it.
/// There are some code example in the following URLs:
/// - https://github.com/awslabs/aws-lambda-rust-runtime/tree/main/examples
/// - https://github.com/aws-samples/serverless-rust-demo/
async fn function_handler(
    event: LambdaEvent<SourceSchedule>,
) -> Result<Response, Box<dyn std::error::Error>> {
    let order = event.payload;
    let table_name = env::var("TABLE_NAME").expect("TABLE_NAME must be set");
    let repo = make_repo(&table_name).await?;

    let endpoint: Source = repo.get_item(&order.source_id, &order.source_id).await?;

    tracing::info!("order: {:?}", order);
    tracing::info!("request_id: {}", event.context.request_id);
    // Prepare the response
    let resp = Response {
        req_id: event.context.request_id,
        msg: format!("order {:?}.", order),
    };
    Ok(resp)
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    tracing_subscriber::fmt()
        .with_env_filter(
            EnvFilter::builder()
                .with_default_directive(LevelFilter::INFO.into())
                .from_env_lossy(),
        )
        // disable printing the name of the module in every log line.
        .with_target(false)
        // disabling time is handy because CloudWatch will add the ingestion time.
        .without_time()
        .init();

    run(service_fn(function_handler)).await
}
