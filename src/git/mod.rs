use std::iter::Iterator;
use std::path::Path;

use git2::Repository;

/// A Wrapper around `git2::Branch`, `git2::Commit`, and `git2::Tag`
pub mod git_reference;
use git_reference::GitReference;

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
    /// # Example
    ///
    /// ```
    /// use git_detective::git::Repo;
    ///
    /// let repo_result = Repo::open(".");
    /// assert!(repo_result.is_ok());
    /// ```
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
    /// # Example
    ///
    /// ```
    /// use std::fs::remove_dir_all;
    ///
    /// use git_detective::git::{Repo};
    /// use git_detective::git::git_reference::GitReference;
    ///
    /// let path = "bspwm";
    ///
    /// let repo = Repo::clone("https://github.com/baskerville/bspwm.git", path).unwrap();
    /// for branch in repo.branches(None).unwrap() {
    ///   println!("{}", branch.name().unwrap());
    /// }
    ///
    /// // Remove Git Repository
    /// remove_dir_all(path);
    /// ```
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
    /// # Example
    ///
    /// ```
    /// use std::fs::remove_dir_all;
    ///
    /// use git_detective::git::{Repo};
    /// use git_detective::git::git_reference::GitReference;
    ///
    /// let path = "tui-rs";
    ///
    /// let repo = Repo::clone("https://github.com/fdehau/tui-rs.git", path).unwrap();
    /// for commit in repo.commits().unwrap() {
    ///   match commit {
    ///     GitReference::Commit(commit) => println!("{:?}", commit.message_raw_bytes()),
    ///     _ => panic!("Expected only commits"),
    ///   }
    /// }
    ///
    /// // Remove Git Repository
    /// remove_dir_all(path);
    /// ```
    ///
    /// # Returns
    ///
    /// Result<Iterator<GitReference>, Error>
    pub fn commits(&self) -> Result<impl Iterator<Item = GitReference>, Error> {
        let mut rev_walk = self.repo.revwalk()?;
        rev_walk.push_head()?;
        Ok(rev_walk
            .into_iter()
            .flatten()
            .filter_map(move |id| self.repo.find_commit(id).map(GitReference::Commit).ok()))
    }

    /// List tags
    ///
    /// NOTE: If a Tag has a name that isn't valid UTF-8 it is filtered out
    ///
    /// # Parameters
    ///
    /// pattern: Option<&str>
    ///
    /// pattern to filter Tags by
    ///
    /// # Example
    ///
    /// ```
    /// use std::fs::remove_dir_all;
    ///
    /// use git_detective::git::{Repo};
    /// use git_detective::git::git_reference::GitReference;
    ///
    /// let path = "polybar";
    ///
    /// let repo = Repo::clone("https://github.com/polybar/polybar.git", path).unwrap();
    /// for tag in repo.tags(None).unwrap() {
    ///   println!("{}", tag.name().unwrap());
    /// }
    ///
    /// // Remove Git Repository
    /// remove_dir_all(path);
    /// ```
    ///
    /// # Returns
    ///
    /// Result<Vec<String>, Error>
    pub fn tags(&self, pattern: Option<&str>) -> Result<Vec<GitReference>, Error> {
        let names = self.repo.tag_names(pattern)?;
        Ok(names
            .iter()
            .filter_map(|name| name)
            .filter_map(move |name| match self.repo.revparse_single(name) {
                Ok(obj) => obj.into_tag().map(GitReference::Tag).ok(),
                Err(_) => None,
            })
            .collect())
    }

    /// Clones a Repository Recursively
    ///
    /// # Example
    ///
    /// ```
    /// use git_detective::git::Repo;
    /// use std::fs::remove_dir_all;
    ///
    /// let path = "TrojanHorse";
    ///
    /// let repo = Repo::clone("https://github.com/jklypchak13/TrojanHorse.git", path);
    /// assert!(repo.is_ok());
    ///
    /// // Cleanup the cloned repository
    /// remove_dir_all(path);
    /// ```
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
        let path = PathBuf::from("globset");
        let git = Repo::clone("https://github.com/BurntSushi/globset", &path);
        assert!(git.is_ok());
        let git = git.unwrap();
        let mut branches = git.branches(None).unwrap();
        assert!(branches.any(|b| b.name().unwrap() == "master"));
        let removed = remove_dir_all(path);
        assert!(removed.is_ok());
    }

    #[test]
    fn test_tags() {
        let path = PathBuf::from("xsv");
        let git = Repo::clone("https://github.com/BurntSushi/xsv.git", &path);
        assert!(git.is_ok());
        let git = git.unwrap();
        let tags = git.tags(None).unwrap();
        assert!(tags.iter().any(|t| t.name().unwrap() == "0.13.0"));
        let removed = remove_dir_all(path);
        assert!(removed.is_ok());
    }

    #[test]
    fn test_commits() {
        let path = PathBuf::from("walkdir");
        let git = Repo::clone("https://github.com/BurntSushi/walkdir.git", &path);
        assert!(git.is_ok());
        let git = git.unwrap();
        let mut commits = git.commits().unwrap();
        assert!(commits.any(|c| c.name().unwrap() == "29c86b2fd5876061c2e882abe71db07c3656b2c8"));
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
