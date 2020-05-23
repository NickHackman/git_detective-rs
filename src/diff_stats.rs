use std::ops::AddAssign;

/// Insertion and Deletion statistics for Commit diffs
///
/// # Example
///
/// ```
/// # use git_detective::Error;
/// use git_detective::GitDetective;
///
/// # fn main() -> Result<(), Error> {
/// let mut gd = GitDetective::open(".")?;
/// let diff_stats = gd.diff_stats()?;
/// for (author, diff_stat) in diff_stats {
///   println!("{}: +{} -{}", author, diff_stat.insertions, diff_stat.deletions);
/// }
///
/// # Ok(())
/// # }
/// ```
#[derive(Debug, PartialEq, Eq, Default)]
pub struct DiffStats {
    /// Lines of code inserted
    pub insertions: usize,
    /// Lines of code deleted
    pub deletions: usize,
}

#[doc(hidden)]
impl AddAssign<git2::DiffStats> for DiffStats {
    fn add_assign(&mut self, other: git2::DiffStats) {
        self.insertions += other.insertions();
        self.deletions += other.deletions();
    }
}
