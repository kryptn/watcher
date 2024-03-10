pub mod commands;
pub mod edge;
pub mod events;
pub mod node;

pub use commands::*;
pub use edge::*;
pub use events::*;
pub use node::*;

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
