use std::fmt;

use git_detective::{ProjectStats, Stats};

const WIDTH: usize = 72;
const ITEMS: usize = 6;

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
            width = self.separator_length / ITEMS,
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
            width = self.separator_length / ITEMS,
        )?;
        self.line_separator(f)
    }

    fn line_separator(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "{}", "-".repeat(self.separator_length))
    }
}

impl fmt::Display for FinalContributionsTable {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.stats.is_empty() {
            return Ok(());
        }
        for (author, lang_map) in self.stats.iter() {
            self.author(f, &author)?;
            self.header(f)?;
            let mut total = Stats::default();

            let mut lang_stats: Vec<_> = lang_map.iter().collect();
            // Sort by
            lang_stats
                .as_mut_slice()
                .sort_unstable_by(|(lang_lhs, _), (lang_rhs, _)| lang_lhs.cmp(lang_rhs));

            for (lang, stats) in lang_stats {
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
