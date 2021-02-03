use yaml_rust::{YamlLoader, Yaml, ScanError};
use std::{env, io, fs};
use bunt::println;
use std::error::Error;
use shellexpand::{full_with_context, LookupError};
use home::home_dir;
use std::env::VarError;
use std::borrow::Cow;

#[derive(Debug)]
enum ConfigError {
    Yaml(ScanError),
    Io(io::Error),
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

pub(crate) fn getOSString(obj: &Yaml) -> Option<&str> {
    return if obj.as_str().is_some() {
        obj.as_str()
    } else {
        obj[env::consts::OS].as_str()
    }
}

pub(crate) fn expand(baseString: &str, vars: &Yaml) -> Result<String, LookupError<VarError>> {
    let result = full_with_context(
        baseString,
        home_dir,
        |var: &str| -> Result<Option<String>, VarError>{
            return if vars[var].is_badvalue() {
                Ok(env::var(var).ok())
            } else {
                Ok(getOSString(&vars[var]).map(String::from))
            }
        }
    );
    result.map(|cow| {String::from(cow)})
}

pub(crate) fn readYAML<F: FnOnce(&Yaml)>(obj: &String, workFun: F) {
    match || -> Result<Vec<Yaml>, ConfigError> {
        let fileString = &fs::read_to_string(obj)?;
        return Ok(YamlLoader::load_from_str(fileString)?);
    }() {
        Err(err) => {
            println!("{$red+bold}Error in reading config file:{/$} {[red]}", err.to_string());
        }
        Ok(docs) => {
            workFun(&docs[0]);
        }
    }
}