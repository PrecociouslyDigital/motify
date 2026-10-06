//! Creating and removing the symlinks themselves.

use std::fs;

use crate::error::EntryError;
use crate::reporter::Reporter;
use crate::resolve::Link;

pub fn deploy(link: &Link, overwrite: bool, report: &Reporter) -> Result<(), EntryError> {
    report.start(&link.target);
    report.info(&link.source);
    let source = fs::canonicalize(&link.source)?;
    report.progress(format_args!(
        "symlinking {} to {}",
        source.display(),
        link.target
    ));
    if !overwrite && fs::metadata(&link.target).is_ok() {
        return Err(EntryError::TargetExists);
    }
    symlink::symlink_auto(&source, &link.target)?;
    Ok(())
}

pub fn undeploy(link: &Link, report: &Reporter) -> Result<(), EntryError> {
    report.start(&link.target);
    report.progress(format_args!(
        "unsymlinking {} from {}",
        link.target, link.source
    ));
    let source = fs::canonicalize(&link.source)?;
    let resolved_target = fs::canonicalize(&link.target)?;
    if resolved_target != source {
        return Err(EntryError::TargetMismatch);
    }
    let _ = if fs::metadata(&resolved_target)?.is_dir() {
        fs::remove_dir(&link.target)
    } else {
        fs::remove_file(&link.target)
    };
    Ok(())
}
