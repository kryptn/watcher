

pub trait Source {
    type Item;

    fn fetch(&self) -> Result<(), Box<dyn std::error::Error>>;
}

pub struct Rss {
    url: String,
}

impl Source for Rss {
    type Item = String;

    fn fetch(&self) -> Result<(), Box<dyn std::error::Error>> {
        println!("fetching rss from {}", self.url);
        Ok(())
    }
}