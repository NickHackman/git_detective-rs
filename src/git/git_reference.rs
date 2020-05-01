use std::borrow::Cow;

use crate::error::Error;

/// A Git Object
pub enum GitReference<'repo> {
    /// A Commit
    Commit(git2::Commit<'repo>),
    /// A Branch
    Branch(git2::Branch<'repo>),
    /// A Tag
    Tag(git2::Tag<'repo>),
}

impl<'repo> GitReference<'repo> {
    /// Get the name of a GitReference
    ///
    /// Commit: Returns the unique Identifier
    ///
    /// NOTE: `GitReferenec::Commit` allocates
    ///
    /// Branch: Branch Name
    ///
    /// Tag: Tag Name
    ///
    /// # Errors
    ///
    /// **Only** Errors when self is GitReference::Branch
    ///
    /// # Returns
    ///
    /// Result<Cow<str>, Error>
    pub fn name(&self) -> Result<Cow<str>, Error> {
        match self {
            Self::Commit(commit) => Ok(commit.id().to_string().into()),
            Self::Tag(tag) => Ok(String::from_utf8_lossy(tag.name_bytes())),
            Self::Branch(branch) => Ok(String::from_utf8_lossy(branch.name_bytes()?)),
        }
    }

    /// Converts a `GitReference` into a `git2::Commit`
    ///
    /// # Errors
    ///
    /// Returns `Error::GitRefConvError` if `self` isn't variant `Commit`
    ///
    /// # Returns
    ///
    /// Result<git2::Commit<'repo>, Error>
    pub fn into_commit(self) -> Result<git2::Commit<'repo>, Error> {
        match self {
            Self::Commit(commit) => Ok(commit),
            Self::Tag(tag) => Err(Error::GitRefConvError(format!("{:?} into commit", tag))),
            Self::Branch(_) => Err(Error::GitRefConvError(
                "GitReference::Branch into commit".to_string(),
            )),
        }
    }

    /// Converts a `GitReference` into a `git2::Tag`
    ///
    /// # Errors
    ///
    /// Returns `Error::GitRefConvError` if `self` isn't variant `Tag`
    ///
    /// # Returns
    ///
    /// Result<git2::Tag<'repo>, Error>
    pub fn into_tag(self) -> Result<git2::Tag<'repo>, Error> {
        match self {
            Self::Commit(commit) => Err(Error::GitRefConvError(format!("{:?} into tag", commit))),
            Self::Tag(tag) => Ok(tag),
            Self::Branch(_) => Err(Error::GitRefConvError(
                "GitReference::Branch into commit".to_string(),
            )),
        }
    }

    /// Converts a `GitReference` into a `git2::Branch`
    ///
    /// # Errors
    ///
    /// Returns `Error::GitRefConvError` if `self` isn't variant `Branch`
    ///
    /// # Returns
    ///
    /// Result<git2::Branch<'repo>, Error>
    pub fn into_branch(self) -> Result<git2::Branch<'repo>, Error> {
        match self {
            Self::Commit(commit) => Err(Error::GitRefConvError(format!("{:?} into tag", commit))),
            Self::Tag(tag) => Err(Error::GitRefConvError(format!("{:?} into branch", tag))),
            Self::Branch(branch) => Ok(branch),
        }
    }
}
