use std::string::FromUtf8Error;

use crate::git::GitReference;
use crate::Error;

/// A wrapper around [`git2::Branch`](https://docs.rs/git2/latest/git2/struct.Branch.html)
///
/// # Example
///
/// ```
/// # use git_detective::Error;
/// use git_detective::GitDetective;
/// # fn main() -> Result<(), Error> {
///
/// let gd = GitDetective::open(".")?;
/// let branches = gd.branches()?;
/// for branch in branches {
///   println!("{}", branch.name()?);
/// }
/// # }
/// ```
pub struct Branch<'repo> {
    inner: git2::Branch<'repo>,
}

impl<'repo> Branch<'_> {
    /// Whether or not this branch is pointed to by HEAD
    pub fn is_head(&self) -> bool {
        self.inner.is_head()
    }

    /// Name of the branch
    ///
    /// # Errors
    /// - Name isn't valid UTF-8
    /// - Branch isn't a local or remote branch
    pub fn name(&self) -> Result<Result<String, FromUtf8Error>, Error> {
        Ok(self
            .inner
            .name_bytes()
            .map(|name| String::from_utf8(name.into()))?)
    }
}

impl<'repo> GitReference<'repo> for Branch<'repo> {
    fn into_object(self) -> Result<git2::Object<'repo>, Error> {
        Ok(self.inner.get().peel(git2::ObjectType::Any)?)
    }

    fn id(&self) -> git2::Oid {
        // Safe to unwrap, Inner must exist
        self.inner.get().target().unwrap()
    }
}

#[doc(hidden)]
impl<'repo> From<git2::Branch<'repo>> for Branch<'repo> {
    fn from(inner: git2::Branch<'repo>) -> Self {
        Self { inner }
    }
}
