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
