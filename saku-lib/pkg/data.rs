use std::fs;

use crate::util;
use crate::prelude::*;
use crate::util::path;
use crate::util::url;

use super::flask::Flask;
use super::pkg::Pkg;
use super::config::Config;

/// Parse a flask file and return a *Pkg*
///
/// - *path*: path to a flask file
pub fn get_pkg_from_path(path: &str) -> Result<Pkg> {
    let fullpath = util::filepath::extend(path)?;

    let fh = fs::read_to_string(fullpath)?;
    let mut pkg = Pkg::from_string(fh)?;

    pkg.path = Some(path.to_string());
    pkg.safe_guard()?;
    pkg.fill()?;

    Ok(pkg)
}

/// Parse a flask file and return a *Pkg*
///
/// - *name*: name of a package
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

/// Write a *Pkg* to a flask file
///
/// - *pkg*: package to save
pub fn save_pkg(pkg: &Pkg) -> Result<()> {
    let str = pkg.to_string()?;

    let group: &str = if pkg.group.len() == 0 {
        "custom"
    } else {
        &pkg.group
    };

    let path = util::path::path_pkg(group, &pkg.name);
    debug!("saving pkg to {:?}", &path);
    fs::write(path, str)?;

    Ok(())
}

/// Write a *Pkg* to flask file for a repo package
///
/// - *pkg*: package to save
fn store_repo_seed(pkg: &mut Pkg) -> Result<()> {
    pkg.group = format!("repo");

    let _ = util::io::mkdir(util::path::gr("repo"))?;

    save_pkg(pkg)?;

    Ok(())
}

/// Write a *Config* to the config file
///
/// - *config*: config to save
///
/// config file is located at `~/.config/saku/saku.toml`
pub fn save_config(config: Config) -> Result<()> {
    let str = config.to_string()?;

    let path = Config::path()?;
    fs::write(path, str)?;

    Ok(())
}

/// Parse a flask file and return a *Flask*
///
/// - *url*: url associated with flask
pub fn get_flask(url: &str) -> Result<Flask> {
    let name = url::url_name(url)?;
    let flask = get_flask_from_name(&name)?;
    Ok(flask)
}

/// Parse a flask file and return a *Flask*
///
/// - *name*: name of a flask
pub fn get_flask_from_name(name: &str) -> Result<Flask> {
    let path = path::flask(name);
    let pkg = get_pkg_from_path(&path)?;
    let flask = Flask::from_pkg(pkg)?;
    Ok(flask)
}

/// Search for all flasks
/// returns a list of names
///
/// searches in `~/.saku/flask/flasks`
pub fn get_flasks() -> Result<Vec<String>> {
    let files = path::flasks()?;
    let flasks = files.iter().map(|s| path::remove_extension(s.to_string())).collect();

    Ok(flasks)
}
