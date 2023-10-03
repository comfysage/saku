use crate::Error;
use std::path::Path;

use super::cli::get_cwd;

pub fn join(path: &str, suffix: &str) -> String {
    Path::new(path).join(suffix).to_str().unwrap().to_string()
}

pub fn abs(path: &str) -> Result<String, Error> {
    todo!()
}

pub fn exists(path: &str) -> bool {
    let p = Path::new(&path);
    p.exists()
}

pub fn is_dir(path: &str) -> bool {
    let p = Path::new(&path);
    p.is_dir()
}

pub fn extend(path: &str) -> Result<String, Error> {
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

pub fn base_name(path: &str) -> Result<String, Error> {
    Ok(Path::new(path)
        .file_name()
        .ok_or(Error::Unexpected)?
        .to_str()
        .ok_or(Error::Unexpected)?
        .to_string())
}
