use serde::{self, Deserialize, Serialize};

pub mod endpoint;
pub use endpoint::*;

pub mod observation;
pub use observation::*;

pub mod sink;
pub use sink::*;

pub mod broadcast;
pub use broadcast::*;

#[derive(Serialize, Deserialize, Clone, Debug)]
#[cfg_attr(test, derive(PartialEq))]
#[serde(rename_all = "snake_case", tag = "node_type")]
pub enum Node {
    Endpoint(Endpoint),
    Observation(Observation),
    Sink(Sink),
    Broadcast(Broadcast),
}

impl From<Endpoint> for Node {
    fn from(endpoint: Endpoint) -> Self {
        Node::Endpoint(endpoint)
    }
}

impl From<Observation> for Node {
    fn from(observation: Observation) -> Self {
        Node::Observation(observation)
    }
}

impl From<Sink> for Node {
    fn from(sink: Sink) -> Self {
        Node::Sink(sink)
    }
}

impl From<Broadcast> for Node {
    fn from(broadcast: Broadcast) -> Self {
        Node::Broadcast(broadcast)
    }
}
