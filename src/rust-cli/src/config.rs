use std::path::Path;

#[derive(Debug, Clone)]
pub struct Config {
    pub files: Option<Vec<Path>>,

    /// Whether or not the output should be colorized
    pub colored_output: bool,
}
