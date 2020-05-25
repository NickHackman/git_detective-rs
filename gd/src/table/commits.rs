use std::fmt;

use git_detective::Commit;

const WIDTH: usize = 96;
const SHORT_ID_LEN: usize = 6;
const LONG_ID_LEN: usize = 10;
const ITEMS: usize = 5;

pub struct CommitsTable<'commit> {
    separator_length: usize,
    id_length: usize,
    commits: Vec<Commit<'commit>>,
}

impl<'commit> CommitsTable<'commit> {
    pub fn new(commits: Vec<Commit<'commit>>, dimensions: Option<(usize, usize)>) -> Self {
        let (mut width, _) = dimensions.unwrap_or((WIDTH, 0));
        if width > WIDTH {
            width = WIDTH;
        }
        let id_length = if CommitsTable::unique_short_ids(&commits) {
            SHORT_ID_LEN
        } else {
            LONG_ID_LEN
        };
        Self {
            commits,
            id_length,
            separator_length: width,
        }
    }

    // Check to see if `SHORT_ID_LEN` is long enough for uniquness
    fn unique_short_ids(commits: &[Commit<'_>]) -> bool {
        for (index, commit) in commits.iter().enumerate() {
            for (i, c) in commits.iter().enumerate() {
                if index != i
                    && commit
                        .id()
                        .to_string()
                        .chars()
                        .take(SHORT_ID_LEN)
                        .collect::<Vec<_>>()
                        == c.id()
                            .to_string()
                            .chars()
                            .take(SHORT_ID_LEN)
                            .collect::<Vec<_>>()
                {
                    return false;
                }
            }
        }
        true
    }

    fn item_width(&self) -> usize {
        let mut item_width = self.separator_length / ITEMS;
        item_width += (item_width - 4) / ITEMS - 1;
        item_width
    }

    fn row(&self, f: &mut fmt::Formatter<'_>, commit: &Commit<'_>) -> fmt::Result {
        let id: String = commit
            .id()
            .to_string()
            .chars()
            .take(self.id_length)
            .collect();
        let author = commit.author().name().unwrap_or_default();
        let committer = commit.committer().name().unwrap_or_default();
        let long_date = commit.date().to_string();
        let no_tmz_date = &long_date[..long_date.len() - 4];
        let mut summary = commit
            .summary()
            .unwrap_or_else(|| Ok(String::new()))
            .unwrap_or_default();
        if summary.len() > self.separator_length / ITEMS {
            let mut truncated_summary: String = summary
                .chars()
                .take(self.separator_length / ITEMS - 3)
                .collect();
            truncated_summary.push_str("...");
            summary = truncated_summary;
        }
        writeln!(
            f,
            "{:^id_length$} {:^width$} {:^width$} {:^width$} {:<width$}",
            id,
            author,
            committer,
            no_tmz_date,
            summary,
            id_length = self.id_length,
            width = self.item_width()
        )
    }

    fn header(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.line_separator(f)?;
        writeln!(
            f,
            "{:^id_length$} {:^width$} {:^width$} {:^width$} {:^width$}",
            "ID",
            "Author",
            "Committer",
            "Date (UTC)",
            "Summary",
            id_length = self.id_length,
            width = self.item_width()
        )?;
        self.line_separator(f)
    }

    fn line_separator(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "{}", "-".repeat(self.separator_length))
    }
}

impl<'commit> fmt::Display for CommitsTable<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.commits.is_empty() {
            return Ok(());
        }
        self.header(f)?;
        for commit in self.commits.iter() {
            self.row(f, &commit)?;
        }
        self.line_separator(f)?;
        writeln!(f)?;
        Ok(())
    }
}
