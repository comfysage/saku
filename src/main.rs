use yuki::{Result, Error, make_err };
use yuki::cli;
use yuki;

use clap::{arg, Command};

fn get_commands() -> Command {
    Command::new("yki")
        .about("a tiny distro-independent package manager written in Go.")
        .subcommand_required(true)
        .arg_required_else_help(true)
        .allow_external_subcommands(true)
        .subcommand(
            Command::new("config")
                .about("Manage global configuration")
                .subcommand_required(true)
                .arg_required_else_help(true)
                .subcommand(Command::new("init").about("Setup hayashi"))
                .subcommand(Command::new("create").about("Setup .hayashi.yaml")),
        )
        .subcommand(
            Command::new("pkg")
                .about("Manage pkg configurations")
                .subcommand_required(true)
                .arg_required_else_help(true)
                .subcommand(Command::new("show").about("Show pkg configuration"))
                .subcommand(Command::new("add").about("Add pkg configuration"))
                .subcommand(Command::new("remove").about("Remove pkg configuration")),
        )
        .subcommand(
            Command::new("add")
                .about("Install a package")
                .arg_required_else_help(true)
                .arg(arg!(<NAME> ... "Package to install")),
        )
        .subcommand(
            Command::new("remove")
                .about("Remove a package")
                .arg_required_else_help(true)
                .arg(arg!(<NAME> ... "Package to remove")),
        )
        .subcommand(
            Command::new("show")
                .about("Show package details")
                .arg_required_else_help(true)
                .arg(arg!(<NAME> ... "Package to show")),
        )
}

fn main() -> Result<()> {
    let matches = get_commands().get_matches();

    match matches.subcommand() {
        Some(("config", sub_matches)) => {
            let stash_command = sub_matches.subcommand().unwrap_or(("init", sub_matches));
            match stash_command {
                ("init", _) => cli::config::init(),
                ("create", _) => cli::config::create(),
                (&_, _) => Err(yuki::Error::Unexpected),
            }
        }
        Some(("pkg", sub_matches)) => {
            let stash_command = sub_matches.subcommand().unwrap_or(("show", sub_matches));
            match stash_command {
                ("add", sub_matches) => {
                    let package = sub_matches.get_one::<String>("NAME");
                    println!("add pkg {package:?}");
                    Ok(())
                }
                ("remove", sub_matches) => {
                    let package = sub_matches.get_one::<String>("NAME");
                    println!("remove pkg {package:?}");
                    Ok(())
                }
                ("show", sub_matches) => {
                    let package = sub_matches.get_one::<String>("NAME");
                    println!("show pkg {package:?}");
                    Ok(())
                }
                (&_, _) => Err(yuki::Error::Unexpected),
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
        Some(("remove", sub_matches)) => {
            let paths = sub_matches
                .get_many::<String>("NAME")
                .into_iter()
                .flatten()
                .collect::<Vec<_>>();
            println!("Removing {paths:?}");
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
            paths
                .iter()
                .map(|name| cli::show::show(name))
                .collect()
        }
        _ => unreachable!(), // If all subcommands are defined above, anything else is unreachable!()
    }
}
