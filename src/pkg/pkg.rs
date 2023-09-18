use crate::pkg::pack::Pack;
use crate::util::{path, self, filepath};
use crate::Error;

pub struct Pkg {
    pub name: String,
    pub desc: String,
    pub url: String,
    pub group: String,
    pub install: Vec<String>,
    pub update: Vec<String>,
    pub pack: Vec<Pack>,
    path: Option<String>,
}

// meta
impl Pkg {
    pub fn fill(&mut self) -> Result<(), Error> {
        self.infer_group(self.get_path()?)?;
        // infer url
        let mut finish = false;
        let mut foundslash = false;
        let mut founddb = false;
        self.url.chars().into_iter().for_each(|s| {
            if finish {
                return;
            }
            if s == '.' {
                // founddot = true

                // found address without protocol
                if foundslash == false {
                    self.url = format!("https://{}", self.url);
                    finish = true;
                    return;
                }

                return;
            }

            if s == '/' {
                foundslash = true;
                finish = true;

                // found protocol
                if founddb {
                    return;
                }

                // potential address already catched
                self.url = format!("https://github.com/{}", self.url);
                return;
            }

            if s == ':' {
                founddb = true
            }
        });

        Ok(())
    }
    pub fn infer_group(&mut self, path: String) -> Result<(), Error> {
        if self.group.len() > 0 {
            return Ok(());
        }
        let cwd = util::cli::get_cwd()?;
        let abspath = filepath::abs(&filepath::join(&cwd, &path));

        let sp = abspath.split("/").collect::<Vec<&str>>();
        if sp.len() < 2 {
            return Err(make_err!(NotFound, "path to pkg not long enough"));
        }
        self.group = sp[sp.len() - 2].to_string();
        Ok(())
    }
    pub fn get_path(&mut self) -> Result<String, Error> {
        match self.path {
            Some(s) => Ok(s),
            None => Ok(path::path_pkg(&self.group, &self.name)),
        }
    }
    pub fn pack(&self) -> Result<(), Error> {
        Ok(())
    }
}

// data
impl Pkg {
    pub fn from_string() -> Pkg {
        todo!()
    }
    pub fn to_string(&self) -> String {
        todo!()
    }
}

// safeguard
impl Pkg {
    // error checking for pkg to avoid unwanted behaviour
    pub fn safe_guard(&self) -> Result<(), Error> {
        // require pkg name
        if self.name.len() < 1 {
            return Err(make_err!(Missing, "no pkg name specified."));
        }

        // require url
        if self.url.len() < 1 {
            return Err(make_err!(Missing, "no url specified."));
        }

        Ok(())
    }
}
