use crate::Error;
use crate::pkg::{pkg::Pkg, data::get_pkg};

pub fn cli_show(args: Vec<String>) -> Result<(), Error> {
    if args.len() < 1 {
        return Err(Error::Missing(format!("not enough arguments provided")));
    }
    let name = &args[0];
    show(name)?;
    Ok(())
}

pub fn show(pkg_name: &String) -> Result<(), Error> {
    let pkg = get_pkg(pkg_name)?;
    pkg.show();
    Ok(())
}

impl Pkg {
    pub fn show(&self) {}
}
