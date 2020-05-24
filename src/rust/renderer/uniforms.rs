use crate::webgl::{
    UniformRepr,
    Vertex
};

#[repr(C)]
pub struct GlobalUniforms {
    pub dimensions: Vertex
}

impl UniformRepr for GlobalUniforms {
    fn as_slice(&self) -> &[f32] {
        unsafe {
            std::slice::from_raw_parts(&self.dimensions.x, 2)
        }
    }

    fn block_name() -> &'static str {
        "global_uniforms"
    }
}

#[repr(C)]
pub struct ShapeUniforms {
    pub offset: Vertex
}

impl UniformRepr for ShapeUniforms {
    fn as_slice(&self) -> &[f32] {
        unsafe {
            std::slice::from_raw_parts(&self.offset.x, 2)
        }
    }

    fn block_name() -> &'static str {
        "shape_uniforms"
    }
}

#[repr(C)]
pub struct FrameUniforms {
    pub offset: Vertex,
    pub time: f32
}

impl UniformRepr for FrameUniforms {
    fn as_slice(&self) -> &[f32] {
        unsafe {
            std::slice::from_raw_parts(&self.offset.x, 3)
        }
    }

    fn block_name() -> &'static str {
        "frame_uniforms"
    }
}
