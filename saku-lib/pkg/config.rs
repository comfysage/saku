use std::fs;
use serde::{Deserialize, Serialize};

use crate::prelude::*;
use crate::util::constants;
use crate::util::filepath;

#[derive(Serialize, Deserialize)]
pub struct ConfigMain {
    // whether to update saku on `saku update`; true means no update
    pub frozen_update: bool,
    // whether to cleanup on `saku install`; true means no cleanup
    pub no_install_cleanup: bool,
}

impl ConfigMain {
    pub fn default() -> Self {
        Self {
            frozen_update: false,
            no_install_cleanup: false,
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct Config {
    pub main: ConfigMain,
}

impl Config {
    pub fn path() -> Result<String> {
        if let Some(path) = directories::ProjectDirs::from("dev", "crispybaccoon", "saku") {
            let dir = path.config_dir();
            let path = filepath::join(dir.to_str().ok_or(make_err!())?, &*constants::CONFIG_NAME);
            return Ok(path);
        } else {
            return Err(make_err!(IO, "no config dir."));
        }
    }
    pub fn new() -> Result<Self> {
        let path = Self::path()?;
        if let Some(content) = fs::read_to_string(path).ok() {
            let config: Self = toml::from_str(&content).unwrap_or(Self::default());
            Ok(config)
        } else {
            return Ok(Self::default());
        }
    }
    pub fn to_string(&self) -> Result<String> {
        toml::to_string(self).map_err(|_| make_err!(Parse, "couldn't create toml from string"))
    }
}

impl Default for Config {
    fn default() -> Self {
        Self {
            main: ConfigMain::default(),
        }
    }
}
