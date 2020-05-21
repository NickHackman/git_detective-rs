//! Errors for GitDetective

use std::path::PathBuf;

/// All possible Errors that can occur in Git Detective
#[derive(thiserror::Error, Debug)]
pub enum Error {
    /// A Git Error occurred
    #[error("Git Error: `{0}`")]
    GitError(#[from] git2::Error),
    /// Failed to parse URL to clone
    ///
    /// Error occurs from calling [`GitDetective::clone()`](struct.GitDetective.html#method.clone)
    #[error("URL Error: `{0}`")]
    GitUrlError(#[from] url::ParseError),
    /// Repository isn't in a clean state
    ///
    /// Returned from `Repo::is_clean`
    #[error("Unclean State Error: Repository is in `{0:?}` state, but expected clean")]
    UncleanState(git2::RepositoryState),
    /// Git String is not valid UTF-8
    ///
    /// Could be a Branch name, commit hash, etc
    #[error("Non UTF-8 Error: named `{0:?}`")]
    NonUTF8String(#[from] std::string::FromUtf8Error),

    /// IO Error
    ///
    /// Occurrred in [`final_contributions`](struct.GitDetective.html#method.final_contributions) or
    /// [`final_contributions_file`](struct.GitDetective.html#method.final_contributions_file)
    #[error("`{0}` in `{1:?}`")]
    IOError(std::io::Error, PathBuf),
}
