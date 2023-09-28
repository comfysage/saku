use crate::pkg::data::{get_pkg_from_path, save_pkg};
use crate::pkg::pkg::Pkg;

use crate::util::{msg, path};

use crate::{Error, Result};

pub fn add(pkg: &Pkg) -> Result<()> {
    if path::pkg_exists("custom", &pkg.name) {
        return Err(make_err!(Conflict, "pkg already exists."));
    }

    save_pkg(&pkg)?;

    Ok(())
}

// add new pkg from file
pub fn add_local(path: &str) -> Result<()> {
    msg::add(path);

    let pkg = get_pkg_from_path(path)?;
    add(&pkg)?;

    Ok(())
}
