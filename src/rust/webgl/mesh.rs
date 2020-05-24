use super::{
    attribute::ActiveAttribute,
    vertex::MeshVertex,
    error::Result,
    buffer::{
        Buffer,
        BufferKind
    },
};

use web_sys::{
    WebGl2RenderingContext,
};

pub struct Mesh {
    vertices: Vec<MeshVertex>,
    buffer: Buffer
}

impl Mesh {
    pub fn new<T: Into<Vec<MeshVertex>>>(context: WebGl2RenderingContext, t: T) -> Result<Mesh> {
        let vertices = t.into();

        let buffer = unsafe {
            // Get a &[f32] from the Vec<MeshVertex>, as MeshVertex is simply a quad of f32s
            let f32_slice = std::slice::from_raw_parts(&vertices[0].pos.x, vertices.len() * 4);

            Buffer::new(context, BufferKind::StaticVertex, f32_slice)?
        };

        Ok(Mesh {
            vertices,
            buffer
        })
    }

    pub fn render(&self, position_attribute: &ActiveAttribute<'_>, texture_attribute: &ActiveAttribute<'_>) {
        self.buffer.with_bound(
            |buffer| {
                position_attribute.vertex_attrib_pointer(2, 0);
                texture_attribute.vertex_attrib_pointer(2, 2);
                buffer.draw_arrays(self.vertices.len() as i32);
            });
    }
}
