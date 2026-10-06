use clap::Parser;
use std::env;
use std::process::ExitCode;

mod cli;
mod config;
mod error;
mod expand;
mod link;
mod reporter;
mod resolve;

use cli::{Cli, Command};
use config::Config;
use reporter::Reporter;

fn main() -> ExitCode {
    let cli = Cli::parse();
    let config = match Config::load(&cli.config) {
        Ok(config) => config,
        Err(err) => {
            bunt::eprintln!("{$red+bold}Error in reading config file:{/$} {[red]}", err);
            return ExitCode::FAILURE;
        }
    };

    let mut failed = false;
    for (name, entry) in &config.deploy {
        let report = Reporter::new(name, cli.command.verb(), cli.verbose);
        let result = resolve::resolve(entry, &config.env, env::consts::OS, |var| {
            env::var(var).ok()
        })
        .and_then(|link| match cli.command {
            Command::Deploy { overwrite } => link::deploy(&link, overwrite, &report),
            Command::Undeploy => link::undeploy(&link, &report),
        });
        match result {
            Ok(()) => report.done(),
            Err(err) => {
                report.error(err);
                failed = true;
            }
        }
    }
    if failed {
        ExitCode::FAILURE
    } else {
        ExitCode::SUCCESS
    }
}
