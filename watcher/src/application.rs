use std::sync::Arc;

use crate::{config, messaging, meta_repo, repository, storage, types::Command};

#[derive(Clone)]
pub struct Application {
    pub watcher: Arc<repository::Repository>,
    pub meta: Arc<meta_repo::Repository>,

    pub storage: Arc<storage::Client>,
    pub queue: Arc<messaging::SqsProvider>,
}

impl Application {
    pub async fn new() -> Self {
        let config = config::init();

        let table_name = config.table_name.expect("TABLE_NAME must be set");
        let bucket = config.bucket.expect("BUCKET must be set");

        let watcher = Arc::new(repository::Repository::lambda_new(table_name.clone()).await);

        let meta = Arc::new(meta_repo::Repository::new(
            table_name.clone(),
            aws_sdk_dynamodb::Client::new(&aws_config::load_from_env().await),
        ));

        let storage = Arc::new(storage::Client::new(bucket).await);
        let queue = Arc::new(
            messaging::SqsProvider::new(config.sqs_queue_url.expect("SQS_QUEUE_URL must be set"))
                .await,
        );

        Self {
            watcher,
            meta,
            storage,
            queue,
        }
    }

    pub async fn handle(
        &self,
        commands: Vec<&Command>,
    ) -> Result<(), Box<dyn std::error::Error + Sync + Send>> {
        for command in commands {
            match command {
                Command::ObserveSource { source_id: _ } => todo!(),
                Command::SendSignal {
                    signal_id: _,
                    sink_id: _,
                } => todo!(),
                Command::Subscribe {
                    source_id: _,
                    sink_id: _,
                } => todo!(),
                Command::Unsubscribe {
                    source_id: _,
                    sink_id: _,
                } => todo!(),
                Command::AddSource { name: _, config: _ } => todo!(),
                Command::DeleteSource { source_id: _ } => todo!(),
                Command::AddSink { name: _, config: _ } => todo!(),
                Command::DeleteSink { sink_id: _ } => todo!(),
            }
        }

        Ok(())
    }
}
