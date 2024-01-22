use crate::prelude::*;
use crate::util;
use crate::util::constants;

use super::run::run_one;

pub fn init_cmd(name: &str, group: &str, function: &str) -> String {
    format!(". {} {group} {name} && {function}", *constants::INIT_FILE)
}

pub enum Cmd {
    Install,
    Build,
    Upgrade,
    CleanUp,
}

pub fn run(cmd: Cmd, name: &str, group: &str) -> Result<()> {
    let arg = match cmd {
        Cmd::Install => "_install",
        Cmd::Build => "_build",
        Cmd::Upgrade => "_upgrade",
        Cmd::CleanUp => "_cleanup",
    };
    run_one(init_cmd(name, group, arg), &util::path::repo(name))
}
