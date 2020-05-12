/// The State of a file
#[derive(Debug)]
pub struct FileStatus {
    /// Path to a file
    pub path: String,
    /// Status of a file
    pub status: git2::Status,
    /// Whether or not it's excluded by GitDetective
    pub excluded: bool,
}

impl<'repo> From<git2::StatusEntry<'repo>> for FileStatus {
    fn from(status_entry: git2::StatusEntry) -> Self {
        let path = String::from_utf8_lossy(status_entry.path_bytes()).to_string();
        Self {
            path,
            status: status_entry.status(),
            excluded: false,
        }
    }
}