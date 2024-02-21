use serde::{Deserialize, Serialize};

use crate::types::{Source, State, StateId, WatcherItem};

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
    pub fn to_watcher_item(self) -> WatcherItem {
        let edge = self.into();
        WatcherItem::Edge(edge)
    }
}

impl Into<WatcherItem> for Witnessed {
    fn into(self) -> WatcherItem {
        WatcherItem::Edge(self.into())
    }
}

#[cfg(test)]
mod test {}
