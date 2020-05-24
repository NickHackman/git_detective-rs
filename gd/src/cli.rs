use clap::{
    crate_authors, crate_name, crate_version, App, AppSettings, Arg, ArgMatches, SubCommand,
};

pub fn clap() -> ArgMatches<'static> {
    App::new(crate_name!())
        .version(crate_version!())
        .author(crate_authors!())
        .setting(AppSettings::SubcommandRequiredElseHelp)
        .setting(AppSettings::VersionlessSubcommands)
        .arg(
            // TODO: Logging
            Arg::with_name("verbose")
                .short("v")
                .long("verbose")
                .help("Set log output level:\n\t1. TODO\n\t2. TODO\n\t3. TODO")
                .multiple(true),
        )
        .subcommand(
            SubCommand::with_name("clone")
                .about("Clone a remote repository to inspect")
                .alias("c")
                .arg(
                    Arg::with_name("repository")
                        .required(true)
                        .help("Git URL to clone then open"),
                )
                .arg(
                    Arg::with_name("path")
                        .help("Path to clone to")
                        .required(true),
                )
                .arg(
                    Arg::with_name("recursive")
                        .short("r")
                        .long("recursive")
                        .takes_value(false)
                        .help("Recursively clone git repository"),
                ),
        )
        .subcommand(
            SubCommand::with_name("list")
                .alias("l")
                .about("List branches, commits, contributors, and tags")
                .arg(
                    Arg::with_name("commits")
                        .short("c")
                        .long("commits")
                        .help("List all commits")
                        .conflicts_with_all(&["tags", "contributors", "branches"]),
                )
                .arg(
                    Arg::with_name("tags")
                        .short("t")
                        .long("tags")
                        .help("List all tags")
                        .conflicts_with_all(&["commits", "contributors", "branches"]),
                )
                .arg(
                    Arg::with_name("contributors")
                        .long("contributors")
                        .help("List all contributors")
                        .conflicts_with_all(&["commits", "tags", "branches"]),
                )
                .arg(
                    Arg::with_name("branches")
                        .short("b")
                        .long("branches")
                        .help("List all branches")
                        .conflicts_with_all(&["commits", "tags", "contributors"]),
                ),
        )
        .subcommand(
            SubCommand::with_name("statistics")
                .about("Statistics about the Git Repository")
                .alias("stats")
                .arg(
                    Arg::with_name("final")
                        .long("final")
                        .help("Statistics in the most recent commit by language and contributor")
                        .conflicts_with_all(&["files", "difference"]),
                )
                .arg(
                    Arg::with_name("difference")
                        .short("d")
                        .long("diff")
                        .help("Insertion/Deletions by contributor")
                        .conflicts_with_all(&["files", "final"]),
                )
                .arg(
                    Arg::with_name("name")
                        .short("n")
                        .long("name")
                        .takes_value(true)
                        .help("Name of contributor to filter by"),
                ),
        )
        .get_matches()
}
