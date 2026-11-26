use super::CATEGORY;
use crate::{CookContext, CookResult, NodeDefinition, node};
use craft_texture::{Image, Kind};
use std::f32::consts::PI;

pub(crate) fn definition() -> NodeDefinition {
    node("directional_noise", "Directional Noise", CATEGORY)
        .desc("Anisotropic stretched noise generator oriented along a customizable angle with stretching and roughness.")
        .float("scale", "Scale", 16.0, 1.0, 64.0)
        .float("stretch", "Stretch", 8.0, 1.0, 64.0)
        .angle("angle", "Angle", 0.0)
        .int("octaves", "Octaves", 4, 1, 8)
        .float("roughness", "Roughness", 0.5, 0.0, 1.0)
        .int("seed", "Random Seed", 1234, 0, 100000)
        .gray_out()
        .cook(cook)
}

fn cook(context: &CookContext<'_>) -> CookResult {
    let (w, h) = (context.w, context.h);
    let scale = context.f("scale").clamp(1.0, 128.0);
    let stretch = context.f("stretch").clamp(1.0, 128.0);
    let angle = context.f("angle") * PI;
    let octaves = context.i("octaves").clamp(1, 8) as usize;
    let persistence = context.f("roughness").clamp(0.0, 1.0);
    let seed = context.i("seed").max(0) as u32;

    let cos_a = angle.cos();
    let sin_a = angle.sin();

    Ok(vec![Image::generate_uv(w, h, Kind::Gray, |u, v| {
        let x = u - 0.5;
        let y = v - 0.5;

        // Rotate and stretch along major axis
        let rx = (x * cos_a - y * sin_a) / stretch + 0.5;
        let ry = (x * sin_a + y * cos_a) + 0.5;

        let val = super::perlin::sample_perlin_fbm(rx, ry, scale, octaves, persistence, 2.0, seed);
        [val, 0.0, 0.0, 1.0]
    })])
}
