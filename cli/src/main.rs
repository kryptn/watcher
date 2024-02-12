use clap::Parser;
use serde_dynamo::{to_item, Item};

use watcher::{
    repository::Repository,
    scheduling::{self, create_schedule},
    types::{Broadcast, Endpoint, Observation, Sink, Subscription, WatcherItem},
};

use aws_sdk_dynamodb as dynamodb;

use rand::{thread_rng, Rng};

mod cli;

fn create_example_data(
    endpoints: u32,
    sinks: u32,
    conn_pct: u32,
) -> (Vec<Endpoint>, Vec<Sink>, Vec<Subscription>) {
    let mut rng = thread_rng();

    let sinks = (0..sinks).map(|_| Sink::mock()).collect::<Vec<_>>();
    let endpoints = (0..endpoints).map(|_| Endpoint::mock()).collect::<Vec<_>>();
    let subscriptions = sinks
        .iter()
        .map(|sink| {
            endpoints.iter().map(|endpoint| {
                let mut rng = rng.clone();
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

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = cli::Cli::parse();

    dbg!(&cli);

    let table_name = "AdjacencyListExplore";

    let config = aws_config::from_env()
        .endpoint_url("http://localhost:8000")
        .load()
        .await;

    let client = aws_sdk_dynamodb::Client::new(&config);
    let repo = Repository::new(table_name.to_string(), client);

    match cli.command {
        cli::Commands::Get { id } => {
            let item: WatcherItem = repo.get_item(&id, &id).await?;
            println!("{:?}", item);
        }
        cli::Commands::Create(cmd) => {
            let cmd = cmd.command.unwrap();
            match cmd {
                cli::CreateCommands::Endpoint { name } => {
                    println!("create endpoint -> {}", name);
                }
                cli::CreateCommands::Sink { name } => {
                    println!("create sink -> {}", name);
                }
                cli::CreateCommands::Subscription {
                    endpoint_id,
                    sink_id,
                } => {
                    println!("create subscription -> {}, {}", endpoint_id, sink_id);
                }
                cli::CreateCommands::Table {} => {
                    println!("create table");
                    repo.create_table().await?;
                }
            }
        }
        cli::Commands::Delete(cmd) => {
            let cmd = cmd.command.unwrap();
            match cmd {
                cli::DeleteCommands::Endpoint { id } => {
                    println!("delete endpoint -> {}", id);
                }
                cli::DeleteCommands::Sink { id } => {
                    println!("delete sink -> {}", id);
                }
                cli::DeleteCommands::Subscription {
                    endpoint_id,
                    sink_id,
                } => {
                    println!("delete subscription -> {}, {}", endpoint_id, sink_id);
                }
                cli::DeleteCommands::Table {} => {
                    println!("delete table");
                    repo.delete_table().await?;
                }
            }
        }
        cli::Commands::GenerateData {
            endpoint_count,
            sink_count,
            connectivity,
        } => {
            let (endpoints, sinks, subscriptions) =
                create_example_data(endpoint_count, sink_count, connectivity);
            for endpoint in endpoints.iter() {
                repo.create_endpoint(endpoint).await?;
            }
            for sink in sinks.iter() {
                repo.create_sink(sink).await?;
            }
            for subscription in subscriptions.iter() {
                repo.create_subscription(subscription).await?;
            }
        }
        cli::Commands::GetSinksForEndpoint { endpoint_id } => {
            let subs = repo.get_sinks_for_endpoint(endpoint_id).await?;
            for sub in subs {
                println!("{:?}", sub);
            }
        }
        cli::Commands::CreateSchedule {
            endpoint_id,
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
            let schedule_name = format!("schedule-{}", &endpoint_id.replace(":", "-"));
            let input = watcher::types::ScheduledObservation {
                endpoint_id: endpoint_id.clone(),
            };

            scheduling::create_schedule(&client, &schedule_name, target_config, &input).await?;
            repo.set_schedule_name_for_endpoint(&endpoint_id, &schedule_name)
                .await?;

            println!("created schedule {}", schedule_name);
        }
        cli::Commands::DeleteSchedule { endpoint_id } => {
            let client = scheduling::new().await;

            let endpoint: Endpoint = repo.get_item(&endpoint_id, &endpoint_id).await?;

            if let Some(schedule_name) = endpoint.schedule_name {
                scheduling::delete_schedule(&client, &schedule_name).await?;
                println!("deleted schedule {}", schedule_name);
                repo.remove::<Endpoint>(&endpoint_id, &endpoint_id, &["schedule_name"])
                    .await?;
            }
        }
    }

    Ok(())
}
