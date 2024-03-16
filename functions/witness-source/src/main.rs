use std::env;

use lambda_runtime::{run, service_fn, Error, LambdaEvent};
use tracing::Instrument;
use tracing_subscriber::filter::{EnvFilter, LevelFilter};

use serde::{Deserialize, Serialize};
use watcher::{
    messaging::SqsProvider,
    repository::Repository,
    types::{Command, Source},
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

/// This is the main body for the function.
/// Write your code inside it.
/// There are some code example in the following URLs:
/// - https://github.com/awslabs/aws-lambda-rust-runtime/tree/main/examples
/// - https://github.com/aws-samples/serverless-rust-demo/
#[tracing::instrument]
async fn function_handler(
    event: LambdaEvent<Command>,
) -> Result<Response, Box<dyn std::error::Error>> {
    let cmd = event.payload;

    let source_id = match &cmd {
        Command::ObserveSource { source_id } => source_id,
        _ => {
            return Err("Invalid command".into());
        }
    };

    let config = watcher::config::init();
    let repo = Repository::lambda_new(config.table_name.expect("TABLE_NAME must be set")).await;
    let sqs = SqsProvider::new(config.sqs_queue_url.expect("SQS_QUEUE_URL must be set")).await;

    let source: Source = repo
        .get_item(&source_id, &source_id)
        .in_current_span()
        .await?;
    tracing::info!(target: "fetch_source", source_id=source.id, request_id=event.context.request_id, "fetching source");

    // Prepare the response
    let resp = Response {
        req_id: event.context.request_id,
        msg: format!("order {:?}.", cmd),
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
