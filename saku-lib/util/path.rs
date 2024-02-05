use std::fs;

use super::constants;

use crate::prelude::*;
use super::filepath;

use glob::glob;

pub fn exists(path: &str) -> bool {
    filepath::exists(path)
}

pub fn store_file() -> String {
	filepath::join(&constants::SAKU_DIR, &constants::STORE_NAME)
}

pub fn gr(name: &str) -> String {
	if name.len() == 0 {
		panic!("argument for group name was nil");
	}
	filepath::join(&constants::PKG_DIR, name)
}

fn pkg_name(name: &str) -> String {
	if name.len() == 0 {
		panic!("argument for pkg name was nil");
	}
	format!("{name}.fl")
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
	filepath::join(&gr, &pkg_name(&name))
}

pub fn pkg_exists(gr: &str, name: &str) -> bool {
	let p = path_pkg(gr, name);
	exists(&p)
}

pub fn path_determine(path: String) -> Result<(String, bool)> {
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

pub fn pkg_search(name: &str) -> Result<String> {
	if pkg_exists("core", name) {
		return Ok(path_pkg("core", name));
	}

	for d in fs::read_dir(&*constants::PKG_DIR)? {
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

pub fn pkg_match(pattern: &str) -> Result<Vec<[String;2]>> {
    let mut result = vec![];
    let found_files = pkgs()?;
    let found: Vec<[String;2]> = found_files.iter().map(|s| [s[0].clone(), remove_extension(s[1].clone())]).collect();
    drop(found_files);

    for i in 0..found.len() {
        let p = &found[i];
        if p[1].contains(pattern) {
            result.push(p.clone());
        }
    }

    Ok(result)
}

pub fn pkgs() -> Result<Vec<[String;2]>> {
    let mut files = vec![];

    debug!("searching for pkgs in {}", &*constants::PKG_DIR);
    for d in fs::read_dir(&*constants::PKG_DIR)? {
        let d = d?;
        let d_path_bind = d.path();
        let d_path = match d_path_bind.to_str() {
            Some(s) => Ok(s),
            None => Err(Error::Unexpected),
        }?;
        let group = filepath::base_name(d_path)?;
        debug!("found pkg dir {}", &group);
        if group == *constants::FLASK_DIR_NAME {
            continue
        }
        for f in fs::read_dir(d_path)? {
            let f = f?;
            let f_path_bind = f.path();
            let f_path = match f_path_bind.to_str() {
                Some(s) => Ok(s),
                None => Err(Error::Unexpected),
            }?;
            let name = filepath::base_name(f_path)?;
            files.push([ group.clone(), name.clone() ]);
        }
    }

    Ok(files)
}

pub fn repo_seed(path: &str) -> String {
	if path.len() == 0 {
		panic!("argument for path was nil")
	}
	filepath::join(path, &constants::REPO_SEED)
}

pub fn is_repo_seed(path: &Vec<&str>) -> bool {
    if let Some(last_part) = path.last() {
        if last_part == &*constants::REPO_SEED {
            return true
        }
    }
    return false
}

pub fn repo(name: &str) -> String {
	if name.len() == 0 {
		panic!("argument for repo name was nil")
	}
	filepath::join(&constants::REPO_DIR, name)
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
	filepath::join(&constants::ROOT_DIR, dir)
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
    filepath::join(&constants::FLASK_DIR, &pkg_name(name))
}

pub fn flask_dir(name: &str) -> String {
	if name.len() == 0 {
		panic!("argument for flask name was nil")
	}
    filepath::join(&repo(name), &constants::FLASK_DIR_NAME)
}

pub fn flasks() -> Result<Vec<String>> {
    let mut files = vec![];
    
    for f in fs::read_dir(&*constants::FLASK_DIR)? {
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

pub fn get_store_dirs() -> Result<Vec<String>> {
    let mut dirs = vec![];
    for d in fs::read_dir(&*constants::STORE_DIR)? {
        let d = d?;
        let d_path_bind = d.path();
        let d_path = match d_path_bind.to_str() {
            Some(s) => Ok(s),
            None => Err(Error::Unexpected),
        }?;
        let name = filepath::base_name(d_path)?;
        dirs.push(name);
    }
    Ok(dirs)
}

pub fn store_dir(name: &str) -> String {
    filepath::join(&*constants::STORE_DIR, name)
}

pub fn get_stored_files(name: &str) -> Result<Vec<String>> {
    let mut files = vec![];
    for d in fs::read_dir(store_dir(name))? {
        let d = d?;
        let d_path_bind = d.path();
        let d_path = match d_path_bind.to_str() {
            Some(s) => Ok(s),
            None => Err(Error::Unexpected),
        }?;
        for entry in glob(&format!("{d_path}/**/*"))? {
            files.push(entry?.to_str().unwrap().to_string());
        }
    }
    Ok(files)
}
