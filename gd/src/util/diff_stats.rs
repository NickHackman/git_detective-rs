use std::collections::HashMap;

use git_detective::DiffStats;

// TODO: Actual scaling to terminal size
// Better scaling to Contributor names
const SEPARATOR_LENGTH: usize = 12 * 2 + 20;

fn header() {
    line_separator();
    println!(
        "{:>15} {:>12} {:>12}",
        "Contributor", "Additions", "Deletions"
    );
    line_separator();
}

pub fn line_separator() {
    println!("{}", "-".repeat(SEPARATOR_LENGTH));
}

fn row(name: &str, stats: &DiffStats) {
    println!(
        "{:>15} {:>12} {:>12}",
        name, stats.insertions, stats.deletions
    );
}

pub fn table(contribs: HashMap<String, DiffStats>) {
    header();
    for (author, diff_stats) in contribs.iter() {
        row(author, diff_stats);
    }
    line_separator()
}
