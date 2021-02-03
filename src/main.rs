use clap::{Clap, AppSettings};
use yaml_rust::{Yaml};

mod reporter;
mod symlink;
mod config;

use std::{fs, io, error, env};
use::strum_macros::AsRefStr;


/// motify is a tool to help you manage symlinks declaratively.
/// it was designed primarily to backup and deploy configuration files.
#[derive(Clap)]
#[clap(version = "1.0", author = "PrecociouslyDigital <skye@hyphen-emdash.com>", setting = AppSettings::SubcommandRequiredElseHelp)]
struct Opts {
    /// Sets a custom config file.
    #[clap(short, long, default_value = "motify.yaml")]
    config: String,
    /// Output verbose output
    #[clap(short, long, parse(from_occurrences))]
    verbose: i32,
    #[clap(subcommand)]
    subcmd: SubCommand
}

#[derive(Clap)]
#[derive(AsRefStr)]
enum SubCommand {
    Deploy(Deploy),
    Undeploy(Undeploy)
}

/// Deploy symlinks according to a motive.yaml file.
#[derive(Clap)]
struct Deploy {
    /// Overwrite destination even if it exists
    #[clap(short)]
    overwrite: bool
}


/// remove symlinks according to a motive.yaml file.
#[derive(Clap)]
struct Undeploy {
}

fn main() {
    let opts: Opts = Opts::parse();
    println!("{}", env::var("TESTVAR").unwrap());
    config::readYAML(&opts.config, |doc : &Yaml| {
        let config =  &doc["deploy"];
        let vars = &doc["env"];
        for (name, target) in config.as_hash().unwrap() {
            let progress = reporter::Reporter {
                name: String::from(name.as_str().unwrap()),
                verbose: opts.verbose,
                verb: String::from(opts.subcmd.as_ref())
            };
            let try_block = || -> Result<(), Box<error::Error>> {
                let source = String::from(config::getOSString(&target["source"]).and_then(|target| {
                    config::expand(target, vars).ok()
                }).unwrap());
                let target = String::from(config::getOSString(&target["target"])
                    .and_then(|target| {
                        config::expand(target, vars).ok()
                    }).unwrap());
                return match &opts.subcmd {
                    SubCommand::Deploy(config) => {
                        symlink::symlink(&source, &target, &config, &progress)
                    }
                    SubCommand::Undeploy(_config) => {
                        symlink::unsymlink(&source, &target, &progress)
                    }
                };
            };
            if let Err(_err) = try_block() {
                progress.error(&_err.to_string());
            }else{
                progress.done();
            }

        }
    });
}


