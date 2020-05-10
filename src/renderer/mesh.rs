use super::{
    vertex::Vertex,
    buffer::{
        Buffer,
        BufferKind
    }
};

use web_sys::{
    WebGl2RenderingContext,
};

pub struct Mesh {
    vertices: Vec<Vertex>,
    buffer: Buffer
}

impl Mesh {
    pub fn new<T: Into<Vec<Vertex>>>(context: &WebGl2RenderingContext, t: T) -> Result<Mesh, String> {
        let vertices = t.into();

        let buffer = unsafe {
            // Get a &[f32] from the Vec<Vertex>, as Vertex is simply a pair of f32s
            let f32_slice = std::slice::from_raw_parts(&vertices[0].x, vertices.len() * 2);

            Buffer::new(context, BufferKind::StaticVertex, f32_slice)?
        };

        Ok(Mesh {
            vertices,
            buffer
        })
    }

    pub fn render(&self, context: &WebGl2RenderingContext, attrib: i32) {
        self.buffer.with_bound(
            context,
            |_| {
                context.vertex_attrib_pointer_with_i32(attrib as u32, 2, WebGl2RenderingContext::FLOAT, false, 0, 0);

                context.draw_arrays(
                    WebGl2RenderingContext::TRIANGLES,
                    0,
                    self.vertices.len() as i32);
            });
    }
}
