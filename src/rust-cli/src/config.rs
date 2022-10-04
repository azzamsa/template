use std::path::Path;

#[derive(Debug, Clone)]
pub struct Config<'a> {
    pub files: Vec<&'a Path>,

    /// Whether or not the output should be colorized
    pub colored_output: bool,
}
