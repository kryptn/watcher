use serde_dynamo::{to_item, Item};

use watcher::{repository::Repository, types::Endpoint};

use aws_sdk_dynamodb as dynamodb;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Hello, world!");

    let table_name = "AdjacencyListExplore";

    let config = aws_config::from_env()
        .endpoint_url("http://localhost:8000")
        .load()
        .await;

    let client = aws_sdk_dynamodb::Client::new(&config);

    let repo = Repository::new(table_name.to_string(), client);
    repo.put_item(Endpoint::mock().to_watcher_item()).await?;

    // let endpoint = Endpoint::mock();
    // let serialized = serde_json::to_string(&endpoint).unwrap();
    // println!("fake endpoint: \n\t{}", serialized);

    // let dynamodb_serialized: Item = to_item(endpoint.clone())?;
    // println!("dynamodb item: \n\t{:#?}", dynamodb_serialized);

    let items = repo.list_all_items().await?;

    println!("items: {:#?}", items);
    Ok(())
}
