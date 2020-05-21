use std::ops::Range;

pub struct Blame {
    blame_hunks: Vec<BlameHunk>,
}

impl Blame {
    pub fn iter(&self) -> std::slice::Iter<'_, BlameHunk> {
        self.blame_hunks.iter()
    }
}

impl From<git2::Blame<'_>> for Blame {
    fn from(blame: git2::Blame<'_>) -> Self {
        let blame_hunks: Vec<_> = blame.iter().map(BlameHunk::from).collect();
        Self { blame_hunks }
    }
}

pub struct BlameHunk {
    final_start_line: usize,
    lines_in_hunk: usize,
    pub author: Result<String, std::string::FromUtf8Error>,
}

impl From<git2::BlameHunk<'_>> for BlameHunk {
    fn from(blame_hunk: git2::BlameHunk<'_>) -> Self {
        Self {
            final_start_line: blame_hunk.final_start_line(),
            lines_in_hunk: blame_hunk.lines_in_hunk(),
            author: String::from_utf8(blame_hunk.final_signature().name_bytes().into()),
        }
    }
}

impl BlameHunk {
    pub fn final_range(&self) -> Range<usize> {
        Range {
            start: self.final_start_line,
            end: self.final_start_line + self.lines_in_hunk,
        }
    }
}
