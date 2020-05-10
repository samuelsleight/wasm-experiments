use crate::dom;

use std::fmt::{
    self,
    Display,
    Formatter
};

pub enum Error {
    Dom(dom::Error),

    // Context Errors
    ContextAcquisitionFailure,
    ContextNotSupported,
    ContextCastFailure,

    // Program/Shader Errors
    ProgramCreationFailure,
    ShaderCreationFailure,
    ShaderCompilationFailure(String),
    ProgramLinkFailure(String),

    // Attribute Errors
    AttributeNotFound(String),

    // Buffer Errors
    BufferCreationFailure,

    // Temporary
    Other(String)
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Error::Dom(ref e) => write!(f, "{}", e),

            // Context Errors
            Error::ContextAcquisitionFailure => write!(f, "Failed getting webl2 context from canvas"),
            Error::ContextNotSupported => write!(f, "webgl2 context is not supported"),
            Error::ContextCastFailure => write!(f, "Failed converting context"),

            // Program/Shader Errors
            Error::ProgramCreationFailure => write!(f, "Failed to create shader program object"),
            Error::ShaderCreationFailure => write!(f, "Failed to create shader object"),
            Error::ShaderCompilationFailure(ref s) => write!(f, "Failed to compile shader: {}", s),
            Error::ProgramLinkFailure(ref s) => write!(f, "Failed to link shader program: {}", s),

            // Attribute Errors
            Error::AttributeNotFound(ref s) => write!(f, "Unable to find attribute '{}' in shader program", s),

            // Buffer Errors
            Error::BufferCreationFailure => write!(f, "Failed to create buffer"),

            // Temporary
            Error::Other(ref s) => write!(f, "{}", s)
        }
    }
}

impl From<dom::Error> for Error {
    fn from(e: dom::Error) -> Error {
        Error::Dom(e)
    }
}

impl From<String> for Error {
    fn from(s: String) -> Error {
        Error::Other(s)
    }
}

pub type Result<T> = std::result::Result<T, Error>;
