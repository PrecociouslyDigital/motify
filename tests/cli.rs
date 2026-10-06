//! End-to-end tests that run the `motify` binary against a temporary directory.

use assert_cmd::Command;
use std::fs;
use std::path::{Path, PathBuf};
use tempfile::TempDir;

/// A scratch working directory plus a fake home directory.
struct Fixture {
    work: TempDir,
    home: TempDir,
}

impl Fixture {
    fn new(config: &str) -> Self {
        let fixture = Fixture {
            work: TempDir::new().unwrap(),
            home: TempDir::new().unwrap(),
        };
        fs::write(fixture.path("motify.yaml"), config).unwrap();
        fs::write(fixture.path("test.txt"), "This is a test!").unwrap();
        fs::create_dir(fixture.path("test")).unwrap();
        fs::write(fixture.path("test/uwu.txt"), "uwu").unwrap();
        fixture
    }

    fn path(&self, rel: &str) -> PathBuf {
        self.work.path().join(rel)
    }

    fn home(&self, rel: &str) -> PathBuf {
        self.home.path().join(rel)
    }

    fn cmd(&self, args: &[&str]) -> Command {
        let mut cmd = Command::cargo_bin("motify").unwrap();
        cmd.current_dir(self.work.path())
            .env("HOME", self.home.path())
            .env("USERPROFILE", self.home.path())
            .env_remove("TESTVAR")
            .args(args);
        cmd
    }

    /// Runs motify and returns (success, stdout + stderr with colour codes removed).
    fn run(&self, args: &[&str]) -> (bool, String) {
        let output = self.cmd(args).output().unwrap();
        let mut text = String::from_utf8_lossy(&output.stdout).into_owned();
        text.push_str(&String::from_utf8_lossy(&output.stderr));
        (output.status.success(), strip_ansi(&text))
    }
}

fn strip_ansi(s: &str) -> String {
    let mut out = String::with_capacity(s.len());
    let mut chars = s.chars();
    while let Some(c) = chars.next() {
        if c == '\x1b' {
            for c in chars.by_ref() {
                if c.is_ascii_alphabetic() {
                    break;
                }
            }
        } else {
            out.push(c);
        }
    }
    out
}

fn assert_links_to(link: &Path, source: &Path) {
    let meta = fs::symlink_metadata(link)
        .unwrap_or_else(|e| panic!("{} should exist: {e}", link.display()));
    assert!(
        meta.file_type().is_symlink(),
        "{} is not a symlink",
        link.display()
    );
    assert_eq!(
        fs::canonicalize(link).unwrap(),
        fs::canonicalize(source).unwrap()
    );
}

fn assert_absent(path: &Path) {
    assert!(
        fs::symlink_metadata(path).is_err(),
        "{} should not exist",
        path.display()
    );
}

const BASIC: &str = "\
deploy:
  file:
    source: ./test.txt
    target:
      windows: ./windows.$suffix.txt
      linux: ./unix.$suffix.txt
      macos: ./unix.$suffix.txt
  dir:
    source: ./$folder
    target: ~/$dirTarget
env:
  folder: test
  suffix: x
  dirTarget:
    windows: winuwu
    linux: linuwu
    macos: linuwu
";

fn file_target(f: &Fixture) -> PathBuf {
    if cfg!(windows) {
        f.path("windows.x.txt")
    } else {
        f.path("unix.x.txt")
    }
}

fn dir_target(f: &Fixture) -> PathBuf {
    if cfg!(windows) {
        f.home("winuwu")
    } else {
        f.home("linuwu")
    }
}

#[test]
fn deploy_links_files_and_directories() {
    let f = Fixture::new(BASIC);
    let (ok, out) = f.run(&["deploy"]);
    assert!(ok, "{out}");

    assert_links_to(&file_target(&f), &f.path("test.txt"));
    assert_links_to(&dir_target(&f), &f.path("test"));

    let lines: Vec<&str> = out.lines().collect();
    assert_eq!(lines.len(), 6, "{out}");
    assert!(lines[0].starts_with("Deploying file to ./"), "{out}");
    assert!(lines[1].starts_with("file: symlinking "), "{out}");
    assert_eq!(lines[2], "Finished Deploying file!");
    assert!(lines[3].starts_with("Deploying dir to "), "{out}");
    assert_eq!(lines[5], "Finished Deploying dir!");
}

#[test]
fn deploy_refuses_existing_target() {
    let f = Fixture::new(BASIC);
    fs::write(file_target(&f), "precious").unwrap();
    let (_, out) = f.run(&["deploy"]);
    assert!(
        out.contains("Error in Deploying file: Target location already exists!"),
        "{out}"
    );
    assert_eq!(fs::read_to_string(file_target(&f)).unwrap(), "precious");
    // Later entries still run.
    assert_links_to(&dir_target(&f), &f.path("test"));
}

#[test]
fn undeploy_removes_file_link() {
    let f = Fixture::new(BASIC);
    assert!(f.run(&["deploy"]).0);
    let (_, out) = f.run(&["undeploy"]);
    assert!(out.contains("Finished Undeploying file!"), "{out}");
    assert_absent(&file_target(&f));
    assert!(f.path("test.txt").exists(), "source must survive");
}

#[test]
fn undeploy_refuses_unrelated_target() {
    let f = Fixture::new(BASIC);
    fs::write(file_target(&f), "precious").unwrap();
    let (_, out) = f.run(&["undeploy"]);
    assert!(
        out.contains("Error in Undeploying file: Target location does not correspond to source!"),
        "{out}"
    );
    assert_eq!(fs::read_to_string(file_target(&f)).unwrap(), "precious");
}

#[test]
fn config_env_beats_process_env_and_process_env_is_fallback() {
    let f = Fixture::new(
        "deploy:\n  a:\n    source: ./test.txt\n    target: ./$name.$other.txt\nenv:\n  name: fromconfig\n",
    );
    let (ok, out) = f
        .cmd(&["deploy"])
        .env("name", "fromenv")
        .env("other", "envonly")
        .output()
        .map(|o| {
            (
                o.status.success(),
                String::from_utf8_lossy(&o.stdout).into_owned(),
            )
        })
        .unwrap();
    assert!(ok, "{out}");
    assert_links_to(&f.path("fromconfig.envonly.txt"), &f.path("test.txt"));
}

#[test]
fn verbose_prints_source() {
    let f = Fixture::new("deploy:\n  a:\n    source: ./test.txt\n    target: ./out.txt\n");
    let (_, quiet) = f.run(&["deploy"]);
    assert!(f.run(&["undeploy"]).0);
    let (_, loud) = f.run(&["-v", "deploy"]);
    assert_eq!(loud.lines().count(), quiet.lines().count() + 1, "{loud}");
    assert!(
        loud.lines().nth(1).unwrap().ends_with("./test.txt"),
        "{loud}"
    );
}

#[test]
fn missing_config_reports_error() {
    let f = Fixture::new("");
    let (_, out) = f.run(&["-c", "nope.yaml", "deploy"]);
    assert!(out.starts_with("Error in reading config file: "), "{out}");
}

#[test]
fn no_subcommand_prints_help() {
    let f = Fixture::new("");
    let (ok, out) = f.run(&[]);
    assert!(!ok);
    assert!(out.contains("deploy") && out.contains("undeploy"), "{out}");
}
