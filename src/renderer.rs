use web_sys::{
    WebGl2RenderingContext,
    WebGlProgram,
    WebGlUniformLocation
};

pub struct Renderer {
    context: WebGl2RenderingContext,

    program: WebGlProgram,

    time_location: WebGlUniformLocation,
    scene_dimensions_location: WebGlUniformLocation,
    scene_offset_location: WebGlUniformLocation,

    vertices: [f32; 6],
}

impl Renderer {
    pub fn new(context: WebGl2RenderingContext) -> Result<Renderer, String> {
        let program = crate::shaders::compile_and_link_program(&context)?;

        let vertices: [f32; 6] = [250.0, 300.0, 450.0, 600.0, 700.0, 250.0];
        create_buffer(&context, &vertices)?;

        let time_location = context.get_uniform_location(&program, "time").ok_or("unable to find time uniform")?;
        let scene_dimensions_location = context.get_uniform_location(&program, "scene_dimensions").ok_or("unable to find scene dimensions uniform")?;
        let scene_offset_location = context.get_uniform_location(&program, "scene_offset").ok_or("unable to find scene offset uniform")?;

        Ok(Renderer {
            context,
            program,
            time_location,
            scene_dimensions_location,
            scene_offset_location,
            vertices,
        })
    }

    pub fn resize_viewport(&self, width: u32, height: u32) {
        self.context.viewport(0, 0, width as i32, height as i32);

        self.context.use_program(Some(&self.program));
        self.context.uniform2f(Some(&self.scene_dimensions_location), width as f32, height as f32);
        self.context.use_program(None);
    }

    pub fn render(&self, time: f32, offset: (i32, i32)) {
        self.context.use_program(Some(&self.program));

        self.context.clear_color(0.0, 0.0, 0.0, 1.0);
        self.context.clear(WebGl2RenderingContext::COLOR_BUFFER_BIT);

        self.context.enable_vertex_attrib_array(0);

        self.context.uniform1f(Some(&self.time_location), (time as i32 % 1000) as f32);
        self.context.uniform2f(Some(&self.scene_offset_location), offset.0 as f32, offset.1 as f32);

        self.context.draw_arrays(
            WebGl2RenderingContext::TRIANGLES,
            0,
            (self.vertices.len() / 2) as i32,
        );

        self.context.disable_vertex_attrib_array(0);
        self.context.use_program(None);
    }
}

fn create_buffer(context: &WebGl2RenderingContext, vertices: &[f32]) -> Result<(), String> {
    let buffer = context.create_buffer().ok_or("failed to create buffer")?;
    context.bind_buffer(WebGl2RenderingContext::ARRAY_BUFFER, Some(&buffer));

    unsafe {
        let array_view = js_sys::Float32Array::view(&vertices);

        context.buffer_data_with_array_buffer_view(
            WebGl2RenderingContext::ARRAY_BUFFER,
            &array_view,
            WebGl2RenderingContext::STATIC_DRAW,
        );
    }

    context.vertex_attrib_pointer_with_i32(0, 2, WebGl2RenderingContext::FLOAT, false, 0, 0);

    context.bind_buffer(WebGl2RenderingContext::ARRAY_BUFFER, None);
    Ok(())
}
