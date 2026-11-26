use super::CATEGORY;
use crate::{CookContext, CookResult, NodeDefinition, node};
use craft_texture::{Image, Kind};

pub(crate) fn definition() -> NodeDefinition {
    node("fractal_sum", "Fractal Sum", CATEGORY)
        .desc("General fractal sum Brownian noise with customizable octaves, persistence, and lacunarity.")
        .float("scale", "Scale", 8.0, 1.0, 64.0)
        .int("octaves", "Octaves", 6, 1, 10)
        .float("roughness", "Roughness", 0.5, 0.0, 1.0)
        .float("lacunarity", "Lacunarity", 2.0, 1.0, 4.0)
        .int("seed", "Random Seed", 1234, 0, 100000)
        .gray_out()
        .cook(cook)
}

fn cook(context: &CookContext<'_>) -> CookResult {
    let (w, h) = (context.w, context.h);
    let scale = context.f("scale").clamp(1.0, 128.0);
    let octaves = context.i("octaves").clamp(1, 10) as usize;
    let persistence = context.f("roughness").clamp(0.0, 1.0);
    let lacunarity = context.f("lacunarity").clamp(1.0, 5.0);
    let seed = context.i("seed").max(0) as u32;

    Ok(vec![Image::generate_uv(w, h, Kind::Gray, |u, v| {
        let val = super::perlin::sample_perlin_fbm(u, v, scale, octaves, persistence, lacunarity, seed);
        [val, 0.0, 0.0, 1.0]
    })])
}
