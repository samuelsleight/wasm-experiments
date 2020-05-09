use web_sys::{
    WebGl2RenderingContext,
    WebGlBuffer,
    WebGlProgram,
    WebGlUniformLocation
};

#[derive(Copy, Clone)]
#[repr(C)]
struct Vertex {
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

struct Mesh {
    vertices: Vec<Vertex>,
    buffer: WebGlBuffer
}

impl Mesh {
    pub fn new<T: Into<Vec<Vertex>>>(context: &WebGl2RenderingContext, t: T) -> Result<Mesh, String> {
        let buffer = context.create_buffer().ok_or("failed to create buffer")?;
        context.bind_buffer(WebGl2RenderingContext::ARRAY_BUFFER, Some(&buffer));

        let vertices = t.into();

        unsafe {
            // Get a &[f32] from the Vec<Vertex>, as Vertex is simply a pair of f32s
            let f32_slice = std::slice::from_raw_parts(&vertices[0].x, vertices.len() * 2);

            // Construct a Float32Array view over the slice - we need to ensure no other allocations are made while we hold this
            let array_view = js_sys::Float32Array::view(f32_slice);

            context.buffer_data_with_array_buffer_view(
                WebGl2RenderingContext::ARRAY_BUFFER,
                &array_view,
                WebGl2RenderingContext::STATIC_DRAW,
            );
        }

        context.bind_buffer(WebGl2RenderingContext::ARRAY_BUFFER, None);

        Ok(Mesh {
            vertices,
            buffer
        })
    }

    pub fn render(&self, context: &WebGl2RenderingContext, attrib: i32) {
        context.bind_buffer(WebGl2RenderingContext::ARRAY_BUFFER, Some(&self.buffer));
        context.vertex_attrib_pointer_with_i32(attrib as u32, 2, WebGl2RenderingContext::FLOAT, false, 0, 0);

        context.draw_arrays(
            WebGl2RenderingContext::TRIANGLES,
            0,
            self.vertices.len() as i32);
    }
}

pub struct Renderer {
    context: WebGl2RenderingContext,

    program: WebGlProgram,

    position_location: i32,

    time_location: WebGlUniformLocation,
    scene_dimensions_location: WebGlUniformLocation,
    scene_offset_location: WebGlUniformLocation,

    meshes: Vec<Mesh>
}

impl Renderer {
    pub fn new(context: WebGl2RenderingContext) -> Result<Renderer, String> {
        let program = crate::shaders::compile_and_link_program(&context)?;

        let position_location = context.get_attrib_location(&program, "scene_position");

        let meshes = vec![
            Mesh::new(&context, [Vertex::new(250.0, 300.0), Vertex::new(450.0, 600.0), Vertex::new(700.0, 250.0)])?,
            Mesh::new(&context, [Vertex::new(550.0, 500.0), Vertex::new(800.0, 750.0), Vertex::new(950.0, 150.0)])?
        ];

        let time_location = context.get_uniform_location(&program, "time").ok_or("unable to find time uniform")?;
        let scene_dimensions_location = context.get_uniform_location(&program, "scene_dimensions").ok_or("unable to find scene dimensions uniform")?;
        let scene_offset_location = context.get_uniform_location(&program, "scene_offset").ok_or("unable to find scene offset uniform")?;

        Ok(Renderer {
            context,
            program,
            position_location,
            time_location,
            scene_dimensions_location,
            scene_offset_location,
            meshes,
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

        self.context.enable_vertex_attrib_array(self.position_location as u32);

        self.context.uniform1f(Some(&self.time_location), (time as i32 % 1000) as f32);
        self.context.uniform2f(Some(&self.scene_offset_location), offset.0 as f32, offset.1 as f32);

        self.context.clear_color(0.0, 0.0, 0.0, 1.0);
        self.context.clear(WebGl2RenderingContext::COLOR_BUFFER_BIT);

        for mesh in &self.meshes {
            mesh.render(&self.context, self.position_location)
        }

        self.context.disable_vertex_attrib_array(self.position_location as u32);

        self.context.use_program(None);
    }
}
