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

    #[test]
    fn final_contributions_file() -> Result<(), Error> {
        let gd = GitDetective::open(".")?;
        let (lang, final_contribs) = gd.final_contributions_file(file!())?;
        assert_eq!(lang, "Rust");
        assert!(final_contribs.contains_key("Nick Hackman"));
        let nh_contribs = final_contribs.get("Nick Hackman").unwrap();
        assert!(nh_contribs.lines >= 175);
        assert!(nh_contribs.blanks >= 14);
        assert!(nh_contribs.comments >= 1);
        Ok(())
    }

    #[test]
    fn final_contributions() -> Result<(), Error> {
        let mut gd = GitDetective::open(".")?;
        let project_stats = gd.final_contributions()?;
        assert!(project_stats
            .contributors()
            .any(|name| name == "Nick Hackman"));
        let nh_stats = project_stats.contribs_by_name("Nick Hackman");
        assert!(nh_stats.is_some());
        assert!(project_stats.total_lines() > 1000);
        let total_nh_stats = project_stats
            .total_contribs_by_name("Nick Hackman")
            .unwrap();
        assert!(total_nh_stats.lines > 1000);
        assert!(total_nh_stats.code > 1000);
        Ok(())
    }

    #[test]
    fn branches() -> Result<(), Error> {
        let gd = GitDetective::open(".")?;
        let branches = gd.branches()?;
        assert!(branches.count() > 0);
        Ok(())
    }

    #[test]
    fn test_tags() -> Result<(), Error> {
        let path = PathBuf::from("xsv");
        let gd = GitDetective::clone("https://github.com/BurntSushi/xsv.git", &path, true)?;
        let tags = gd.tags()?;
        assert!(tags.iter().any(|t| t.name().unwrap() == "0.13.0"));
        let removed = remove_dir_all(path);
        assert!(removed.is_ok());
        Ok(())
    }

    #[test]
    fn test_commits() -> Result<(), Error> {
        let path = PathBuf::from("walkdir");
        let gd = GitDetective::clone("https://github.com/BurntSushi/walkdir.git", &path, true)?;
        let mut commits = gd.commits()?;
        assert!(commits.any(|c| c.id().to_string() == "29c86b2fd5876061c2e882abe71db07c3656b2c8"));
        let removed = remove_dir_all(path);
        assert!(removed.is_ok());
        Ok(())
    }

    #[test]
    fn test_contibutors() -> Result<(), Error> {
        let path = PathBuf::from("imdb-rename");
        let gd = GitDetective::clone("https://github.com/BurntSushi/imdb-rename.git", &path, true)?;
        let contributors = gd.contributors()?;
        assert!(contributors.contains("Andrew Gallant"));
        assert!(contributors.contains("Samuel Walladge"));
        let removed = remove_dir_all(path);
        assert!(removed.is_ok());
        Ok(())
    }

    #[test]
    fn checkout_commit() -> Result<(), Error> {
        let path = PathBuf::from("awesome-rust");
        let gd = GitDetective::clone(
            "https://github.com/rust-unofficial/awesome-rust.git",
            &path,
            true,
        )?;
        let mut commits = gd.commits()?;
        let commit_option =
            commits.find(|c| c.id().to_string() == "bc7268a41e6cf7cc5391b1fbfec8f1394c5d88b6");
        assert!(commit_option.is_some());
        let commit = commit_option.unwrap();
        let _ = gd.checkout(commit)?;
        let removed = remove_dir_all(path);
        assert!(removed.is_ok());
        Ok(())
    }

    #[test]
    fn checkout_branch() -> Result<(), Error> {
        let path = PathBuf::from("rust-book");
        let gd = GitDetective::clone("https://github.com/rust-lang/book.git", &path, true)?;
        let mut branches = gd.branches()?;
        let branch_option = branches.find(|c| c.name().unwrap() == "origin/gh-pages");
        assert!(branch_option.is_some());
        let branch = branch_option.unwrap();
        let _ = gd.checkout(branch)?;
        let removed = remove_dir_all(path);
        assert!(removed.is_ok());
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
}
