use std::fs;

use ansi_term::Colour::Green;

use crate::{config::Config, error::Result};

pub fn print_files(config: &Config) -> Result<bool> {
    let no_errors: bool = true;
    let colored_output = config.colored_output;
    let files = &config.files;

    if files.is_none() {
        return Err(crate::Error::Msg("no file".into()))
    }

    for file in files.unwrap() {
        let content =
            fs::read_to_string(file).map_err(|e| format!("'{}': {}", file.display(), e))?;

        println!(
            "{}",
            if colored_output {
                format!("{}", Green.paint(content))
            } else {
                content
            }
        );
    }

    Ok(no_errors)
}
