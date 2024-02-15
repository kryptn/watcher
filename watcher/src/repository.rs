use aws_sdk_dynamodb::types;
use serde_dynamo::{to_attribute_value, Item};

use crate::types::{Edge, Node, Sink, Source, Subscription, WatcherItem};

pub struct Repository {
    table_name: String,
    client: aws_sdk_dynamodb::Client,
}

impl Repository {
    pub fn new(table_name: String, client: aws_sdk_dynamodb::Client) -> Self {
        Self { table_name, client }
    }

    pub async fn get_item<T>(
        &self,
        primary_key: &str,
        sort_key: &str,
    ) -> Result<T, Box<dyn std::error::Error>>
    where
        T: serde::de::DeserializeOwned,
    {
        let response = self
            .client
            .get_item()
            .table_name(self.table_name.clone())
            .key("PK", to_attribute_value(primary_key)?)
            .key("SK", to_attribute_value(sort_key)?)
            .send()
            .await?;

        let item = response.item.unwrap_or_default();

        let item: T = serde_dynamo::from_item(item)?;

        Ok(item)
    }

    pub async fn put_item<T>(&self, item: T) -> Result<(), Box<dyn std::error::Error>>
    where
        T: serde::Serialize,
        T: Into<WatcherItem>,
    {
        let watcher_item: WatcherItem = item.into();
        let item: Item = serde_dynamo::to_item(watcher_item)?;

        self.client
            .put_item()
            .table_name(self.table_name.clone())
            .set_item(Some(item.into()))
            .send()
            .await?;

        Ok(())
    }

    pub async fn remove<T>(
        &self,
        pk: &str,
        sk: &str,
        fields: &[&str],
    ) -> Result<(), Box<dyn std::error::Error>>
    where
        T: Into<WatcherItem>,
    {
        let _ = self
            .client
            .update_item()
            .table_name(self.table_name.clone())
            .key("PK", to_attribute_value(pk)?)
            .key("SK", to_attribute_value(sk)?)
            .update_expression(format!("REMOVE {}", fields.join(", ")))
            .return_values(types::ReturnValue::UpdatedOld)
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

    pub async fn get_sinks_for_endpoint(
        &self,
        source_id: String,
    ) -> Result<Vec<Subscription>, Box<dyn std::error::Error>> {
        let response = self
            .client
            .query()
            .table_name(self.table_name.clone())
            .key_condition_expression("PK = :pk and begins_with(SK, :sink)")
            .expression_attribute_values(":pk", to_attribute_value(source_id)?)
            .expression_attribute_values(":sink", to_attribute_value("Sink")?)
            .send()
            .await?;

        let items = response.items.unwrap_or_default();

        dbg!(&items);

        let items: Vec<Subscription> = items
            .into_iter()
            .filter_map(|item| Some(serde_dynamo::from_item(item).unwrap()))
            .collect();

        Ok(items)
    }

    pub async fn set_schedule_name_for_endpoint(
        &self,
        source_id: &str,
        schedule_name: &str,
    ) -> Result<(), Box<dyn std::error::Error>> {
        self.client
            .update_item()
            .table_name(self.table_name.clone())
            .key("PK", to_attribute_value(source_id)?)
            .key("SK", to_attribute_value(source_id)?)
            .update_expression("SET schedule_name = :schedule_name")
            .expression_attribute_values(":schedule_name", to_attribute_value(schedule_name)?)
            .send()
            .await?;

        Ok(())
    }
}
