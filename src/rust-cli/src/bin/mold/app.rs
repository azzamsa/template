use std::path::{Path, PathBuf};

use atty::{self, Stream};
use clap::ArgMatches;
use mold::{config::Config, error::Result};

use crate::clap_app;

pub struct App {
    pub matches: ArgMatches,
    interactive_output: bool,
}

impl App {
    pub fn new() -> Self {
        #[cfg(windows)]
        let _ = ansi_term::enable_ansi_support();

        let interactive_output = atty::is(Stream::Stdout);

        App {
            matches: Self::matches(interactive_output),
            interactive_output,
        }
    }

    fn matches(interactive_output: bool) -> ArgMatches {
        clap_app::build(interactive_output).get_matches_from(wild::args())
    }

    pub fn config(&self) -> Result<Config> {
        let files = self.files();
        let colored_output = match self.matches.get_one::<String>("color").map(|s| s.as_str()) {
            Some("always") => true,
            Some("never") => false,
            _ => self.interactive_output,
        };
        Ok(Config {
            files,
            colored_output,
        })
    }
    fn files(&self) -> Option<Vec<&Path>> {
        self.matches
            .get_many::<PathBuf>("FILE")
            .map(|vs| vs.map(|p| p.as_path()).collect::<Vec<_>>())
    }
}
