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

#[derive(Serialize, Deserialize)]
#[cfg_attr(test, derive(Debug, PartialEq))]
#[serde(rename_all = "snake_case", tag = "node_type")]
pub enum Edge {
    Measurement(Measurement),
    Subscription(Subscription),
    Emission(Emission),
    Sent(Sent),
}
