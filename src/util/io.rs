use std::fs;

use crate::Error;

use super::path;

pub fn mkdir(path: String) -> Result<bool, Error> {
    if path::exists(&path) {
        return Ok(false);
    }

    fs::create_dir(path)?;

    Ok(true)
}

pub fn link(target: &str, path: &str) -> Result<(), Error> {
    std::os::unix::fs::symlink(target, path)?;

    Ok(())
}
