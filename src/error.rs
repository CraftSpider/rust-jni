use std::fmt::{Display, Formatter};
use std::error;

/// For when JNI method return error codes
#[derive(Debug)]
pub struct Error {
    from: Option<Box<dyn error::Error>>,
    error: String,
    code: i32
}

impl Error {
    pub fn new(msg: &str, code: i32) -> Error {
        Error {
            from: None,
            error: String::from(msg),
            code
        }
    }

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

pub type Result<T> = std::result::Result<T, Error>;
