use std::ops::AddAssign;

/// TODO: docs
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
