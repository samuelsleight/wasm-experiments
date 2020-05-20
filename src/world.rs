use crate::webgl::Colour;

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
        World,
        Tile,
        tile::{
            Constraint,
            ConstraintType
        }
    }
};

pub fn generate(seed: &str, w: i64, h: i64) -> Vec<Colour> {
    let noise = PerlinNoise::new();

    let nm1 = NoiseMap::new(noise)
        .set(Seed::of::<String>(seed.chars().rev().collect()))
        .set(Step::of(0.002, 0.002));

    let nm2 = NoiseMap::new(noise)
        .set(Seed::of(seed))
        .set(Step::of(0.05, 0.05));

    let nm = Box::new(nm1 + nm2 * 3);

    let world = World::new()
        .set(Size::of(w, h))

        // Water
        .add(Tile::new(Colour::new(0, 0, 255, 255))
            .when(constraint!(nm.clone(), < -0.1)))

        // Grass
        .add(Tile::new(Colour::new(0, 255, 0, 255))
            .when(constraint!(nm.clone(), < 0.45)))

        // Mountains
        .add(Tile::new(Colour::new(200, 200, 200, 255))
            .when(constraint!(nm, > 0.8)))

        // Hills
        .add(Tile::new(Colour::new(0, 180, 69, 255)));

    world.generate(0, 0).unwrap().into_iter().flatten().collect()
}
