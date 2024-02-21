use std::{ffi::OsString, path::PathBuf};

use clap::Parser;
use dynamodb::config::endpoint;
use serde_dynamo::{to_item, Item};

use watcher::{
    config::{self, Config},
    messaging::SqsProvider,
    meta_repo,
    repository::Repository,
    scheduling::{self, create_schedule},
    types::{Event, Item, Signal, Sink, SinkSignalCreated, Source, State, Subscription},
};

use aws_sdk_dynamodb as dynamodb;

mod cli;

fn readfile<T: serde::de::DeserializeOwned>(
    filename: PathBuf,
) -> Result<T, Box<dyn std::error::Error>> {
    let f = std::fs::File::open(filename)?;
    let config_data: T = serde_json::from_reader(f)?;
    Ok(config_data)
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = cli::Cli::parse();
    let config = config::init();

    let table_name = config.table_name.unwrap();
    let sqs_queue_url = config.sqs_queue_url.unwrap();

    let aws_config = {
        let mut c = aws_config::from_env();
        if let Some(endpoint) = config.endpoint {
            c = c.endpoint_url(endpoint);
        }
        c.load().await
    };

    let client = aws_sdk_dynamodb::Client::new(&aws_config);
    let repo = Repository::new(table_name.to_string(), client.clone());
    let metarepo = meta_repo::Repository::new(table_name.to_string(), client);
    let queue = SqsProvider::new(sqs_queue_url).await;

    match cli.command {
        cli::Commands::Get { id } => {
            let item: Item = repo.get_item(&id, &id).await?;
            println!("{:?}", item);
        }
        cli::Commands::Create(cmd) => {
            let cmd = cmd.command.unwrap();
            match cmd {
                cli::CreateCommands::Source { name } => {
                    println!("create source -> {}", name);
                    let source = readfile::<Source>(cli.file.unwrap())?;
                    let item: Item = source.into();
                    repo.put_item(item).await?;
                }
                cli::CreateCommands::Sink { name } => {
                    println!("create sink -> {}", name);
                    let sink = readfile::<Sink>(cli.file.unwrap())?;
                    let item: Item = sink.into();
                    repo.put_item(item).await?;
                }
                cli::CreateCommands::Subscription { source_id, sink_id } => {
                    println!("create subscription -> {}, {}", source_id, sink_id);
                    let subscription = readfile::<Subscription>(cli.file.unwrap())?;
                    let item: Item = subscription.into();
                    repo.put_item(item).await?;
                }
                cli::CreateCommands::Table {} => {
                    println!("create table");
                    metarepo.create_table().await?;
                }
                cli::CreateCommands::Signal {} => {
                    println!("create signal");
                    let signal = readfile::<Signal>(cli.file.unwrap())?;
                    let item: Item = signal.into();
                    repo.put_item(item).await?;
                }
            }
        }
        cli::Commands::Delete(cmd) => {
            let cmd = cmd.command.unwrap();
            match cmd {
                cli::DeleteCommands::Source { id } => {
                    println!("delete endpoint -> {}", id);
                    let source = readfile::<Source>(cli.file.unwrap())?;
                    let source_id = source.id;
                }
                cli::DeleteCommands::Sink { id } => {
                    println!("delete sink -> {}", id);
                }
                cli::DeleteCommands::Subscription { source_id, sink_id } => {
                    println!("delete subscription -> {}, {}", source_id, sink_id);
                }
                cli::DeleteCommands::Table {} => {
                    println!("delete table");
                    metarepo.delete_table().await?;
                }
            }
        }
        cli::Commands::CreateSchedule {
            source_id,
            function_name,
            region,
            account_id,
            role_arn,
        } => {
            let target_config = watcher::scheduling::TargetConfig {
                function_name,
                region,
                account_id,
                role_arn,
            };
            let client = scheduling::new().await;
            let schedule_name = format!("schedule-{}", &source_id.replace(":", "-"));
            let input = watcher::types::SourceSchedule {
                source_id: source_id.clone(),
            };

            scheduling::create_schedule(&client, &schedule_name, target_config, &input).await?;

            println!("created schedule {}", schedule_name);
        }
        cli::Commands::DeleteSchedule { source_id } => {
            let client = scheduling::new().await;

            let endpoint: Source = repo.get_item(&source_id, &source_id).await?;

            if let Some(schedule_name) = endpoint.schedule_name {
                scheduling::delete_schedule(&client, &schedule_name).await?;
                println!("deleted schedule {}", schedule_name);
                repo.remove::<Source>(&source_id, &source_id, &["schedule_name"])
                    .await?;
            }
        }
        cli::Commands::GetSinksForSource { source_id } => todo!(),

        cli::Commands::SendEvent {} => {
            let event = readfile::<Event>(cli.file.unwrap())?;
            queue.send(event).await?;
        }
    }

    Ok(())
}
