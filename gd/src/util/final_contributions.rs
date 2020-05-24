use git_detective::{ProjectStats, Stats};

// TODO: Actual scaling to terminal size
const SEPARATOR_LENGTH: usize = 12 * 6;

fn header() {
    line_separator();
    println!(
        "{:>12} {:>12} {:>12} {:>12} {:>12}",
        "Language", "Lines", "Code", "Comments", "Blanks"
    );
    line_separator();
}

pub fn line_separator() {
    println!("{}", "-".repeat(SEPARATOR_LENGTH));
}

fn row(row_name: &str, stats: &Stats) {
    println!(
        "{:>12} {:>12} {:>12} {:>12} {:>12}",
        row_name, stats.lines, stats.code, stats.comments, stats.blanks
    );
}

pub fn table(final_contribs: ProjectStats) {
    for (author, lang_map) in final_contribs.iter() {
        let author_title = format!("{}'s Contributions", author);
        let padding = (SEPARATOR_LENGTH - author_title.len()) / 2;
        println!("{0}{1}{0}", " ".repeat(padding), author_title);
        header();
        let mut total = Stats::default();

        for (lang, stats) in lang_map {
            total += *stats;
            row(lang, stats);
        }
        line_separator();
        row("Total", &total);
        line_separator();
        println!();
    }
}
