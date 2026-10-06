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
    match fs::symlink_metadata(&link.target) {
        Err(err) if err.kind() == io::ErrorKind::NotFound => {}
        Err(err) => return Err(err.into()),
        Ok(_) if !overwrite => return Err(EntryError::TargetExists),
        Ok(meta) if meta.file_type().is_symlink() => remove_symlink(Path::new(&link.target))?,
        Ok(_) => return Err(EntryError::TargetNotSymlink),
    }
    create_symlink(&source, Path::new(&link.target))?;
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
    remove_symlink(Path::new(&link.target))?;
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

#[cfg(unix)]
fn create_symlink(source: &Path, target: &Path) -> io::Result<()> {
    std::os::unix::fs::symlink(source, target)
}

/// Windows has separate file and directory symlinks; pick the kind matching the source.
#[cfg(windows)]
fn create_symlink(source: &Path, target: &Path) -> io::Result<()> {
    use std::os::windows::fs::{symlink_dir, symlink_file};
    if fs::metadata(source)?.is_dir() {
        symlink_dir(source, target)
    } else {
        symlink_file(source, target)
    }
}

/// Removes the symlink at `path` itself, never what it points to.
fn remove_symlink(path: &Path) -> io::Result<()> {
    #[cfg(windows)]
    {
        use std::os::windows::fs::FileTypeExt;
        if fs::symlink_metadata(path)?.file_type().is_symlink_dir() {
            return fs::remove_dir(path);
        }
    }
    fs::remove_file(path)
}
