use crate::dom;

use super::{
    program::ProgramBuilder,
    mesh::Mesh,
    vertex::Vertex,
    error::{
        Error,
        Result
    },
};

use wasm_bindgen::JsCast;

use web_sys::WebGl2RenderingContext;

pub struct WebGlContext {
    context: WebGl2RenderingContext
}

impl WebGlContext {
    pub fn from_canvas_with_id(id: &str) -> Result<WebGlContext> {
        Ok(WebGlContext {
            context: dom::canvas(id)?
                .get_context("webgl2")
                .map_err(|_| Error::ContextAcquisitionFailure)?
                .ok_or(Error::ContextNotSupported)?
                .dyn_into()
                .map_err(|_| Error::ContextCastFailure)?
        })
    }

    pub fn build_program(&self) -> Result<ProgramBuilder<()>> {
        ProgramBuilder::new(self.context.clone())
    }

    pub fn build_mesh<T: Into<Vec<Vertex>>>(&self, data: T) -> Result<Mesh> {
        Mesh::new(self.context.clone(), data)
    }

    // Temporary - until all functionality is implemented
    pub fn into_context(self) -> WebGl2RenderingContext {
        self.context
    }
}