use saku_lib::exec;
use saku_lib::pkg;
use saku_lib::pkg::config::Config;
use saku_lib::prelude::*;
use saku_lib::util::filepath;
use saku_lib::util::{constants, io, msg, path};

use crate::{flask, update};

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
    io::mkdir(constants::SAKU_DIR.to_string())?;
    io::mkdir(constants::PKG_DIR.to_string())?;
    io::mkdir(constants::REPO_DIR.to_string())?;

    io::mkdir(constants::FLASK_DIR.to_string())?;

    if !filepath::exists(&*constants::LIB_DIR) {
        let repo_dir: String = path::repo("saku");
        let lib_dir: String = constants::LIB_DIR_NAME.to_string();
        let target = filepath::join(&repo_dir, &lib_dir);
        io::link(&target, &*constants::LIB_DIR)?;
    }

    create_root()?;

    io::mkdir(path::gr("custom"))?;

    if !path::pkg_exists("flasks", "core") {
        msg::fetch("core", "https://github.com/crispybaccoon/pkg");

        flask::add_with_name("core", "crispybaccoon/pkg")?;
    }

    if !path::repo_exists("core") {
        update::update()?;
    }

    Ok(())
}

pub fn create() -> Result<()> {
    let config_path = Config::path()?;
    if path::exists(&config_path) {
        return Err(make_err!(
            Conflict,
            "config file {config_path} already exists."
        ));
    }
    msg::create_config(&config_path);
    pkg::data::save_config(Config::default())?;

    Ok(())
}
