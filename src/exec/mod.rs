use crate::{Error, util};

use self::{cmd::{clone_cmd, fetch_cmd, log_cmd, pack_cmd, curl_cmd}, run::{run_one, run}};

pub mod cmd;
mod run;

pub fn clone(url: &str, name: &str) -> Result<(), Error> {
  run_one(clone_cmd(url, &util::path::repo(name)), &util::constants::REPO_DIR)
}

pub fn fetch(name: &str) -> Result<(), Error> {
  run(fetch_cmd(), &util::path::repo(name))
}

pub fn changelof(name: &str) -> Result<(), Error> {
  run_one(log_cmd(), &util::path::repo(name))
}

pub fn pack(name: &str, path: &str, prefix: &str) -> Result<(), Error> {
  run_one(pack_cmd(name, path, prefix)?, &util::constants::ROOT_DIR)
}

pub fn curl(url: &str, file: &str) -> Result<(), Error> {
  run_one(curl_cmd(url, file), &util::constants::HAYASHI_DIR)
}

pub fn run_in_repo(name: &str, cmd: Vec<String>) -> Result<(), Error> {
  run(cmd, &util::path::repo(name))
}

pub fn run_in_pack(prefix: &str, cmd: Vec<String>) -> Result<(), Error> {
  run(cmd, &util::path::pack_dir(prefix))
}
