use indexmap::IndexMap;
use serde::Deserialize;
use serde::de::{self, Deserializer, Visitor};
use std::collections::HashMap;
use std::fs;
use std::path::Path;

use crate::error::ConfigError;

/// The contents of a `motify.yaml` file.
#[derive(Debug, Deserialize)]
pub struct Config {
    /// Links to manage, in the order they appear in the file.
    pub deploy: IndexMap<String, Entry>,
    /// Variables available to `$var` expansion, taking precedence over the process environment.
    #[serde(default)]
    pub env: HashMap<String, PerOs>,
}

/// One named link: `target` becomes a symlink pointing at `source`.
#[derive(Debug, Deserialize)]
pub struct Entry {
    pub source: PerOs,
    pub target: PerOs,
}

/// A value that is either the same everywhere or chosen by operating system
/// (keyed by [`std::env::consts::OS`], e.g. `linux`, `windows`, `macos`).
#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum PerOs {
    Any(Scalar),
    ByOs(HashMap<String, Scalar>),
}

impl PerOs {
    pub fn resolve(&self, os: &str) -> Option<&str> {
        match self {
            PerOs::Any(value) => Some(&value.0),
            PerOs::ByOs(values) => values.get(os).map(|value| value.0.as_str()),
        }
    }
}

/// A YAML scalar read as text, so `n: 1` or `flag: true` can be used as variables.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Scalar(pub String);

impl<'de> Deserialize<'de> for Scalar {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        struct ScalarVisitor;

        impl Visitor<'_> for ScalarVisitor {
            type Value = Scalar;

            fn expecting(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                f.write_str("a string, number or boolean")
            }

            fn visit_str<E: de::Error>(self, v: &str) -> Result<Scalar, E> {
                Ok(Scalar(v.to_owned()))
            }

            fn visit_bool<E: de::Error>(self, v: bool) -> Result<Scalar, E> {
                Ok(Scalar(v.to_string()))
            }

            fn visit_i64<E: de::Error>(self, v: i64) -> Result<Scalar, E> {
                Ok(Scalar(v.to_string()))
            }

            fn visit_u64<E: de::Error>(self, v: u64) -> Result<Scalar, E> {
                Ok(Scalar(v.to_string()))
            }

            fn visit_f64<E: de::Error>(self, v: f64) -> Result<Scalar, E> {
                Ok(Scalar(v.to_string()))
            }
        }

        deserializer.deserialize_any(ScalarVisitor)
    }
}

impl Config {
    pub fn load(path: &Path) -> Result<Self, ConfigError> {
        Self::parse(&fs::read_to_string(path)?)
    }

    pub fn parse(yaml: &str) -> Result<Self, ConfigError> {
        Ok(serde_saphyr::from_str(yaml)?)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn per_os_resolves_plain_and_keyed_values() {
        let config = Config::parse(
            "deploy:\n  a:\n    source: ./src\n    target:\n      linux: ./lin\n      windows: ./win\n",
        )
        .unwrap();
        let entry = &config.deploy["a"];
        assert_eq!(entry.source.resolve("linux"), Some("./src"));
        assert_eq!(entry.source.resolve("macos"), Some("./src"));
        assert_eq!(entry.target.resolve("windows"), Some("./win"));
        assert_eq!(entry.target.resolve("macos"), None);
        assert!(config.env.is_empty());
    }

    #[test]
    fn non_string_scalars_are_text() {
        let config =
            Config::parse("deploy: {}\nenv:\n  n: 1\n  flag: true\n  per_os:\n    linux: 2\n")
                .unwrap();
        assert_eq!(config.env["n"].resolve("linux"), Some("1"));
        assert_eq!(config.env["flag"].resolve("linux"), Some("true"));
        assert_eq!(config.env["per_os"].resolve("linux"), Some("2"));
    }

    #[test]
    fn deploy_entries_keep_file_order() {
        let config = Config::parse(
            "deploy:\n  zeta: {source: a, target: b}\n  alpha: {source: a, target: b}\n  mid: {source: a, target: b}\n",
        )
        .unwrap();
        let names: Vec<_> = config.deploy.keys().map(String::as_str).collect();
        assert_eq!(names, ["zeta", "alpha", "mid"]);
    }

    #[test]
    fn malformed_configs_are_errors() {
        assert!(Config::parse("").is_err());
        assert!(Config::parse("env: {}\n").is_err());
        assert!(Config::parse("deploy:\n  a:\n    source: x\n").is_err());
        assert!(Config::parse("deploy: [").is_err());
    }
}
