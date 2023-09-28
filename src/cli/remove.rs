use crate::util::msg;
use crate::util::path;
use crate::{Error, Result};

use std::fs;

pub fn remove(name: &str) -> Result<()> {
    if path::pkg_exists("custom", name) {
        msg::remove(name);
        fs::remove_file(path::path_pkg("custom", name))?;
        return Ok(());
    }

    Err(make_err!(NotFound, "package not found."))
}
