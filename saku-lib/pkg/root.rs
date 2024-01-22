use std::fs;

use crate::prelude::*;
use crate::exec;
use crate::pkg::pkg::Pkg;
use crate::util::{filepath, path};

impl Pkg {
    pub fn install_root(&self) -> Result<()> {
        exec::install(&self.name, &self.group)
    }
    pub fn uninstall_root(&self) -> Result<()> {
        for d in fs::read_dir(path::root_dir(&format!("{}/bin", self.name)))? {
            let d = d?;
            let d_path_bind = d.path();
            let d_path = match d_path_bind.to_str() {
                Some(s) => Ok(s),
                None => Err(Error::Unexpected),
            }?;
            if filepath::is_dir(d_path) {
                continue;
            }
            let name = filepath::base_name(d_path)?;
            fs::remove_file(d_path)?;
            fs::remove_file(path::root_file("bin", &name))?;
        }
        Ok(())
    }
}
