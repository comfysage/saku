use std::fs;

use serde::Deserialize;

use crate::{ Result, Error };

#[derive(Deserialize)]
pub struct Config {}

impl Config {
    pub fn new() -> Result<Self> {
        if let Some(path) = directories::ProjectDirs::from("dev", "crispybaccoon", "yuki") {
            let content = fs::read_to_string(path.config_dir())?;
            let config: Self = serde_yaml::from_str(&content).unwrap_or(Self::default());
            Ok(config)
        } else {
            Err(make_err!(IO, "no config dir."))
        }
    }
}

impl Default for Config {
    fn default() -> Self {
        Self {}
    }
}
