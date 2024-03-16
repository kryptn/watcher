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
    app: Application,
    commands: Vec<&Command>,
) -> Result<(), Box<dyn std::error::Error + Sync + Send>> {
    for command in commands {
        match command {
            Command::ObserveSource { source_id } => {
                todo!();
            }
            Command::SendSignal { signal_id, sink_id } => {
                todo!();
            }
        }
    }
    Ok(())
}
