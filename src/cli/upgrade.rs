use crate::{exec, Result};

use crate::pkg::data;

pub fn upgrade(name: &str) -> Result<()> {
    let pkg = data::get_pkg(name)?;

    exec::pull(&pkg.name)?;

    if pkg.update.len() > 0 {
        exec::run_in_repo(&name, pkg.update.clone())?;
    } else {
        super::install::run_install(&pkg)?;
    }

    pkg.install_root()?;

    Ok(())
}
