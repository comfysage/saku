use crate::pkg::pkg::Pkg;
use crate::Error;
use crate::exec;

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
    pub fn uninstall(&self, pkg: &Pkg) -> Result<(), Error> {
        todo!()
    }
}

impl Pkg {
    pub fn install_pack(&self) -> Result<(), Error> {
        self.pack.iter().map(|p| p.install(self)).collect()
    }
    pub fn uninstall_pack(&self) -> Result<(), Error> {
        self.pack.iter().map(|p| p.uninstall(self)).collect()
    }
}
