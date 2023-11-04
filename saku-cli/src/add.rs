use saku_lib::pkg::data::{get_pkg_from_path, save_pkg};
use saku_lib::pkg::pkg::Pkg;

use saku_lib::util::{msg, path};

use saku_lib::prelude::*;

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
