use super::vertex::Vertex;

use std::marker::PhantomData;

use web_sys::{
    WebGl2RenderingContext,
    WebGlProgram,
    WebGlBuffer
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
    buffer: WebGlBuffer,
    index: u32,

    _phantom: PhantomData<T>
}

impl<T: UniformRepr> Uniform<T> {
    pub fn new(context: &WebGl2RenderingContext, program: &WebGlProgram, default: &T) -> Result<Uniform<T>, String> {
        let buffer = context.create_buffer().ok_or("failed to create buffer")?;
        context.bind_buffer(WebGl2RenderingContext::UNIFORM_BUFFER, Some(&buffer));

        let index = context.get_uniform_block_index(program, T::block_name());
        context.uniform_block_binding(program, index, index + 1);

        Self::update_bound(&context, default);
        context.bind_buffer(WebGl2RenderingContext::UNIFORM_BUFFER, None);

        Ok(Uniform {
            buffer,
            index,

            _phantom: PhantomData
        })
    }

    pub fn bind_base(&self, context: &WebGl2RenderingContext) {
        context.bind_buffer_base(WebGl2RenderingContext::UNIFORM_BUFFER, self.index + 1, Some(&self.buffer));
    }

    pub fn update(&self, context: &WebGl2RenderingContext, value: &T) {
        context.bind_buffer(WebGl2RenderingContext::UNIFORM_BUFFER, Some(&self.buffer));
        Self::update_bound(context, value);
        context.bind_buffer(WebGl2RenderingContext::UNIFORM_BUFFER, None);
    }

    fn update_bound(context: &WebGl2RenderingContext, value: &T) {
        unsafe {
            let f32_slice = value.as_slice();

            // Construct a Float32Array view over the slice - we need to ensure no other allocations are made while we hold this
            let array_view = js_sys::Float32Array::view(f32_slice);

            context.buffer_data_with_array_buffer_view(
                WebGl2RenderingContext::UNIFORM_BUFFER,
                &array_view,
                WebGl2RenderingContext::DYNAMIC_DRAW,
            );
        }
    }
}
