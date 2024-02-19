use serde::{Deserialize, Serialize};

#[cfg(any(test, feature = "fake"))]
use fake::{Dummy, Fake, Faker};

use crate::types::{Source, State, StateId, WatcherItem};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Asserted {
    #[serde(rename = "PK")]
    pub source_id: String,
    #[serde(rename = "SK")]
    pub state_id: StateId,
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
    pub fn to_watcher_item(self) -> WatcherItem {
        let edge = self.into();
        WatcherItem::Edge(edge)
    }
}

impl Into<WatcherItem> for Asserted {
    fn into(self) -> WatcherItem {
        WatcherItem::Edge(self.into())
    }
}

#[cfg(test)]
mod test {}
