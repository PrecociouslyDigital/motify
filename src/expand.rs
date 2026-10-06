use std::collections::HashMap;

use crate::config::PerOs;
use crate::error::ExpandError;

/// Expands `~` and `$var`/`${var}` in `input`.
///
/// Variables are looked up in the config's `env:` section first, then via `env_lookup`
/// (normally the process environment). Unknown variables are left as written.
pub fn expand(
    input: &str,
    vars: &HashMap<String, PerOs>,
    os: &str,
    env_lookup: impl Fn(&str) -> Option<String>,
) -> Result<String, ExpandError> {
    let expanded = shellexpand::full_with_context(input, home_dir, |name| {
        Ok::<_, std::convert::Infallible>(match vars.get(name) {
            Some(value) => value.resolve(os).map(str::to_owned),
            None => env_lookup(name),
        })
    })?;
    Ok(expanded.into_owned())
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
    fn unknown_vars_are_left_as_written() {
        assert_eq!(expand("./$nope", &vars(), "linux", env).unwrap(), "./$nope");
        assert_eq!(
            expand("./$os_only", &vars(), "macos", env).unwrap(),
            "./$os_only"
        );
    }
}
