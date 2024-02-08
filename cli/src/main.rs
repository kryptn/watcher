use serde_dynamo::{to_item, Item};

use watcher::types::Endpoint;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Hello, world!");

    let endpoint = Endpoint::mock();
    let serialized = serde_json::to_string(&endpoint).unwrap();
    println!("fake endpoint: \n\t{}", serialized);

    let dynamodb_serialized: Item = to_item(endpoint.clone())?;
    println!("dynamodb item: \n\t{:#?}", dynamodb_serialized);

    Ok(())
}
