use std::fmt;

use git_detective::Tag;

const WIDTH: usize = 96;
const ITEMS: usize = 3;

pub struct TagsTable<'tag> {
    separator_length: usize,
    tags: Vec<Tag<'tag>>,
}

impl<'tag> TagsTable<'tag> {
    pub fn new(tags: Vec<Tag<'tag>>, dimensions: Option<(usize, usize)>) -> Self {
        let (mut width, _) = dimensions.unwrap_or((WIDTH, 0));
        if width > WIDTH {
            width = WIDTH;
        }
        Self {
            tags,
            separator_length: width,
        }
    }

    fn row(&self, f: &mut fmt::Formatter<'_>, tag: &Tag<'_>) -> fmt::Result {
        let (tagger, date) = match tag.tagger() {
            Some(tagger) => {
                let long_date = tagger.date().to_string();
                let date = long_date[..long_date.len() - 4].to_string();
                let tagger = tagger.name().unwrap_or(String::new());
                (tagger, date)
            }
            None => (String::new(), String::new()),
        };
        let name = tag.name().unwrap_or(String::new());
        writeln!(
            f,
            "{:<width$} {:^width$} {:^width$}",
            name,
            tagger,
            date,
            width = self.separator_length / ITEMS
        )?;
        Ok(())
    }

    fn header(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.line_separator(f)?;
        writeln!(
            f,
            "{:<width$} {:^width$} {:^width$}",
            "Name",
            "Tagger",
            "Date (UTC)",
            width = self.separator_length / ITEMS,
        )?;
        self.line_separator(f)
    }

    fn line_separator(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "{}", "-".repeat(self.separator_length))
    }
}

impl<'tag> fmt::Display for TagsTable<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.tags.is_empty() {
            return Ok(());
        }
        self.header(f)?;
        for tag in self.tags.iter() {
            self.row(f, &tag)?;
        }
        self.line_separator(f)?;
        writeln!(f)?;
        Ok(())
    }
}
