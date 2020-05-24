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

mod table;
use table::{CommitsTable, DiffStatsTable, FinalContributionsTable, TagsTable};

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
        _ => unreachable!(),
    }
}

fn stats(matches: &ArgMatches, gd: &mut GitDetective) -> Result<(), Error> {
    let _name = matches.value_of("name");
    let dimensions = term_size::dimensions();
    if matches.is_present("difference") {
        let diff_stats = gd.diff_stats()?;
        println!("{}", DiffStatsTable::new(diff_stats, dimensions));
    } else {
        let final_contribs = gd.final_contributions()?;
        println!(
            "{}",
            FinalContributionsTable::new(final_contribs, dimensions)
        );
    }
    Ok(())
}

fn list(matches: &ArgMatches, gd: &GitDetective) -> Result<(), Error> {
    let dimensions = term_size::dimensions();
    if matches.is_present("commits") {
        let commits: Vec<_> = gd.commits()?.collect();
        println!("{}", CommitsTable::new(commits, dimensions));
    } else if matches.is_present("tags") {
        let tags = gd.tags()?;
        println!("{}", TagsTable::new(tags, dimensions));
    } else if matches.is_present("branches") {
        let branches = gd.branches()?;
        for branch in branches {
            if let Ok(name) = branch.name() {
                println!("{}", name);
            }
        }
    } else {
        let contributors = gd.contributors()?;
        for contributor in contributors {
            println!("{}", contributor);
        }
    }
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
