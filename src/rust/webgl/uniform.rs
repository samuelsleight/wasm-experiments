use super::{
    error::Result,
    buffer::{
        Buffer,
        BufferKind,
    },
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

pub struct Uniform<T: UniformRepr> {
    buffer: Buffer,
    index: u32,

    _phantom: PhantomData<T>
}

impl<T: UniformRepr> Uniform<T> {
    pub fn new(context: WebGl2RenderingContext, program: &WebGlProgram, default: &T) -> Result<Uniform<T>> {
        let index = context.get_uniform_block_index(program, T::block_name());

        let buffer = Buffer::new_with_init(
            context,
            BufferKind::Uniform,
            default.as_slice(),
            |buffer| buffer.uniform_block_binding(program, index, index + 1))?;

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
        self.buffer.with_bound(|buffer| buffer.update(value.as_slice()));
    }
}
