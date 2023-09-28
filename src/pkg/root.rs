use std::fs;

use crate::pkg::pkg::Pkg;
use crate::Error;
use crate::exec;
use crate::util;

use serde::{Serialize, Deserialize};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Root {
    pub prefix: String,
    pub path: String,
}

impl Root {
    pub fn is_bin(&self) -> bool {
        todo!()
    }
}

impl Root {
    pub fn install(&self, pkg: &Pkg) -> Result<(), Error> {
        exec::root(&pkg.name, &self.path, &self.prefix)
    }
    pub fn uninstall(&self) -> Result<(), Error> {
        if self.path.len() == 0 || self.prefix.len() == 0 {
            return Err(make_err!(Missing, "invalid installation files"));
        }

        let dst = util::path::root_file(&self.prefix, &self.path);
        if ! util::filepath::exists(&dst) {
            return Ok(())
        }

        fs::remove_file(dst)?;

        Ok(())
    }
}

impl Pkg {
    pub fn install_root(&self) -> Result<(), Error> {
        self.root.iter().map(|p| p.install(self)).collect()
    }
    pub fn uninstall_root(&self) -> Result<(), Error> {
        self.root.iter().map(|p| p.uninstall()).collect()
    }
}
