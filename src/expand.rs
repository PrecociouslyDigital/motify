use std::collections::HashMap;

use crate::config::PerOs;
use crate::error::ExpandError;

/// Expands `~` and `$var`/`${var}` in `input`.
///
/// Variables are looked up in the config's `env:` section first, then via `env_lookup`
/// (normally the process environment). A variable found in neither is an error, as is one
/// whose `env:` entry has no value for `os`.
pub fn expand(
    input: &str,
    vars: &HashMap<String, PerOs>,
    os: &str,
    env_lookup: impl Fn(&str) -> Option<String>,
) -> Result<String, ExpandError> {
    let lookup = |name: &str| match vars.get(name) {
        Some(value) => value
            .resolve(os)
            .map(|value| Some(value.to_owned()))
            .ok_or_else(|| ExpandError::MissingForOs {
                var: name.to_owned(),
                os: os.to_owned(),
            }),
        None => env_lookup(name)
            .map(Some)
            .ok_or_else(|| ExpandError::Undefined(name.to_owned())),
    };
    shellexpand::full_with_context(input, home_dir, lookup)
        .map(|expanded| expanded.into_owned())
        .map_err(|err| err.cause)
}

fn home_dir() -> Option<String> {
    home::home_dir().and_then(|path| path.to_str().map(str::to_owned))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::Scalar;

    fn vars() -> HashMap<String, PerOs> {
        HashMap::from([
            ("name".to_owned(), PerOs::Any(Scalar("config".to_owned()))),
            (
                "os_only".to_owned(),
                PerOs::ByOs(HashMap::from([(
                    "linux".to_owned(),
                    Scalar("lin".to_owned()),
                )])),
            ),
        ])
    }

    fn env(name: &str) -> Option<String> {
        match name {
            "name" | "from_env" => Some("env".to_owned()),
            _ => None,
        }
    }

    #[test]
    fn config_vars_beat_environment() {
        assert_eq!(
            expand("$name/${from_env}", &vars(), "linux", env).unwrap(),
            "config/env"
        );
    }

    #[test]
    fn per_os_vars_resolve_for_current_os() {
        assert_eq!(
            expand("./$os_only", &vars(), "linux", env).unwrap(),
            "./lin"
        );
    }

    #[test]
    fn unknown_vars_are_errors() {
        assert_eq!(
            expand("./$nope", &vars(), "linux", env)
                .unwrap_err()
                .to_string(),
            "variable nope is not defined"
        );
        assert_eq!(
            expand("./$os_only", &vars(), "macos", env)
                .unwrap_err()
                .to_string(),
            "variable os_only has no value for macos"
        );
    }
}
