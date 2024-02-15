#![allow(dead_code, unused_imports)]
use serde::{Deserialize, Serialize};

pub mod measurement;
pub use measurement::*;

pub mod subscription;
pub use subscription::*;

pub mod last_signal;
pub use last_signal::*;

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

    Witnessed {
        #[serde(rename = "PK")]
        source_id: String,
        #[serde(rename = "SK")]
        state_id: String,
    },
    Attested {
        #[serde(rename = "PK")]
        source_id: String,
        #[serde(rename = "SK")]
        state_id: String,
    },
    Measured {
        #[serde(rename = "PK")]
        state_id: String,
        #[serde(rename = "SK")]
        signal_id: String,
    },
    Derived {
        #[serde(rename = "PK")]
        state_id: String,
        #[serde(rename = "SK")]
        signal_id: String,
    },
    Last {
        #[serde(rename = "PK")]
        source_id: String,
        #[serde(rename = "SK")]
        signal_id: String,
    },
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

impl From<Sent> for Edge {
    fn from(sent: Sent) -> Self {
        Edge::Sent(sent)
    }
}
