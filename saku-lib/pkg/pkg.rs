use crate::prelude::*;
use crate::util::url::extend_url;
use crate::util::{self, filepath, path};
use crate::util::{msg, url};

use crate::pkg::flaskfile::PkgBuild;

use serde::{Serialize, Deserialize};

/// a package struct
///
/// - *name*: name of the package
/// - *desc*: short description
/// - *url*: url of the repo
/// - *group*: flask group of the package
/// - *path*: path to the flask file of the package
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Pkg {
    #[serde(rename = "pkg")]
    pub name: String,
    pub desc: String,
    pub url: String,

    #[serde(skip_serializing, default)]
    pub group: String,
    #[serde(default)]
    pub path: Option<String>,
}

impl Pkg {
    /// Create a new *Pkg*
    ///
    /// - *name*: name of the package
    /// - *url*: url of the repo
    pub fn new(name: &str, url: &str) -> Self {
        Self {
            name: format!("{name}"),
            url: format!("{url}"),
            desc: format!(""),
            group: format!(""),
            path: None,
        }
    }
}

// meta
impl Pkg {
    /// Auto-fill *Pkg* fields
    ///
    /// - *group* is infered from the path to the flask file
    /// - *url* is extended using `util::url::extend_url`
    pub fn fill(&mut self) -> Result<()> {
        let local_path = self.get_path()?;
        self.infer_group(local_path)?;
        // infer url
        let url = extend_url(&self.url)?;
        self.url = url;

        Ok(())
    }
    /// Infer group of a package
    ///
    /// group is set to the dirname of the flask file of the package
    pub fn infer_group(&mut self, path: String) -> Result<()> {
        if self.group.len() > 0 {
            return Ok(());
        }
        let cwd = util::cli::get_cwd()?;
        let abspath = filepath::abs(&filepath::join(&cwd, &path));

        let abs_bind = abspath?;
        let sp = abs_bind.split("/").collect::<Vec<&str>>();

        let required_index = 2;
        if sp.len() < required_index {
            return Err(make_err!(NotFound, "path to pkg not long enough"));
        }
        self.group = sp[sp.len() - required_index].to_string();
        Ok(())
    }
    pub fn get_path(&mut self) -> Result<String> {
        match &self.path {
            Some(s) => Ok(s.to_string()),
            None => {
                if self.group.len() > 0 {
                    Ok(path::path_pkg(&self.group, &self.name))
                } else{
                    path::pkg_search(&self.name)
                }
            },
        }
    }
}

// data
impl Pkg {
    pub fn from_string(str: String) -> Result<Self> {
        let (_, build) = PkgBuild::parse(&str).map_err(|e| {
            let e = e.to_string();
            make_err!(Parse, "{e}")
            })?;
        let pkg = Self {
            name: build.pkgname,
            url: build.url,
            desc: build.pkgdesc,
            group: "".to_string(),
            path: None,
        };
        Ok(pkg)
    }
    pub fn to_string(&self) -> Result<String> {
        let build = PkgBuild {
            pkgname: self.name.to_string(),
            pkgdesc: self.desc.to_string(),
            url: self.url.to_string(),
        };
        Ok(PkgBuild::to_string(&build))
    }
}

// safeguard
impl Pkg {
    // error checking for pkg to avoid unwanted behaviour
    pub fn safe_guard(&self) -> Result<()> {
        // require pkg name
        if self.name.len() < 1 {
            return Err(make_err!(Missing, "no pkg name specified."));
        }

        // require url
        if self.url.len() < 1 {
            return Err(make_err!(Missing, "no url specified."));
        }

        Ok(())
    }
}

impl Pkg {
    pub fn show(&self) -> Result<()> {
        println!("{}", msg::general::present_name(&self.name, &self.group));

        if self.desc.len() > 0 {
            println!("{}", self.desc);
        }
        if self.url.len() > 0 {
            println!(
                "url  {}",
                msg::general::url_f(&url::shorten_url(&self.url)?)
            );
        }
        Ok(())
    }
}
