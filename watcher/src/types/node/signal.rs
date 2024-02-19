use chrono::prelude::*;
use chrono::serde::ts_microseconds;
#[cfg(any(test, feature = "fake"))]
use fake::{Dummy, Fake, Faker};
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use serde_json::to_value;

use crate::types::WatcherItem;

#[derive(Clone, Debug)]
pub struct SignalId {
    source_id: String,
    created_at: chrono::DateTime<chrono::Utc>,
}

impl Serialize for SignalId {
    fn serialize<S>(&self, s: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let ts = to_value(self.created_at).unwrap();
        let ts = ts.as_str().unwrap();
        let id = format!("Signal:State:{}:{}", self.source_id, ts);
        s.serialize_str(&id)
    }
}

impl<'de> Deserialize<'de> for SignalId {
    fn deserialize<D>(deserializer: D) -> Result<SignalId, D::Error>
    where
        D: Deserializer<'de>,
    {
        let id = String::deserialize(deserializer)?;
        let parts: Vec<&str> = id.splitn(4, ':').collect();
        let source_id = parts[2].to_string();
        let created_at = parts[3].parse().unwrap();

        Ok(SignalId {
            source_id,
            created_at,
        })
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Signal {
    #[serde(rename = "PK")]
    pub id: SignalId,
    #[serde(rename = "SK")]
    pub sk: String,

    pub source_id: String,

    pub created_at: chrono::DateTime<chrono::Utc>,

    // this should really be serde_json::Value
    // i haven't tried to handle that with faker yet
    pub contents: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    ttl: Option<u64>,

    #[serde(skip)]
    latest: bool,
}

impl Into<WatcherItem> for Signal {
    fn into(self) -> WatcherItem {
        WatcherItem::Node(self.into())
    }
}

#[cfg(test)]
mod test {

    use serde_json::json;

    use super::*;

    #[test]
    fn test_signal_serialize() {
        let signal = Signal {
            id: SignalId {
                source_id: "source_id".to_string(),
                created_at: chrono::Utc::now(),
            },
            sk: "sk".to_string(),
            source_id: "source_id".to_string(),
            created_at: chrono::Utc::now(),
            contents: "contents".to_string(),
            ttl: Some(60),
            latest: true,
        };

        let serialized = serde_json::to_string(&signal).unwrap();
        println!("{}", serialized);
    }

    #[test]
    fn test_signal_deserialize() {
        let serialized = json!({
            "PK": "Signal:State:source_id:2024-02-19T07:34:20.987349Z",
            "SK": "sk",
            "source_id": "source_id",
            "created_at": "2024-02-19T07:34:20.987452Z",
            "contents": "contents",
            "ttl": 60,
            "latest": true
        })
        .to_string();

        let mut d = serde_json::Deserializer::from_str(&serialized);
        let result: Result<Signal, _> = serde_path_to_error::deserialize(&mut d);
        // let out = serde_path_to_error::deserialize(d);

        println!("{:?}", result);
    }
}
