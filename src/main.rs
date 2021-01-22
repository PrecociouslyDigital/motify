use clap::{Clap, AppSettings};
use yaml_rust::{YamlLoader, Yaml, ScanError};

mod reporter;
mod symlink;
use std::{fs, env, io, error};
use bunt::println;




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


#[derive(Debug)]
enum ConfigError {
    Yaml(ScanError),
    Io(io::Error)
}
impl From<io::Error> for ConfigError {
    fn from(err: io::Error) -> ConfigError {
        ConfigError::Io(err)
    }
}

impl From<ScanError> for ConfigError {
    fn from(err: ScanError) -> ConfigError {
        ConfigError::Yaml(err)
    }
}
impl ToString for ConfigError {
    fn to_string(&self) -> String {
        match self{
            ConfigError::Yaml(scan) => {
                scan.to_string()
            }
            ConfigError::Io(io) => {
                io.to_string()
            }
        }
    }
}


fn main() {
    let opts: Opts = Opts::parse();

    match || -> Result<Vec<Yaml>, ConfigError> {
        let fileString = &fs::read_to_string(&opts.config)?;
        return Ok(YamlLoader::load_from_str(fileString)?);
    }() {
        Err(err) => {
            println!("{$red+bold}Error in reading config file:{/$} {[red]}", err.to_string());
        }
        Ok(docs) => {
            let config =  &docs[0];
            for (name, target) in config.as_hash().unwrap() {
                let progress = reporter::Reporter {
                    name: String::from(name.as_str().unwrap()),
                    verbose: opts.verbose
                };
                let try_block = || -> Result<(), Box<error::Error>> {
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
    }
}


