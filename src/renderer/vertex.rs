#[derive(Copy, Clone)]
#[repr(C)]
pub struct Vertex {
    pub x: f32,
    pub y: f32
}

impl Vertex {
    pub fn new(x: f32, y: f32) -> Vertex {
        Vertex {
            x, y
        }
    }
}

