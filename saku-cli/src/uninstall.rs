use saku_lib::prelude::*;

use saku_lib::pkg::pkg::Pkg;
use saku_lib::pkg::data;

pub fn remove(name: &str) -> Result<()> {
    let p: Pkg = data::get_pkg(name)?;
    p.uninstall_root()?;
    p.cleanup_store()?;
    p.cleanup_repo()?;
    Ok(())
}
