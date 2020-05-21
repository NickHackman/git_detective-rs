use git_detective::{Error, GitDetective};

fn main() -> Result<(), Error> {
    let mut gd = GitDetective::open(".")?;
    let contributions = gd.final_contributions()?;
    println!("{:#?}", contributions);
    Ok(())
}
