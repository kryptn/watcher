#![allow(dead_code, unused_imports)]
use serde::{Deserialize, Serialize};

pub mod measurement;
pub use measurement::*;

pub mod subscription;
pub use subscription::*;

pub mod emission;
pub use emission::*;

pub mod sent;
pub use sent::*;

#[derive(Serialize, Deserialize, Clone, Debug)]
#[cfg_attr(test, derive(PartialEq))]
#[serde(rename_all = "snake_case", tag = "node_type")]
pub enum Edge {
    Measurement(Measurement),
    Subscription(Subscription),
    Emission(Emission),
    Sent(Sent),
}

impl From<Measurement> for Edge {
    fn from(measurement: Measurement) -> Self {
        Edge::Measurement(measurement)
    }
}

impl From<Subscription> for Edge {
    fn from(subscription: Subscription) -> Self {
        Edge::Subscription(subscription)
    }
}

impl From<Emission> for Edge {
    fn from(emission: Emission) -> Self {
        Edge::Emission(emission)
    }
}

impl From<Sent> for Edge {
    fn from(sent: Sent) -> Self {
        Edge::Sent(sent)
    }
}
