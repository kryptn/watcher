use aws_sdk_s3::primitives::SdkBody;
use tokio::io::BufReader;

use crate::storage::Storable;

pub struct State {
    pub headers: Vec<(String, String)>,
    pub body: Option<feed_rs::model::Feed>,
    pub status: u16,
}

impl State {
    async fn from_response(
        response: reqwest::Response,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        let headers = response
            .headers()
            .iter()
            .map(|(k, v)| (k.as_str().to_string(), v.to_str().unwrap().to_string()))
            .collect();
        let status = response.status().as_u16();

        let body = response.bytes().await.ok().and_then(|bytes| {
            let reader = BufReader::new(&bytes[..]);
            match feed_rs::parser::parse(reader.buffer()) {
                Ok(feed) => Some(feed),
                Err(e) => {
                    println!("Error parsing feed: {:?}", e);
                    None
                }
            }
        });

        Ok(State {
            headers,
            body,
            status,
        })
    }
}

impl Storable for State {
    fn manifest(&self) -> Vec<(String, impl Into<SdkBody>)> {
        let mut manifest = self
            .headers
            .iter()
            .map(|(k, v)| (k.clone(), v.clone()))
            .collect::<Vec<_>>();

        if let Some(feed) = &self.body {
            manifest.push((
                "feed.json".to_string(),
                serde_json::to_string(feed).unwrap(),
            ));
        }

        manifest
    }
}
