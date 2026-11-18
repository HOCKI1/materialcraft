use super::ATOMIC;
use crate::ops;
use crate::{CookContext, CookResult, NodeDefinition, node};
use craft_texture::Image;

mod mask_field;

pub(super) fn definition() -> NodeDefinition {
    node("distance", "Distance", ATOMIC)
        .desc("Grows white areas of a mask into a distance gradient; with a source, spreads its colors outward.")
        .atomic()
        .gray_in("mask", "Mask input")
        .any_in("source", "Source input")
        .float("distance", "Maximum distance", 10.0, 0.0, 256.0)
        .boolean("combine", "Combine distance with source", true)
        .any_out()
        .cook(distance)
}

fn distance(context: &CookContext) -> CookResult {
    let mask = context.gray_or(0, 0.0);
    const BASE_RESOLUTION: f32 = 256.0;
    const MIN_DISTANCE_PIXELS: f32 = 1e-3;
    let strength = context.f("distance").max(0.0) / BASE_RESOLUTION;
    let Some(source) = context.sized(1) else {
        return Ok(vec![mask_field::render(&mask, strength * mask.width().min(mask.height()) as f32, context.tiling_axes())?]);
    };
    let maximum_distance = (strength * context.w as f32).max(MIN_DISTANCE_PIXELS);
    let data = mask.shared_data().clone();
    let inside = |i| data.get(i).copied().unwrap_or(0.0) > 0.5;

    // Propagate the exact nearest site without gaps from gradient-walk stalls
    let (squared_distances, sites) = ops::distance_sq_with_sites(context.w, context.h, inside, context.tile);
    let combine = context.b("combine");
    Ok(vec![Image::generate(context.w, context.h, source.kind(), |x, y| {
        let i = y as usize * context.w as usize + x as usize;
        let site = sites.get(i).copied().unwrap_or(usize::MAX);
        let mut pixel = if site == usize::MAX {
            [0.0, 0.0, 0.0, if source.is_gray() { 1.0 } else { 0.0 }]
        } else {
            source.get((site % context.w as usize) as i64, (site / context.w as usize) as i64, false)
        };
        let d = squared_distances.get(i).copied().unwrap_or(f32::INFINITY).sqrt();
        if d > maximum_distance {
            pixel = [0.0, 0.0, 0.0, if source.is_gray() { 1.0 } else { 0.0 }];
        } else if combine {
            let falloff = (1.0 - d / maximum_distance).clamp(0.0, 1.0);
            pixel = [pixel[0] * falloff, pixel[1] * falloff, pixel[2] * falloff, pixel[3]];
        }
        pixel
    })])
}
