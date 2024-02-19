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

    #[arg(short, long, global = true)]
    pub file: Option<PathBuf>,
}

#[derive(Debug, Subcommand)]
pub enum Commands {
    Get {
        id: String,
    },

    #[command(arg_required_else_help = true)]
    Create(CreateArgs),

    #[command(arg_required_else_help = true)]
    Delete(DeleteArgs),

    // GenerateData {
    //     endpoint_count: u32,
    //     sink_count: u32,
    //     connectivity: u32,
    // },
    GetSinksForSource {
        source_id: String,
    },

    CreateSchedule {
        source_id: String,
        function_name: String,
        region: String,
        account_id: String,
        role_arn: String,
    },
    DeleteSchedule {
        source_id: String,
    },

    SendEvent {},
    // #[command(arg_required_else_help = true)]
    // Fake(FakeDataArgs),
}

// #[derive(Debug, Args)]
// #[command(args_conflicts_with_subcommands = true)]
// pub struct FakeDataArgs {
//     #[command(subcommand)]
//     pub command: FakeDataCommands,
// }

// #[derive(Debug, Subcommand)]
// pub enum FakeDataCommands {
//     #[command(arg_required_else_help = true)]
//     Source {},

//     Sink {},

//     Subscription {},

//     Signal {},

//     Event {
//         sink_id: String,
//         signal_id: String,
//     },
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
    Source {
        #[arg(required = true)]
        name: String,
    },

    Sink {
        #[arg(required = true)]
        name: String,
    },

    Subscription {
        #[arg(required = true)]
        source_id: String,
        sink_id: String,
    },

    Signal {},

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
    Source {
        #[arg(required = true)]
        id: String,
    },

    Sink {
        #[arg(required = true)]
        id: String,
    },

    Subscription {
        #[arg(required = true)]
        source_id: String,
        sink_id: String,
    },

    Table {},
}
