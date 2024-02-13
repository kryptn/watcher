use serde::{de::DeserializeOwned, Deserialize, Serialize};

pub trait Source<'a> {
    type Item;
    type Metadata: Serialize + Deserialize<'a>;

    async fn fetch(&self, metadata: &Self::Metadata) -> Result<(), Box<dyn std::error::Error>>;
}

#[derive(Serialize, Deserialize)]
pub struct RssMetadata {
    url: String,
}

pub struct Rss {
    client: reqwest::Client,
}

impl Source<'_> for Rss {
    type Item = String;
    type Metadata = RssMetadata;

    async fn fetch(&self, metadata: &Self::Metadata) -> Result<(), Box<dyn std::error::Error>> {
        let response = self.client.get(&metadata.url).send().await?;

        println!("fetching rss from {}", metadata.url);
        Ok(())
    }
}
