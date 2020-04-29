//! Git Detective
//!
//! A Terminal User Interface to view git contributions
#![deny(
    missing_docs,
    unsafe_code,
    unused_import_braces,
    unused_qualifications,
    unused_must_use
)]

/// Performs git operations
mod git;
use git::Repo;

/// Errors for Git Detective
pub mod error;
use error::Error;

/// Main Entry point
pub struct GitDetective {
    repo: Repo,
}

/// Mode for GitDetective
pub enum Mode {
    /// Open local git repository
    Open,
    /// Clone remote git repository
    Clone(String),
}

impl GitDetective {
    /// Construct a New instance of GitDetective
    ///
    /// # Parameters
    ///
    /// mode: Mode
    ///
    /// Either `Mode::Open` or `Mode::Clone` where Clone takes a path to clone to
    ///
    /// uri: S
    ///
    /// Either a URL or a Path to a directory
    ///
    /// branch: &str
    ///
    /// Branch to checkout when opening
    ///
    /// # Returns
    ///
    /// Result<GitDetective, Error>
    pub fn new<S: Into<String>>(mode: Mode, uri: S, branch: &str) -> Result<Self, Error> {
        let uri = uri.into();
        let repo = match mode {
            Mode::Open => Repo::open(uri)?,
            Mode::Clone(path) => Repo::clone(&uri, path)?,
        };
        repo.checkout(branch)?;
        Ok(Self { repo })
    }
}
