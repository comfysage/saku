use crate::pkg::data::get_pkg;
use crate::pkg::pkg::Pkg;
use crate::util::{msg, url};
use crate::Result;

pub fn show(pkg_name: &String) -> Result<()> {
    let pkg = get_pkg(pkg_name)?;
    pkg.show()?;
    Ok(())
}

impl Pkg {
    pub fn show(&self) -> Result<()> {
        println!("{}", msg::general::present_name(&self.name, &self.group));

        if self.desc.len() > 0 {
            println!("{}", self.desc);
        }
        if self.url.len() > 0 {
            println!(
                "url  {}",
                msg::general::url_f(&url::shorten_url(&self.url)?)
            );
        }
        if self.install.len() > 0 {
            println!("bash");
            for s in &self.install {
                println!("  {}", s);
            }
        }
        Ok(())
    }
}
