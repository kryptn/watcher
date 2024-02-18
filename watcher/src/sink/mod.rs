use crate::types::{node, SinkSignalCreated};

pub mod discord;

#[allow(async_fn_in_trait)]
pub trait Sink {
    async fn send(&self, payload: &impl Render) -> Result<(), String>;
}

pub trait Render {
    fn render(&self) -> String;
}

pub async fn send_signal<T: Render>(sink: &impl Sink, signal: &impl Render) -> Result<(), String> {
    sink.send(signal).await?;
    Ok(())
}
