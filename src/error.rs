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
}

impl From<git2::Error> for Error {
    fn from(error: git2::Error) -> Self {
        Self::GitError(error)
    }
}
