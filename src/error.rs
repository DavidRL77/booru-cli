use std::error;
use core::fmt;

#[derive(Debug)]
pub enum CliError {
    InvalidArgument(String)
}

impl fmt::Display for CliError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CliError::InvalidArgument(s) => write!(f, "Invalid arguemnt: '{}'", s)
        }
    }
}

impl error::Error for CliError {}