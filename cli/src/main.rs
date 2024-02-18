use std::{ffi::OsString, path::PathBuf};

use clap::Parser;
use dynamodb::config::endpoint;
use envconfig::Envconfig;
use serde_dynamo::{to_item, Item};

use watcher::{
    meta_repo,
    repository::Repository,
    scheduling::{self, create_schedule},
    types::{Signal, Sink, Source, State, Subscription, WatcherItem},
};

use aws_sdk_dynamodb as dynamodb;

use rand::{thread_rng, Rng};

mod cli;

fn create_example_data(
    endpoints: u32,
    sinks: u32,
    conn_pct: u32,
) -> (Vec<Source>, Vec<Sink>, Vec<Subscription>) {
    let sinks = (0..sinks).map(|_| Sink::mock()).collect::<Vec<_>>();
    let endpoints = (0..endpoints).map(|_| Source::mock()).collect::<Vec<_>>();
    let subscriptions = sinks
        .iter()
        .map(|sink| {
            endpoints.iter().map(|endpoint| {
                let mut rng = thread_rng();
                let n: &u32 = &rng.gen_range(0..=100);

                if n <= &conn_pct {
                    let subscription = Subscription::new(endpoint.id.clone(), sink.id.clone());
                    Some(subscription)
                } else {
                    None
                }
            })
        })
        .flatten()
        .filter_map(|subscription| subscription)
        .collect::<Vec<_>>();

    // let items = sinks
    //     .into_iter()
    //     .map(|sink| sink.to_watcher_item())
    //     .chain(
    //         endpoints
    //             .into_iter()
    //             .map(|endpoint| endpoint.to_watcher_item()),
    //     )
    //     .chain(
    //         subscriptions
    //             .into_iter()
    //             .map(|subscription| subscription.to_watcher_item()),
    //     )
    //     .collect::<Vec<_>>();

    (endpoints, sinks, subscriptions)
}

fn readfile<T: serde::de::DeserializeOwned>(
    filename: PathBuf,
) -> Result<T, Box<dyn std::error::Error>> {
    let f = std::fs::File::open(filename)?;
    let config_data: T = serde_json::from_reader(f)?;
    Ok(config_data)
}

#[derive(Envconfig)]
struct Config {
    #[envconfig(from = "TABLE_NAME")]
    pub table_name: String,

    #[envconfig(from = "ENDPOINT_URL")]
    pub endpoint: Option<String>,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = cli::Cli::parse();
    let config = Config::init_from_env().unwrap();

    let table_name = config.table_name;

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

    match cli.command {
        cli::Commands::Get { id } => {
            let item: WatcherItem = repo.get_item(&id, &id).await?;
            println!("{:?}", item);
        }
        cli::Commands::Create(cmd) => {
            let cmd = cmd.command.unwrap();
            match cmd {
                cli::CreateCommands::Source { name } => {
                    println!("create source -> {}", name);
                    let source = readfile::<Source>(cli.file.unwrap())?;
                    let item: WatcherItem = source.into();
                    repo.put_item(item).await?;
                }
                cli::CreateCommands::Sink { name } => {
                    println!("create sink -> {}", name);
                    let sink = readfile::<Sink>(cli.file.unwrap())?;
                    let item: WatcherItem = sink.into();
                    repo.put_item(item).await?;
                }
                cli::CreateCommands::Subscription { source_id, sink_id } => {
                    println!("create subscription -> {}, {}", source_id, sink_id);
                    let subscription = readfile::<Subscription>(cli.file.unwrap())?;
                    let item: WatcherItem = subscription.into();
                    repo.put_item(item).await?;
                }
                cli::CreateCommands::Table {} => {
                    println!("create table");
                    metarepo.create_table().await?;
                }
            }
        }
        cli::Commands::Delete(cmd) => {
            let cmd = cmd.command.unwrap();
            match cmd {
                cli::DeleteCommands::Source { id } => {
                    println!("delete endpoint -> {}", id);
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
        // cli::Commands::GenerateData {
        //     endpoint_count,
        //     sink_count,
        //     connectivity,
        // } => {
        //     let (endpoints, sinks, subscriptions) =
        //         create_example_data(endpoint_count, sink_count, connectivity);
        //     for endpoint in endpoints.iter() {
        //         repo.put_item(endpoint).await?;
        //     }
        //     for sink in sinks.iter() {
        //         repo.put_item(sink).await?;
        //     }
        //     for subscription in subscriptions.iter() {
        //         repo.put_item(subscription).await?;
        //     }
        // }
        // cli::Commands::GetSinksForSource { source_id } => {
        //     let subs = repo.get_sinks_for_endpoint(source_id).await?;
        //     for sub in subs {
        //         println!("{:?}", sub);
        //     }
        // }
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
            // repo.set_schedule_name_for_endpoint(&source_id, &schedule_name)
            // .await?;

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
    }

    Ok(())
}
