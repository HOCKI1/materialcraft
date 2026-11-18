use super::ATOMIC;
use crate::{CookContext, CookResult, NodeDefinition, node};

mod mask_field;

pub(super) fn definition() -> NodeDefinition {
    node("distance", "Distance", ATOMIC)
        .desc("Grows white areas of a mask into a distance gradient; with a source, spreads its colors outward.")
        .atomic()
        .gray_in("mask", "Mask input")
        .any_in("source", "Source input")
        .float("distance", "Maximum distance", 10.0, 0.0, 256.0)
        .choice("metric", "Distance metric", &["Euclidean", "Manhattan", "Chebyshev"], 0)
        .boolean("combine", "Combine distance with source", true)
        .any_out()
        .cook(distance)
}

fn distance(context: &CookContext) -> CookResult {
    let mask = context.gray_or(0, 0.0);
    let source = context.sized(1);
    let radius = (context.f("distance").max(0.0) * (mask.width().min(mask.height()) as f32 / 256.0)).min(f32::MAX);
    Ok(vec![mask_field::render(&mask, source.as_ref(), radius, context.tiling_axes(), context.i("metric"), context.b("combine"))?])
}
