use crate::Error;
use crate::pkg::{pkg::Pkg, data::get_pkg};

pub fn show(pkg_name: &String) -> Result<(), Error> {
    let pkg = get_pkg(pkg_name)?;
    pkg.show();
    Ok(())
}

impl Pkg {
    pub fn show(&self) {
        todo!()
    }
}
