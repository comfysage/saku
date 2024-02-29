use crate::util;
use crate::prelude::*;

pub use self::cmd::Cmd;
use self::run::run;

pub mod cmd;
mod run;
pub mod pkg;

pub fn install(name: &str, group: &str) -> Result<()> {
  pkg::run(pkg::Cmd::Install, name, group)
}

pub fn build(name: &str, group: &str) -> Result<()> {
  pkg::run(pkg::Cmd::Build, name, group)
}

pub fn upgrade(name: &str, group: &str) -> Result<()> {
  pkg::run(pkg::Cmd::Upgrade, name, group)
}

pub fn cleanup(name: &str, group: &str) -> Result<()> {
  pkg::run(pkg::Cmd::CleanUp, name, group)
}

pub fn run_in_repo(name: &str, cmd: Vec<String>) -> Result<()> {
  run(cmd, &util::path::repo(name))
}

pub fn run_in_root(prefix: &str, cmd: Vec<String>) -> Result<()> {
  run(cmd, &util::path::root_dir(prefix))
}
