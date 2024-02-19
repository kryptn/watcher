use serde::{Deserialize, Serialize};

#[cfg(any(test, feature = "fake"))]
use fake::{Dummy, Fake, Faker};

use crate::types::WatcherItem;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Source {
    #[serde(rename = "PK")]
    pub id: String,

    pub name: String,
    // endpoint_type: SourceType,
    // endpoint_data: Value,
    #[serde(flatten)]
    pub endpoint: SourceType,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub rate: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schedule_name: Option<String>,
}

impl Source {
    pub fn to_watcher_item(self) -> WatcherItem {
        let node = self.into();
        WatcherItem::Node(node)
    }
}

impl Into<WatcherItem> for Source {
    fn into(self) -> WatcherItem {
        WatcherItem::Node(self.into())
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "snake_case")]
pub struct Rss {
    pub url: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "snake_case")]
pub struct Http {
    pub url: String,
    pub method: String,
    pub headers: Vec<(String, String)>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timeout: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(
    rename_all = "snake_case",
    tag = "endpoint_type",
    content = "endpoint_data"
)]
pub enum SourceType {
    Rss(Rss),
    Http(Http),
}

#[cfg(test)]
mod test {}
