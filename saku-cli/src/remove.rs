use saku_lib::util::msg;
use saku_lib::util::path;
use saku_lib::{Error, Result, make_err};

use std::fs;

pub fn remove(name: &str) -> Result<()> {
    if path::pkg_exists("custom", name) {
        msg::remove(name);
        fs::remove_file(path::path_pkg("custom", name))?;
        return Ok(());
    }

    Err(make_err!(NotFound, "package not found."))
}
