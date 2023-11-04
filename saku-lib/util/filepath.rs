use crate::prelude::*;
use std::path::{Path, PathBuf};

use super::cli::{get_cwd, self};

pub fn join(path: &str, suffix: &str) -> String {
    Path::new(path).join(suffix).to_str().unwrap().to_string()
}

pub fn abs(path: &str) -> Result<String> {
    let path = Path::new(path);
    let mut components = path.components();

    let mut normalized = if path.is_absolute() {
        // "If a pathname begins with two successive <slash> characters, the
        // first component following the leading <slash> characters may be
        // interpreted in an implementation-defined manner, although more than
        // two leading <slash> characters shall be treated as a single <slash>
        // character."
        if path.starts_with("//") && !path.starts_with("///") {
            components.next();
            PathBuf::from("//")
        } else {
            PathBuf::new()
        }
    } else {
        PathBuf::from(cli::get_cwd()?)
    };
    normalized.extend(components);

    // "Interfaces using pathname resolution may specify additional constraints
    // when a pathname that does not name an existing directory contains at
    // least one non- <slash> character and contains one or more trailing
    // <slash> characters".
    // A trailing <slash> is also meaningful if "a symbolic link is
    // encountered during pathname resolution".
    if path.ends_with("/") {
        normalized.push("");
    }

    Ok(normalized.to_str().unwrap().to_string())
}

pub fn exists(path: &str) -> bool {
    let p = Path::new(&path);
    p.exists()
}

pub fn is_dir(path: &str) -> bool {
    let p = Path::new(&path);
    p.is_dir()
}

pub fn extend(path: &str) -> Result<String> {
    if path.len() == 0 {
        return Err(make_err!(Missing, "path not long enough"));
    }

    // absolute path
    if path.starts_with("/") {
        return Ok(path.to_string());
    }

    let cwd = get_cwd()?;
    let extended = join(&cwd, path);
    let abs_path = abs(&extended)?;

    Ok(abs_path)
}

pub fn base_name(path: &str) -> Result<String> {
    Ok(Path::new(path)
        .file_name()
        .ok_or(Error::Unexpected)?
        .to_str()
        .ok_or(Error::Unexpected)?
        .to_string())
}
