use std::string::FromUtf8Error;

use crate::git::GitReference;
use crate::Error;
use crate::Signature;

/// A wrapper around [`git2::Tag`](https://docs.rs/git2/latest/git2/struct.Tag.html)
///
/// # Example
///
/// ```
/// # use git_detective::Error;
/// use git_detective::GitDetective;
/// # fn main() -> Result<(), Error> {
///
/// let gd = GitDetective::open(".")?;
/// let tags = gd.tags()?;
/// for tag in tags {
///   println!("{}", tag.name()?);
/// }
/// # }
/// ```
pub struct Tag<'repo> {
    inner: git2::Tag<'repo>,
}

impl<'repo> Tag<'_> {
    /// Name of Tag
    ///
    /// # Errors
    /// - Name isn't valid UTF-8
    pub fn name(&self) -> Result<String, FromUtf8Error> {
        String::from_utf8(self.inner.name_bytes().into())
    }

    /// Message of Tag
    ///
    /// # Optional
    /// - No message provided
    ///
    /// # Errors
    /// - Message isn't valid UTF-8
    pub fn message(&self) -> Option<Result<String, FromUtf8Error>> {
        self.inner
            .message_bytes()
            .map(|message| String::from_utf8(message.into()))
    }

    /// Tagger of a tag
    ///
    /// # Optional
    /// - Creator of tag is unknown
    pub fn tagger(&self) -> Option<Signature<'_>> {
        self.inner.tagger().map(|tagger| tagger.into())
    }
}

impl<'repo> GitReference<'repo> for Tag<'repo> {
    fn id(&self) -> git2::Oid {
        self.inner.id()
    }

    fn into_object(self) -> Result<git2::Object<'repo>, Error> {
        Ok(self.inner.into_object())
    }
}

#[doc(hidden)]
impl<'repo> From<git2::Tag<'repo>> for Tag<'repo> {
    fn from(inner: git2::Tag<'repo>) -> Self {
        Self { inner }
    }
}
