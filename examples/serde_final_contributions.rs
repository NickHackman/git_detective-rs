use std::fs::remove_dir_all;

use git_detective::{Error, GitDetective};

fn main() -> Result<(), Error> {
    let path = "serde-example";

    let gd = GitDetective::clone("https://github.com/serde-rs/serde.git", path, true)?;
    let _ = gd.final_contributions()?;
    remove_dir_all(path).unwrap();
}
