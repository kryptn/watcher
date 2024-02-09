use serde_dynamo::{to_item, Item};

use watcher::{repository::Repository, types::Endpoint};

use aws_sdk_dynamodb as dynamodb;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Hello, world!");

    let table_name = "AdjacencyListExplore";

    let config = aws_config::load_from_env().await;
    let client = aws_sdk_dynamodb::Client::new(&config);

    let repo = Repository::new(table_name.to_string(), client);

    

    let endpoint = Endpoint::mock();
    let serialized = serde_json::to_string(&endpoint).unwrap();
    println!("fake endpoint: \n\t{}", serialized);

    let dynamodb_serialized: Item = to_item(endpoint.clone())?;
    println!("dynamodb item: \n\t{:#?}", dynamodb_serialized);

    Ok(())
}
