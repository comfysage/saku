use saku_lib::exec;
use saku_lib::prelude::*;

use saku_lib::pkg::data;

pub fn upgrade(name: &str) -> Result<()> {
    let pkg = data::get_pkg(name)?;

    exec::Cmd::Pull{ name: pkg.name.clone() }.exec()?;

    exec::upgrade(&pkg.name.clone(), &pkg.group.clone())?;

    pkg.install_root()?;

    Ok(())
}
