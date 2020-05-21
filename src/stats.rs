use std::ops::{Add, AddAssign};

use tokei::LineType;

/// TODO: Docs
#[derive(Default, Debug, PartialEq, Eq, Clone, Copy)]
pub struct Stats {
    /// TODO: Docs
    pub lines: usize,
    /// TODO: Docs
    pub blanks: usize,
    /// TODO: Docs
    pub comments: usize,
    /// TODO: Docs
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
