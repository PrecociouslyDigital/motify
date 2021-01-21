mod reporter;

use clap::Clap;


use yaml_rust::{YamlLoader, Yaml};


use std::{fs, env};
use yaml_rust::yaml::Hash;
use std::path::{PathBuf, Path};

#[cfg(target_family = "unix")]
    use std::os::unix::fs as platformfs;
#[cfg(target_family = "windows")]
    use std::os::windows::fs as platformfs;



/// motify is a tool to help you manage symlinks declaratively.
/// it was designed primarily to backup and deploy configuration files.
#[derive(Clap)]
#[clap(version = "1.0", author = "PrecociouslyDigital <skye@hyphen-emdash.com>")]
struct Opts {
    /// Sets a custom config file. Could have been an Option<T> with no default too
    #[clap(short, long, default_value = "motify.yaml")]
    config: String,
    #[clap(short, long, parse(from_occurrences))]
    verbose: i32,

}


fn main() {
    let opts: Opts = Opts::parse();
    let docs= YamlLoader::load_from_str(&fs::read_to_string(opts.config).unwrap()).unwrap();
    let config =  &docs[0];
    for (name, target) in config.as_hash().unwrap() {
        let progress = reporter::Reporter {
            name: String::from(name.as_str().unwrap()),
            verbose: opts.verbose
        };
        symlink(&String::from(target["source"].as_str().unwrap()), &String::from(target["target"][env::consts::OS].as_str().unwrap()), progress);
    }
}

fn symlink(source: &String, target: &String, progress: reporter::Reporter) {
    progress.start(target);
    let sourcePath = fs::canonicalize(source).expect("valid source path");

    progress.progress(&format!("symlinking {} to {}", &sourcePath.to_str().unwrap(), target));

    #[cfg(target_family = "windows")] {
        if fs::metadata(&sourcePath).unwrap().is_dir() {
            progress.progress(&String::from("is a directory symlink"));
            platformfs::symlink_dir(&sourcePath, &target).expect("Symlink Directory Created");
        }else {
            progress.progress(&String::from("is a file symlink"));
            platformfs::symlink_file(&sourcePath, &target).expect("Symlink File Created");
        }

    } #[cfg(target_family = "unix")] {
        platformfs::symlink(&sourcePath, &targetPath).expect("Symlink Created");
    }
    progress.done();
}

