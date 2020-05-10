use web_sys::{
    WebGl2RenderingContext,
    WebGlBuffer,
};

pub enum BufferKind {
    StaticVertex,
    Uniform
}

pub struct Buffer {
    buffer: WebGlBuffer,

    target: u32,
    usage: u32
}

pub struct BoundBuffer<'a> {
    buffer: &'a Buffer
}

impl Buffer {
    pub fn new(context: &WebGl2RenderingContext, kind: BufferKind, data: &[f32]) -> Result<Buffer, String> {
        Self::new_with_init(context, kind, data, |_| ())
    }

    pub fn new_with_init<F: FnOnce(BoundBuffer)>(context: &WebGl2RenderingContext, kind: BufferKind, data: &[f32], f: F) -> Result<Buffer, String> {
        let (target, usage) = target_and_usage(kind);

        let buffer = Buffer {
            buffer: context.create_buffer().ok_or("failed to create buffer")?,

            target,
            usage
        };

        buffer.with_bound(
            context,
            |buffer| {
                buffer.update(context, data);
                f(buffer);
            });

        Ok(buffer)
    }

    pub fn with_bound<U, F: FnOnce(BoundBuffer) -> U>(&self, context: &WebGl2RenderingContext, f: F) -> U {
        context.bind_buffer(self.target, Some(&self.buffer));
        let result = f(BoundBuffer { buffer: self });
        context.bind_buffer(self.target, None);
        result
    }

    pub fn bind_base(&self, context: &WebGl2RenderingContext, index: u32) {
        context.bind_buffer_base(WebGl2RenderingContext::UNIFORM_BUFFER, index, Some(&self.buffer));
    }
}

impl<'a> BoundBuffer<'a> {
    pub fn update(&self, context: &WebGl2RenderingContext, data: &[f32]) {
        unsafe {
            // Construct a Float32Array view over the slice - we need to ensure no other allocations are made while we hold this
            let array_view = js_sys::Float32Array::view(data);

            context.buffer_data_with_array_buffer_view(
                self.buffer.target,
                &array_view,
                self.buffer.usage);
        }
    }
}

fn target_and_usage(kind: BufferKind) -> (u32, u32) {
    match kind {
        BufferKind::StaticVertex => (WebGl2RenderingContext::ARRAY_BUFFER, WebGl2RenderingContext::STATIC_DRAW),
        BufferKind::Uniform => (WebGl2RenderingContext::UNIFORM_BUFFER, WebGl2RenderingContext::DYNAMIC_COPY)
    }
}
