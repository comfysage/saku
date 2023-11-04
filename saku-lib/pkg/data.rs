use std::fs;

use crate::util;
use crate::prelude::*;
use crate::util::path;
use crate::util::url;

use super::flask::Flask;
use super::pkg::Pkg;
use super::config::Config;

pub fn get_pkg_from_path(path: &str) -> Result<Pkg> {
    let fullpath = util::filepath::extend(path)?;

    let fh = fs::read_to_string(fullpath)?;
    let mut pkg = Pkg::from_string(fh)?;

    pkg.safe_guard()?;
    pkg.fill()?;

    Ok(pkg)
}

pub fn get_pkg(name: &str) -> Result<Pkg> {
    let (path, is_dir): (String, bool) = util::path::path_determine(name.to_string())?;

    if is_dir {
        let pkg_path = util::path::repo_seed(&path);
        let mut pkg = get_pkg_from_path(&pkg_path)?;
        store_repo_seed(&mut pkg)?;

        return Ok(pkg);
    }

    let pkg = get_pkg_from_path(&path)?;
    Ok(pkg)
}

pub fn save_pkg(pkg: &Pkg) -> Result<()> {
    let str = pkg.to_string()?;

    let group: &str = if pkg.group.len() == 0 {
        "custom"
    } else {
        &pkg.group
    };

    let path = util::path::path_pkg(group, &pkg.name);
    fs::write(path, str)?;

    Ok(())
}

fn store_repo_seed(pkg: &mut Pkg) -> Result<()> {
    pkg.group = format!("repo");

    let _ = util::io::mkdir(util::path::gr("repo"))?;

    save_pkg(pkg)?;

    Ok(())
}

pub fn save_config(config: Config) -> Result<()> {
    let str = config.to_string()?;

    let path = path::config();
    fs::write(path, str)?;

    Ok(())
}

pub fn get_flask(url: &str) -> Result<Flask> {
    let name = url::url_name(url)?;
    let flask = get_flask_from_name(&name)?;
    Ok(flask)
}

pub fn get_flask_from_name(name: &str) -> Result<Flask> {
    let path = path::flask(name);
    let pkg = get_pkg_from_path(&path)?;
    let flask = Flask::from_pkg(pkg)?;
    Ok(flask)
}

pub fn get_flasks() -> Result<Vec<String>> {
    let files = path::flasks()?;
    let flasks = files.iter().map(|s| path::remove_extension(s.to_string())).collect();

    Ok(flasks)
}
