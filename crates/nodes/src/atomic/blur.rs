use super::{ATOMIC, box_filter};
use crate::{CookContext, CookResult, NodeDefinition, node};
use craft_texture::Kind;

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
    let input = context.any_or(0, Kind::Gray, [0.0; 4]);
    const BASE_RESOLUTION: f32 = 256.0;
    let radius = context.f("intensity").max(0.0) * context.w.min(context.h) as f32 / BASE_RESOLUTION;
    let tile = context.tiling_axes();
    let source_format = context.input_formats.first().copied().unwrap_or(materialcraft_core::OutputFormat::F32);

    let reach = radius - 0.5;
    if reach > 0.0 && reach <= 2.0 {
        let horizontal = box_filter::short_axis(&input, reach, [1, 0], tile, source_format);
        let stored = crate::output_precision::apply(horizontal, context.format);
        return Ok(vec![box_filter::short_axis(&stored, reach, [0, -1], tile, context.format)]);
    }
    if radius <= 0.5 || !radius.is_finite() {
        return Ok(vec![input]);
    }
    let horizontal = box_filter::axis(&input, reach, true, tile[0], source_format, context.format);
    let stored = crate::output_precision::apply(horizontal, context.format);
    Ok(vec![box_filter::axis(&stored, reach, false, tile[1], context.format, context.format)])
}
