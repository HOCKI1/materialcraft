use super::{ATOMIC, box_filter, input_resize};
use crate::{CookContext, CookResult, NodeDefinition, node};

pub(super) fn definition() -> NodeDefinition {
    node("blur", "Blur", ATOMIC)
        .desc("Box blur with fractional pixel coverage.")
        .atomic()
        .any_in("input", "Input")
        .float("intensity", "Intensity", 10.0, 0.0, 32.0)
        .any_out()
        .cook(blur)
}

fn blur(context: &CookContext) -> CookResult {
    let intensity = context.f("intensity");
    let (input, source_format) = input_resize::blur_input(context, intensity);
    const BASE_RESOLUTION: f32 = 256.0;
    let radius = intensity.max(0.0) * context.w.min(context.h) as f32 / BASE_RESOLUTION;
    let tile = context.tiling_axes();

    let reach = radius - 0.5;
    if reach > 0.0 && reach <= 2.0 {
        let horizontal = box_filter::short_line(&input, reach, [1.0, 0.0], tile, source_format);
        let stored = crate::output_precision::apply(horizontal, context.format);
        return Ok(vec![box_filter::short_line(&stored, reach, [0.0, -1.0], tile, context.format)]);
    }
    if radius <= 0.5 || !radius.is_finite() {
        return Ok(vec![input]);
    }
    let horizontal = box_filter::axis(&input, reach, true, tile[0], source_format, context.format);
    let stored = crate::output_precision::apply(horizontal, context.format);
    Ok(vec![box_filter::axis(&stored, reach, false, tile[1], context.format, context.format)])
}
