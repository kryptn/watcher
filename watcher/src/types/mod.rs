pub mod node;
pub use node::*;

pub mod edge;
pub use edge::*;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
#[cfg_attr(test, derive(Debug, PartialEq))]
#[serde(rename_all = "snake_case", tag = "item_type")]
pub enum WatcherItem {
    Node(Node),
    Edge(Edge),
}

impl From<Node> for WatcherItem {
    fn from(node: Node) -> Self {
        WatcherItem::Node(node)
    }
}

impl From<Edge> for WatcherItem {
    fn from(edge: Edge) -> Self {
        WatcherItem::Edge(edge)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use fake::{Fake, Faker};
    use serde_dynamo::{to_item, Item};
    use serde_json::json;

    #[test]
    fn test_item_from_chain() {
        let endpoint: Endpoint = Faker.fake();
        let item: WatcherItem = endpoint.to_watcher_item();
        let serialized = serde_json::to_string(&item).unwrap();
        let dynamodb_serialized: Item = to_item(item.clone()).expect("pass");

        dbg!(serialized);
    }
}
