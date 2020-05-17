use std::string::FromUtf8Error;

use chrono::{DateTime, NaiveDateTime, Utc};

/// A wrapper around [`git2::Signature`](https://docs.rs/git2/latest/git2/struct.Signature.html)
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
///   let author = commit.author();
///   println!("Name = {}, Email = {}", author.name()?, author.email()?);
/// }
/// # Ok(())
/// # }
/// ```
pub struct Signature<'repo> {
    inner: git2::Signature<'repo>,
}

impl<'repo> Signature<'_> {
    /// Name of [`Author`](struct.Commit.html#method.author), [`Committer`](struct.Commit.html#method.committer), or [`Tagger`](struct.Tag.html#method.tagger)
    pub fn name(&self) -> Result<String, FromUtf8Error> {
        String::from_utf8(self.inner.name_bytes().into())
    }

    /// Email of [`Author`](struct.Commit.html#method.author), [`Committer`](struct.Commit.html#method.committer), or [`Tagger`](struct.Tag.html#method.tagger)
    pub fn email(&self) -> Result<String, FromUtf8Error> {
        String::from_utf8(self.inner.email_bytes().into())
    }

    /// Date commited
    pub fn date(&self) -> DateTime<Utc> {
        let timestamp = self.inner.when().seconds();
        let naive = NaiveDateTime::from_timestamp(timestamp, 0);
        DateTime::<Utc>::from_utc(naive, Utc)
    }
}

#[doc(hidden)]
impl<'repo> From<git2::Signature<'repo>> for Signature<'repo> {
    fn from(inner: git2::Signature<'repo>) -> Self {
        Self { inner }
    }
}
