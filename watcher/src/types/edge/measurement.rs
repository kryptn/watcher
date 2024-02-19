use serde::{Deserialize, Serialize};

#[cfg(any(test, feature = "fake"))]
use fake::{Dummy, Fake, Faker};

use crate::types::{Source, State, WatcherItem};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Measurement {
    #[serde(rename = "PK")]
    pub source_id: String,
    #[serde(rename = "SK")]
    pub state_id: String,
}

impl From<(&Source, &State)> for Measurement {
    fn from((endpoint, observation): (&Source, &State)) -> Self {
        Measurement {
            source_id: endpoint.id.clone(),
            state_id: observation.id.clone(),
        }
    }
}

impl Measurement {
    pub fn to_watcher_item(self) -> WatcherItem {
        let edge = self.into();
        WatcherItem::Edge(edge)
    }
}

impl Into<WatcherItem> for Measurement {
    fn into(self) -> WatcherItem {
        WatcherItem::Edge(self.into())
    }
}

#[cfg(test)]
mod test {}
