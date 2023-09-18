use crate::Error;
use std::path::Path;

pub fn join(path: &str, suffix: &str) -> String {
    Path::new(path).join(suffix).to_str().unwrap().to_string()
}

pub fn abs(path: &str) -> String {
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
    todo!()
}

pub fn base_name(path: &str) -> Result<String, Error> {
    todo!()
}
