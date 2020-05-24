use std::fmt;
use std::io;

pub(crate) mod final_contributions;
pub use final_contributions::FinalContributionsTable;

pub trait Table: fmt::Display {
    fn header(&self, _: &mut fmt::Formatter<'_>) -> fmt::Result;

    fn line_separator(&self, _: &mut fmt::Formatter<'_>) -> fmt::Result;
}
