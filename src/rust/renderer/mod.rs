mod uniforms;

use self::{
    uniforms::{
        GlobalUniforms,
        ShapeUniforms,
        FrameUniforms
    }
};

use circular_vec::CircularVec;

use crate::webgl::{
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
    chunks: CircularVec<CircularVec<Texture>>
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

        let mut chunks = CircularVec::new();

        for _ in 0..4 {
            let mut chunk = CircularVec::new();
            chunk.push(context.build_texture()?);
            chunk.push(context.build_texture()?);
            chunk.push(context.build_texture()?);
            chunk.push(context.build_texture()?);
            chunks.push(chunk);
        }

        global_uniforms.bind_base();
        frame_uniforms.bind_base();
        shape_uniforms.bind_base();

        let renderer = Renderer {
            context,
            program,
            position_attribute,
            texture_attribute,
            global_uniforms,
            shape_uniforms,
            frame_uniforms,
            mesh,
            chunks,
            sampler
        };

        renderer.update_texture("default")?;
        Ok(renderer)
    }

    pub fn update_texture(&self, seed: &str) -> Result<()> {
        for (y, row) in self.chunks.iter().enumerate() {
            for(x, chunk) in row.iter().enumerate() {
                chunk.update(256, crate::world::generate(seed, 256, 256, x as i64, y as i64))?;
            }
        }

        Ok(())
    }

    pub fn resize_viewport(&mut self, width: u32, height: u32) -> Result<()> {
        self.context.viewport(0, 0, width as i32, height as i32);
        self.global_uniforms.update(
            &GlobalUniforms {
                dimensions: Vertex::new(width as f32, height as f32)
            });

        let x_chunks = (width / 256) + 2;
        let y_chunks = (height / 256) + 2;

        while self.chunks.len() <= y_chunks as usize {
            self.chunks.push(CircularVec::new());
        }

        for y in 0..self.chunks.len() {
            let chunk = &mut self.chunks[y];

            while chunk.len() <= x_chunks as usize {
                chunk.push(self.context.build_texture()?);
            }
        }

        Ok(())
    }

    pub fn rotate_chunks(&mut self, x: i32, y: i32) {
        if y < 0 {
            self.chunks.rotate_right(y.abs() as usize);
        } else if y > 0 {
            self.chunks.rotate_left(y as usize);
        }

        for y in 0..self.chunks.len() {
            let chunk = &mut self.chunks[y];

            if x < 0  {
                chunk.rotate_right(x.abs() as usize);
            } else if x > 0 {
                chunk.rotate_left(x as usize);
            }
        }
    }

    pub fn render(&mut self, time: f32, offset: (i32, i32)) {
        self.frame_uniforms.update(
            &FrameUniforms {
                offset: Vertex::new(offset.0 as f32 + 256.0, offset.1 as f32 + 256.0),
                time: (time as i32 % 1000) as f32
            });

        self.context.clear_colour(0.0, 0.0, 0.0, 1.0);

        self.program.with(
            || {
                self.position_attribute.with(
                    |position_attribute| {
                        self.texture_attribute.with(
                            |texture_attribute| {
                                for (y, row) in self.chunks.iter().enumerate() {
                                    for (x, chunk) in row.iter().enumerate() {
                                        self.shape_uniforms.update(
                                            &ShapeUniforms {
                                                offset: Vertex::new((x * 256) as f32, (y * 256) as f32)
                                            });

                                        chunk.with(
                                            |texture| {
                                                self.sampler.update(&texture);
                                                self.mesh.render(&position_attribute, &texture_attribute);
                                            });
                                    }
                                }
                            });
                    });
            });
    }
}
