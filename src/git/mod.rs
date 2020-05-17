use std::collections::HashSet;
use std::iter::Iterator;
use std::path::Path;

use git2::{Repository, StatusOptions, StatusShow};

pub(crate) mod git_reference;
pub(crate) use git_reference::GitReference;

pub(crate) mod commit;
pub use commit::Commit;

pub(crate) mod branch;
pub use branch::Branch;

pub(crate) mod tag;
pub use tag::Tag;

pub(crate) mod signature;
pub use signature::Signature;

/// Status for a file
pub(crate) mod file_status;
pub use file_status::FileStatus;

use crate::Error;

/// A Git Repository
///
/// Wrapper around git2::Repository
pub struct Repo {
    repo: Repository,
    excluded_files: HashSet<String>,
}

impl Repo {
    /// Consturcts a Repository from the local filesystem
    ///
    /// Recursively goes up directories until a git repo is found
    pub fn open<P: AsRef<Path>>(path: P) -> Result<Self, Error> {
        Ok(Self {
            repo: Repository::discover(path)?,
            excluded_files: HashSet::new(),
        })
    }

    /// List Branches
    pub fn branches(
        &self,
        filter: Option<git2::BranchType>,
    ) -> Result<impl Iterator<Item = Branch<'_>>, Error> {
        Ok(self
            .repo
            .branches(filter)?
            .flatten()
            .map(|(branch, _)| Branch::from(branch)))
    }

    /// List Commits
    pub fn commits(&self) -> Result<impl Iterator<Item = Commit<'_>>, Error> {
        let mut rev_walk = self.repo.revwalk()?;
        rev_walk.push_head()?;
        Ok(rev_walk
            .flatten()
            .filter_map(move |id| self.repo.find_commit(id).map(Commit::from).ok()))
    }

    /// List Contributors
    ///
    /// **NOTE**: If author name isn't valid UTF-8 they will be filtered out
    pub fn contributors(&self) -> Result<HashSet<String>, Error> {
        let mut rev_walk = self.repo.revwalk()?;
        rev_walk.push_head()?;
        Ok(rev_walk
            .flatten()
            .filter_map(|id| self.repo.find_commit(id).ok())
            .fold(HashSet::new(), |mut set, commit| {
                if let Some(name) = commit.author().name() {
                    set.insert(name.to_string());
                }
                set
            }))
    }

    /// List tags
    ///
    /// **NOTE**: If a Tag has a name that isn't valid UTF-8 it is filtered out
    pub fn tags(&self, pattern: Option<&str>) -> Result<Vec<Tag<'_>>, Error> {
        let names = self.repo.tag_names(pattern)?;
        Ok(names
            .iter()
            .filter_map(|name| name)
            .filter_map(move |name| match self.repo.revparse_single(name) {
                Ok(obj) => obj.into_tag().map(Tag::from).ok(),
                Err(_) => None,
            })
            .collect())
    }

    /// Clones a Repository Recursively
    pub fn clone<S: AsRef<str>, P: AsRef<Path>>(
        url: S,
        path: P,
        recursive: bool,
    ) -> Result<Self, Error> {
        let repo = if recursive {
            Repository::clone_recurse(url.as_ref(), path)?
        } else {
            Repository::clone(url.as_ref(), path)?
        };

        Ok(Self {
            repo,
            excluded_files: HashSet::new(),
        })
    }

    /// Get the current state of the repository
    ///
    /// Either Clean, Merge, Revert, RevertSequence
    ///
    /// # Returns
    ///
    /// git2::RepositoryState
    ///
    /// - Clean
    /// - Merge
    /// - Revert
    /// - RevertSequence
    /// - CherryPick
    /// - CherryPickSequence
    /// - Bisect
    /// - Rebase
    /// - RebaseInteractive
    /// - RebaseMerge
    /// - ApplyMailbox
    /// - ApplyMailboxOrRebase
    pub fn state(&self) -> git2::RepositoryState {
        self.repo.state()
    }

    /// Exclude a file from being listed or counted
    ///
    /// Useful for removing files like `setup.py`, tests, etc
    pub fn exclude_file<S: Into<String>>(&mut self, file: S) {
        self.excluded_files.insert(file.into());
    }

    /// List files in the Git Repository
    ///
    /// Filters files based on `excluded_files`
    pub fn ls(&self) -> Result<Vec<FileStatus>, Error> {
        let mut base_options = StatusOptions::new();
        let options = base_options
            .show(StatusShow::IndexAndWorkdir)
            .include_unmodified(true);
        Ok(self
            .repo
            .statuses(Some(options))?
            .iter()
            .map(FileStatus::from)
            .filter(|file_stat| !self.excluded_files.contains(&file_stat.path))
            .collect())
    }

    /// Checkout a GitReference
    ///
    /// **NOTE**: HEAD is detached, this isn't meant to allow edits, but solely
    /// to view the state of the repository at this stage
    ///
    /// # Errors
    ///
    /// - `self.is_clean() != true`
    /// - Reference doesn't exist in repository
    pub fn checkout<'repo, GitRef: GitReference<'repo>>(
        &self,
        git_ref: GitRef,
    ) -> Result<(), Error> {
        let state = self.state();
        if state != git2::RepositoryState::Clean {
            return Err(Error::UncleanState(state));
        }
        let oid = git_ref.id();
        self.repo.checkout_tree(&git_ref.into_object()?, None)?;
        self.repo.set_head_detached(oid)?;
        Ok(())
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
        let branches: Vec<Branch<'_>> = branches.unwrap().collect();
        assert!(branches.len() > 0);
        let mut branches = git.branches(None).unwrap();
        assert!(
            branches.any(|b| b.name().unwrap() == "master" || b.name().unwrap() == "development")
        );
    }

    #[test]
    fn test_clone() {
        let path = PathBuf::from("globset");
        let git = Repo::clone("https://github.com/BurntSushi/globset", &path, true);
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
        let git = Repo::clone("https://github.com/BurntSushi/xsv.git", &path, true);
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
        let git = Repo::clone("https://github.com/BurntSushi/walkdir.git", &path, true);
        assert!(git.is_ok());
        let git = git.unwrap();
        let mut commits = git.commits().unwrap();
        assert!(commits.any(|c| c.id().to_string() == "29c86b2fd5876061c2e882abe71db07c3656b2c8"));
        let removed = remove_dir_all(path);
        assert!(removed.is_ok());
    }

    #[test]
    fn test_contibutors() {
        let path = PathBuf::from("imdb-rename");
        let git = Repo::clone("https://github.com/BurntSushi/imdb-rename.git", &path, true);
        assert!(git.is_ok());
        let git = git.unwrap();
        let contributors = git.contributors().unwrap();
        assert!(contributors.contains("Andrew Gallant"));
        assert!(contributors.contains("Samuel Walladge"));
        let removed = remove_dir_all(path);
        assert!(removed.is_ok());
    }

    #[test]
    fn checkout_tag() {
        let path = PathBuf::from("cursive");
        let clone = Repo::clone("https://github.com/gyscos/cursive.git", &path, true);
        assert!(clone.is_ok());
        let repo = clone.unwrap();
        let tags_result = repo.tags(Some("v0.14.0"));
        assert!(tags_result.is_ok());
        let tags = tags_result.unwrap();
        assert_eq!(tags.len(), 1);
        for tag in tags {
            let checkout_result = repo.checkout(tag);
            assert!(checkout_result.is_ok());
        }
        let removed = remove_dir_all(path);
        assert!(removed.is_ok());
    }

    #[test]
    fn checkout_commit() {
        let path = PathBuf::from("awesome-rust");
        let clone = Repo::clone(
            "https://github.com/rust-unofficial/awesome-rust.git",
            &path,
            true,
        );
        assert!(clone.is_ok());
        let repo = clone.unwrap();
        let commits_result = repo.commits();
        assert!(commits_result.is_ok());
        let mut commits = commits_result.unwrap();
        let commit_option =
            commits.find(|c| c.id().to_string() == "bc7268a41e6cf7cc5391b1fbfec8f1394c5d88b6");
        assert!(commit_option.is_some());
        let commit = commit_option.unwrap();
        let checkout_result = repo.checkout(commit);
        assert!(checkout_result.is_ok());
        let removed = remove_dir_all(path);
        assert!(removed.is_ok());
    }

    #[test]
    fn checkout_branch() {
        let path = PathBuf::from("rust-book");
        let clone = Repo::clone("https://github.com/rust-lang/book.git", &path, true);
        assert!(clone.is_ok());
        let repo = clone.unwrap();
        let branches_result = repo.branches(Some(git2::BranchType::Remote));
        assert!(branches_result.is_ok());
        let mut branches = branches_result.unwrap();
        let branch_option = branches.find(|c| c.name().unwrap() == "origin/gh-pages");
        assert!(branch_option.is_some());
        let branch = branch_option.unwrap();
        let checkout_result = repo.checkout(branch);
        assert!(checkout_result.is_ok());
        let removed = remove_dir_all(path);
        assert!(removed.is_ok());
    }

    #[test]
    fn checkout_tag_then_different_tag() {
        let path = PathBuf::from("spotify-tui");
        let clone = Repo::clone("https://github.com/Rigellute/spotify-tui.git", &path, true);
        assert!(clone.is_ok());
        let repo = clone.unwrap();
        let tags_result = repo.tags(Some("v0.10.0"));
        assert!(tags_result.is_ok());
        let tags = tags_result.unwrap();
        assert_eq!(tags.len(), 1);
        let tags_result = repo.tags(Some("v0.9.0"));
        assert!(tags_result.is_ok());
        let tags = tags_result.unwrap();
        assert_eq!(tags.len(), 1);
        for tag in tags {
            let checkout_result = repo.checkout(tag);
            assert!(checkout_result.is_ok());
        }
        let removed = remove_dir_all(path);
        assert!(removed.is_ok());
    }

    #[test]
    fn ls() {
        let repo_result = Repo::open(".");
        assert!(repo_result.is_ok());
        let repo = repo_result.unwrap();
        let list_result = repo.ls();
        assert!(list_result.is_ok());
        let list = list_result.unwrap();
        assert!(list
            .iter()
            .find(|stat| &stat.path == "Cargo.lock")
            .is_some());
        assert!(list
            .iter()
            .find(|stat| &stat.path == "Cargo.toml")
            .is_some());
        assert!(list.iter().find(|stat| &stat.path == "README.md").is_some());
    }

    #[test]
    fn ls_exclude() {
        let repo_result = Repo::open(".");
        assert!(repo_result.is_ok());
        let mut repo = repo_result.unwrap();
        let list_result = repo.ls();
        assert!(list_result.is_ok());
        let list = list_result.unwrap();
        assert!(list
            .iter()
            .find(|stat| &stat.path == "Cargo.lock")
            .is_some());
        assert!(list
            .iter()
            .find(|stat| &stat.path == "Cargo.toml")
            .is_some());
        assert!(list.iter().find(|stat| &stat.path == "README.md").is_some());
        repo.exclude_file("Cargo.toml");
        repo.exclude_file("README.md");
        repo.exclude_file("Cargo.lock");
        let list_after_result = repo.ls();
        assert!(list_after_result.is_ok());
        let list_after = list_after_result.unwrap();
        assert_eq!(
            list_after
                .iter()
                .find(|stat| &stat.path == "Cargo.lock")
                .is_some(),
            false
        );
        assert_eq!(
            list_after
                .iter()
                .find(|stat| &stat.path == "Cargo.toml")
                .is_some(),
            false
        );
        assert_eq!(
            list_after
                .iter()
                .find(|stat| &stat.path == "README.md")
                .is_some(),
            false
        );
    }
}
