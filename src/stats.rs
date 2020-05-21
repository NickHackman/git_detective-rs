use std::ops::{Add, AddAssign};

use tokei::LineType;

/// Statistics for a file or collection of files
///
/// # Example
///
/// ```
/// # use git_detective::Error;
/// use git_detective::GitDetective;
///
/// # fn main() -> Result<(), Error> {
/// let gd = GitDetective::open(".")?;
/// let (lang, final_contribs) = gd.final_contributions_file(file!())?;
///
/// println!("Language = {}", lang);
/// for (contributor, stats) in final_contribs {
///   println!("{} wrote Total lines = {}", contributor, stats.lines);
/// }
///
/// # Ok(())
/// # }
/// ```
#[derive(Default, Debug, PartialEq, Eq, Clone, Copy)]
pub struct Stats {
    /// The number of total lines
    pub lines: usize,
    /// The number of blank lines
    pub blanks: usize,
    /// The number of comment lines
    pub comments: usize,
    /// The number of lines of code
    pub code: usize,
}

impl AddAssign for Stats {
    fn add_assign(&mut self, other: Self) {
        self.lines += other.lines;
        self.code += other.code;
        self.comments += other.comments;
        self.blanks += other.blanks;
    }
}

impl Add for Stats {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            lines: self.lines + other.lines,
            code: self.code + other.code,
            comments: self.comments + other.comments,
            blanks: self.blanks + other.blanks,
        }
    }
}

#[doc(hidden)]
impl AddAssign<LineType> for Stats {
    fn add_assign(&mut self, other: LineType) {
        match other {
            LineType::Blank => self.blanks += 1,
            LineType::Code => self.code += 1,
            LineType::Comment => self.comments += 1,
        }
        self.lines += 1;
    }
}

#[doc(hidden)]
impl AddAssign<&LineType> for Stats {
    fn add_assign(&mut self, other: &LineType) {
        match other {
            LineType::Blank => self.blanks += 1,
            LineType::Code => self.code += 1,
            LineType::Comment => self.comments += 1,
        }
        self.lines += 1;
    }
}
