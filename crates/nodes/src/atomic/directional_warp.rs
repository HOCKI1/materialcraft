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
    let target = [context.w, context.h].map(|side| side.clamp(1, craft_texture::MAX_SIDE));
    let tiling = context.tiling_axes();
    let (image, format) = warp_input(context, 0, target, tiling);
    let (map, map_format) = warp_input(context, 1, target, tiling);
    let map = map.to_gray();
    let same_size = [map.width(), map.height()] == target;
    let radians = context.f("angle") * std::f32::consts::TAU;
    // The horizontal phase includes a quarter turn rounded in radians before evaluating the sine.
    let component = |phase: f32| crate::function::truncate_float(f64::from(phase).sin());
    let direction = [component(radians + std::f32::consts::FRAC_PI_2), component(radians)];
    const BASE_RESOLUTION: f32 = 256.0;
    let [width, height] = target.map(|side| side as f32);
    let strength = context.f("intensity") / BASE_RESOLUTION;
    let shorter = width.min(height);
    let step = [direction[0] * strength * (shorter / width), direction[1] * strength * (shorter / height)];
    Ok(vec![Image::generate(context.w, context.h, image.kind(), |x, y| {
        let (u, v) = ((x as f32 + 0.5) / width, (y as f32 + 0.5) / height);
        let amount =
            if same_size { map.get_gray(i64::from(x), i64::from(y), false) } else { crate::texture_sampler::sample(&map, [u, v], tiling, map_format, None)[0] };
        let uv = [step[0].mul_add(amount, u), step[1].mul_add(amount, v)];
        crate::texture_sampler::sample(&image, uv, tiling, format, None)
    })])
}

fn warp_input(context: &CookContext, index: usize, target: [u32; 2], tiling: [bool; 2]) -> (Image, OutputFormat) {
    let Some(mut image) = context.input(index).cloned() else {
        return (Image::filled(target[0], target[1], Kind::Gray, [0.0; 4]), context.format);
    };
    let mut format = context.input_formats.get(index).copied().unwrap_or(if image.integer_sampling() { OutputFormat::U16 } else { OutputFormat::F32 });
    let mut size = [image.width(), image.height()];
    // A source up to twice the output on either axis can supply the final taps directly.
    if size.into_iter().zip(target).all(|(side, output)| (output..=output.saturating_mul(2)).contains(&side)) {
        return (image, format);
    }
    let storage = match (format, context.format) {
        (OutputFormat::F32, _) | (_, OutputFormat::F32) => OutputFormat::F32,
        (OutputFormat::F16, _) | (_, OutputFormat::F16) => OutputFormat::F16,
        (OutputFormat::U8, OutputFormat::U8) => OutputFormat::U8,
        _ => OutputFormat::U16,
    };
    while size != target {
        // Hold an axis within one level of the output until both axes can take their final step together.
        let next = std::array::from_fn(|i| {
            if size[i].saturating_mul(2) < target[i] {
                size[i] * 2
            } else if size[i] > target[i].saturating_mul(2) {
                size[i].div_ceil(2)
            } else {
                size[i]
            }
        });
        size = if next == size { target } else { next };
        image = crate::texture_sampler::resize_input_level(&image, size[0], size[1], format, tiling);
        image = crate::output_precision::apply(image, storage);
        format = storage;
    }
    (image, format)
}
