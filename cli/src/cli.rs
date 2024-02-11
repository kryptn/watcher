use std::ffi::OsStr;
use std::ffi::OsString;
use std::path::PathBuf;

use clap::{Args, Parser, Subcommand, ValueEnum};

/// A fictional versioning CLI
#[derive(Debug, Parser)] // requires `derive` feature
#[command(name = "git")]
#[command(about = "A fictional versioning CLI", long_about = None)]
pub(crate) struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Debug, Subcommand)]
pub enum Commands {
    #[command(arg_required_else_help = true)]
    Create(CreateArgs),

    #[command(arg_required_else_help = true)]
    Delete(DeleteArgs),
}

// #[derive(Debug, Args)]
// #[command(args_conflicts_with_subcommands = true)]
// struct StashArgs {
//     #[command(subcommand)]
//     command: Option<StashCommands>,

//     #[command(flatten)]
//     push: StashPushArgs,
// }

// #[derive(Debug, Subcommand)]
// enum StashCommands {
//     Push(StashPushArgs),
//     Pop { stash: Option<String> },
//     Apply { stash: Option<String> },
// }

// #[derive(Debug, Args)]
// struct StashPushArgs {
//     #[arg(short, long)]
//     message: Option<String>,
// }

#[derive(Debug, Args)]
#[command(args_conflicts_with_subcommands = true)]
pub struct CreateArgs {
    #[command(subcommand)]
    pub command: Option<CreateCommands>,
}

#[derive(Debug, Subcommand)]
pub enum CreateCommands {
    #[command(arg_required_else_help = true)]
    Endpoint {
        #[arg(required = true)]
        name: String,
    },

    Sink {
        #[arg(required = true)]
        name: String,
    },

    Subscription {
        #[arg(required = true)]
        endpoint_id: String,
        sink_id: String,
    },

    Table {},
}

#[derive(Debug, Args)]
#[command(args_conflicts_with_subcommands = true)]
pub struct DeleteArgs {
    #[command(subcommand)]
    pub command: Option<DeleteCommands>,
}

#[derive(Debug, Subcommand)]
pub enum DeleteCommands {
    #[command(arg_required_else_help = true)]
    Endpoint {
        #[arg(required = true)]
        id: String,
    },

    Sink {
        #[arg(required = true)]
        id: String,
    },

    Subscription {
        #[arg(required = true)]
        endpoint_id: String,
        sink_id: String,
    },

    Table {},
}

// fn main() {
//     let args = Cli::parse();

//     match args.command {

//         Commands::Stash(stash) => {
//             let stash_cmd = stash.command.unwrap_or(StashCommands::Push(stash.push));
//             match stash_cmd {
//                 StashCommands::Push(push) => {
//                     println!("Pushing {push:?}");
//                 }
//                 StashCommands::Pop { stash } => {
//                     println!("Popping {stash:?}");
//                 }
//                 StashCommands::Apply { stash } => {
//                     println!("Applying {stash:?}");
//                 }
//             }
//         }
//     }

//     // Continued program logic goes here...
// }
