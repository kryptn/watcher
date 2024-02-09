use serde_dynamo::Item;

use crate::types::WatcherItem;

pub struct Repository {
    table_name: String,
    client: aws_sdk_dynamodb::Client,
}

impl Repository {
    pub fn new(table_name: String, client: aws_sdk_dynamodb::Client) -> Self {
        Self { table_name, client }
    }

    pub async fn put_item(&self, item: WatcherItem) -> Result<(), Box<dyn std::error::Error>>
where
        // T: serde::Serialize,
    {
        let item: Item = serde_dynamo::to_item(item)?;

        self.client
            .put_item()
            .table_name(self.table_name.clone())
            .set_item(Some(item.into()))
            .send()
            .await?;

        Ok(())
    }

    pub async fn list_all_items(&self) -> Result<Vec<WatcherItem>, Box<dyn std::error::Error>> {
        let response = self
            .client
            .scan()
            .table_name(self.table_name.clone())
            .send()
            .await?;

        let items = response.items.unwrap_or_default();

        let items: Vec<WatcherItem> = items
            .into_iter()
            .filter_map(|item| serde_dynamo::from_item(item).ok())
            .collect();

        Ok(items)
    }
}
