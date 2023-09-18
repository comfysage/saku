use crate::Error;
use crate::exec;
use crate::pkg::data::get_pkg;
use crate::pkg::pkg::Pkg;
use crate::util::{msg, path};

pub fn cli_install(args: Vec<String>) -> Result<(), Error> {
    args.iter().map(|s| install(s)).collect()
}

pub fn install(pkg_name: &String) -> Result<(), Error> {
    let p: Pkg;

    p = get_pkg(pkg_name)?;

    start_install(p)?;

    Ok(())
}

pub fn clone_pkg(p: &Pkg) -> Result<(), Error> {
    if path::exists(&path::repo(&p.name)) {
        /* if force {
            err := os.RemoveAll(util.PathRepo(p.Name))
            if err != nil {
                return err
            }
        } else { */
        return Err(make_err!(Conflict, "repo already cloned. try again with --force."));
        // }
    }

    msg::clone(&p.name, &p.url);

    exec::clone(&p.url, &p.name);

    Ok(())
}

pub fn run_install(p: &Pkg /* , force bool, deep_clone bool */) -> Result<(), Error> {
    msg::build(&p.name, &path::repo(&p.name));

    exec::run_in_repo(&p.name, p.install.clone());

    Ok(())
}

pub fn start_install(p: Pkg /* , force bool, deep_clone bool */) -> Result<(), Error> {
    clone_pkg(&p)?;

    run_install(&p)?;

    p.pack()?;

    p.install_pack()?;

    Ok(())
}
