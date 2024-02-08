use crate::prelude::*;

use super::path;

pub fn mkdir(path: String) -> Result<bool> {
    if path::exists(&path) {
        return Ok(false);
    }

    std::fs::create_dir_all(path)?;

    Ok(true)
}

pub fn rmdir(path: &str) -> Result<()> {
    if path::exists(&path) {
        // [NOTE]: removes a directory after cleaning its contents or removes a symlink
        std::fs::remove_dir_all(&path)?;
    }
    Ok(())
}

pub fn link(target: &str, path: &str) -> Result<()> {
    std::os::unix::fs::symlink(target, path)?;

    Ok(())
}
