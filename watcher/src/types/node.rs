use serde::{Deserialize, Serialize};

// use super::{endpoint::Endpoint, Broadcast, Observation, Sink};

use crate::types;

#[derive(Serialize, Deserialize)]
#[cfg_attr(test, derive(Debug, PartialEq))]
#[serde(rename_all = "snake_case", tag = "node_type")]
pub enum Node {
    Endpoint(types::Endpoint),
    Observation(types::Observation),
    Sink(types::Sink),
    Broadcast(types::Broadcast),
}

#[cfg(test)]
mod test {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_node_serialization() {
        let node = Node::Endpoint(types::Endpoint {
            id: "id".to_string(),
            sk: "sk".to_string(),
            name: "name".to_string(),
            endpoint: types::EndpointType::Rss(types::Rss {
                url: "url".to_string(),
            }),
            rate: Some("rate".to_string()),
            schedule_name: Some("schedule_name".to_string()),
        });

        let expected = json!({
            "PK": "id",
            "SK": "sk",
            "node_type": "endpoint",
            "name": "name",
            "endpoint_type": "rss",
            "endpoint_data": {
                "url": "url"
            },
            "rate": "rate",
            "schedule_name": "schedule_name"
        });

        let serialized = serde_json::to_value(&node).unwrap();
        assert_eq!(serialized, expected);
    }

    #[test]
    fn test_node_deserialization() {
        let json = json!({
            "PK": "id",
            "SK": "sk",
            "node_type": "endpoint",
            "name": "name",
            "endpoint_type": "rss",
            "endpoint_data": {
                "url": "url"
            },
            "rate": "rate",
            "schedule_name": "schedule_name"
        });

        let deserialized: Node = serde_json::from_value(json).unwrap();
        assert_eq!(
            deserialized,
            Node::Endpoint(types::Endpoint {
                id: "id".to_string(),
                sk: "sk".to_string(),
                name: "name".to_string(),
                endpoint: types::EndpointType::Rss(types::Rss {
                    url: "url".to_string(),
                }),
                rate: Some("rate".to_string()),
                schedule_name: Some("schedule_name".to_string()),
            })
        );
    }
}
