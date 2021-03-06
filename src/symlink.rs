use std::fs;
use symlink::{symlink_auto, remove_symlink_auto};

use crate::{ reporter, Deploy };
use std::io;
use std::error;

pub(crate) fn symlink(source: &String, target: &String, config:&Deploy, progress: &reporter::Reporter) -> Result<(), Box<error::Error>>{
    progress.start(target);
    progress.info(source);
    let source_path = fs::canonicalize(source)?;

    progress.progress(&format!("symlinking {} to {}", &source_path.to_str().unwrap(), target));
    if !config.overwrite {
        if let Err(_err) = fs::metadata(target) {
        }
        else {
            return Err(Box::from(io::Error::new(io::ErrorKind::Other, "Target location already exists!")));
        }
    }
    symlink_auto(&source_path, &target)?;
    return Ok(());
}

pub(crate) fn unsymlink(source: &String, target: &String, progress: &reporter::Reporter) -> Result<(), Box<error::Error>> {
    progress.start(target);

    progress.progress(&format!("unsymlinking {} from {}", target, source));
    let source_path = &fs::canonicalize(&source)?;
    let target_path = fs::canonicalize(target)?;
    if target_path.eq(source_path) {
        if fs::metadata(target_path)?.is_dir(){
            fs::remove_dir(target);
        }else{
            fs::remove_file(target);
        }

    }else {
        return Err(Box::from(io::Error::new(io::ErrorKind::Other, "Target location does not correspond to source!")));
    }
    return Ok(());
}