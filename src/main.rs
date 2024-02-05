use clap::builder::styling;
use saku_lib as saku;
use saku_cli as cli;
use saku::pkg::{config, data};
use saku::pkg::pkg::Pkg;
use saku::util::msg;
use saku::prelude::*;

use clap::{arg, Command};

fn get_commands() -> Command {
    let effects = (styling::Effects::BOLD | styling::Effects::UNDERLINE).clear();
    let styles = styling::Styles::styled()
        .header(styling::AnsiColor::White.on_default() | effects)
        .usage(styling::AnsiColor::White.on_default() | effects)
        .literal(styling::AnsiColor::BrightWhite.on_default() | effects)
        .placeholder(styling::AnsiColor::BrightWhite.on_default() | effects);

    let version = env!("PKG_VERSION");

    Command::new("saku")
        .about("a distro-independent package manager written in Rust.")
        .subcommand_required(true)
        .arg_required_else_help(true)
        .allow_external_subcommands(true)
        .styles(styles)
        .version(version)
        .subcommand(Command::new("env").about("Show environment script"))
        .subcommand(
            Command::new("config")
                .about("Manage global configuration")
                .subcommand_required(true)
                .arg_required_else_help(true)
                .subcommand(Command::new("init").about("Setup saku"))
                .subcommand(Command::new("create").about("Setup saku.toml")),
        )
        .subcommand(
            Command::new("pkg")
                .about("Manage pkg configurations")
                .subcommand_required(true)
                .arg_required_else_help(true)
                .subcommand(
                    Command::new("add")
                        .about("Add pkg configuration")
                        .arg_required_else_help(true)
                        .args([
                            arg!(<NAME> "Package to add"),
                            arg!([URL] "Package url"),
                        ]),
                )
                .subcommand(
                    Command::new("remove")
                        .about("Remove pkg configuration")
                        .arg_required_else_help(true)
                        .arg(arg!(<NAME> "Package to remove")),
                )
                .subcommand(
                    Command::new("show")
                        .about("Show pkg configuration")
                        .arg_required_else_help(true)
                        .arg(arg!(<NAME> "Package to show")),
                ),
        )
        .subcommand(
            Command::new("add")
                .visible_alias("install")
                .about("Install a package")
                .arg_required_else_help(true)
                .arg(arg!(<NAME> ... "Package to install")),
        )
        .subcommand(
            Command::new("upgrade")
                .about("Upgrade a package")
                .arg_required_else_help(true)
                .arg(arg!(<NAME> ... "Package to upgrade")),
        )
        .subcommand(
            Command::new("remove")
                .visible_alias("uninstall")
                .about("Remove a package")
                .arg_required_else_help(true)
                .arg(arg!(<NAME> ... "Package to remove")),
        )
        .subcommand(
            Command::new("update")
                .about("Update flasks")
                .arg(arg!([NAME] ... "Flask to update")),
        )
        .subcommand(
            Command::new("flask")
                .about("Add flasks")
                .arg(arg!([NAME] ... "Flask to add")),
        )
        .subcommand(
            Command::new("show")
                .about("Show package details")
                .arg_required_else_help(true)
                .arg(arg!(<NAME> ... "Package to show")),
        )
        .subcommand(
            Command::new("search")
                .about("search for a package")
                .arg_required_else_help(true)
                .arg(arg!(<NAME> ... "Pattern to search for")),
        )
        .subcommand(
            Command::new("list")
                .about("List flasks")
                .arg(
                    arg!(-i --installed ... "List installed packages")
                )
        )
        .subcommand(
            Command::new("task")
                .about("Run a task for a package")
                .subcommand_required(true)
                .arg_required_else_help(true)
                .subcommand(
                    Command::new("clone")
                        .about("Clone a package")
                        .arg_required_else_help(true)
                        .arg(arg!(<NAME> ... "Package to clone")),
                )
                .subcommand(
                    Command::new("build")
                        .about("Build a package")
                        .arg_required_else_help(true)
                        .arg(arg!(<NAME> ... "Package to build")),
                )
                .subcommand(
                    Command::new("install")
                        .about("Install a package")
                        .arg_required_else_help(true)
                        .arg(arg!(<NAME> ... "Package to install")),
                )
        )
        .subcommand(
            Command::new("changelog")
                .about("show package changelog")
                .arg_required_else_help(false)
                .arg(arg!([NAME] "Package name"))
        )
}

fn main() -> Result<()> {
    saku_logger::init();
    let config = config::Config::new()?;

    let matches = get_commands().get_matches();

    match matches.subcommand() {
        Some(("env", _)) => {
            cli::env::env()?;
            Ok(())
        }
        Some(("config", sub_matches)) => {
            let subcommand = sub_matches.subcommand().unwrap_or(("init", sub_matches));
            match subcommand {
                ("init", _) => cli::config::init(),
                ("create", _) => cli::config::create(),
                (&_, _) => Err(Error::Unexpected),
            }
        }
        Some(("pkg", sub_matches)) => {
            let subcommand = sub_matches.subcommand().unwrap_or(("show", sub_matches));
            match subcommand {
                ("add", sub_matches) => {
                    let name = sub_matches
                        .get_one::<String>("NAME")
                        .ok_or(make_err!(Missing, "no package name specified."))?;
                    let url = sub_matches.get_one::<String>("URL");

                    msg::add(&name);
                    let pkg = Pkg::new(&name, url.map_or("", |url| &url));
                    cli::add::add(&pkg)?;
                    Ok(())
                }
                ("remove", sub_matches) => {
                    let name = sub_matches
                        .get_one::<String>("NAME")
                        .ok_or(make_err!(Missing, "no package name specified."))?;
                    cli::remove::remove(&name)?;
                    Ok(())
                }
                ("show", sub_matches) => {
                    let package = sub_matches.get_one::<String>("NAME");
                    println!("show pkg {package:?}");
                    Ok(())
                }
                (&_, _) => Err(Error::Unexpected),
            }
        }
        Some(("add", sub_matches)) => {
            let paths = sub_matches
                .get_many::<String>("NAME")
                .into_iter()
                .flatten()
                .collect::<Vec<_>>();
            if paths.len() < 1 {
                return Err(make_err!(Missing, "not enough arguments provided"));
            }
            paths
                .iter()
                .map(|name| cli::install::install(name))
                .collect()
        }
        Some(("upgrade", sub_matches)) => {
            let paths = sub_matches
                .get_many::<String>("NAME")
                .into_iter()
                .flatten()
                .collect::<Vec<_>>();
            if paths.len() < 1 {
                return Err(make_err!(Missing, "not enough arguments provided"));
            }
            paths
                .iter()
                .map(|name| cli::upgrade::upgrade(name))
                .collect()
        }
        Some(("remove", sub_matches)) => {
            let paths = sub_matches
                .get_many::<String>("NAME")
                .into_iter()
                .flatten()
                .collect::<Vec<_>>();
            println!("Removing {paths:?}");
            Ok(())
        }
        Some(("update", sub_matches)) => {
            let urls = sub_matches
                .get_many::<String>("NAME")
                .into_iter()
                .flatten()
                .collect::<Vec<_>>();
            if urls.len() < 1 {
                cli::update::update()?;
            } else {
                urls.iter()
                    .map(|url| cli::update::update_flask_from_url(url))
                    .collect::<Result<()>>()?;
            }
            if config.main.frozen_update {
                return Ok(());
            }
            cli::upgrade::upgrade("saku")?;
            Ok(())
        }
        Some(("flask", sub_matches)) => {
            let urls = sub_matches
                .get_many::<String>("NAME")
                .into_iter()
                .flatten()
                .collect::<Vec<_>>();
            if urls.len() < 1 {
                // `saku flask` is currently an alias for `saku update`
                cli::update::update()?;
            } else {
                urls.iter()
                    .map(|url| cli::flask::add(url))
                    .collect::<Result<()>>()?;
            }
            Ok(())
        }
        Some(("show", sub_matches)) => {
            let paths = sub_matches
                .get_many::<String>("NAME")
                .into_iter()
                .flatten()
                .collect::<Vec<_>>();
            if paths.len() < 1 {
                return Err(make_err!(Missing, "not enough arguments provided"));
            }
            paths.iter().map(|name| cli::show::show(name)).collect()
        }
        Some(("search", sub_matches)) => {
            let pattern = sub_matches.get_one::<String>("NAME").ok_or(make_err!(Missing, "pattern missing"))?;
            saku_cli::search::search(&pattern)?;

            Ok(())
        }
        Some(("list", sub_matches)) => {
            let flag = sub_matches.get_one::<u8>("installed").ok_or(make_err!())?;
            if *flag > 0 {
                saku_cli::list::list_installed()?;
                return Ok(());
            }

            // list flasks
            saku_cli::list::list()?;

            Ok(())
        }
        Some(("task", sub_matches)) => {
            let subcommand = sub_matches.subcommand().ok_or(make_err!(Missing, "subcommand missing"))?;
            match subcommand {
                ("clone", sub_matches) => {
                    let name = sub_matches
                        .get_one::<String>("NAME")
                        .ok_or(make_err!(Missing, "no package name specified."))?;

                    let pkg = data::get_pkg(name)?;
                    cli::install::clone_pkg(&pkg)?;

                    Ok(())
                }
                ("build", sub_matches) => {
                    let name = sub_matches
                        .get_one::<String>("NAME")
                        .ok_or(make_err!(Missing, "no package name specified."))?;

                    let pkg = data::get_pkg(name)?;
                    cli::install::build_pkg(&pkg)?;

                    Ok(())
                }
                ("install", sub_matches) => {
                    let name = sub_matches
                        .get_one::<String>("NAME")
                        .ok_or(make_err!(Missing, "no package name specified."))?;

                    let pkg = data::get_pkg(name)?;
                    pkg.install_root()?;

                    Ok(())
                }
                (&_, _) => Err(Error::Unexpected),
            }
        }
        Some(("changelog", sub_matches)) => {
            let name: String = sub_matches
                .get_one::<String>("NAME").map_or("saku".to_string(), |v| v.to_owned());

            cli::changelog::changelog(&name)?;
            Ok(())
        }
        // If all subcommands are defined above, anything else is unreachable!()
        _ => {
            Err(make_err!(Missing, "missing command. run saku --help."))
        },
    }
}
