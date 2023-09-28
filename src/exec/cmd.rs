use std::path::Path;

use crate::util;
use crate::Error;

pub fn clone_cmd(url: &str, out: &str) -> String {
  format!("git clone --filter=blob:none {} {}", url, out)
}

pub fn fetch_cmd() -> Vec<String> {
  vec!["git fetch -q".to_string(), "git checkout -q".to_string()]
}

pub fn log_cmd() -> String {
  "git -c pager.show=false show --format=' - %C(yellow)%h%C(reset) %<(80,trunc)%s' -q @@{1}..@@{0}".to_string()
}

pub fn install_cmd(src: &str, dst: &str, perm: i64) -> String {
  let mode = perm.to_string();
  format!("install -m {mode} {src} {dst}")
}

pub fn root_cmd(name: &str, path: &str, prefix: &str) -> Result<String, Error> {
  if path.len()  == 0 || prefix.len() == 0 {
    return Err(Error::Missing("root struct has missing properties".to_string()))
  }

  let src = util::path::repo_file(&name, &path);
  let dst = util::path::root_file(&prefix, &path);

  if !Path::new(&src).exists() {
   return  Err(Error::NotFound(format!("root file '{}' not found", src)))
  }

  // create util::path_root_dir(prefix) directory

  if prefix == "bin" {
    return Ok(install_cmd(&src, &dst, 0755))
  } else {
    return Ok(install_cmd(&src, &dst, 0644))
  }
}

pub fn ln_cmd(target: &str, file: &str) -> String {
    format!("ln -s {target} {file}")
}

pub fn curl_cmd(url: &str, file: &str) -> String {
    format!("curl -fsSL {url} -o {file}")
}
