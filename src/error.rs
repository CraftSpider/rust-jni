use std::fmt::{Display, Formatter};
use std::error;

/// Error type for this library. Most often used to represent a case where an environment
/// action caused Java to begin throwing an error
#[derive(Debug)]
pub struct Error {
    from: Option<Box<dyn error::Error>>,
    error: String,
    code: i32
}

impl Error {

    /// Create a new error, with a messge and numeric code
    pub fn new(msg: &str, code: i32) -> Error {
        Error {
            from: None,
            error: String::from(msg),
            code
        }
    }

    /// Create a new error, based on an existing [error::Error]
    pub fn from(err: Box<dyn error::Error>) -> Error {
        let desc = err.to_string();

        Error {
            from: Some(err),
            error: desc,
            code: -1
        }
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Error in JVM: message \"{}\", code {}", self.error, self.code)
    }
}

impl error::Error for Error {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        if let Some(err) = &self.from {
            Some(err.as_ref())
        } else {
            None
        }
    }
}

/// Common result type using the local error type
pub type Result<T> = std::result::Result<T, Error>;
