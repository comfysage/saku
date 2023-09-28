use crate::{Result, Error, exec, pkg};
use crate::util::{constants, path, io, msg};

use super::install;

fn create_pack() -> Result<()> {
    io::mkdir(constants::ROOT_DIR.to_string())?;
    io::mkdir(path::pack_dir("man"))?;
    io::mkdir(path::pack_dir("share"))?;

    if !path::exists(&path::pack_dir("share/man")) {
        exec::run_in_pack("share", vec![exec::cmd::ln_cmd("../man", "man")])?;
    }
    Ok(())
}

pub fn init() -> Result<()> {
    io::mkdir(constants::HAYASHI_DIR.to_string())?;
    io::mkdir(constants::PKG_DIR.to_string())?;
    io::mkdir(constants::REPO_DIR.to_string())?;

    create_pack()?;

    io::mkdir(path::gr("custom"))?;

    if !path::pkg_exists("custom", "core") {
        msg::fetch("core", "https://github.com/crispybaccoon/saku");

        let p = path::path_pkg("custom", "core");

        exec::curl("https://raw.githubusercontent.com/CrispyBaccoon/saku/mega/core.yaml", &p)?;
    }

    if !path::exists(&path::gr("core")) {
        let p = pkg::data::get_pkg_from_path(&path::path_pkg("custom", "core"))?;
        install::start_install(p)?;
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
