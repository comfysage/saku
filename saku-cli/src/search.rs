use saku_lib::pkg::data;
use saku_lib::prelude::*;
use saku_lib::util::path;

pub fn search(pattern: &str) -> Result<()> {
    let pkgs: Vec<[String;2]> = path::pkg_match(&pattern)?;
    if pkgs.len() == 0 {
        return Err(make_err!(NotFound, "no packages matching that name were found"))
    }

    pkgs.iter().map(
        |p| {
            let (group, name) = (p[0].clone(), p[1].clone());
            debug!("found pkg {}/{}", &group, &name);
            let path = path::path_pkg(&group, &name);
            let pkg = data::get_pkg_from_path(&path)?;
            pkg.show()?;
            Ok(())
        }).collect::<Result<()>>()?;

    Ok(())
}
