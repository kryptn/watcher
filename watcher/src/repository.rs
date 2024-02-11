use aws_sdk_dynamodb::types;
use serde_dynamo::{to_attribute_value, to_item, Item};

use crate::types::{Edge, Endpoint, Node, Sink, Subscription, WatcherItem};

pub struct Repository {
    table_name: String,
    client: aws_sdk_dynamodb::Client,
}

impl Repository {
    pub fn new(table_name: String, client: aws_sdk_dynamodb::Client) -> Self {
        Self { table_name, client }
    }

    pub async fn delete_table(&self) -> Result<(), Box<dyn std::error::Error>> {
        self.client
            .delete_table()
            .table_name(self.table_name.clone())
            .send()
            .await?;

        Ok(())
    }

    pub async fn create_table(&self) -> Result<(), Box<dyn std::error::Error>> {
        self.client
            .create_table()
            .table_name(self.table_name.clone())
            .attribute_definitions(
                types::AttributeDefinition::builder()
                    .attribute_name("PK")
                    .attribute_type(types::ScalarAttributeType::S)
                    .build()?,
            )
            .attribute_definitions(
                types::AttributeDefinition::builder()
                    .attribute_name("SK")
                    .attribute_type(types::ScalarAttributeType::S)
                    .build()?,
            )
            .key_schema(
                types::KeySchemaElement::builder()
                    .attribute_name("PK")
                    .key_type(types::KeyType::Hash)
                    .build()?,
            )
            .key_schema(
                types::KeySchemaElement::builder()
                    .attribute_name("SK")
                    .key_type(types::KeyType::Range)
                    .build()?,
            )
            .global_secondary_indexes(
                types::GlobalSecondaryIndex::builder()
                    .index_name("AdjacencyList")
                    .projection(
                        types::Projection::builder()
                            .projection_type(types::ProjectionType::All)
                            .build(),
                    )
                    .key_schema(
                        types::KeySchemaElement::builder()
                            .attribute_name("SK")
                            .key_type(types::KeyType::Hash)
                            .build()?,
                    )
                    .key_schema(
                        types::KeySchemaElement::builder()
                            .attribute_name("PK")
                            .key_type(types::KeyType::Range)
                            .build()?,
                    )
                    .provisioned_throughput(
                        types::ProvisionedThroughput::builder()
                            .read_capacity_units(60)
                            .write_capacity_units(60)
                            .build()?,
                    )
                    .build()?,
            )
            .provisioned_throughput(
                types::ProvisionedThroughput::builder()
                    .read_capacity_units(60)
                    .write_capacity_units(60)
                    .build()?,
            )
            .send()
            .await?;

        Ok(())
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

    // pub async fn delete_all_items(&self) -> Result<(), Box<dyn std::error::Error>> {
    //     let items = self.list_all_items().await?;
    //     for item in items {
    //         self.client
    //             .delete_item()
    //             .table_name(self.table_name.clone())
    //             .key(item.primary_key())
    //             .send()
    //             .await?;
    //     }
    //     Ok(())
    // }

    pub async fn create_endpoint(
        &self,
        endpoint: &Endpoint,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let node: Node = endpoint.clone().into();
        self.put_item(node).await?;
        Ok(())
    }

    pub async fn create_sink(&self, sink: &Sink) -> Result<(), Box<dyn std::error::Error>> {
        let node: Node = sink.clone().into();
        self.put_item(node).await?;
        Ok(())
    }

    pub async fn create_subscription(
        &self,
        subscription: &Subscription,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let edge: Edge = subscription.clone().into();
        self.put_item(edge).await?;
        Ok(())
    }

    pub async fn get_sinks_for_endpoint(
        &self,
        endpoint_id: String,
    ) -> Result<Vec<Subscription>, Box<dyn std::error::Error>> {
        let response = self
            .client
            .query()
            .table_name(self.table_name.clone())
            .key_condition_expression("PK = :pk and begins_with(SK, :sink)")
            .expression_attribute_values(":pk", to_attribute_value(endpoint_id)?)
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
}
