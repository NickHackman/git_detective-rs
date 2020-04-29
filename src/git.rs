use std::path::Path;

use git2::Repository;

use crate::error::Error;

/// A Git Repository
///
/// Wrapper around git2::Repository
pub struct Repo {
    repo: Repository,
}

impl Repo {
    /// Consturcts a Repository from the local filesystem
    ///
    /// Recursively goes up directories until a git repo is found
    ///
    /// # Returns
    ///
    /// Result<Self, git2::Error>
    pub fn open<P: AsRef<Path>>(path: P) -> Result<Self, Error> {
        Ok(Self {
            repo: Repository::discover(path)?,
        })
    }

    /// List Branches
    ///
    /// # Returns
    ///
    /// Result<Vec<String>>, git2::Error>
    pub fn branch_names(&self, filter: Option<git2::BranchType>) -> Result<Vec<String>, Error> {
        Ok(self
            .repo
            .branches(filter)?
            .flatten()
            .map(|(branch, _)| {
                branch
                    .name_bytes()
                    .map_or(String::new(), |name| String::from_utf8_lossy(name).into())
            })
            .collect())
    }

    /// Clones a Repository Recursively
    ///
    /// # Returns
    ///
    /// Result<Self, git2::Error>
    pub fn clone<P: AsRef<Path>>(url: &str, path: P) -> Result<Self, Error> {
        Ok(Self {
            repo: Repository::clone_recurse(url, path)?,
        })
    }

    /// Checkout a branch by name
    ///
    /// # Returns
    ///
    /// Result<(), Error>
    pub fn checkout(&self, branch_name: &str) -> Result<(), Error> {
        let head = self.repo.head()?;
        let oid = head.target().unwrap();
        let commit = self.repo.find_commit(oid)?;
        self.repo.branch(branch_name, &commit, false)?;
        let full_branch_name = format!("refs/heads/{}", branch_name);
        let obj = self.repo.revparse_single(&full_branch_name)?;

        self.repo.checkout_tree(&obj, None)?;

        self.repo.set_head(&full_branch_name).map_err(Error::from)
    }
}

#[cfg(test)]
mod git_tests {
    use std::fs::remove_dir_all;
    use std::path::PathBuf;

    use super::*;

    #[test]
    fn test_new() {
        let git = Repo::open(".");
        assert!(git.is_ok());
    }

    #[test]
    fn test_branches() {
        let git = Repo::open(".");
        assert!(git.is_ok());
        let git = git.unwrap();
        let branches = git.branch_names(None);
        assert!(branches.is_ok());
        let branches = branches.unwrap();
        assert!(branches.len() > 0);
        assert!(branches.contains(&"master".to_string()));
        assert!(branches.contains(&"development".to_string()));
    }

    #[test]
    fn test_clone() {
        let path = PathBuf::from("walkdir");
        let git = Repo::clone("https://github.com/BurntSushi/walkdir", &path);
        assert!(git.is_ok());
        let branches = git.unwrap().branch_names(None).unwrap();
        assert!(branches.len() > 0);
        assert!(branches.contains(&"master".to_string()));
        let removed = remove_dir_all(path);
        assert!(removed.is_ok());
    }

    #[test]
    fn test_checkout() {
        let path = PathBuf::from("ripgrep");
        let git = Repo::clone("https://github.com/BurntSushi/ripgrep", &path);
        assert!(git.is_ok());
        let git = git.unwrap();
        let branches = git.branch_names(None).unwrap();
        assert!(branches.len() > 0);
        assert!(branches.contains(&"master".to_string()));
        let result = git.checkout("origin/ag/libripgrep-freeze-2");
        assert!(result.is_ok());
        let removed = remove_dir_all(path);
        assert!(removed.is_ok());
    }
}
