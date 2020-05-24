use std::collections::{HashMap, HashSet};
use std::path::PathBuf;

// TODO: Actual scaling to terminal size
const SEPARATOR_LENGTH: usize = 12 * 2 + 15;

fn header() {
    line_separator();
    println!("{:>12} {:>12}", "Contributor", "Files");
    line_separator();
}

pub fn line_separator() {
    println!("{}", "-".repeat(SEPARATOR_LENGTH));
}

fn row(name: &str, files: &HashSet<PathBuf>) {
    println!("{:>12} {:>12}", name, files.len());
}

pub fn table(contrib_files: HashMap<String, HashSet<PathBuf>>) {
    header();
    for (author, files) in contrib_files {
        row(&author, &files);
    }
    line_separator();
    println!();
}
