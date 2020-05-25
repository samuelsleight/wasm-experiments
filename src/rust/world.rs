use crate::{
    renderer::Renderer,
    webgl::{
        Colour,
        Texture,
        Result
    }
};

use std::cmp::Ordering;

use circular_vec::CircularVec;

use worldgen::{
    constraint,
    noise::perlin::PerlinNoise,
    noisemap::{
        NoiseMapGenerator,
        NoiseMap,
        Seed,
        Step,
        Size
    },
    world::{
        World as WorldGen,
        Tile,
        tile::{
            Constraint,
            ConstraintType
        }
    }
};

pub struct Chunk {
    pub texture: Texture
}

pub struct World {
    worldgen: WorldGen<Colour>,
    chunks: CircularVec<CircularVec<Chunk>>,

    chunk_size: (u32, u32),
    chunk_offset: (i32, i32)
}

impl World {
    pub fn new(seed: &str, chunk_width: u32, chunk_height: u32) -> World {
        let chunk_size = (chunk_width, chunk_height);

        World {
            worldgen: construct_world(seed, chunk_size),
            chunks: CircularVec::new(),

            chunk_size,
            chunk_offset: (0, 0)
        }
    }

    pub fn resize(&mut self, renderer: &Renderer, width: u32, height: u32) -> Result<()> {
        let x_chunks = (width / 256) + 2;
        let y_chunks = (height / 256) + 2;

        while self.chunks.len() <= y_chunks as usize {
            self.chunks.push(CircularVec::new());
        }

        for y in 0..self.chunks.len() {
            loop {
                let row_len = self.chunks[y].len();

                if row_len > x_chunks as usize {
                    break;
                }

                let texture = renderer.build_texture()?;
                let chunk = self.generate_chunk(row_len as i64, y as i64);
                texture.update(self.chunk_size.0 as usize, chunk)?;

                self.chunks[y].push(Chunk {
                   texture
                });
            }
        }

        Ok(())
    }

    pub fn set_seed(&mut self, seed: &str) -> Result<()> {
        self.worldgen = construct_world(seed, self.chunk_size);

        for (y, row) in self.chunks.iter().enumerate() {
            for(x, chunk) in row.iter().enumerate() {
                chunk.texture.update(self.chunk_size.0 as usize, self.generate_chunk(x as i64, y as i64))?
            }
        }

        Ok(())
    }

    pub fn rotate_chunks(&mut self, x: i32, y: i32) -> Result<()> {
        self.chunk_offset.0 += x;
        self.chunk_offset.1 += y;

        let mut chunks_to_generate = Vec::new();

        match y.cmp(&0) {
            Ordering::Less => {
                self.chunks.rotate_right(y.abs() as usize);

                for y in 0..y.abs() {
                    let x = self.chunks[y as usize].len();

                    for x in 0..x {
                        chunks_to_generate.push((x, y as usize));
                    }
                }
            },

            Ordering::Greater => {
                self.chunks.rotate_left(y as usize);

                for y in 0..y {
                    let len = self.chunks[y as usize].len();

                    for x in 0..len {
                        chunks_to_generate.push((x, (self.chunks.len() as i32 - (y + 1)) as usize));
                    }
                }
            },

            _ => ()
        }

        match x.cmp(&0) {
            Ordering::Less => {
                for y in 0..self.chunks.len() {
                    self.chunks[y].rotate_right(x.abs() as usize);

                    for x in 0..x.abs() {
                        chunks_to_generate.push((x as usize, y));
                    }
                }
            },

            Ordering::Greater => {
                for y in 0..self.chunks.len() {
                    self.chunks[y].rotate_left(x as usize);

                    for x in 0..x {
                        chunks_to_generate.push(((self.chunks[y].len() as i32 - (x + 1)) as usize, y));
                    }
                }
            },

            _ => ()
        }

        for (x, y) in chunks_to_generate {
            self.chunks[y as usize][x as usize].texture.update(self.chunk_size.0 as usize, self.generate_chunk(x as i64, y as i64))?;
        }

        Ok(())
    }

    pub fn chunks(&self) -> &CircularVec<CircularVec<Chunk>> {
        &self.chunks
    }

    fn generate_chunk(&self, x: i64, y: i64) -> Vec<Colour> {
        self.worldgen.generate(x + self.chunk_offset.0 as i64, y + self.chunk_offset.1 as i64).unwrap().into_iter().flatten().collect()
    }
}

fn construct_world(seed: &str, chunk_size: (u32, u32)) -> WorldGen<Colour> {
    let noise = PerlinNoise::new();

    let nm1 = NoiseMap::new(noise)
        .set(Seed::of::<String>(seed.chars().rev().collect()))
        .set(Step::of(0.001, 0.001));

    let nm2 = NoiseMap::new(noise)
        .set(Seed::of(seed))
        .set(Step::of(0.02, 0.02));

    let nm = Box::new(nm1 + nm2 * 3);

    WorldGen::new()
        .set(Size::of(chunk_size.0 as i64, chunk_size.1 as i64))

        // Water
        .add(Tile::new(Colour::new(0, 0, 255, 255))
            .when(constraint!(nm.clone(), < -0.1)))

        // Grass
        .add(Tile::new(Colour::new(0, 255, 0, 255))
            .when(constraint!(nm.clone(), < 0.45)))

        // Mountains
        .add(Tile::new(Colour::new(200, 200, 200, 255))
            .when(constraint!(nm, > 0.75)))

        // Hills
        .add(Tile::new(Colour::new(0, 180, 69, 255)))
}
