use std::sync::Arc;

use crate::{config, meta_repo, repository, storage};

#[derive(Clone)]
pub struct Application {
    watcher: Arc<repository::Repository>,
    meta: Arc<meta_repo::Repository>,

    storage: Arc<storage::Client>,
}

impl Application {
    pub async fn new() -> Self {
        let watcher = Arc::new(
            repository::Repository::lambda_new(
                config::init().table_name.expect("TABLE_NAME must be set"),
            )
            .await,
        );

        let meta = Arc::new(meta_repo::Repository::new(
            config::init().table_name.expect("TABLE_NAME must be set"),
            aws_sdk_dynamodb::Client::new(&aws_config::load_from_env().await),
        ));

        let storage = Arc::new(
            storage::Client::new(config::init().bucket.expect("BUCKET must be set")).await,
        );

        Self {
            watcher,
            meta,
            storage,
        }
    }
}
