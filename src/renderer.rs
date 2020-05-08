use web_sys::{
    WebGlRenderingContext,
    WebGlProgram,
    WebGlBuffer
};

pub struct Renderer {
    context: WebGlRenderingContext,

    program: WebGlProgram,

    vertices: [f32; 9],
    buffer: WebGlBuffer
}

impl Renderer {
    pub fn new(context: WebGlRenderingContext) -> Result<Renderer, String> {
        let program = crate::shaders::compile_and_link_program(&context)?;

        let vertices: [f32; 9] = [-0.7, -0.7, 0.0, 0.7, -0.7, 0.0, 0.0, 0.7, 0.0];
        let buffer = create_buffer(&context, &vertices)?;

        Ok(Renderer {
            context,
            program,
            vertices,
            buffer,
        })
    }

    pub fn render(&self) {
        self.context.use_program(Some(&self.program));

        self.context.clear_color(0.0, 0.0, 0.0, 1.0);
        self.context.clear(WebGlRenderingContext::COLOR_BUFFER_BIT);

        self.context.bind_buffer(WebGlRenderingContext::ARRAY_BUFFER, Some(&self.buffer));
        self.context.enable_vertex_attrib_array(0);

        self.context.draw_arrays(
            WebGlRenderingContext::TRIANGLES,
            0,
            (self.vertices.len() / 3) as i32,
        );

        self.context.disable_vertex_attrib_array(0);
        self.context.bind_buffer(WebGlRenderingContext::ARRAY_BUFFER, None);

        self.context.use_program(None);
    }
}

fn create_buffer(context: &WebGlRenderingContext, vertices: &[f32]) -> Result<WebGlBuffer, String> {
    let buffer = context.create_buffer().ok_or("failed to create buffer")?;
    context.bind_buffer(WebGlRenderingContext::ARRAY_BUFFER, Some(&buffer));

    unsafe {
        let array_view = js_sys::Float32Array::view(&vertices);

        context.buffer_data_with_array_buffer_view(
            WebGlRenderingContext::ARRAY_BUFFER,
            &array_view,
            WebGlRenderingContext::STATIC_DRAW,
        );
    }

    context.vertex_attrib_pointer_with_i32(0, 3, WebGlRenderingContext::FLOAT, false, 0, 0);

    context.bind_buffer(WebGlRenderingContext::ARRAY_BUFFER, None);
    Ok(buffer)
}
