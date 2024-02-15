#![allow(dead_code, unused_imports)]
use serde::{Deserialize, Serialize};

pub mod measurement;
pub use measurement::*;

pub mod subscription;
pub use subscription::*;

pub mod last_signal;
pub use last_signal::*;

pub mod witnessed;
pub use witnessed::*;

pub mod asserted;
pub use asserted::*;

pub mod sent;
pub use sent::*;

#[derive(Serialize, Deserialize, Clone, Debug)]
#[cfg_attr(test, derive(PartialEq))]
#[serde(rename_all = "snake_case", tag = "edge_type")]
pub enum Edge {
    Measurement(Measurement),
    Subscription(Subscription),
    LastSignal(LastSignal),
    Sent(Sent),
    Asserted(Asserted),
    Witnessed(Witnessed),
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

impl From<LastSignal> for Edge {
    fn from(emission: LastSignal) -> Self {
        Edge::LastSignal(emission)
    }
}

impl From<Asserted> for Edge {
    fn from(asserted: Asserted) -> Self {
        Edge::Asserted(asserted)
    }
}

impl From<Witnessed> for Edge {
    fn from(witnessed: Witnessed) -> Self {
        Edge::Witnessed(witnessed)
    }
}

impl From<Sent> for Edge {
    fn from(sent: Sent) -> Self {
        Edge::Sent(sent)
    }
}
