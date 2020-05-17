use std::string::FromUtf8Error;

use crate::error::Error;
use crate::git::GitReference;
use crate::Signature;

/// A wrapper around [`git2::Commit`](https://docs.rs/git2/latest/git2/struct.Commit.html)
///
/// # Example
///
/// ```
/// # use git_detective::Error;
/// use git_detective::GitDetective;
/// # fn main() -> Result<(), Error> {
///
/// let gd = GitDetective::open(".")?;
/// let commits = gd.commits()?;
/// for commit in commits {
///   println!("{}", commit.author().name()?);
/// }
/// # }
/// ```
pub struct Commit<'repo> {
    inner: git2::Commit<'repo>,
}

impl<'repo> Commit<'_> {
    /// Author of a commit
    ///
    /// An author is the person who originally wrote the code,
    /// while a committer is the person who committed the code on behalf of the author
    pub fn author(&self) -> Signature<'_> {
        self.inner.author().into()
    }

    /// Committer of a commit
    ///
    /// An author is the person who originally wrote the code,
    /// while a committer is the person who committed the code on behalf of the author
    pub fn committer(&self) -> Signature<'_> {
        self.inner.committer().into()
    }

    /// Commit message
    ///
    /// # Errors
    /// - Message isn't valid UTF-8
    pub fn message(&self) -> Result<String, FromUtf8Error> {
        String::from_utf8(self.inner.message_raw_bytes().into())
    }

    /// First line of the commit message
    ///
    /// # Optional
    /// - No summary provided
    ///
    /// # Errors
    /// - Summary isn't valid UTF-8
    pub fn summary(&self) -> Option<Result<String, FromUtf8Error>> {
        self.inner
            .summary_bytes()
            .map(|summary| String::from_utf8(summary.into()))
    }
}

impl<'repo> GitReference<'repo> for Commit<'repo> {
    fn into_object(self) -> Result<git2::Object<'repo>, Error> {
        Ok(self.inner.into_object())
    }

    fn id(&self) -> git2::Oid {
        self.inner.id()
    }
}

#[doc(hidden)]
impl<'repo> From<git2::Commit<'repo>> for Commit<'repo> {
    fn from(inner: git2::Commit<'repo>) -> Self {
        Self { inner }
    }
}
