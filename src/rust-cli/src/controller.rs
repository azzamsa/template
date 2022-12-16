use crate::{
    config::Config,
    error::{default_error_handler, Result},
    printer::print_files,
};

pub struct Controller {
    config: Config,
}

impl Controller {
    pub fn new(config:  Config) -> Controller {
        Controller { config }
    }

    pub fn run(&self) -> Result<bool> {
        let result = print_files(self.config);
        match result {
            Ok(true) => Ok(true),
            Ok(false) => Ok(false),
            Err(error) => {
                default_error_handler(&error);
                Ok(false)
            }
        }
    }
}
