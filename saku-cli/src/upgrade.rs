use saku_lib::exec;
use saku_lib::prelude::*;

use saku_lib::pkg::data;

pub fn upgrade(name: &str) -> Result<()> {
    let pkg = data::get_pkg(name)?;

    exec::pull(&pkg.name)?;

    exec::upgrade(&pkg.name, &pkg.group)?;

    pkg.install_root()?;

    Ok(())
}
