use saku_lib::pkg::data;
use saku_lib::pkg::flask::Flask;
use saku_lib::pkg::pkg::Pkg;
use saku_lib::prelude::*;
use saku_lib::util::{msg, path};

pub fn list() -> Result<()> {
    let flask_files = data::get_flasks()?;

    let flasks: Vec<Result<Flask>> = flask_files
        .iter()
        .map(|f| data::get_flask_from_name(f))
        .collect();

    for f in &flasks {
        if let Ok(flask) = f {
            println!(" - {} from {}", msg::general::name_f(&flask.name()), msg::general::url_f(&flask.url()));
        } else if let Err(err) = f {
            warn!("error while listing flask: {err}");
        }
    }

    Ok(())
}

pub fn list_installed() -> Result<()> {
    let dirs = path::get_store_dirs()?;

    let pkgs: Vec<Result<Pkg>> = dirs
        .iter()
        .map(|name| data::get_pkg(&name))
        .collect();

    for p in &pkgs {
        if let Ok(pkg) = p {
            println!(" - {} from {}", msg::general::name_f(&pkg.name), msg::general::url_f(&pkg.url));
        } else if let Err(err) = p {
            warn!("error while listing pkg: {err}");
        }
    }

    Ok(())
}
