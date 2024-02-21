use serde::{de::DeserializeOwned, Deserialize, Serialize};

pub mod discord;
pub mod rss;

pub trait Source {
    type Metadata: Serialize + DeserializeOwned;
    type Output: Serialize + DeserializeOwned;

    async fn fetch<T>(
        &self,
        config: impl Into<reqwest::RequestBuilder>,
    ) -> Result<(Self::Output, Self::Metadata), Box<dyn std::error::Error>>;
}

pub trait State {
    type Output;

    async fn key_prefix(&self) -> String;
    async fn put(
        &self,
        filename: String,
        data: Self::Output,
    ) -> Result<(), Box<dyn std::error::Error>>;

    async fn get(&self, filename: String) -> Result<Self::Output, Box<dyn std::error::Error>>;
    async fn metadata(&self) -> Result<Self::Output, Box<dyn std::error::Error>>;
}

pub trait Signal {
    type Output;

    async fn derive<T>(&self, other: &T) -> Result<Self::Output, Box<dyn std::error::Error>>;
}
