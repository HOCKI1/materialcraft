use super::CATEGORY;
use crate::{CookContext, CookResult, NodeDefinition, node};
use craft_texture::{Image, Kind};

pub(crate) fn definition() -> NodeDefinition {
    node("bnw_spots", "BnW Spots", CATEGORY)
        .desc("High-contrast spot noise generator for dust, plaster, rust pitting, and speckled surface variations.")
        .float("scale", "Scale", 16.0, 1.0, 128.0)
        .float("balance", "Balance", 0.5, 0.0, 1.0)
        .float("contrast", "Contrast", 2.0, 0.5, 8.0)
        .int("octaves", "Octaves", 4, 1, 8)
        .int("seed", "Random Seed", 1234, 0, 100000)
        .gray_out()
        .cook(cook)
}

fn cook(context: &CookContext<'_>) -> CookResult {
    let (w, h) = (context.w, context.h);
    let scale = context.f("scale").clamp(1.0, 256.0);
    let balance = context.f("balance").clamp(0.0, 1.0);
    let contrast = context.f("contrast").clamp(0.1, 10.0);
    let octaves = context.i("octaves").clamp(1, 8) as usize;
    let seed = context.i("seed").max(0) as u32;

    Ok(vec![Image::generate_uv(w, h, Kind::Gray, |u, v| {
        let fbm = super::perlin::sample_perlin_fbm(u, v, scale, octaves, 0.6, 2.1, seed);
        // Balance controls the threshold point; contrast drives the steepness
        let t = (fbm - (1.0 - balance)) * contrast + 0.5;
        let val = t.clamp(0.0, 1.0);

        [val, 0.0, 0.0, 1.0]
    })])
}
