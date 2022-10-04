use ansi_term::Colour::Red;
use thiserror::Error;

pub type Result<T> = std::result::Result<T, Error>;

/// all possible errors returned by the app.
#[derive(Error, Debug)]
pub enum Error {
    #[error(transparent)]
    Io(#[from] std::io::Error),

    #[error("{0}")]
    Msg(String),
}

impl From<String> for Error {
    fn from(s: String) -> Self {
        Error::Msg(s)
    }
}

pub fn default_error_handler(error: &Error) {
    match error {
        Error::Io(ref io_error) if io_error.kind() == ::std::io::ErrorKind::BrokenPipe => {
            ::std::process::exit(0);
        }
        _ => {
            eprintln!("{}: {}", Red.paint("[cli error]"), error);
        }
    };
}
