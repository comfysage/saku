use crate::prelude::*;

use util::constants;
use util::path;

use super::run::run;

pub enum Cmd {
    Clone { url: String, name: String },
    Fetch { name: String },
    Pull { name: String },
    Log { name: String },
    Link { target: String, path: String },
    Unlink { path: String },
}

impl Cmd {
    pub fn get_cmd(&self) -> Vec<String> {
        match self {
            Cmd::Clone { url, name } => vec![format!(
                "git clone --filter=blob:none {} {}",
                url,
                path::repo(name)
            )],
            Cmd::Fetch { .. } => vec!["git fetch -q".to_string(), "git checkout -q".to_string()],
            Cmd::Pull { .. } => vec!["git pull".to_string()],
            Cmd::Log { .. } => vec!["git -c pager.show=false show --format=' - %C(yellow)%h%C(reset) %<(80,trunc)%s' -q @@{1}..@@{0}".to_string()],
            Cmd::Link { target, path } => vec![format!("ln -s {target} {path}")],
            Cmd::Unlink { path } => vec![format!("unlink {path}")],
        }
    }
    pub fn get_pwd(&self) -> String {
        match self {
            Cmd::Clone { .. } => constants::REPO_DIR.to_string(),
            Cmd::Fetch { name } => path::repo(name),
            Cmd::Pull { name } => path::repo(name),
            Cmd::Log { name } => path::repo(name),
            Cmd::Link { .. } => constants::SAKU_DIR.to_string(),
            Cmd::Unlink { .. } => constants::SAKU_DIR.to_string(),
        }
    }
    pub fn exec(self) -> Result<()> {
        run(self.get_cmd(), &self.get_pwd())
    }
}
