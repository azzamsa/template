#![deny(unsafe_code)]

pub mod config;
pub mod controller;
pub mod error;
pub mod printer;

// Aliases
pub use error::Error;
