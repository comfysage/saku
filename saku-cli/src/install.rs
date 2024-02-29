use saku_lib::prelude::*;
use saku_lib::exec;
use saku_lib::pkg::config;
use saku_lib::pkg::data::get_pkg;
use saku_lib::pkg::pkg::Pkg;
use saku_lib::util::io;
use saku_lib::util::{msg, path};

pub fn install(pkg_name: &String) -> Result<()> {
    let p: Pkg = get_pkg(pkg_name)?;

    install_pkg(p)?;

    Ok(())
}

pub fn clone_pkg(p: &Pkg) -> Result<()> {
    if path::exists(&path::repo(&p.name)) {
        info!("removing existing repo at {}", msg::general::path_f(&path::repo(&p.name)));
        io::rmdir(&path::repo(&p.name))?;
    }

    msg::clone(&p.name, &p.url);

    exec::Cmd::Clone{ url: p.url.clone(), name: p.name.clone() }.exec()?;

    Ok(())
}

pub fn build_pkg(p: &Pkg) -> Result<()> {
    msg::build(&p.name, &path::repo(&p.name));
    debug!("{:?}", &p);

    exec::build(&p.name, &p.group)?;

    Ok(())
}

pub fn install_pkg(p: Pkg) -> Result<()> {
    clone_pkg(&p)?;

    build_pkg(&p)?;

    p.install_root()?;

    let config = config::Config::new()?;
    if !config.main.no_install_cleanup {
        exec::cleanup(&p.name, &p.group)?;
    }

    Ok(())
}
