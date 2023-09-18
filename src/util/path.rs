use std::fs;

use super::constants::{PACK_ROOT, PKG_ROOT, HAYASHI_ROOT, CONFIG_NAME, STORE_NAME, REPO_ROOT, REPO_PKG};

use crate::Error;
use super::filepath;

pub fn exists(path: &str) -> bool {
    filepath::exists(path)
}

pub fn config() -> String {
	filepath::join(HAYASHI_ROOT, CONFIG_NAME)
}

pub fn store_file() -> String {
	filepath::join(HAYASHI_ROOT, STORE_NAME)
}

pub fn gr(name: &str) -> String {
	if name.len() == 0 {
		panic!("argument for group name was nil");
	}
	filepath::join(PKG_ROOT, name)
}

fn pkg_name(name: &str) -> String {
	if name.len() == 0 {
		panic!("argument for pkg name was nil");
	}
	format!("{name}.yaml")
}

pub fn remove_extension(name: String) -> String {
	if name.len() == 0 {
		panic!("argument for pkg name was nil")
	}
  
  let mut parts = name.split('.');
  parts.next_back();
  String::from(parts.collect::<Vec<&str>>().join("."))
}

pub fn path_pkg(group: &str, name: &str) -> String {
	let gr = gr(group);
	let pkg_name = pkg_name(name);
	filepath::join(&gr, &pkg_name)
}

pub fn pkg_exists(gr: &str, name: &str) -> bool {
	let p = path_pkg(gr, name);
	exists(&p)
}

pub fn path_determine(path: String) -> Result<(String, bool), Error> {
	let mut dir = false;
	let p;

	if path.starts_with(".") || path.starts_with("/") {
		// path is either relative or absolute
		p = &filepath::extend(&path)?;

		// path points to either a file or a directory
		if filepath::is_dir(p) {
			dir = true;
		}

		return Ok( (p.to_string(), dir) );
	}

	// path is a name
	p = &pkg_search(&path)?;

	Ok(( p.to_string(), false ))
}

pub fn pkg_search(name: &str) -> Result<String, Error> {
	if pkg_exists("core", name) {
		return Ok(path_pkg("core", name));
	}

	for d in fs::read_dir(PKG_ROOT)? {
        let d = d?;
        let d_path = match d.path().to_str() {
            Some(s) => Ok(s),
            None => Err(Error::Unexpected),
        }?;
		if !filepath::is_dir(d_path) {
			continue
		}
		let gr_name = filepath::base_name(d_path)?;

		if pkg_exists(&gr_name, name) {
			return Ok(path_pkg(&gr_name, name));
		}
	}

	Err(make_err!(NotFound, "pkg not found."))
}

pub fn repo_pkg(path: &str) -> String {
	if path.len() == 0 {
		panic!("argument for path was nil")
	}
	filepath::join(path, REPO_PKG)
}

pub fn repo(name: &str) -> String {
	if name.len() == 0 {
		panic!("argument for repo name was nil")
	}
	filepath::join(REPO_ROOT, name)
}

pub fn repo_file(name: &str, path: &str) -> String {
	if path.len() == 0 {
		panic!("argument for file name was nil")
	}
	filepath::join(&repo(name), path)
}

pub fn pack_dir(dir: &str) -> String {
	if dir.len() == 0 {
		panic!("argument for type dir was nil")
	}
	filepath::join(PACK_ROOT, dir)
}

pub fn pack_file(dir: &str, path: &str) -> String {
	if path.len() == 0 {
		panic!("argument for path was nil")
	}
	let file = filepath::base_name(&path).unwrap();
	if file == "." {
		panic!("argument for file was invalid")
	}
	filepath::join(&pack_dir(dir), &file)
}
