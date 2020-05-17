#[cfg(test)]
mod git_detective_integration_tests {
    use chrono::offset::Utc;
    use std::fs::remove_dir_all;
    use std::mem::discriminant;
    use std::path::PathBuf;

    use git_detective::Error;
    use git_detective::{GitDetective, RepositoryState};

    #[test]
    fn clone() -> Result<(), Error> {
        let path = "git_detective_cloned_integration_tests";

        let _ = GitDetective::clone(
            "https://github.com/NickHackman/Git-Detective.git",
            path,
            false,
        )?;

        // Clean up cloned repository
        let result = remove_dir_all(path);
        assert!(result.is_ok());
        Ok(())
    }

    #[test]
    fn open() -> Result<(), Error> {
        let _ = GitDetective::open(".")?;
        Ok(())
    }

    #[test]
    fn open_error() {
        let root = if cfg!(target_os = "windows") {
            PathBuf::from(r"C:\windows")
        } else {
            PathBuf::from("/")
        };
        let repo = GitDetective::open(root);
        assert!(repo.is_err());
        let error = repo.err().unwrap();
        let expected = Error::GitError(git2::Error::from_str(""));
        assert_eq!(discriminant(&error), discriminant(&expected));
    }

    #[test]
    fn invalid_url() {
        let url = "http:::";
        let repo = GitDetective::clone(url, "bad_url", true);
        assert!(repo.is_err());
        let error = repo.err().unwrap();
        let expected = Error::GitUrlError(url::ParseError::Overflow);
        assert_eq!(discriminant(&error), discriminant(&expected));
    }

    #[test]
    fn master_is_not_head() -> Result<(), Error> {
        let repo = GitDetective::open(".")?;
        let mut branches = repo.branches()?;
        let dev = branches
            .find(|branch| branch.name().unwrap() == "origin/master")
            .unwrap();
        assert!(!dev.is_head());
        Ok(())
    }

    #[test]
    fn first_commit() -> Result<(), Error> {
        let gd = GitDetective::open(".")?;
        let first_commit = gd.commits()?.last().unwrap();
        let first_msg = first_commit.message()?;
        assert!(first_msg.contains("Initial Commit"));
        let first_summary = first_commit.summary().unwrap()?;
        assert!(first_summary.contains("Initial Commit"));
        let first_author = first_commit.author();
        assert_eq!(first_author.name()?, "NickHackman");
        assert!(first_author.email()?.contains("snickhackman"));
        assert!(Utc::now() > first_author.date());
        let first_committer = first_commit.committer();
        assert_eq!(first_committer.name()?, "NickHackman");
        assert!(first_committer.email()?.contains("snickhackman"));
        assert!(Utc::now() > first_committer.date());
        Ok(())
    }

    #[test]
    fn ls() -> Result<(), Error> {
        let gd = GitDetective::open(".")?;
        let list = gd.ls()?;
        assert!(list
            .iter()
            .find(|stat| &stat.path == "Cargo.lock")
            .is_some());
        assert!(list
            .iter()
            .find(|stat| &stat.path == "Cargo.toml")
            .is_some());
        assert!(list.iter().find(|stat| &stat.path == "README.md").is_some());
        Ok(())
    }

    #[test]
    fn ls_exclude() -> Result<(), Error> {
        let mut gd = GitDetective::open(".")?;
        let list = gd.ls()?;
        assert!(list
            .iter()
            .find(|stat| &stat.path == "Cargo.lock")
            .is_some());
        assert!(list
            .iter()
            .find(|stat| &stat.path == "Cargo.toml")
            .is_some());
        assert!(list.iter().find(|stat| &stat.path == "README.md").is_some());
        gd.exclude_file("Cargo.toml");
        gd.exclude_file("README.md");
        gd.exclude_file("Cargo.lock");
        let list_after = gd.ls()?;
        assert!(!list_after
            .iter()
            .find(|stat| &stat.path == "Cargo.lock")
            .is_some());
        assert!(!list_after
            .iter()
            .find(|stat| &stat.path == "Cargo.toml")
            .is_some());
        assert!(!list_after
            .iter()
            .find(|stat| &stat.path == "README.md")
            .is_some());
        Ok(())
    }

    #[test]
    fn checkout_tag_then_different_tag() -> Result<(), Error> {
        let path = PathBuf::from("spotify-tui");
        let gd = GitDetective::clone("https://github.com/Rigellute/spotify-tui.git", &path, true)?;
        let tags = gd.tags()?;
        assert!(tags.len() > 1);
        for tag in tags {
            let _ = gd.checkout(tag)?;
        }
        let removed = remove_dir_all(path);
        assert!(removed.is_ok());
        Ok(())
    }

    #[test]
    fn checkout_tag() -> Result<(), Error> {
        let path = PathBuf::from("cursive");
        let gd = GitDetective::clone("https://github.com/gyscos/cursive.git", &path, true)?;
        let tags = gd.tags()?;
        assert!(tags.len() > 1);
        let tag = tags
            .iter()
            .find(|tag| tag.name().unwrap() == "0.11.2")
            .unwrap();
        let message = tag.message().unwrap()?;
        assert!(message.contains("(cargo-release) cursive version 0.11.2"));
        let tagger = tag.tagger().unwrap();
        assert_eq!(tagger.name()?, "Alexandre Bury");
        assert!(tagger.email()?.contains("alexandre.bury"));
        let removed = remove_dir_all(path);
        assert!(removed.is_ok());
        Ok(())
    }

    #[test]
    fn contributor() -> Result<(), Error> {
        let gd = GitDetective::open(".")?;
        let contributors = gd.contributors()?;
        assert!(contributors.contains("NickHackman"));
        Ok(())
    }

    #[test]
    fn state() -> Result<(), Error> {
        let gd = GitDetective::open(".")?;
        assert_eq!(
            discriminant(&RepositoryState::Clean),
            discriminant(&gd.state())
        );
        Ok(())
    }
}
