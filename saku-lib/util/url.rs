use crate::prelude::*;

use regex::Regex;

pub fn extend_url(url: &str) -> Result<String> {
    let mut extended = String::default();

    let mut founddb = false;

    for s in url.chars().into_iter() {
        if s == '.' {
            extended = format!("https://{}", url);
            return Ok(extended);
        }

        if s == '/' {
            // found protocol
            if founddb {
                return Ok(url.to_string());
            }

            // potential address already catched
            extended = format!("https://github.com/{}", url);
            return Ok(extended);
        }

        if s == ':' {
            founddb = true
        }
    }
    Ok(extended)
}

// changes url into human readable format
// NOTE: this should not be used for networking
pub fn shorten_url(url: &str) -> Result<String> {
    // let long_url = format!("{url}");
    // let mut parts = long_url.split('/');
    // let last = parts
    //     .nth_back(0)
    //     .ok_or(make_err!(Missing, "url for flask is missing"))?;
    // let name = path::remove_extension(last.to_string());

    let mut shortened = format!("{url}");

    let protocol_re = Regex::new(r"\w+://")?;
    let mut parts: Vec<&str> = protocol_re.split(&shortened).collect();
    if parts.len() > 1 {
        parts.remove(0);
        shortened = parts.join("");
    }

    match shortened.strip_prefix("github.com/") {
        Some(s) => {
            shortened = s.to_string();
        }
        None => {}
    };
    match shortened.strip_suffix(".git") {
        Some(s) => {
            shortened = s.to_string();
        }
        None => {}
    };

    Ok(shortened)
}

// changes url into
pub fn url_name(url: &str) -> Result<String> {
    let mut name = shorten_url(url)?;

    name = name.replace("/", ".");

    Ok(name)
}

mod test {
    #[allow(unused_imports)]
    use crate::prelude::*;

    #[test]
    fn extend_url() -> Result<()> {
        assert_eq!(
            super::extend_url("crispybaccoon/pkg")?,
            "https://github.com/crispybaccoon/pkg"
        );
        assert_eq!(
            super::extend_url("https://github.com/crispybaccoon/pkg")?,
            "https://github.com/crispybaccoon/pkg"
        );
        assert_eq!(
            super::extend_url("aur.archlinux.org/pkg")?,
            "https://aur.archlinux.org/pkg"
        );

        Ok(())
    }

    #[test]
    fn shorten_url() -> Result<()> {
        assert_eq!(
            super::shorten_url("https://aur.archlinux.org/pkg.git")?,
            "aur.archlinux.org/pkg"
        );
        assert_eq!(
            super::shorten_url("aur.archlinux.org/pkg.git")?,
            "aur.archlinux.org/pkg"
        );
        assert_eq!(
            super::shorten_url("aur.archlinux.org/pkg")?,
            "aur.archlinux.org/pkg"
        );

        assert_eq!(
            super::shorten_url("https://github.com/crispybaccoon/pkg.git")?,
            "crispybaccoon/pkg"
        );
        assert_eq!(
            super::shorten_url("github.com/crispybaccoon/pkg.git")?,
            "crispybaccoon/pkg"
        );
        assert_eq!(
            super::shorten_url("github.com/crispybaccoon/pkg")?,
            "crispybaccoon/pkg"
        );
        assert_eq!(
            super::shorten_url("crispybaccoon/pkg")?,
            "crispybaccoon/pkg"
        );
        Ok(())
    }

    #[test]
    fn url_name() -> Result<()> {
        assert_eq!(super::url_name("crispybaccoon/core")?, "crispybaccoon.core");
        assert_eq!(
            super::url_name("aur.archlinux.org/pkg")?,
            "aur.archlinux.org.pkg"
        );
        Ok(())
    }
}
