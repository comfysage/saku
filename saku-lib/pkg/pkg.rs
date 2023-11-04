use crate::pkg::root::Root;
use crate::prelude::*;
use crate::util::url::extend_url;
use crate::util::{self, filepath, path};
use crate::util::{msg, url};

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Pkg {
    pub name: String,
    pub desc: String,
    pub url: String,
    pub group: String,
    pub install: Vec<String>,
    pub update: Vec<String>,
    pub root: Vec<Root>,
    path: Option<String>,
}

impl Pkg {
    pub fn new(name: &str, url: &str) -> Self {
        Self {
            name: format!("{name}"),
            url: format!("{url}"),
            desc: format!(""),
            group: format!(""),
            install: vec![],
            update: vec![],
            root: vec![],
            path: None,
        }
    }
}

// meta
impl Pkg {
    pub fn fill(&mut self) -> Result<()> {
        let local_path = self.get_path()?;
        self.infer_group(local_path)?;
        // infer url
        let url = extend_url(&self.url)?;
        self.url = url;

        Ok(())
    }
    pub fn infer_group(&mut self, path: String) -> Result<()> {
        if self.group.len() > 0 {
            return Ok(());
        }
        let cwd = util::cli::get_cwd()?;
        let abspath = filepath::abs(&filepath::join(&cwd, &path));

        let abs_bind = abspath?;
        let sp = abs_bind.split("/").collect::<Vec<&str>>();
        if sp.len() < 2 {
            return Err(make_err!(NotFound, "path to pkg not long enough"));
        }
        self.group = sp[sp.len() - 2].to_string();
        Ok(())
    }
    pub fn get_path(&mut self) -> Result<String> {
        match &self.path {
            Some(s) => Ok(s.to_string()),
            None => Ok(path::path_pkg(&self.group, &self.name)),
        }
    }
}

// data
impl Pkg {
    pub fn from_string(str: String) -> Result<Pkg> {
        serde_yaml::from_str(&str).map_err(|_| make_err!())
    }
    pub fn to_string(&self) -> Result<String> {
        serde_yaml::to_string(self).map_err(|_| make_err!())
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
        if self.install.len() > 0 {
            println!("bash");
            for s in &self.install {
                println!("  {}", s);
            }
        }
        Ok(())
    }
}
