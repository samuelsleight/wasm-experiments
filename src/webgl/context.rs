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

    pub fn viewport(&self, x: i32, y: i32, w: i32, h: i32) {
        self.context.viewport(x, y, w, h);
    }

    pub fn clear_colour(&self, r: f32, g: f32, b: f32, a: f32) {
        self.context.clear_color(r, g, b, a);
        self.context.clear(WebGl2RenderingContext::COLOR_BUFFER_BIT);
    }
}
