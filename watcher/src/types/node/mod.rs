use serde::{self, Deserialize, Serialize};

pub mod source;
pub use source::*;

pub mod state;
pub use state::*;

pub mod sink;
pub use sink::*;

pub mod signal;
pub use signal::*;

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "snake_case", tag = "node_type")]
pub enum Node {
    Source(Source),
    State(State),
    Sink(Sink),
    Signal(Signal),
}

impl From<Source> for Node {
    fn from(endpoint: Source) -> Self {
        Node::Source(endpoint)
    }
}

impl From<State> for Node {
    fn from(observation: State) -> Self {
        Node::State(observation)
    }
}

impl From<Sink> for Node {
    fn from(sink: Sink) -> Self {
        Node::Sink(sink)
    }
}

impl From<Signal> for Node {
    fn from(broadcast: Signal) -> Self {
        Node::Signal(broadcast)
    }
}
