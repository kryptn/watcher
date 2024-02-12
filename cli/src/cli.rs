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
    Get {
        id: String,
    },

    #[command(arg_required_else_help = true)]
    Create(CreateArgs),

    #[command(arg_required_else_help = true)]
    Delete(DeleteArgs),

    GenerateData {
        endpoint_count: u32,
        sink_count: u32,
        connectivity: u32,
    },

    GetSinksForEndpoint {
        endpoint_id: String,
    },

    CreateSchedule {
        endpoint_id: String,
        function_name: String,
        region: String,
        account_id: String,
    },
    DeleteSchedule {
        schedule_name: String,
    },
}

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
