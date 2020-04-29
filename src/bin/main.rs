//! Git Detective
//!
//! A Terminal User Interface to view git contributions
#![deny(
    missing_docs,
    unsafe_code,
    unused_import_braces,
    unused_qualifications,
    unused_must_use
)]

use clap::{crate_authors, crate_version, App, AppSettings, Arg, ArgMatches};

use git_detective::{error::Error, GitDetective, Mode};

fn cli() -> ArgMatches<'static> {
    App::new("gd")
        .version(crate_version!())
        .author(crate_authors!())
        .setting(AppSettings::SubcommandRequiredElseHelp)
        .subcommand(
            App::new("clone")
                .about("Clone a remote repository to inspect")
                .alias("c")
                .arg(
                    Arg::with_name("path")
                        .short("p")
                        .long("path")
                        .help("Path to clone to")
                        .default_value("git_detective_repo"),
                )
                .arg(
                    Arg::with_name("branch")
                        .short("b")
                        .help("Branch to checkout")
                        .long("branch")
                        .default_value("master"),
                )
                .arg(
                    Arg::with_name("URL")
                        .required(true)
                        .help("Git URL to clone then open"),
                ),
        )
        .subcommand(
            App::new("open")
                .about("Open a local repository to inspect")
                .alias("o")
                .arg(
                    Arg::with_name("path")
                        .default_value(".")
                        .help("Path to Git repository to open"),
                )
                .arg(
                    Arg::with_name("branch")
                        .short("b")
                        .help("Branch to checkout")
                        .long("branch")
                        .default_value("master"),
                ),
        )
        .get_matches()
}

fn construct_git_detective(matches: ArgMatches) -> Result<GitDetective, Error> {
    let branch = matches.value_of("branch").unwrap_or("master");
    match matches.subcommand_name() {
        Some("clone") => {
            let path = matches.value_of("path").unwrap_or("git_detective_repo");
            let uri = match matches.value_of("URL") {
                Some(uri) => uri,
                None => panic!("URL is required"),
            };
            GitDetective::new(Mode::Clone(path.to_string()), uri, branch)
        }
        Some("open") => {
            let path = matches.value_of("path").unwrap_or(".");
            GitDetective::new(Mode::Open, path, branch)
        }
        _ => panic!("Must enter a subcommand"),
    }
}

fn main() {
    let matches = cli();
    let _git_detective = match construct_git_detective(matches) {
        Ok(detective) => detective,
        Err(e) => panic!("{:?}", e),
    };
}
