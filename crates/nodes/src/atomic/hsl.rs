use super::ATOMIC;
use crate::function::pixel_divide;
use crate::{CookContext, CookResult, NodeDefinition, node};
use craft_texture::Kind;

pub(super) fn definition() -> NodeDefinition {
    node("hsl", "HSL", ATOMIC)
        .desc("Adjust hue, saturation and lightness around the midpoint of each slider.")
        .atomic()
        .color_in("input", "Input")
        .float("hue", "Hue", 0.5, 0.0, 1.0)
        .float("saturation", "Saturation", 0.5, 0.0, 1.0)
        .float("lightness", "Lightness", 0.5, 0.0, 1.0)
        .color_out()
        .cook(hsl)
}

fn hsl(context: &CookContext) -> CookResult {
    let image = context.any_or(0, Kind::Color, [0.0; 4]).to_color();
    let [hue_shift, saturation_shift, lightness_shift] = ["hue", "saturation", "lightness"].map(|name| (context.f(name) - 0.5) * 2.0);

    Ok(vec![image.map(Kind::Color, |[red, green, blue, alpha]| {
        let gain = red.max(green).max(blue).max(1.0);
        let scale = pixel_divide(1.0, gain);
        let [r, g, b] = [red * scale, green * scale, blue * scale];
        let low = r.min(g).min(b);
        let high = r.max(g).max(b);
        let chroma = high - low;
        let lightness = (low + high) * 0.5;

        // The nonzero divisors also affect ordinary colors at the slider midpoints.
        const EPSILON: f32 = 1.0 / 4096.0;
        let saturation = if chroma == 0.0 { 0.0 } else { pixel_divide(chroma, 1.0 - (2.0 * lightness - 1.0).abs() + EPSILON) };
        let s = (saturation + saturation_shift).clamp(0.0, 1.0);
        let l = (lightness + lightness_shift).clamp(0.0, 1.0);

        // Express hue in the third of a turn selected by the lowest channel.
        let (difference, offset) = if low == r {
            (b - g, 0.0)
        } else if low == g {
            (r - b, 1.0 / 3.0)
        } else {
            (g - r, -1.0 / 3.0)
        };
        let hue = (3.0_f32.mul_add(chroma, difference) / (6.0 * (chroma + EPSILON)) + offset) + hue_shift;
        let upper = s.mul_add(l.min(1.0 - l), l);
        let lower = 2.0 * l - upper;
        let channel = |phase: f32| {
            let wrapped = phase - phase.floor();
            let weight = (wrapped.mul_add(6.0, -3.0).abs() - 1.0).clamp(0.0, 1.0);
            (upper - lower).mul_add(weight, lower) * gain
        };
        [channel(hue), channel(hue - 1.0 / 3.0), channel(hue + 1.0 / 3.0), alpha]
    })])
}
