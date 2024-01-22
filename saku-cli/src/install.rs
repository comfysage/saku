use saku_lib::prelude::*;
use saku_lib::exec;
use saku_lib::pkg::config;
use saku_lib::pkg::data::get_pkg;
use saku_lib::pkg::pkg::Pkg;
use saku_lib::util::{msg, path};

pub fn install(pkg_name: &String) -> Result<()> {
    let p: Pkg = get_pkg(pkg_name)?;

    start_install(p)?;

    Ok(())
}

pub fn clone_pkg(p: &Pkg) -> Result<()> {
    if path::exists(&path::repo(&p.name)) {
        // TODO: `--force` is not yet implemented
        return Err(make_err!(Conflict, "repo already cloned. try again with --force."));
    }

    msg::clone(&p.name, &p.url);

    exec::clone(&p.url, &p.name)?;

    Ok(())
}

pub fn run_install(p: &Pkg) -> Result<()> {
    msg::build(&p.name, &path::repo(&p.name));
    dbg!(&p);

    exec::build(&p.name, &p.group)?;

    Ok(())
}

pub fn start_install(p: Pkg) -> Result<()> {
    clone_pkg(&p)?;

    run_install(&p)?;

    p.install_root()?;

    let config = config::Config::new()?;
    if !config.main.no_install_cleanup {
        exec::cleanup(&p.name, &p.group)?;
    }

    Ok(())
}
