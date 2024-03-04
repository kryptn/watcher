use aws_lambda_events::event::sqs::SqsEvent;
use lambda_runtime::{run, service_fn, Error, LambdaEvent};
use tracing_subscriber::filter::{EnvFilter, LevelFilter};
use watcher::{
    repository::Repository,
    types::{self, Signal, SinkSignalCreated},
};

/// This is the main body for the function.
/// Write your code inside it.
/// There are some code example in the following URLs:
/// - https://github.com/awslabs/aws-lambda-rust-runtime/tree/main/examples
/// - https://github.com/aws-samples/serverless-rust-demo/
async fn function_handler(event: LambdaEvent<SqsEvent>) -> Result<(), Error> {
    // Extract some useful information from the request

    let config = watcher::config::init();

    let repo = Repository::lambda_new(config.table_name.expect("TABLE_NAME must be set")).await;

    for record in event.payload.records {
        if let Some(body) = record.body {
            let payload: SinkSignalCreated = serde_json::from_str(&body).unwrap();
            tracing::info!(target: "", signal_id = payload.signal_id, sink_id = payload.sink_id);

            let signal: Signal = repo
                .get_item(&payload.signal_id, &payload.signal_id)
                .await
                .unwrap();

            let sink: types::Sink = repo
                .get_item(&payload.sink_id, &payload.sink_id)
                .await
                .unwrap();

            let source: ext::Source = types::ext::get_source(&signal.source).unwrap();

            match sink.sink {
                types::SinkType::Discord(d) => {
                    let resp = d.send(signal).await.unwrap();
                }
            }
        }
    }

    Ok(())
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
