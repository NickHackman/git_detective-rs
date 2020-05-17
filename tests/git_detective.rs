#[cfg(test)]
mod git_detective_integration_tests {
    use std::fs::remove_dir_all;
    use std::mem::discriminant;
    use std::path::PathBuf;

    use git_detective::Error;
    use git_detective::GitDetective;

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
    fn branches() -> Result<(), Error> {
        let repo = GitDetective::open(".")?;
        let mut branches = repo.branches()?;
        assert!(branches.any(|branch| {
            let b_name = branch.name().unwrap();
            b_name == "development" || b_name == "master"
        }));
        Ok(())
    }
}
