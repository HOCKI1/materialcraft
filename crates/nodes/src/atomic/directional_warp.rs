use super::ATOMIC;
use crate::{CookContext, CookResult, NodeDefinition, node};
use craft_texture::{Image, Kind};
use materialcraft_core::OutputFormat;

pub(super) fn definition() -> NodeDefinition {
    node("directional_warp", "Directional warp", ATOMIC)
        .desc("Offsets the input along a direction, by an intensity map.")
        .atomic()
        .any_in("input", "Input")
        .gray_in("intensity_input", "Intensity input")
        .float("intensity", "Intensity", 10.0, 0.0, 64.0)
        .angle("angle", "Warp angle", 0.0)
        .any_out()
        .cook(directional_warp)
}

fn directional_warp(context: &CookContext) -> CookResult {
    let image = context.any_or(0, Kind::Gray, [0.0; 4]);
    let map = context.gray_or(1, 0.0);
    let (sine, cosine) = (context.f("angle") * std::f32::consts::TAU).sin_cos();
    const BASE_RESOLUTION: f32 = 256.0;
    let width = image.width() as f32;
    let height = image.height() as f32;
    let distance = context.f("intensity") * width.min(height) / BASE_RESOLUTION;
    let step = [cosine * distance / width, sine * distance / height];
    let tiling = context.tiling_axes();
    let format = context.input_formats.first().copied().unwrap_or(if image.integer_sampling() { OutputFormat::U16 } else { OutputFormat::F32 });
    Ok(vec![Image::generate(context.w, context.h, image.kind(), |x, y| {
        let amount = map.get_gray(i64::from(x), i64::from(y), false);
        let uv = [(x as f32 + 0.5) / width + step[0] * amount, (y as f32 + 0.5) / height + step[1] * amount];
        crate::texture_sampler::sample(&image, uv, tiling, format, None)
    })])
}
