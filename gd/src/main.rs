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

use clap::ArgMatches;

use git_detective::{Error, GitDetective};

mod cli;
use cli::clap;

fn construct_git_detective(matches: ArgMatches) -> Result<GitDetective, Error> {
    if let Some(clone) = matches.subcommand_matches("clone") {
        let path = matches.value_of("path").unwrap_or("git_detective_repo");
        GitDetective::clone(
            clone.value_of("repository").unwrap(),
            path,
            clone.is_present("recursive"),
        )
    } else if let Some(open) = matches.subcommand_matches("open") {
        let path = open.value_of("repository").unwrap_or(".");
        GitDetective::open(path)
    } else {
        panic!("Must enter a subcommand")
    }
}

fn main() {
    let matches = clap();
    let _git_detective = match construct_git_detective(matches) {
        Ok(detective) => detective,
        Err(e) => panic!("{:?}", e),
    };
}
