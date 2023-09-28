use std::fs;

use crate::util;
use crate::Error;

use super::pkg::Pkg;
use super::config::Config;

pub fn get_pkg_from_path(path: &str) -> Result<Pkg, Error> {
    let fullpath = util::filepath::extend(path)?;

    let fh = fs::read_to_string(fullpath)?;
    let mut pkg = Pkg::from_string(fh)?;

    pkg.safe_guard()?;
    pkg.fill()?;

    Ok(pkg)
}

pub fn get_pkg(name: &str) -> Result<Pkg, Error> {
    let (path, is_dir) = util::path::path_determine(name.to_string())?;

    if is_dir {
        let pkg_path = util::path::repo_seed(&path);
        let mut pkg = get_pkg_from_path(&pkg_path)?;
        store_repo_seed(&mut pkg)?;

        return Ok(pkg);
    }

    let pkg = get_pkg_from_path(&path)?;
    Ok(pkg)
}

pub fn save_pkg(pkg: &Pkg) -> Result<(), Error> {
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

fn store_repo_seed(pkg: &mut Pkg) -> Result<(), Error> {
    pkg.group = format!("repo");

    let _ = util::io::mkdir(util::path::gr("repo"))?;

    save_pkg(pkg)?;

    Ok(())
}

pub fn save_config(default: Config) -> Result<(), Error> {
    todo!()
}
