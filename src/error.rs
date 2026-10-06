use std::io;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ConfigError {
    #[error(transparent)]
    Io(#[from] io::Error),
    #[error(transparent)]
    Yaml(#[from] serde_saphyr::Error),
}

#[derive(Debug, Error)]
pub enum ExpandError {
    #[error("variable {0} is not defined")]
    Undefined(String),
    #[error("variable {var} has no value for {os}")]
    MissingForOs { var: String, os: String },
}

/// Why a single deploy entry failed.
#[derive(Debug, Error)]
pub enum EntryError {
    #[error("no {field} configured for {os}")]
    MissingForOs {
        field: &'static str,
        os: &'static str,
    },
    #[error(transparent)]
    Expand(#[from] ExpandError),
    #[error("Target location already exists!")]
    TargetExists,
    #[error("Target location already exists and is not a symlink!")]
    TargetNotSymlink,
    #[error("Target location does not correspond to source!")]
    TargetMismatch,
    #[error(transparent)]
    Io(#[from] io::Error),
}
