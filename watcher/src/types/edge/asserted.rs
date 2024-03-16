use serde::{Deserialize, Serialize};

use crate::types::{Item, Source, State, StateId};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Asserted {
    #[serde(rename = "PK")]
    pub state_id: StateId,
    #[serde(rename = "SK")]
    pub source_id: String,
}

impl From<(&Source, &State)> for Asserted {
    fn from((source, state): (&Source, &State)) -> Self {
        Asserted {
            source_id: source.id.clone(),
            state_id: state.id.clone(),
        }
    }
}

impl Asserted {
    pub fn to_watcher_item(self) -> Item {
        let edge = self.into();
        Item::Assertion(edge)
    }
}

impl Into<Item> for Asserted {
    fn into(self) -> Item {
        Item::Assertion(self.into())
    }
}

#[cfg(test)]
mod test {}
