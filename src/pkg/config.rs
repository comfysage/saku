use std::fs;

use serde::Deserialize;

use crate::{Error, Result};

#[derive(Deserialize)]
pub struct ConfigMain {
    pub frozen_update: bool,
}

impl ConfigMain {
    pub fn default() -> Self {
        Self {
            frozen_update: false,
        }
    }
}

#[derive(Deserialize)]
pub struct Config {
    pub main: ConfigMain,
}

impl Config {
    pub fn new() -> Result<Self> {
        if let Some(path) = directories::ProjectDirs::from("dev", "crispybaccoon", "saku") {
            if let Some(content) = fs::read_to_string(path.config_dir()).ok() {
                let config: Self = toml::from_str(&content).unwrap_or(Self::default());
                Ok(config)
            } else {
                return Ok(Self::default());
            }
        } else {
            Err(make_err!(IO, "no config dir."))
        }
    }
}

impl Default for Config {
    fn default() -> Self {
        Self {
            main: ConfigMain::default(),
        }
    }
}
