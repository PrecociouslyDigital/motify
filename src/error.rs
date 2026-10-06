use std::io;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ConfigError {
    #[error(transparent)]
    Io(#[from] io::Error),
    #[error(transparent)]
    Yaml(#[from] serde_saphyr::Error),
}

pub type ExpandError = shellexpand::LookupError<std::convert::Infallible>;

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
    #[error("Target location does not correspond to source!")]
    TargetMismatch,
    #[error(transparent)]
    Io(#[from] io::Error),
}
