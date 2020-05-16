//! Git Detective
//!
//! A Terminal User Interface to view git contributions
#![deny(
    missing_docs,
    unsafe_code,
    unused_import_braces,
    unused_qualifications,
    rust_2018_idioms,
    unused_must_use
)]

use std::path::Path;

use url::Url;

/// Performs git operations
pub mod git;
use git::Repo;

/// Errors for Git Detective
pub mod error;
use error::Error;

/// Main Entry point
pub struct GitDetective {
    repo: Repo,
}

impl GitDetective {
    /// Open a Local Git Repository
    ///
    /// # Example
    ///
    /// ```
    /// use git_detective::GitDetective;
    ///
    /// let repo = GitDetective::open(".");
    /// assert!(repo.is_ok());
    /// ```
    pub fn open<P: AsRef<Path>>(path: P) -> Result<Self, Error> {
        let repo = Repo::open(path)?;
        Ok(Self { repo })
    }

    /// Clone a remote Git Repository
    ///
    /// # Example
    ///
    /// ```
    /// use std::fs::remove_dir_all;
    ///
    /// use git_detective::GitDetective;
    ///
    /// let path = "git_detective_cloned";
    ///
    /// let repo = GitDetective::clone("https://github.com/NickHackman/Git-Detective.git", path, true);
    /// assert!(repo.is_ok());
    ///
    /// // Clean up cloned repository
    /// remove_dir_all(path);
    /// ```
    pub fn clone<S: AsRef<str>, P: AsRef<Path>>(
        url: S,
        path: P,
        recursive: bool,
    ) -> Result<Self, Error> {
        let valid_url = Url::parse(url.as_ref())?;
        let repo = Repo::clone(valid_url.as_str(), path.as_ref(), recursive)?;
        Ok(Self { repo })
    }
}
