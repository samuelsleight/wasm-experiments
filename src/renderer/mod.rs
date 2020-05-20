mod uniforms;

use self::{
    uniforms::{
        GlobalUniforms,
        FrameUniforms
    }
};

use crate::webgl::{
    WebGlContext,
    Attribute,
    Mesh,
    Vertex,
    MeshVertex,
    Uniform,
    Program,
    Texture,
    Result
};

pub struct Renderer {
    context: WebGlContext,

    program: Program,

    position_attribute: Attribute,
    texture_attribute: Attribute,

    global_uniforms: Uniform<GlobalUniforms>,
    frame_uniforms: Uniform<FrameUniforms>,

    texture: Texture,

    meshes: Vec<Mesh>
}

impl Renderer {
    pub fn new(context: WebGlContext) -> Result<Renderer> {
        let program = context
            .build_program()?
            .fragment_shader(include_str!("shaders/fragment.glsl"))?
            .vertex_shader(include_str!("shaders/vertex.glsl"))?
            .link()?;

        let position_attribute = program.attribute("scene_position")?;
        let texture_attribute = program.attribute("tex_coords")?;

        let meshes = vec![
            context.build_mesh([MeshVertex::new(250.0, 300.0, 0.0, 0.0), MeshVertex::new(450.0, 600.0, 1.0, 0.0), MeshVertex::new(450.0, 110.0, 0.0, 1.0), MeshVertex::new(700.0, 250.0, 1.0, 1.0)])?,
            context.build_mesh([MeshVertex::new(550.0, 500.0, 0.0, 0.0), MeshVertex::new(800.0, 750.0, 0.0, 1.0), MeshVertex::new(950.0, 150.0, 1.0, 1.0)])?
        ];

        let global_uniforms = program.uniform(
            &GlobalUniforms {
                dimensions: Vertex::new(0.0, 0.0)
            })?;

        let frame_uniforms = program.uniform(
            &FrameUniforms {
                offset: Vertex::new(0.0, 0.0),
                time: 0.0
            })?;

        let texture = context.build_texture()?;
        texture.update(256, crate::world::generate(256, 256))?;

        let sampler = program.sampler("tex")?;
        program.with(|| texture.with(|texture| sampler.update(&texture)));

        Ok(Renderer {
            context,
            program,
            position_attribute,
            texture_attribute,
            global_uniforms,
            frame_uniforms,
            texture,
            meshes,
        })
    }

    pub fn resize_viewport(&self, width: u32, height: u32) {
        self.context.viewport(0, 0, width as i32, height as i32);
        self.global_uniforms.update(
            &GlobalUniforms {
                dimensions: Vertex::new(width as f32, height as f32)
            });
    }

    pub fn render(&self, time: f32, offset: (i32, i32)) {
        self.frame_uniforms.update(
            &FrameUniforms {
                offset: Vertex::new(offset.0 as f32, offset.1 as f32),
                time: (time as i32 % 1000) as f32
            });

        self.global_uniforms.bind_base();
        self.frame_uniforms.bind_base();

        self.context.clear_colour(0.0, 0.0, 0.0, 1.0);

        self.program.with(
            || {
                self.texture.with(
                    |_| {
                        self.position_attribute.with(
                            |position_attribute| {
                                self.texture_attribute.with(
                                    |texture_attribute| {
                                        for mesh in &self.meshes {
                                            mesh.render(&position_attribute, &texture_attribute);
                                        }
                                    });
                            });
                    });
            });
    }
}
