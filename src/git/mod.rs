use std::iter::Iterator;
use std::path::Path;

use git2::Repository;

mod git_reference;
use crate::error::Error;
use git_reference::GitReference;

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
    /// Result<Iterator<GitReference>>, git2::Error>
    pub fn branches<'repo>(
        &self,
        filter: Option<git2::BranchType>,
    ) -> Result<impl Iterator<Item = GitReference>, Error> {
        Ok(self
            .repo
            .branches(filter)?
            .flatten()
            .map(|(branch, _)| GitReference::Branch(branch)))
    }

    /// List Commits
    ///
    /// # Returns
    ///
    /// Iterator<GitReference>
    pub fn commits(&self) -> impl Iterator<Item = GitReference> {
        self.repo
            .revwalk()
            .into_iter()
            .flatten()
            .filter_map(move |id| match id {
                Ok(id) => self
                    .repo
                    .find_commit(id)
                    .map(|commit| GitReference::Commit(commit))
                    .ok(),
                Err(_) => None,
            })
    }

    /// Clones a Repository Recursively
    ///
    /// # Returns
    ///
    /// Result<Self, git2::Error>
    pub fn clone<S: AsRef<str>, P: AsRef<Path>>(url: S, path: P) -> Result<Self, Error> {
        Ok(Self {
            repo: Repository::clone_recurse(url.as_ref(), path)?,
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
        let branches = git.branches(None);
        assert!(branches.is_ok());
        let branches: Vec<GitReference> = branches.unwrap().collect();
        assert!(branches.len() > 0);
        let mut branches = git.branches(None).unwrap();
        assert!(
            branches.any(|b| b.name().unwrap() == "master" || b.name().unwrap() == "development")
        );
    }

    #[test]
    fn test_clone() {
        let path = PathBuf::from("walkdir");
        let git = Repo::clone("https://github.com/BurntSushi/walkdir", &path);
        assert!(git.is_ok());
        let git = git.unwrap();
        let mut branches = git.branches(None).unwrap();
        assert!(branches.any(|b| b.name().unwrap() == "master"));
        let removed = remove_dir_all(path);
        assert!(removed.is_ok());
    }

    #[test]
    fn test_checkout() {
        let path = PathBuf::from("ripgrep");
        let git = Repo::clone("https://github.com/BurntSushi/ripgrep", &path);
        assert!(git.is_ok());
        let git = git.unwrap();
        let mut branches = git.branches(None).unwrap();
        assert!(branches.any(|b| b.name().unwrap() == "master"));
        let result = git.checkout("origin/ag/libripgrep-freeze-2");
        assert!(result.is_ok());
        let removed = remove_dir_all(path);
        assert!(removed.is_ok());
    }
}
