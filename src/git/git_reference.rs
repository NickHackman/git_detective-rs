use std::borrow::Cow;

use crate::error::Error;

pub enum GitReference<'repo> {
    Commit(git2::Commit<'repo>),
    Branch(git2::Branch<'repo>),
    Tag(git2::Tag<'repo>),
}

impl<'repo> GitReference<'repo> {
    /// Get the name of a GitReference
    ///
    /// Commit: Returns the unique Identifier
    ///
    /// NOTE: Commit allocates
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
}
