//! Creating and removing the symlinks themselves.

use std::fs;
use std::io;
use std::path::Path;

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
    if !is_symlink_to(Path::new(&link.target), &source)? {
        return Err(EntryError::TargetMismatch);
    }
    symlink::remove_symlink_auto(&link.target)?;
    Ok(())
}

/// Whether `target` is itself a symlink that ultimately resolves to `source` (already canonical).
///
/// Checking the link itself, rather than only where the path resolves, means a real file
/// reached through a symlinked parent directory is never mistaken for our link.
fn is_symlink_to(target: &Path, source: &Path) -> io::Result<bool> {
    let is_symlink = fs::symlink_metadata(target)?.file_type().is_symlink();
    Ok(is_symlink && fs::canonicalize(target).is_ok_and(|resolved| resolved == source))
}
