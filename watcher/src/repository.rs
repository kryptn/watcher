use aws_sdk_dynamodb::types::{
    builders::{AttributeDefinitionBuilder, KeySchemaElementBuilder, ProvisionedThroughputBuilder},
    AttributeDefinition, GlobalSecondaryIndex, KeySchemaElement, KeyType, Projection,
    ProjectionType, ProvisionedThroughput, ScalarAttributeType,
};
use serde_dynamo::Item;

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
                AttributeDefinition::builder()
                    .attribute_name("PK")
                    .attribute_type(ScalarAttributeType::S)
                    .build()?,
            )
            .attribute_definitions(
                AttributeDefinition::builder()
                    .attribute_name("SK")
                    .attribute_type(ScalarAttributeType::S)
                    .build()?,
            )
            .key_schema(
                KeySchemaElement::builder()
                    .attribute_name("PK")
                    .key_type(KeyType::Hash)
                    .build()?,
            )
            .key_schema(
                KeySchemaElement::builder()
                    .attribute_name("SK")
                    .key_type(KeyType::Range)
                    .build()?,
            )
            .global_secondary_indexes(
                GlobalSecondaryIndex::builder()
                    .index_name("AdjacencyList")
                    .projection(
                        Projection::builder()
                            .projection_type(ProjectionType::All)
                            .build(),
                    )
                    .key_schema(
                        KeySchemaElement::builder()
                            .attribute_name("SK")
                            .key_type(KeyType::Hash)
                            .build()?,
                    )
                    .key_schema(
                        KeySchemaElement::builder()
                            .attribute_name("PK")
                            .key_type(KeyType::Range)
                            .build()?,
                    )
                    .build()?,
            )
            .provisioned_throughput(
                ProvisionedThroughput::builder()
                    .read_capacity_units(5)
                    .write_capacity_units(5)
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
}
