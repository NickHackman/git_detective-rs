use clap::{crate_authors, crate_version, App, AppSettings, Arg, ArgMatches, SubCommand};

pub fn clap() -> ArgMatches<'static> {
    App::new("gd")
        .version(crate_version!())
        .author(crate_authors!())
        .setting(AppSettings::SubcommandRequiredElseHelp)
        .setting(AppSettings::VersionlessSubcommands)
        .subcommand(
            SubCommand::with_name("clone")
                .about("Clone a remote repository to inspect")
                .alias("c")
                .arg(
                    Arg::with_name("path")
                        .short("p")
                        .long("path")
                        .help("Path to clone to")
                        .takes_value(true)
                        .default_value("git_detective_repo"),
                )
                .arg(
                    Arg::with_name("recursive")
                        .short("r")
                        .long("recursive")
                        .takes_value(false)
                        .help("Recursively clone git repository"),
                )
                .arg(
                    Arg::with_name("repository")
                        .required(true)
                        .help("Git URL to clone then open"),
                ),
        )
        .subcommand(
            SubCommand::with_name("open")
                .about("Open a local repository to inspect")
                .alias("o")
                .arg(
                    Arg::with_name("repository")
                        .default_value(".")
                        .required(true)
                        .help("Path to Git repository to open"),
                ),
        )
        .get_matches()
}
