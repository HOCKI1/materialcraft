use super::CATEGORY;
use crate::{CookContext, CookResult, NodeDefinition, node};
use craft_texture::{Image, Kind};

pub(crate) fn definition() -> NodeDefinition {
    node("clouds", "Clouds", CATEGORY)
        .desc("Soft multi-octave cloud noise with turbulence and contrast shaping.")
        .float("scale", "Scale", 4.0, 1.0, 32.0)
        .int("octaves", "Octaves", 6, 1, 10)
        .float("roughness", "Roughness", 0.5, 0.0, 1.0)
        .float("contrast", "Contrast", 1.0, 0.1, 4.0)
        .int("seed", "Random Seed", 1234, 0, 100000)
        .gray_out()
        .cook(cook)
}

fn cook(context: &CookContext<'_>) -> CookResult {
    let (w, h) = (context.w, context.h);
    let scale = context.f("scale").clamp(1.0, 64.0);
    let octaves = context.i("octaves").clamp(1, 10) as usize;
    let persistence = context.f("roughness").clamp(0.0, 1.0);
    let contrast = context.f("contrast").clamp(0.1, 8.0);
    let seed = context.i("seed").max(0) as u32;

    Ok(vec![Image::generate_uv(w, h, Kind::Gray, |u, v| {
        // Multi-frequency smooth Perlin FBM with soft midpoint
        let val = super::perlin::sample_perlin_fbm(u, v, scale, octaves, persistence, 2.0, seed);
        // Soft contrast around 0.5
        let centered = (val - 0.5) * contrast + 0.5;
        let final_val = centered.clamp(0.0, 1.0);
        // Smoothstep curve for soft cloud edges
        let smooth = final_val * final_val * (3.0 - 2.0 * final_val);

        [smooth, 0.0, 0.0, 1.0]
    })])
}
