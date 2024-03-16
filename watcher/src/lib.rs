pub mod application;
pub mod config;
pub mod ext;
pub mod messaging;
pub mod meta_repo;
pub mod repository;
pub mod scheduling;
pub mod storage;
pub mod types;

use application::Application;
use types::Command;

pub async fn handle(
    _app: Application,
    commands: Vec<&Command>,
) -> Result<(), Box<dyn std::error::Error + Sync + Send>> {
    for command in commands {
        match command {
            Command::ObserveSource { source_id: _ } => todo!(),
            Command::SendSignal {
                signal_id: _,
                sink_id: _,
            } => todo!(),
            Command::Subscribe {
                source_id: _,
                sink_id: _,
            } => todo!(),
            Command::Unsubscribe {
                source_id: _,
                sink_id: _,
            } => todo!(),
            Command::AddSource { name: _, config: _ } => todo!(),
            Command::DeleteSource { source_id: _ } => todo!(),
            Command::AddSink { name: _, config: _ } => todo!(),
            Command::DeleteSink { sink_id: _ } => todo!(),
        }
    }
    Ok(())
}
