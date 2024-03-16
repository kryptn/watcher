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
        let config = config::init();

        let table_name = config.table_name.expect("TABLE_NAME must be set");
        let bucket = config.bucket.expect("BUCKET must be set");

        let watcher = Arc::new(repository::Repository::lambda_new(table_name.clone()).await);

        let meta = Arc::new(meta_repo::Repository::new(
            table_name.clone(),
            aws_sdk_dynamodb::Client::new(&aws_config::load_from_env().await),
        ));

        let storage = Arc::new(storage::Client::new(bucket).await);

        Self {
            watcher,
            meta,
            storage,
        }
    }
}
