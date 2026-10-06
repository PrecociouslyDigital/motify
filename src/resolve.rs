use std::collections::HashMap;

use crate::config::{Entry, PerOs};
use crate::error::EntryError;
use crate::expand::expand;

/// A config entry with OS choices and variables resolved: `target` should be a symlink to `source`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Link {
    pub source: String,
    pub target: String,
}

pub fn resolve(
    entry: &Entry,
    vars: &HashMap<String, PerOs>,
    os: &'static str,
    env_lookup: impl Fn(&str) -> Option<String>,
) -> Result<Link, EntryError> {
    let field = |field: &'static str, value: &PerOs| -> Result<String, EntryError> {
        let raw = value
            .resolve(os)
            .ok_or(EntryError::MissingForOs { field, os })?;
        Ok(expand(raw, vars, os, &env_lookup)?)
    };
    Ok(Link {
        source: field("source", &entry.source)?,
        target: field("target", &entry.target)?,
    })
}
