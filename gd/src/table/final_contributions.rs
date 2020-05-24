use std::fmt;

use git_detective::{ProjectStats, Stats};

use super::Table;

const WIDTH: usize = 72;

pub struct FinalContributionsTable {
    separator_length: usize,
    stats: ProjectStats,
}

impl FinalContributionsTable {
    pub fn new(stats: ProjectStats, dimensions: Option<(usize, usize)>) -> Self {
        let (mut width, _) = dimensions.unwrap_or((WIDTH, 0));
        if width > WIDTH {
            width = WIDTH;
        }
        Self {
            stats,
            separator_length: width,
        }
    }

    fn row(&self, f: &mut fmt::Formatter<'_>, row_name: &str, stats: &Stats) -> fmt::Result {
        writeln!(
            f,
            "{:>width$} {:>width$} {:>width$} {:>width$} {:>width$}",
            row_name,
            stats.lines,
            stats.code,
            stats.comments,
            stats.blanks,
            width = self.separator_length / 6,
        )
    }

    fn author(&self, f: &mut fmt::Formatter<'_>, author: &str) -> fmt::Result {
        writeln!(
            f,
            "{:^width$}",
            format!("{}'s contributions", author),
            width = self.separator_length
        )
    }
}

impl Table for FinalContributionsTable {
    fn header(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.line_separator(f)?;
        writeln!(
            f,
            "{:>width$} {:>width$} {:>width$} {:>width$} {:>width$}",
            "Language",
            "Lines",
            "Code",
            "Comments",
            "Blanks",
            width = self.separator_length / 6,
        )?;
        self.line_separator(f)
    }

    fn line_separator(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "{}", "-".repeat(self.separator_length))
    }
}

impl fmt::Display for FinalContributionsTable {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for (author, lang_map) in self.stats.iter() {
            self.author(f, &author)?;
            self.header(f)?;
            let mut total = Stats::default();

            for (lang, stats) in lang_map {
                total += *stats;
                self.row(f, lang, stats)?;
            }
            self.line_separator(f)?;
            self.row(f, "Total", &total)?;
            self.line_separator(f)?;
            writeln!(f, "")?;
        }
        Ok(())
    }
}
