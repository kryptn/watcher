use std::collections::HashMap;

use async_trait::async_trait;
use aws_sdk_dynamodb::types::{self, AttributeValue};
use itertools::Itertools;
use serde_dynamo::{to_attribute_value, Item as DynamoItem};

// use serde_json::Value;

use crate::types::{Item, Sink, Source, Subscription};

// use serde::value::Value;

fn ensure_sk(item: DynamoItem) -> DynamoItem {
    let mut item = item;
    if !item.contains_key("SK") {
        let pk = item.get("PK").unwrap().clone();
        item.insert("SK".to_string(), pk);
    }
    item
}

pub struct Repository {
    table_name: String,
    client: aws_sdk_dynamodb::Client,
}

impl Repository {
    pub async fn lambda_new(table_name: String) -> Self {
        let aws_config = aws_config::load_from_env().await;
        let client = aws_sdk_dynamodb::Client::new(&aws_config);
        Self { table_name, client }
    }
}

fn as_av_hashmap(item: DynamoItem) -> HashMap<String, AttributeValue> {
    item.iter()
        .map(|(k, v)| (k.clone(), to_attribute_value(v.clone()).unwrap()))
        .collect()
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
        T: Into<Item>,
    {
        let watcher_item: Item = item.into();
        let item: DynamoItem = serde_dynamo::to_item(watcher_item)?;
        let item = ensure_sk(item);

        self.client
            .put_item()
            .table_name(self.table_name.clone())
            .set_item(Some(item.into()))
            .send()
            .await?;

        Ok(())
    }

    pub async fn put_items<T>(
        &self,
        items: impl Iterator<Item = T>,
    ) -> Result<(), Box<dyn std::error::Error>>
    where
        T: Into<Item>,
    {
        let items = items
            .map(|i| i.into())
            // convert to a Item
            .map(Item::from)
            // convert to a DynamoDB item
            .map(serde_dynamo::to_item)
            // convert Result to Option, filter out failures
            .filter_map(Result::ok)
            // adds SK to Node items
            .map(ensure_sk)
            // convert to a HashMap
            .map(as_av_hashmap);

        let chunks = items.chunks(25);

        for chunk in &chunks {
            let mut request = self.client.batch_write_item();

            for item in chunk {
                let item_request = types::PutRequest::builder().set_item(Some(item)).build()?;
                request = request.request_items(
                    self.table_name.clone(),
                    vec![types::WriteRequest::builder()
                        .put_request(item_request)
                        .build()],
                );
            }

            request.send().await?;
        }

        Ok(())
    }

    pub async fn remove<T>(
        &self,
        pk: &str,
        sk: &str,
        fields: &[&str],
    ) -> Result<(), Box<dyn std::error::Error>>
    where
        T: Into<Item>,
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

    pub async fn list_all_items(&self) -> Result<Vec<Item>, Box<dyn std::error::Error>> {
        let response = self
            .client
            .scan()
            .table_name(self.table_name.clone())
            .send()
            .await?;

        let items = response.items.unwrap_or_default();

        let items: Vec<Item> = items
            .into_iter()
            .filter_map(|item| serde_dynamo::from_item(item).ok())
            .collect();

        Ok(items)
    }

    pub async fn get_sinks_for_source(
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

    pub async fn set_schedule_name_for_source(
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
