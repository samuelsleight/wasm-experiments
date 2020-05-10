use crate::dom;

use super::error::{
    Error,
    Result
};

use wasm_bindgen::JsCast;

use web_sys::WebGl2RenderingContext;

pub struct WebGlContext {
    context: WebGl2RenderingContext
}

impl WebGlContext {
    pub fn from_canvas_with_id(id: &str) -> Result<WebGlContext> {
        let canvas = dom::canvas(id)?;

        let context: WebGl2RenderingContext = canvas
            .get_context("webgl2")
            .map_err(|_| Error::ContextAcquisitionFailure)?
            .ok_or(Error::ContextNotSupported)?
            .dyn_into()
            .map_err(|_| Error::ContextCastFailure)?;

        Ok(WebGlContext {
            context
        })
    }

    // Temporary - until all functionality is implemented
    pub fn into_context(self) -> WebGl2RenderingContext {
        self.context
    }
}
