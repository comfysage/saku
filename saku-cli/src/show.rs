use saku_lib::pkg::data::get_pkg;
use saku_lib::prelude::*;

pub fn show(pkg_name: &String) -> Result<()> {
    let pkg = get_pkg(pkg_name)?;
    pkg.show()?;
    Ok(())
}
