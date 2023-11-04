use std::fs;

use serde::{Serialize, Deserialize};

use crate::prelude::*;

#[derive(Serialize, Deserialize)]
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

#[derive(Serialize, Deserialize)]
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
