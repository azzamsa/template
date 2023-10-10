use cfg_if::cfg_if;
use thiserror::Error;

#[derive(Error, Debug, Clone)]
pub enum Error {
    #[error("Internal error")]
    Internal(String),

    #[error("{0}")]
    NotFound(String),

    #[error("{0}")]
    InvalidArgument(String),
}

impl std::convert::From<tracing_subscriber::filter::ParseError> for Error {
    fn from(err: tracing_subscriber::filter::ParseError) -> Self {
        Error::Internal(err.to_string())
    }
}

impl std::convert::From<tracing_subscriber::filter::FromEnvError> for Error {
    fn from(err: tracing_subscriber::filter::FromEnvError) -> Self {
        Error::Internal(err.to_string())
    }
}

cfg_if! {
    if #[cfg(feature = "ssr")] {
        impl std::convert::From<hyper::Error> for Error {
            fn from(err: hyper::Error) -> Self {
                Error::Internal(err.to_string())
            }
        }
    }
}
