use crate::Error;

/// Trait bounding for [`Commit`](struct.Commit.html), [`Tag`](struct.Tag.html), and [`Branch`](struct.Branch.html)
/// specifically for use in [`checkout`](git/struct.Repo.html#method.checkout)
pub trait GitReference<'repo> {
    /// Converts a GitReference into a [`git2::Object`](https://docs.rs/git2/latest/git2/struct.Object.html)
    ///
    /// # Errors
    /// - Only in [`Branch`](struct.Branch.html) implementation
    fn into_object(self) -> Result<git2::Object<'repo>, Error>;

    /// Get the [`git2::Oid`](https://docs.rs/git2/latest/git2/struct.Oid.html) of a GitReference
    fn id(&self) -> git2::Oid;
}
