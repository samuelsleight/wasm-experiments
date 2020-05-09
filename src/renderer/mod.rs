mod mesh;
mod vertex;
mod shaders;

use self::{
    mesh::Mesh,
    vertex::Vertex
};

use web_sys::{
    WebGl2RenderingContext,
    WebGlProgram,
    WebGlUniformLocation
};

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
        let program = shaders::compile_and_link_program(&context)?;

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
