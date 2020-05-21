use std::fs::remove_dir_all;

use git_detective::{Error, GitDetective};

fn main() -> Result<(), Error> {
    let path = "serde-example";

    let mut gd = GitDetective::clone("https://github.com/serde-rs/serde.git", path, true)?;
    let contributions = gd.final_contributions()?;
    println!("{:#?}", contributions);
    println!("Total lines = {}", contributions.total_lines());
    remove_dir_all(path).unwrap();
    Ok(())
}
