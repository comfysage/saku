use std::fs;

use crate::pkg::pkg::Pkg;
use crate::Error;
use crate::exec;
use crate::util;

use serde::{Serialize, Deserialize};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Pack {
    pub prefix: String,
    pub path: String,
}

impl Pack {
    pub fn is_bin(&self) -> bool {
        todo!()
    }
}

impl Pack {
    pub fn install(&self, pkg: &Pkg) -> Result<(), Error> {
        exec::pack(&pkg.name, &self.path, &self.prefix)
    }
    pub fn uninstall(&self) -> Result<(), Error> {
        if self.path.len() == 0 || self.prefix.len() == 0 {
            return Err(make_err!(Missing, "invalid installation files"));
        }

        let dst = util::path::pack_file(&self.prefix, &self.path);
        if ! util::filepath::exists(&dst) {
            return Ok(())
        }

        fs::remove_file(dst)?;

        Ok(())
    }
}

impl Pkg {
    pub fn install_pack(&self) -> Result<(), Error> {
        self.pack.iter().map(|p| p.install(self)).collect()
    }
    pub fn uninstall_pack(&self) -> Result<(), Error> {
        self.pack.iter().map(|p| p.uninstall()).collect()
    }
}
