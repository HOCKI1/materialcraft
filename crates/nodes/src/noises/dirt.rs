use super::CATEGORY;
use crate::{CookContext, CookResult, NodeDefinition, node};
use craft_texture::{Image, Kind};

pub(crate) fn definition() -> NodeDefinition {
    node("dirt", "Dirt", CATEGORY)
        .desc("Organic multi-layered dirt and smudge noise generator.")
        .float("scale", "Scale", 8.0, 1.0, 64.0)
        .float("contrast", "Contrast", 1.5, 0.2, 5.0)
        .float("grunge", "Grunge", 0.5, 0.0, 1.0)
        .int("seed", "Random Seed", 1234, 0, 100000)
        .gray_out()
        .cook(cook)
}

fn cook(context: &CookContext<'_>) -> CookResult {
    let (w, h) = (context.w, context.h);
    let scale = context.f("scale").clamp(1.0, 64.0);
    let contrast = context.f("contrast").clamp(0.1, 10.0);
    let grunge = context.f("grunge").clamp(0.0, 1.0);
    let seed = context.i("seed").max(0) as u32;

    Ok(vec![Image::generate_uv(w, h, Kind::Gray, |u, v| {
        let fbm_low = super::perlin::sample_perlin_fbm(u, v, scale, 3, 0.5, 2.0, seed);
        let fbm_high = super::perlin::sample_perlin_fbm(u, v, scale * 3.5, 4, 0.7, 2.2, seed.wrapping_add(555));

        // Multiply low-frequency blotches with high-frequency dirt flecks
        let mixed = fbm_low * (1.0 - grunge * 0.4) + (fbm_low * fbm_high) * (grunge * 0.8);
        let val = ((mixed - 0.5) * contrast + 0.5).clamp(0.0, 1.0);

        [val, 0.0, 0.0, 1.0]
    })])
}
