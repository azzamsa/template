use std::path::Path;

use atty::{self, Stream};
use clap::ArgMatches;
use cli::{config::Config, error::Result};

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
        let colored_output = match self.matches.value_of("color") {
            Some("always") => true,
            Some("never") => false,
            _ => self.interactive_output,
        };
        Ok(Config {
            files,
            colored_output,
        })
    }
    fn files(&self) -> Vec<&Path> {
        match self.matches.values_of("FILE") {
            Some(files) => {
                files
                    .into_iter()
                    .map(Path::new)
                    // .take_while(|f| f.exists())
                    // Filtering only existing fails can't fails the program if
                    // only one file passed
                    .collect()
            }
            None => unreachable!("No file supplied"),
        }
    }
}
