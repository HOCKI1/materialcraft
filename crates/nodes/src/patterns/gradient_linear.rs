use super::CATEGORY;
use crate::{CookContext, CookResult, NodeDefinition, node};
use craft_texture::{Image, Kind};
use std::f32::consts::PI;

pub const CYCLES: &[&str] = &["Clamp", "Repeat", "Mirror"];

pub(crate) fn definition() -> NodeDefinition {
    node("gradient_linear", "Gradient Linear", CATEGORY)
        .desc("Generates a linear ramp gradient with angle, repetition, and cycling modes.")
        .generator()
        .angle("angle", "Angle", 0.0)
        .int("tiling", "Tiling", 1, 1, 64)
        .choice("cycle", "Cycle mode", CYCLES, 0)
        .boolean("reverse", "Reverse", false)
        .gray_out()
        .cook(cook)
}

fn cook(context: &CookContext<'_>) -> CookResult {
    let angle = context.f("angle") * PI;
    let tiling = context.i("tiling").max(1) as f32;
    let cycle = context.i("cycle");
    let reverse = context.b("reverse");

    let cos_a = angle.cos();
    let sin_a = angle.sin();

    let (w_f, h_f) = (context.w as f32, context.h as f32);

    Ok(vec![Image::generate(context.w, context.h, Kind::Gray, |x, y| {
        let u = (x as f32 + 0.5) / w_f;
        let v = (y as f32 + 0.5) / h_f;

        let dx = u - 0.5;
        let dy = v - 0.5;

        let proj = dx * cos_a - dy * sin_a + 0.5;
        let scaled = proj * tiling;

        let val = match cycle {
            // Repeat
            1 => scaled.rem_euclid(1.0),
            // Mirror
            2 => 1.0 - (scaled.rem_euclid(2.0) - 1.0).abs(),
            // Clamp (default)
            _ => scaled.clamp(0.0, 1.0),
        };

        let val = if reverse { 1.0 - val } else { val };
        [val.clamp(0.0, 1.0), 0.0, 0.0, 0.0]
    })])
}
