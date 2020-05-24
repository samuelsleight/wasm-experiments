mod uniforms;

use circular_vec::CircularVec;

use self::{
    uniforms::{
        GlobalUniforms,
        ShapeUniforms,
        FrameUniforms
    }
};

use crate::{
    world::Chunk,
    webgl::{
        WebGlContext,
        Attribute,
        Mesh,
        Vertex,
        MeshVertex,
        Uniform,
        Program,
        Texture,
        Sampler,
        Result
    }
};

pub struct Renderer {
    context: WebGlContext,

    program: Program,

    position_attribute: Attribute,
    texture_attribute: Attribute,

    global_uniforms: Uniform<GlobalUniforms>,
    shape_uniforms: Uniform<ShapeUniforms>,
    frame_uniforms: Uniform<FrameUniforms>,

    mesh: Mesh,

    sampler: Sampler,
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

        let mesh = context.build_mesh([
            MeshVertex::new(0.0, 0.0, 0.0, 0.0),
            MeshVertex::new(256.0, 0.0, 1.0, 0.0),
            MeshVertex::new(0.0, 256.0, 0.0, 1.0),
            MeshVertex::new(256.0, 256.0, 1.0, 1.0)])?;

        let global_uniforms = program.uniform(
            &GlobalUniforms {
                dimensions: Vertex::new(0.0, 0.0)
            })?;

        let shape_uniforms = program.uniform(
            &ShapeUniforms {
                offset: Vertex::new(0.0, 0.0)
            })?;

        let frame_uniforms = program.uniform(
            &FrameUniforms {
                offset: Vertex::new(0.0, 0.0),
                time: 0.0
            })?;

        let sampler = program.sampler("tex")?;

        global_uniforms.bind_base();
        frame_uniforms.bind_base();
        shape_uniforms.bind_base();

        Ok(Renderer {
            context,
            program,
            position_attribute,
            texture_attribute,
            global_uniforms,
            shape_uniforms,
            frame_uniforms,
            mesh,
            sampler
        })
    }

    pub fn build_texture(&self) -> Result<Texture> {
        self.context.build_texture()
    }

    pub fn resize_viewport(&mut self, width: u32, height: u32) {
        self.context.viewport(0, 0, width as i32, height as i32);

        self.global_uniforms.update(
            &GlobalUniforms {
                dimensions: Vertex::new(width as f32, height as f32)
            });
    }

    pub fn render(&mut self, chunks: &CircularVec<CircularVec<Chunk>>, time: f32, offset: (i32, i32)) {
        self.frame_uniforms.update(
            &FrameUniforms {
                offset: Vertex::new(offset.0 as f32 + 256.0, offset.1 as f32 + 256.0),
                time: (time as i32 % 1000) as f32
            });

        self.context.clear_colour(0.0, 0.0, 0.0, 1.0);

        self.program.render_frame(
            &self.position_attribute,
            &self.texture_attribute,
            |frame| {
                for (y, row) in chunks.iter().enumerate() {
                    for (x, chunk) in row.iter().enumerate() {
                        self.shape_uniforms.update(
                            &ShapeUniforms {
                                offset: Vertex::new((x * 256) as f32, (y * 256) as f32)
                            });

                        chunk.texture.with(
                            |texture| {
                                self.sampler.update(&texture);
                                frame.render(&self.mesh);
                            });
                    }
                }
            });
    }
}
