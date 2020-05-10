use crate::webgl::{
    Vertex,
    Buffer,
    BufferKind,
    Result
};

use std::marker::PhantomData;

use web_sys::{
    WebGl2RenderingContext,
    WebGlProgram
};

pub trait UniformRepr {
    fn as_slice(&self) -> &[f32];
    fn block_name() -> &'static str;
}

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

pub struct Uniform<T: UniformRepr> {
    buffer: Buffer,
    index: u32,

    _phantom: PhantomData<T>
}

impl<T: UniformRepr> Uniform<T> {
    pub fn new(context: &WebGl2RenderingContext, program: &WebGlProgram, default: &T) -> Result<Uniform<T>> {
        let index = context.get_uniform_block_index(program, T::block_name());

        let buffer = Buffer::new_with_init(
            context.clone(),
            BufferKind::Uniform,
            default.as_slice(),
            |_| context.uniform_block_binding(program, index, index + 1))?;

        Ok(Uniform {
            buffer,
            index,

            _phantom: PhantomData
        })
    }

    pub fn bind_base(&self) {
        self.buffer.bind_base(self.index + 1);
    }

    pub fn update(&self, value: &T) {
        self.buffer.with_bound(
            |buffer| buffer.update(value.as_slice()));
    }
}
