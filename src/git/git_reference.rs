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
    /// Branch: Branch Name
    ///
    /// Tag: Tag Name
    ///
    /// # Errors
    ///
    /// When The name isn't valid UTF-8
    ///
    /// # Returns
    ///
    /// Result<Cow<str>, Error>
    pub fn name(&self) -> Result<String, Error> {
        match self {
            Self::Commit(commit) => Ok(commit.id().to_string()),
            Self::Tag(tag) => Ok(String::from_utf8(tag.name_bytes().into())?),
            Self::Branch(branch) => Ok(String::from_utf8(branch.name_bytes()?.into())?),
        }
    }
}
