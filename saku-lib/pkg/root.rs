use crate::pkg::rebuild::Root;
use crate::prelude::*;
use crate::exec;
use crate::pkg::pkg::Pkg;
use crate::util::io;
use crate::util::msg;
use crate::util::path;

impl Pkg {
    /// run install task
    /// - package files in store dir
    /// - link stored files to root
    pub fn install_root(&self) -> Result<()> {
        self.store()?;
        self.link_root()?;
        Ok(())
    }
    /// package files in store dir
    pub fn store(&self) -> Result<()> {
        trace!("storing files");
        let has_artifacts = !io::mkdir(path::store_dir(&self.name))?;
        if has_artifacts {
            debug!("cleaning up artifacts in store");
            let files = path::get_stored_bin(&self.name)?;
            for entry in files {
                debug!("removing artifact {}", msg::general::path_f(&entry));
                exec::unlink(&entry)?;
            }
            let store_path = path::store_dir(&self.name);
            let dirs = path::get_artifact_dirs(&self.name)?;
            for entry in dirs {
                debug!("removing artifact {}",  msg::general::path_f(&entry));
                io::rmdir(&entry)?;
            }
            io::mkdir(store_path)?;
        }
        exec::install(&self.name, &self.group)?;
        Ok(())
    }
    /// link stored files to root
    pub fn link_root(&self) -> Result<()> {
        trace!("linking root");
        Root::new(&self)?.build()?.link()?;
        Ok(())
    }
    /// remove stored files in root
    pub fn uninstall_root(&self) -> Result<()> {
        trace!("uninstalling pkg from root");
        Root::new(&self)?.build()?.uninstall()?;
        Ok(())
    }
    /// remove store path for pkg
    pub fn cleanup_store(&self) -> Result<()> {
        trace!("cleaning store");
        let store_path = path::store_dir(&self.name); 
        msg::remove_file(&store_path);
        io::rmdir(&store_path)?;
        Ok(())
    }
    /// remove repo path for pkg
    pub fn cleanup_repo(&self) -> Result<()> {
        trace!("cleaning repo");
        let repo_path = path::repo(&self.name); 
        msg::remove_file(&repo_path);
        io::rmdir(&repo_path)?;
        Ok(())
    }
}
