//! Basic error handling mechanisms
use std::error::Error;
use std::convert::From;
use std::{io, fmt};
use term;

/// The result type for the Parsing
pub type PathResult<'a, T> = Result<T, PathError>;

/// Representation for an error of the library
pub struct PathError {
    /// The error variant
    pub code: ErrorType,

    /// Additional description for the error
    pub description: String,

    /// The cause for this error
    pub cause: Option<Box<Error>>,
}

impl fmt::Display for PathError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f,
               "Code: {:?}, Description: {}",
               self.code,
               self.description)
    }
}

impl fmt::Debug for PathError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Display::fmt(self, f)
    }
}

impl Error for PathError {
    fn description(&self) -> &str {
        &self.description
    }
}

#[derive(Debug, PartialEq, Eq)]
/// Error codes as indicator what happened
pub enum ErrorType {
    /// The internal packet counter is overflown
    PacketCounterOverflow,

    /// Connection removed because of a timeout
    Timeout,

    /// Errors not directly from the library (like OS errors)
    Other,

    /// Internal errors which should not happen at all
    Internal,
}

// Error conversion
macro_rules! from_error {
    ($($p:ty,)*) => (
        $(impl From<$p> for PathError {
            fn from(err: $p) -> PathError {
                PathError {
                    code: ErrorType::Other,
                    description: err.description().to_owned(),
                    cause: Some(Box::new(err)),
                }
            }
        })*
    )
}

from_error! {
    io::Error,
    term::Error,
}

/// Throw an internal error
pub fn bail(code: ErrorType, description: &fmt::Display) -> PathError {
    PathError {
        code: code,
        description: description.to_string(),
        cause: None,
    }
}

macro_rules! bail {($code:expr, $($fmt:tt)*) => (
    return Err(::error::bail($code, &format_args!($($fmt)*)))
)}
