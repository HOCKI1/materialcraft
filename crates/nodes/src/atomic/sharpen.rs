use super::ATOMIC;
use super::input_resize::{self, InputResize};
use crate::{CookContext, CookResult, NodeDefinition, node};
use craft_texture::{Image, Kind};

pub(super) fn definition() -> NodeDefinition {
    node("sharpen", "Sharpen", ATOMIC)
        .desc("Sharpens local contrast with a four-neighbor Laplacian.")
        .atomic()
        .any_in("input", "Input")
        .float("intensity", "Intensity", 1.0, 0.0, 10.0)
        .any_out()
        .cook(sharpen)
}

fn sharpen(context: &CookContext) -> CookResult {
    let input = input_resize::resized_input(context, 0, InputResize::Filter).unwrap_or_else(|| Image::filled(context.w, context.h, Kind::Gray, [0.0; 4]));
    let intensity = context.f("intensity");
    let tile = context.tiling_axes();
    let short_side = context.w.min(context.h) as f32;
    let horizontal = intensity * short_side / context.w.max(1) as f32;
    let vertical = intensity * short_side / context.h.max(1) as f32;
    let center_weight = 1.0 + 2.0 * (horizontal + vertical);

    Ok(vec![Image::generate(context.w, context.h, input.kind(), |x, y| {
        let (x, y) = (i64::from(x), i64::from(y));
        let center = input.get(x, y, false);
        let left = input.get_axes(x - 1, y, tile);
        let right = input.get_axes(x + 1, y, tile);
        let top = input.get_axes(x, y - 1, tile);
        let bottom = input.get_axes(x, y + 1, tile);
        std::array::from_fn(|channel| {
            let [center, left, right, top, bottom] = [center[channel], left[channel], right[channel], top[channel], bottom[channel]];
            let sum = center.mul_add(center_weight, -horizontal * left);
            let sum = right.mul_add(-horizontal, sum);
            let sum = top.mul_add(-vertical, sum);
            let result = bottom.mul_add(-vertical, sum);
            if result.is_finite() || ![center, left, right, top, bottom].iter().all(|v| v.is_finite()) {
                return result;
            }
            // A finite HDR plateau can overflow the weighted center before its neighbors cancel.
            let center = f64::from(center);
            let across = 2.0 * center - (f64::from(left) + f64::from(right));
            let down = 2.0 * center - (f64::from(top) + f64::from(bottom));
            (center + across * f64::from(horizontal) + down * f64::from(vertical)) as f32
        })
    })])
}
