use std::fs;
use symlink::{symlink_auto, remove_symlink_auto};

use crate::{ reporter, Deploy };
use std::io;
use std::error;

pub(crate) fn symlink(source: &String, target: &String, config:&Deploy, progress: &reporter::Reporter) -> Result<(), Box<error::Error>>{
    progress.start(target);
    let sourcePath = fs::canonicalize(source)?;

    progress.progress(&format!("symlinking {} to {}", &sourcePath.to_str().unwrap(), target));
    symlink_auto(&sourcePath, &target);
    return Ok(());
}

pub(crate) fn unsymlink(source: &String, target: &String, progress: &reporter::Reporter) -> Result<(), Box<error::Error>> {
    progress.start(target);
    let sourcePath = fs::canonicalize(source)?;
    let targetPath = fs::canonicalize(target)?;

    progress.progress(&format!("unsymlinking {} from {}", targetPath.to_str().unwrap(), &sourcePath.to_str().unwrap()));

    if !fs::read_link(&targetPath)?.eq(&sourcePath) {
        return Err(Box::from(io::Error::new(io::ErrorKind::Other, "Target location does not correspond to source!")));
    }
    remove_symlink_auto(&targetPath)?;
    return Ok(());
}