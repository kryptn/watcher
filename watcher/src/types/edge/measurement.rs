use serde::{Deserialize, Serialize};

use crate::types::{Item, Source, State, StateId};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Measurement {
    #[serde(rename = "PK")]
    pub state_id: StateId,
    #[serde(rename = "SK")]
    pub source_id: String,
}

impl From<(&Source, &State)> for Measurement {
    fn from((endpoint, state): (&Source, &State)) -> Self {
        Measurement {
            source_id: endpoint.id.clone(),
            state_id: state.id.clone(),
        }
    }
}

impl Measurement {
    pub fn to_watcher_item(self) -> Item {
        let edge = self.into();
        Item::Measurement(edge)
    }
}

impl Into<Item> for Measurement {
    fn into(self) -> Item {
        Item::Measurement(self.into())
    }
}

#[cfg(test)]
mod test {}
