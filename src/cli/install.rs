use crate::Error;
use crate::exec;
use crate::pkg::data::get_pkg;
use crate::pkg::pkg::Pkg;
use crate::util::{msg, path};

pub fn install(pkg_name: &String) -> Result<(), Error> {
    let p: Pkg = get_pkg(pkg_name)?;

    start_install(p)?;

    Ok(())
}

pub fn clone_pkg(p: &Pkg) -> Result<(), Error> {
    if path::exists(&path::repo(&p.name)) {
        // TODO: `--force` is not yet implemented
        return Err(make_err!(Conflict, "repo already cloned. try again with --force."));
    }

    msg::clone(&p.name, &p.url);

    exec::clone(&p.url, &p.name)?;

    Ok(())
}

pub fn run_install(p: &Pkg) -> Result<(), Error> {
    msg::build(&p.name, &path::repo(&p.name));

    if p.install.len() > 0 {
        exec::run_in_repo(&p.name, p.install.clone())?;
    }

    Ok(())
}

pub fn start_install(p: Pkg) -> Result<(), Error> {
    clone_pkg(&p)?;

    run_install(&p)?;

    p.install_root()?;

    Ok(())
}
