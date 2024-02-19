pub mod node;
pub use node::*;

pub mod edge;
pub use edge::*;

pub mod events;
pub use events::*;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
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
mod test {}
