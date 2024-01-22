// A library to parse PKGBUILD files with Rust and Nom

use nom::{
    branch::permutation,
    bytes::complete::{tag, take_until, take_while1},
    character::complete::space0,
    IResult,
};

#[derive(Debug, PartialEq)]
pub struct PkgBuild {
    //mandatory fields
    pub pkgname: String,
    pub pkgdesc: String,
    pub url: String,
}

impl PkgBuild {
    // parsing mandatory fields in any order
    pub fn parse(input: &str) -> IResult<&str, PkgBuild> {
        permutation((
            PkgBuild::parse_pkgname,
            PkgBuild::parse_pkgdesc,
            PkgBuild::parse_url,
        ))(input)
        .map(|(next_input, (pkgname, pkgdesc, url))| {
            (
                next_input,
                PkgBuild {
                    pkgname,
                    pkgdesc,
                    url,
                },
            )
        })
    }

    fn parse_field<'a>(input: &'a str, field: &str) -> IResult<&'a str, String> {
        let (input, _) = tag(field)(input)?;
        let (input, _) = space0(input)?;
        let (input, _) = tag("=")(input)?;
        let (input, _) = space0(input)?;
        let (input, value) = take_until("\n")(input)?;
        let (input, _) = tag("\n")(input)?;
        Ok((input, value.to_string().trim_matches('"').to_string()))
    }

    fn parse_pkgname(input: &str) -> IResult<&str, String> {
        Self::parse_field(input, "pkgname")
    }

    fn parse_pkgdesc(input: &str) -> IResult<&str, String> {
        Self::parse_field(input, "pkgdesc")
    }

    fn parse_url(input: &str) -> IResult<&str, String> {
        Self::parse_field(input, "url")
    }

    pub fn to_string(&self) -> String {
        format!("pkgname=\"{}\"\nurl=\"{}\"\npkgdesc=\"{}\"", self.pkgname, self.url, self.pkgdesc)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn pkgname() {
        let input = "pkgname=foo\n";
        let expected = "foo";
        let (input, pkgname) = super::PkgBuild::parse_pkgname(input).unwrap();
        assert_eq!(input, "");
        assert_eq!(pkgname, expected);
    }

    #[test]
    fn pkgdesc() {
        let input = "pkgdesc=bar\n";
        let expected = "bar";
        let (input, pkgdesc) = super::PkgBuild::parse_pkgdesc(input).unwrap();
        assert_eq!(input, "");
        assert_eq!(pkgdesc, expected);
    }

    #[test]
    fn url() {
        let input = "url=foo/bar\n";
        let expected = "foo/bar";
        let (input, url) = super::PkgBuild::parse_url(input).unwrap();
        assert_eq!(input, "");
        assert_eq!(url, expected);
    }

    #[test]
    fn pkgbuild() {
        let input = "pkgname=foo\npkgdesc=bar\nurl=foo/bar\n";
        let expected = super::PkgBuild {
            pkgname: "foo".to_string(),
            pkgdesc: "bar".to_string(),
            url: "foo/bar".to_string(),
        };
        let (input, pkgbuild) = super::PkgBuild::parse(input).unwrap();
        assert_eq!(input, "");
        assert_eq!(pkgbuild, expected);
    }
}

