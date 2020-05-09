use super::vertex::Vertex;

use web_sys::{
    WebGl2RenderingContext,
    WebGlBuffer
};

pub struct Mesh {
    vertices: Vec<Vertex>,
    buffer: WebGlBuffer
}

impl Mesh {
    pub fn new<T: Into<Vec<Vertex>>>(context: &WebGl2RenderingContext, t: T) -> Result<Mesh, String> {
        let buffer = context.create_buffer().ok_or("failed to create buffer")?;
        context.bind_buffer(WebGl2RenderingContext::ARRAY_BUFFER, Some(&buffer));

        let vertices = t.into();

        unsafe {
            // Get a &[f32] from the Vec<Vertex>, as Vertex is simply a pair of f32s
            let f32_slice = std::slice::from_raw_parts(&vertices[0].x, vertices.len() * 2);

            // Construct a Float32Array view over the slice - we need to ensure no other allocations are made while we hold this
            let array_view = js_sys::Float32Array::view(f32_slice);

            context.buffer_data_with_array_buffer_view(
                WebGl2RenderingContext::ARRAY_BUFFER,
                &array_view,
                WebGl2RenderingContext::STATIC_DRAW,
            );
        }

        context.bind_buffer(WebGl2RenderingContext::ARRAY_BUFFER, None);

        Ok(Mesh {
            vertices,
            buffer
        })
    }

    pub fn render(&self, context: &WebGl2RenderingContext, attrib: i32) {
        context.bind_buffer(WebGl2RenderingContext::ARRAY_BUFFER, Some(&self.buffer));
        context.vertex_attrib_pointer_with_i32(attrib as u32, 2, WebGl2RenderingContext::FLOAT, false, 0, 0);

        context.draw_arrays(
            WebGl2RenderingContext::TRIANGLES,
            0,
            self.vertices.len() as i32);
    }
}
