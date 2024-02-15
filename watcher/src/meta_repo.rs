use aws_sdk_dynamodb::types;

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
}
