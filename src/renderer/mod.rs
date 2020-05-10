mod mesh;
mod vertex;
mod shaders;
mod uniform;
mod buffer;

use self::{
    mesh::Mesh,
    vertex::Vertex,
    uniform::{
        Uniform,
        GlobalUniforms,
        FrameUniforms
    }
};

use web_sys::{
    WebGl2RenderingContext,
    WebGlProgram,
};

pub struct Renderer {
    context: WebGl2RenderingContext,

    program: WebGlProgram,

    position_location: i32,

    global_uniforms: Uniform<GlobalUniforms>,
    frame_uniforms: Uniform<FrameUniforms>,

    meshes: Vec<Mesh>
}

impl Renderer {
    pub fn new(context: WebGl2RenderingContext) -> Result<Renderer, String> {
        let program = shaders::compile_and_link_program(&context)?;

        let position_location = context.get_attrib_location(&program, "scene_position");

        let meshes = vec![
            Mesh::new(&context, [Vertex::new(250.0, 300.0), Vertex::new(450.0, 600.0), Vertex::new(700.0, 250.0)])?,
            Mesh::new(&context, [Vertex::new(550.0, 500.0), Vertex::new(800.0, 750.0), Vertex::new(950.0, 150.0)])?
        ];

        let global_uniforms = Uniform::new(
            &context,
            &program,
            &GlobalUniforms {
                dimensions: Vertex::new(0.0, 0.0)
            })?;

        let frame_uniforms = Uniform::new(
            &context,
            &program,
            &FrameUniforms {
                offset: Vertex::new(0.0, 0.0),
                time: 0.0
            })?;

        Ok(Renderer {
            context,
            program,
            position_location,
            global_uniforms,
            frame_uniforms,
            meshes,
        })
    }

    pub fn resize_viewport(&self, width: u32, height: u32) {
        self.context.viewport(0, 0, width as i32, height as i32);
        self.global_uniforms.update(
            &self.context,
            &GlobalUniforms {
                dimensions: Vertex::new(width as f32, height as f32)
            });
    }

    pub fn render(&self, time: f32, offset: (i32, i32)) {
        self.context.use_program(Some(&self.program));

        self.frame_uniforms.update(
            &self.context,
            &FrameUniforms {
                offset: Vertex::new(offset.0 as f32, offset.1 as f32),
                time: (time as i32 % 1000) as f32
            });

        self.global_uniforms.bind_base(&self.context);
        self.frame_uniforms.bind_base(&self.context);

        self.context.clear_color(0.0, 0.0, 0.0, 1.0);
        self.context.clear(WebGl2RenderingContext::COLOR_BUFFER_BIT);

        self.context.enable_vertex_attrib_array(self.position_location as u32);

        for mesh in &self.meshes {
            mesh.render(&self.context, self.position_location)
        }

        self.context.disable_vertex_attrib_array(self.position_location as u32);

        self.context.use_program(None);
    }
}
