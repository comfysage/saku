use crate::pkg::data::{save_pkg, get_pkg_from_path};
use crate::pkg::pkg::Pkg;

use crate::util::{path, msg};

use crate::{Result, Error};

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
