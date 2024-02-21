pub mod node;
pub use node::*;

pub mod edge;
pub use edge::*;

pub mod events;
pub use events::*;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "snake_case", tag = "item_type")]
pub enum Item {
    Assertion(edge::Asserted),
    Witness(edge::Witnessed),
    LastSignal(edge::LastSignal),
    Measurement(edge::Measurement),
    Sent(edge::Sent),
    Subscription(edge::Subscription),
    Sink(node::Sink),
    Source(node::Source),
    State(node::State),
    Signal(node::Signal),
}

#[cfg(test)]
mod test {}
