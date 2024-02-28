use crate::exec;
use crate::util::{constants, io, path, url};
use crate::prelude::*;

use crate::pkg::pkg::Pkg;

// examples:
// Flask({ name: "comfysage.core", url: "comfysage/core" }, "comfysage/core")
// Flask({ name: "aur.archlinux.org.pkg", url: "aur.archlinux.org/pkg" }, "aur.archlinux.org/pkg")

#[derive(Debug)]
pub struct Flask(Pkg, String);

impl Flask {
    pub fn new(name: &str, url: &str) -> Result<Self> {
        let shortened = url::shorten_url(url)?;
        let pkg = Flask::new_pkg(name, url)?;
        Ok(Flask(pkg, shortened))
    }
    pub fn from_url(url: &str) -> Result<Self> {
        let shortened = url::shorten_url(url)?;
        let name = url::url_name(url)?;
        let pkg = Flask::new_pkg(&name, url)?;
        Ok(Flask(pkg, shortened))
    }
    pub fn from_pkg(pkg: Pkg) -> Result<Self> {
        let shortened = url::shorten_url(&pkg.url)?;
        Ok(Flask(pkg, shortened))
    }
}

impl Flask {
    fn new_pkg(name: &str, url: &str) -> Result<Pkg> {
        let mut pkg = Pkg::new(name, url);
        pkg.desc = format!("flasks from {url}");
        pkg.group = constants::FLASK_DIR_NAME.to_string();
        pkg.fill()?;
        Ok(pkg)
    }
}

impl Flask {
    pub fn name(&self) -> String {
        self.pkg().name.clone()
    }
    pub fn url(&self) -> String {
        self.1.clone()
    }
    pub fn pkg(&self) -> &Pkg {
        &self.0
    }

    pub fn full_url(&self) -> Result<String> {
        let url = &self.url();
        let full = url::extend_url(url)?;
        Ok(full)
    }
}

impl Flask {
    pub fn link(&self) -> Result<()> {
        // NOTE: creates link `flask/owner.repo -> repo/owner.repo/flasks`
        let name = &self.name();
        io::link(&path::flask_dir(name), &path::gr(name))?;
        Ok(())
    }

    // pull from git repo
    pub fn update(&self) -> Result<()> {
        let name = &self.name();
        if path::repo_exists(name) {
            exec::pull(name)
        } else {
            exec::clone(&self.full_url()?, name)
        }
    }
}

mod test {
    #![allow(unused_imports)]
    use crate::prelude::*;

    use super::Flask;

    #[test]
    fn new() -> Result<()> {
        let (name, url) = ("comfysage.core", "comfysage/core");
        let flask = Flask::new(name, url)?;
        assert_eq!(flask.pkg().name, name);
        assert_eq!(flask.url(), url);

        let (name, url) = ("core", "comfysage/core");
        let flask = Flask::new(name, url)?;
        assert_eq!(flask.pkg().name, name);
        assert_eq!(flask.url(), url);

        Ok(())
    }
    #[test]
    fn from_url() -> Result<()> {
        let (name, url) = ("comfysage.core", "comfysage/core");
        let flask = Flask::from_url(url)?;
        assert_eq!(flask.pkg().name, name);
        assert_eq!(flask.url(), url);

        let (name, url) = ("comfysage.core", "comfysage/core");
        let flask = Flask::from_url(url)?;
        assert_eq!(flask.pkg().name, name);
        assert_eq!(flask.url(), url);

        Ok(())
    }
    #[test]
    fn from_pkg() -> Result<()> {
        let (name, url) = ("comfysage.core", "comfysage/core");
        let flask = Flask::new(name, url)?;
        assert_eq!(flask.pkg().name, name);
        assert_eq!(flask.url(), url);

        let pkg = flask.pkg().clone();
        let flask_pkg = Flask::from_pkg(pkg)?;
        assert_eq!(flask_pkg.pkg().name, name);
        assert_eq!(flask_pkg.url(), url);

        let (name, url) = ("core", "comfysage/core");
        let flask = Flask::new(name, url)?;
        assert_eq!(flask.pkg().name, name);
        assert_eq!(flask.url(), url);

        let pkg = flask.pkg().clone();
        let flask_pkg = Flask::from_pkg(pkg)?;
        assert_eq!(flask_pkg.pkg().name, name);
        assert_eq!(flask_pkg.url(), url);

        Ok(())
    }
}
