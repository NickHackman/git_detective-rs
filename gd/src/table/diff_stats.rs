use std::collections::HashMap;
use std::fmt;

use git_detective::DiffStats;

const WIDTH: usize = 60;
const ITEMS: usize = 5;

pub struct DiffStatsTable {
    separator_length: usize,
    stats: HashMap<String, DiffStats>,
}

impl DiffStatsTable {
    pub fn new(stats: HashMap<String, DiffStats>, dimensions: Option<(usize, usize)>) -> Self {
        let (mut width, _) = dimensions.unwrap_or((WIDTH, 0));
        if width > WIDTH {
            width = WIDTH;
        }
        Self {
            stats,
            separator_length: width,
        }
    }

    fn author_width(&self) -> usize {
        2 * (self.separator_length / ITEMS)
    }

    fn row(&self, f: &mut fmt::Formatter<'_>, name: &str, stats: &DiffStats) -> fmt::Result {
        let author_width = self.author_width();
        // Truncate names that are too long
        let name = if name.len() > author_width {
            let mut name: String = name.chars().take(author_width - 3).collect();
            name.push_str("...");
            name
        } else {
            name.to_string()
        };
        writeln!(
            f,
            "{:^author_width$} {:>width$} {:>width$}",
            name,
            stats.insertions,
            stats.deletions,
            author_width = author_width,
            width = self.separator_length / ITEMS,
        )
    }

    fn header(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.line_separator(f)?;
        writeln!(
            f,
            "{:^author_width$} {:>width$} {:>width$}",
            "Contributor",
            "Insertions",
            "Deletions",
            author_width = self.author_width(),
            width = self.separator_length / ITEMS,
        )?;
        self.line_separator(f)
    }

    fn line_separator(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "{}", "-".repeat(self.separator_length))
    }
}

impl fmt::Display for DiffStatsTable {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.stats.is_empty() {
            return Ok(());
        }
        self.header(f)?;
        let mut authored_diffs: Vec<_> = self.stats.iter().collect();
        authored_diffs
            .as_mut_slice()
            .sort_unstable_by(|(author_lhs, _), (author_rhs, _)| author_lhs.cmp(author_rhs));
        for (author, diff_stats) in authored_diffs.iter() {
            self.row(f, &author, diff_stats)?;
        }
        self.line_separator(f)?;
        writeln!(f, "")?;
        Ok(())
    }
}
