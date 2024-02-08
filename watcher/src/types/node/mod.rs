use serde::{self, Deserialize, Serialize};

use crate::types;
pub mod endpoint;
pub use endpoint::*;

pub mod observation;
pub use observation::*;

pub mod sink;
pub use sink::*;

pub mod broadcast;
pub use broadcast::*;

#[derive(Serialize, Deserialize)]
#[cfg_attr(test, derive(Debug, PartialEq))]
#[serde(rename_all = "snake_case", tag = "node_type")]
pub enum Node {
    Endpoint(types::Endpoint),
    Observation(types::Observation),
    Sink(types::Sink),
    Broadcast(types::Broadcast),
}
