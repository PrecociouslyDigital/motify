use clap::{Parser, Subcommand};
use std::path::PathBuf;

/// motify is a tool to help you manage symlinks declaratively.
/// it was designed primarily to backup and deploy configuration files.
#[derive(Debug, Parser)]
#[command(
    version,
    author = "PrecociouslyDigital <skye@hyphen-emdash.com>",
    subcommand_required = true,
    arg_required_else_help = true
)]
pub struct Cli {
    /// Sets a custom config file.
    #[arg(short, long, default_value = "motify.yaml")]
    pub config: PathBuf,
    /// Output verbose output
    #[arg(short, long, action = clap::ArgAction::Count)]
    pub verbose: u8,
    #[command(subcommand)]
    pub command: Command,
}

#[derive(Debug, Clone, Copy, Subcommand)]
pub enum Command {
    /// Deploy symlinks according to a motify.yaml file.
    Deploy {
        /// Replace the destination if it is already a symlink
        #[arg(short, long)]
        overwrite: bool,
    },
    /// Remove symlinks according to a motify.yaml file.
    Undeploy,
}

impl Command {
    /// The verb used in progress messages ("Deploying foo...").
    pub fn verb(self) -> &'static str {
        match self {
            Command::Deploy { .. } => "Deploy",
            Command::Undeploy => "Undeploy",
        }
    }
}
