//!
//!
//!

/// All possible Errors that can occur in Git Detective
#[derive(Debug)]
pub enum Error {
    /// A Git Error occurred
    GitError(git2::Error),
    /// Branch doesn't exist in the repository
    ///
    /// Error occurs from calling `git::Repo::checkout()`
    BranchDoesntExist(String),

    /// Failed to parse URL to clone
    ///
    /// Error occurs from calling `GitDetective::clone()`
    GitUrlError(url::ParseError),

    /// Failed to convert GitReference into a subtype
    ///
    /// using one of `into_commit`, `into_tag`, or `into_branch`
    GitRefConvError(String),
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
