use super::CATEGORY;
use crate::{CookContext, CookResult, NodeDefinition, node};
use craft_texture::{Image, Kind};

pub(crate) fn definition() -> NodeDefinition {
    node("grunge_map", "Grunge Map 001", CATEGORY)
        .desc("Organic weathering and dirt map blending multi-scale gradient noise and turbulence.")
        .float("contrast", "Contrast", 1.0, 0.0, 2.0)
        .float("balance", "Balance", 0.5, 0.0, 1.0)
        .float("scale", "Scale", 4.0, 1.0, 16.0)
        .int("seed", "Random Seed", 0, 0, 10000)
        .gray_out()
        .cook(cook)
}

fn cook(context: &CookContext<'_>) -> CookResult {
    let contrast = context.f("contrast").clamp(0.0, 2.0);
    let balance = context.f("balance").clamp(0.0, 1.0);
    let scale = context.f("scale").clamp(1.0, 16.0);
    let user_seed = context.i("seed").max(0) as u32;

    Ok(vec![Image::generate_uv(context.w, context.h, Kind::Gray, |u, v| {
        // Multi-frequency synthesis
        let n1 = super::perlin::sample_perlin_fbm(u, v, scale * 2.0, 4, 0.5, 2.0, user_seed);
        let n2 = super::turbulence::sample_turbulence_fbm(u, v, scale * 4.0, 4, 0.5, user_seed.wrapping_add(101));

        let raw = n1 * 0.6 + n2 * 0.4;
        let centered = raw - balance;
        let val = (centered * (contrast * 2.0) + 0.5).clamp(0.0, 1.0);

        [val, val, val, 1.0]
    })])
}
