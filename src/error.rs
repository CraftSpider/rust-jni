//!
//! Module containing implementation of a rust_jni error type, as well as a Result type alias
//!

use std::fmt::{Display, Formatter};
use std::error;

/// Error type for this library. Most often used to represent a case where an environment
/// action caused Java to begin throwing an error
#[derive(Debug)]
pub enum Error {
    /// JNI error returned from a different error type
    Induced(Box<dyn error::Error>),
    /// JNI error returned with a message and code
    General(String, i32),
    /// JNI error returned when a pointer is null
    NullPointer(String)
}

impl Error {

    /// Create a new error, with a message and numeric code
    pub fn new(msg: &str, code: i32) -> Error {
        match code {
            _ => {
                Error::General(String::from(msg), code)
            }
        }
    }

    /// Create a new null-pointer error, with a message
    pub fn new_null(ctx: &str) -> Error {
        Error::NullPointer(String::from(ctx))
    }

    /// Create a new error, based on an existing [error::Error]
    pub fn from(err: Box<dyn error::Error>) -> Error {
        Error::Induced(err)
    }

}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::General(msg, code) =>
                write!(f, "Error in JVM: message \"{}\", code {}", msg, code),
            Error::Induced(e) => {
                write!(f, "Error occurred in JNI, source: {}", e)
            }
            Error::NullPointer(context) => {
                write!(f, "Error in JNI: Pointer was null in {}", context)
            }
        }
    }
}

impl error::Error for Error {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        if let Error::Induced(err) = self {
            Some(err.as_ref())
        } else {
            None
        }
    }
}

/// Common result type using the local error type
pub type Result<T> = std::result::Result<T, Error>;
