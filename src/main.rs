use clap::{Clap, AppSettings};
use yaml_rust::{YamlLoader};

mod reporter;
mod symlink;
use std::{fs, env};
use std::error::Error;




/// motify is a tool to help you manage symlinks declaratively.
/// it was designed primarily to backup and deploy configuration files.
#[derive(Clap)]
#[clap(version = "1.0", author = "PrecociouslyDigital <skye@hyphen-emdash.com>", setting = AppSettings::SubcommandRequiredElseHelp)]
struct Opts {
    /// Sets a custom config file. Could have been an Option<T> with no default too
    #[clap(short, long, default_value = "motify.yaml")]
    config: String,
    /// Output verbose output
    #[clap(short, long, parse(from_occurrences))]
    verbose: i32,
    #[clap(subcommand)]
    subcmd: SubCommand
}

#[derive(Clap)]
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
    let docs= YamlLoader::load_from_str(&fs::read_to_string(&opts.config).unwrap()).unwrap();
    let config =  &docs[0];
    for (name, target) in config.as_hash().unwrap() {
        let progress = reporter::Reporter {
            name: String::from(name.as_str().unwrap()),
            verbose: opts.verbose
        };
        let try_block = || -> Result<(), Box<Error>> {
            let source = &String::from(target["source"].as_str().unwrap());
            let target = &String::from(target["target"][env::consts::OS].as_str().unwrap());
            match &opts.subcmd {
                SubCommand::Deploy(config) => {
                    symlink::symlink(source, target, &config, &progress)
                }
                SubCommand::Undeploy(_config) => {
                    symlink::unsymlink(source, target, &progress)
                }
            };
            return Ok(());
        };
        if let Err(_err) = try_block() {
            progress.error(&_err.to_string());
        }else{
            progress.done();
        }

    }
}


