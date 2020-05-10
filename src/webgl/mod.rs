mod error;
mod context;
mod program;
mod attribute;

pub use error::{Error, Result};
pub use context::WebGlContext;
pub use program::ProgramBuilder;
pub use attribute::{Attribute, ActiveAttribute};
