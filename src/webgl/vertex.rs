#[derive(Copy, Clone)]
#[repr(C)]
pub struct Vertex {
    pub x: f32,
    pub y: f32,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct MeshVertex {
    pub pos: Vertex,
    pub tex: Vertex
}

impl MeshVertex {
    pub fn new(x: f32, y: f32, u: f32, v: f32) -> MeshVertex {
        MeshVertex {
            pos: Vertex::new(x, y),
            tex: Vertex::new(u, v)
        }
    }
}

impl Vertex {
    pub fn new(x: f32, y: f32) -> Vertex {
        Vertex {
            x, y
        }
    }
}
