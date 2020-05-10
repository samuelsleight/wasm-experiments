use crate::dom;

use std::fmt::{
    self,
    Display,
    Formatter
};

pub enum Error {
    Dom(dom::Error),
    ContextAcquisitionFailure,
    ContextNotSupported,
    ContextCastFailure
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Error::Dom(ref e) => write!(f, "{}", e),
            Error::ContextAcquisitionFailure => write!(f, "Failed getting webl2 context from canvas"),
            Error::ContextNotSupported => write!(f, "webgl2 context is not supported"),
            Error::ContextCastFailure => write!(f, "Failed converting context")
        }
    }
}

impl From<dom::Error> for Error {
    fn from(e: dom::Error) -> Error {
        Error::Dom(e)
    }
}

pub type Result<T> = std::result::Result<T, Error>;
