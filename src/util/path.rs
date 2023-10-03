use std::fs;

use super::constants::{ROOT_DIR, PKG_DIR, HAYASHI_DIR, CONFIG_NAME, STORE_NAME, REPO_DIR, REPO_SEED, FLASK_DIR, FLASK_DIR_NAME};

use crate::Error;
use super::filepath;

pub fn exists(path: &str) -> bool {
    filepath::exists(path)
}

pub fn config() -> String {
	filepath::join(&HAYASHI_DIR, &CONFIG_NAME)
}

pub fn store_file() -> String {
	filepath::join(&HAYASHI_DIR, &STORE_NAME)
}

pub fn gr(name: &str) -> String {
	if name.len() == 0 {
		panic!("argument for group name was nil");
	}
	filepath::join(&PKG_DIR, name)
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
	let p: &str;

	if path.starts_with(".") || path.starts_with("/") {
		// path is either relative or absolute
        let path_bind = filepath::extend(&path)?;
        p = &path_bind;

		// path points to either a file or a directory
		if filepath::is_dir(p) {
			dir = true;
		}

		return Ok( (p.to_string(), dir) );
	}

	// path is a name
    let path_bind = pkg_search(&path)?;
    p = &path_bind;

	Ok(( p.to_string(), false ))
}

pub fn pkg_search(name: &str) -> Result<String, Error> {
	if pkg_exists("core", name) {
		return Ok(path_pkg("core", name));
	}

	for d in fs::read_dir(&*PKG_DIR)? {
        let d = d?;
        let d_path_bind = d.path();
        let d_path = match d_path_bind.to_str() {
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

pub fn repo_seed(path: &str) -> String {
	if path.len() == 0 {
		panic!("argument for path was nil")
	}
	filepath::join(path, &REPO_SEED)
}

pub fn repo(name: &str) -> String {
	if name.len() == 0 {
		panic!("argument for repo name was nil")
	}
	filepath::join(&REPO_DIR, name)
}

pub fn repo_exists(name: &str) -> bool {
    filepath::exists(&repo(name))
}

pub fn repo_file(name: &str, path: &str) -> String {
	if path.len() == 0 {
		panic!("argument for file name was nil")
	}
	filepath::join(&repo(name), path)
}

pub fn root_dir(dir: &str) -> String {
	if dir.len() == 0 {
		panic!("argument for type dir was nil")
	}
	filepath::join(&ROOT_DIR, dir)
}

pub fn root_file(dir: &str, path: &str) -> String {
	if path.len() == 0 {
		panic!("argument for path was nil")
	}
	let file = filepath::base_name(&path).unwrap();
	if file == "." {
		panic!("argument for file was invalid")
	}
	filepath::join(&root_dir(dir), &file)
}

pub fn flask(name: &str) -> String {
	if name.len() == 0 {
		panic!("argument for flask name was nil")
	}
    filepath::join(&FLASK_DIR, &pkg_name(name))
}

pub fn flask_dir(name: &str) -> String {
	if name.len() == 0 {
		panic!("argument for flask name was nil")
	}
    filepath::join(&repo(name), &FLASK_DIR_NAME)
}

pub fn flasks() -> Result<Vec<String>, Error> {
    let mut files = vec![];
    
    for f in fs::read_dir(&*FLASK_DIR)? {
        let f = f?;
        let f_path_bind = f.path();
        let f_path = match f_path_bind.to_str() {
            Some(s) => Ok(s),
            None => Err(Error::Unexpected),
        }?;
        let name = filepath::base_name(f_path)?;
        files.push(name);
    }

    Ok(files)
}
