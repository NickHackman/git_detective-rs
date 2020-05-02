//! Errors for GitDetective

/// All possible Errors that can occur in Git Detective
#[derive(Debug)]
pub enum Error {
    /// A Git Error occurred
    GitError(git2::Error),
    /// Failed to parse URL to clone
    ///
    /// Error occurs from calling `GitDetective::clone()`
    GitUrlError(url::ParseError),
    /// Repository isn't in a clean state
    ///
    /// Returned from `Repo::is_clean`
    UncleanState(git2::RepositoryState),
    /// Git String is not valid UTF-8
    ///
    /// Could be a Branch name, commit hash, etc
    NonUTF8String,
}

impl From<git2::Error> for Error {
    fn from(error: git2::Error) -> Self {
        Self::GitError(error)
    }
}

impl From<url::ParseError> for Error {
    fn from(error: url::ParseError) -> Self {
        Self::GitUrlError(error)
    }
}
