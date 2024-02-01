use crate::util;
use crate::prelude::*;

use self::cmd::{clone_cmd, fetch_cmd, log_cmd, root_cmd, curl_cmd, pull_cmd};
use self::run::{run_one, run};

pub mod cmd;
mod run;
pub mod pkg;

pub fn clone(url: &str, name: &str) -> Result<()> {
  run_one(clone_cmd(url, &util::path::repo(name)), &util::constants::REPO_DIR)
}

pub fn fetch(name: &str) -> Result<()> {
  run(fetch_cmd(), &util::path::repo(name))
}

pub fn pull(name: &str) -> Result<()> {
  run(pull_cmd(), &util::path::repo(name))
}

pub fn changelog(name: &str) -> Result<()> {
  run_one(log_cmd(), &util::path::repo(name))
}

pub fn root(name: &str, path: &str, prefix: &str) -> Result<()> {
  run_one(root_cmd(name, path, prefix)?, &util::constants::ROOT_DIR)
}

pub fn curl(url: &str, file: &str) -> Result<()> {
  run_one(curl_cmd(url, file), &util::constants::SAKU_DIR)
}

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
