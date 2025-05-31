use std::fmt;
use std::fmt::write;
use colored::Colorize;

pub(crate) struct CloneFailure;

impl fmt::Display for CloneFailure {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", "[ERROR] - An error occured while cloning the website!".bright_red())
    }
}
