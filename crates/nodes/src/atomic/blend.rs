use super::ATOMIC;
use crate::ops;
use crate::ops::BLEND_MODES;
use crate::{CookContext, CookResult, NodeDefinition, node};
use craft_texture::{Image, Kind};

pub(super) fn definition() -> NodeDefinition {
    node("blend", "Blend", ATOMIC)
        .desc("Blends a foreground over a background with a blending mode, weighted by opacity and an optional mask.")
        .atomic()
        .any_in("foreground", "Foreground")
        .any_in("background", "Background")
        .primary_input("background")
        .gray_in("mask", "Opacity")
        .float("opacity", "Opacity", 1.0, 0.0, 1.0)
        .choice("mode", "Blending mode", BLEND_MODES, 0)
        .boolean("alpha", "Alpha blending", true)
        .choice("color_blending", "Alpha interpretation", &["Source alpha", "Ignore alpha", "Straight", "Premultiplied"], 0)
        .any_out()
        .cook(blend)
}

fn blend(context: &CookContext) -> CookResult {
    let (foreground, background) = (context.sized(0), context.sized(1));
    let kind = if [&foreground, &background].iter().any(|i| i.as_ref().is_some_and(|i| !i.is_gray())) { Kind::Color } else { Kind::Gray };
    let foreground = foreground.unwrap_or_else(|| Image::filled(context.w, context.h, kind, [0.0, 0.0, 0.0, 0.0]));
    let background = background.unwrap_or_else(|| Image::filled(context.w, context.h, kind, [0.0, 0.0, 0.0, 1.0]));
    let mask = context.sized(2).map(|m| m.to_gray());
    let (mode, opacity) = (context.i("mode"), context.f("opacity"));
    let opacity = if matches!(mode, 1 | 2) { opacity.clamp(0.0, 1.0) } else { opacity };
    let color_blending = context.i("color_blending");
    let blend_alpha = context.b("alpha") && !foreground.is_gray();
    let clamp =
        matches!(context.format, materialcraft_core::OutputFormat::U8 | materialcraft_core::OutputFormat::U16 | materialcraft_core::OutputFormat::Inherit);
    Ok(vec![Image::generate(context.w, context.h, kind, |x, y| {
        let (x, y) = (x as i64, y as i64);
        let m = mask.as_ref().map_or(1.0, |m| m.get_gray(x, y, false));
        // These modes read the mask through its transparency before applying global opacity.
        let m = if ((0..=2).contains(&mode) && color_blending == 0) || matches!(mode, 3 | 7 | 8) { 1.0 - (1.0 - m) } else { m };
        ops::blend_px(mode, background.get(x, y, false), foreground.get(x, y, false), opacity * m, color_blending, blend_alpha, clamp)
    })])
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::resolve_params;
    use materialcraft_core::{OutputFormat, Value};
    use std::collections::BTreeMap;

    #[test]
    fn blend_rounds_mask_color_and_coverage_independently() {
        let foreground = [116.0, 38.0, 60.0, 36.0].map(|v| v / 255.0);
        let background = [158.0, 163.0, 10.0, 68.0].map(|v| v / 255.0);
        let mask = 44.0 / 255.0;
        for (mode, interpretation, opacity, alpha, hdr, expected) in [
            (0, 0, 1.0, true, false, [0x3f1d97ad, 0x3f209511, 0x3d343117, 0x3e91adfe]),
            (0, 0, 0.07, true, false, [0x3f1e8c37, 0x3f236cdd, 0x3d21ff39, 0x3e892c70]),
            (0, 0, 0.3, false, false, [0x3f1e4fbd, 0x3f22b8df, 0x3d267f2b, 0x3e888889]),
            (0, 0, 0.3, true, true, [0x3f514e8e, 0x3f4b8353, 0xbf55520b, 0xbdcc9712]),
            (3, 0, 0.6875, true, false, [0x3f1d2bec, 0x3f214e9d, 0x3d1e9201, 0x3e8ed249]),
            (3, 1, 0.6875, true, false, [0x3f145cda, 0x3f131eb2, 0x3d120e5e, 0x3e8ed249]),
            (3, 3, 0.6875, true, false, [0x3f1d2bec, 0x3f214e9d, 0x3d1e9201, 0x3e8ed249]),
            (7, 0, 0.6137, true, false, [0x3f1a2797, 0x3f1659c4, 0x3d75aca1, 0x3e81bac6]),
            (8, 0, 0.6137, true, false, [0x3f21760a, 0x3f319bf2, 0x3d286e88, 0x3e8e257d]),
            (8, 1, 0.6137, false, false, [0x3f32bf2c, 0x3f834be5, 0x3d57e86e, 0x3e888889]),
        ] {
            let level = |v: f32| if hdr { v * 3.0 - 1.0 } else { v };
            let inputs = [
                Some(Image::filled(1, 1, Kind::Color, foreground.map(level))),
                Some(Image::filled(1, 1, Kind::Color, background.map(level))),
                Some(Image::gray(1, 1, |_, _| level(mask))),
            ];
            let vars = BTreeMap::new();
            let params = resolve_params(
                &definition(),
                &BTreeMap::from([
                    ("mode".into(), Value::Enum(mode)),
                    ("color_blending".into(), Value::Enum(interpretation)),
                    ("opacity".into(), Value::Float(opacity)),
                    ("alpha".into(), Value::Bool(alpha)),
                ]),
            );
            let context = CookContext {
                input_formats: &[],
                format: OutputFormat::F32,
                w: 1,
                h: 1,
                seed: 0,
                tile: true,
                inputs: &inputs,
                params: &params,
                resource: &|_| None,
                vars: &vars,
            };
            let result = blend(&context).unwrap();
            assert_eq!(result[0].get(0, 0, false).map(f32::to_bits), expected, "mode {mode}, interpretation {interpretation}, opacity {opacity}");
        }
    }
}
