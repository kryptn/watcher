use serde::{Deserialize, Serialize};

#[cfg(any(test, feature = "fake"))]
use fake::{faker::name::raw::*, locales::*, Dummy, Fake, Faker};

use crate::types::{Broadcast, Sink, WatcherItem};

#[derive(Serialize, Deserialize, Clone, Debug)]
#[cfg_attr(any(test, feature = "fake"), derive(PartialEq, Dummy))]
pub struct Sent {
    #[serde(rename = "PK")]
    pub broadcast_id: String,
    #[serde(rename = "SK")]
    pub sink_id: String,
}

impl From<(&Broadcast, &Sink)> for Sent {
    fn from((broadcast, sink): (&Broadcast, &Sink)) -> Self {
        Sent {
            broadcast_id: broadcast.id.clone(),
            sink_id: sink.id.clone(),
        }
    }
}

impl Sent {
    pub fn to_watcher_item(self) -> WatcherItem {
        let edge = self.into();
        WatcherItem::Edge(edge)
    }
}

#[cfg(test)]
mod test {
    use crate::types::sink;

    use super::*;
    use serde_json::json;

    use fake::{Fake, Faker};

    #[test]
    fn test_sent_serialization() {
        let sent = Sent {
            broadcast_id: "broadcast_id".to_string(),
            sink_id: "sink_id".to_string(),
        };

        let expected = json!({
            "PK": "broadcast_id",
            "SK": "sink_id",
        });

        let serialized = serde_json::to_value(&sent).unwrap();
        assert_eq!(serialized, expected);
    }

    #[test]
    fn test_sent_deserialization() {
        let expected = Sent {
            broadcast_id: "broadcast_id".to_string(),
            sink_id: "sink_id".to_string(),
        };

        let deserialized: Sent = serde_json::from_value(json!({
            "PK": "broadcast_id",
            "SK": "sink_id",
        }))
        .unwrap();
        assert_eq!(deserialized, expected);
    }

    #[test]
    fn test_sent_from_broadcast_sink() {
        let broadcast_id = "Broadcast:TestId".to_string();
        let sink_id = "Sink:TestId".to_string();

        let mut broadcast = Faker.fake::<Broadcast>();
        broadcast.id = broadcast_id.clone();
        let mut sink = Faker.fake::<sink::Sink>();
        sink.id = sink_id.clone();

        let expected = Sent {
            broadcast_id,
            sink_id,
        };

        let sent: Sent = (&broadcast, &sink).into();
        assert_eq!(sent, expected);
    }
}
