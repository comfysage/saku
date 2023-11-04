use saku_lib::prelude::*;
use saku_lib::pkg;
use saku_lib::exec;
use saku_lib::util::{constants, path, io, msg};

fn create_root() -> Result<()> {
    io::mkdir(constants::ROOT_DIR.to_string())?;
    io::mkdir(path::root_dir("man"))?;
    io::mkdir(path::root_dir("share"))?;

    if !path::exists(&path::root_dir("share/man")) {
        exec::run_in_root("share", vec![exec::cmd::ln_cmd("../man", "man")])?;
    }
    Ok(())
}

pub fn init() -> Result<()> {
    io::mkdir(constants::HAYASHI_DIR.to_string())?;
    io::mkdir(constants::PKG_DIR.to_string())?;
    io::mkdir(constants::REPO_DIR.to_string())?;

    io::mkdir(constants::FLASK_DIR.to_string())?;

    create_root()?;

    io::mkdir(path::gr("custom"))?;

    if !path::pkg_exists("flasks", "core") {
        msg::fetch("core", "https://github.com/crispybaccoon/pkg");

        crate::flask::add_with_name("core", "crispybaccoon/pkg")?;
    }

    if !path::repo_exists("core") {
        crate::update::update()?;
    }

    Ok(())
}

pub fn create() -> Result<()> {
    let config_path = path::config();
    if path::exists(&config_path) {
        return Err(make_err!(Conflict, "config file {config_path} already exists."))
    }
    msg::create_config(&config_path);
    pkg::data::save_config(pkg::config::Config::default())?;

    Ok(())
}
