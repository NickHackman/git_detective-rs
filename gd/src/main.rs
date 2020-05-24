//! Git Detective
//!
//! A Command line application to better inspect contributions in a Git Repository
#![deny(
    missing_docs,
    unsafe_code,
    unused_import_braces,
    unused_qualifications,
    unused_must_use
)]

use std::process;

use clap::ArgMatches;

use git_detective::{Error, GitDetective};

mod cli;
use cli::clap;

mod util;
use util::{diff_table, final_table};

fn construct_gd(matches: &ArgMatches) -> Result<GitDetective, Error> {
    let gd = match matches.subcommand() {
        ("clone", Some(c_matches)) => GitDetective::clone(
            c_matches.value_of("repository").unwrap(),
            c_matches.value_of("path").unwrap(),
            c_matches.is_present("recursive"),
        )?,
        _ => GitDetective::open(".")?,
    };
    Ok(gd)
}

fn run(matches: ArgMatches) -> Result<(), Error> {
    let mut gd = construct_gd(&matches)?;
    match matches.subcommand() {
        ("list", Some(list_args)) => Ok(list(list_args, &gd)?),
        ("statistics", Some(stats_args)) => Ok(stats(stats_args, &mut gd)?),
        ("clone", _) => Ok(()),
        ("checkout", Some(checkout_args)) => Ok(checkout(checkout_args, &gd)?),
        _ => unreachable!(),
    }
}

fn stats(matches: &ArgMatches, gd: &mut GitDetective) -> Result<(), Error> {
    let _name = matches.value_of("name");
    if matches.is_present("final") {
        let final_contribs = gd.final_contributions()?;
        final_table(final_contribs);
    }
    if matches.is_present("files") {
        let files = gd.files_contributed_to()?;
        println!("{:#?}", files);
    }
    if matches.is_present("difference") {
        let diff_stats = gd.diff_stats()?;
        diff_table(diff_stats);
    }
    Ok(())
}

fn list(matches: &ArgMatches, _gd: &GitDetective) -> Result<(), Error> {
    // TODO: call functions
    if matches.is_present("commits") {}
    if matches.is_present("tags") {}
    if matches.is_present("contributors") {}
    if matches.is_present("files") {}
    if matches.is_present("branches") {}
    Ok(())
}

// TODO: implement
fn checkout(matches: &ArgMatches, _gd: &GitDetective) -> Result<(), Error> {
    Ok(())
}

fn main() {
    let matches = clap();
    match run(matches) {
        Ok(_) => process::exit(0),
        Err(e) => {
            println!("{}", e);
            process::exit(-1);
        }
    }
}
