use serde::{Deserialize, Deserializer, Serialize, Serializer};

use serde_json::to_value;

use crate::types::WatcherItem;

#[derive(Clone, Debug)]
pub struct StateId {
    source_id: String,
    created_at: Option<chrono::DateTime<chrono::Utc>>,
    latest: Option<bool>,
}

impl Serialize for StateId {
    fn serialize<S>(&self, s: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        if let Some(true) = self.latest {
            let id = format!("State:{}:{}", self.source_id, "latest");
            return s.serialize_str(&id);
        }

        let ts = to_value(self.created_at).unwrap();
        let ts = ts.as_str().unwrap();

        let id = format!("State:{}:{}", self.source_id, ts);
        s.serialize_str(&id)
    }
}

impl<'de> Deserialize<'de> for StateId {
    fn deserialize<D>(deserializer: D) -> Result<StateId, D::Error>
    where
        D: Deserializer<'de>,
    {
        let id = String::deserialize(deserializer)?;
        let parts: Vec<&str> = id.splitn(4, ':').collect();
        let source_id = format!("Source:{}", parts[2].to_string());

        let part_3 = parts[3].to_string();

        if part_3 == "latest" {
            return Ok(StateId {
                source_id,
                created_at: None,
                latest: Some(true),
            });
        }

        let created_at = Some(part_3.parse().unwrap());

        Ok(StateId {
            source_id,
            created_at,
            latest: None,
        })
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct State {
    #[serde(rename = "PK")]
    pub id: StateId,
    created_at: chrono::DateTime<chrono::Utc>,
    #[serde(skip_serializing_if = "Option::is_none")]
    s3_key: Option<String>,
    headers: Vec<(String, String)>,
    status_code: u8,
    #[serde(skip_serializing_if = "Option::is_none")]
    ttl: Option<u64>,
}

impl State {
    pub fn to_watcher_item(self) -> WatcherItem {
        let node = self.into();
        WatcherItem::Node(node)
    }
}

impl Into<WatcherItem> for State {
    fn into(self) -> WatcherItem {
        WatcherItem::Node(self.into())
    }
}

#[cfg(test)]
mod test {}
