use serde::{de::DeserializeOwned, Deserialize, Serialize};

use crate::{
    ext,
    types::{Item, Source, State, StateId},
};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Witnessed {
    #[serde(rename = "PK")]
    pub state_id: StateId,
    #[serde(rename = "SK")]
    pub source_id: String,
}

impl From<(&Source, &State)> for Witnessed {
    fn from((source, state): (&Source, &State)) -> Self {
        Witnessed {
            source_id: source.id.clone(),
            state_id: state.id.clone(),
        }
    }
}

impl Witnessed {
    pub fn to_watcher_item(self) -> Item {
        let edge = self.into();
        Item::Witness(edge)
    }
}

impl Into<Item> for Witnessed {
    fn into(self) -> Item {
        Item::Witness(self.into())
    }
}

#[cfg(test)]
mod test {}
