use super::{ATOMIC, box_filter, direction, input_resize};
use crate::{CookContext, CookResult, NodeDefinition, node};
use craft_texture::Image;

pub(super) fn definition() -> NodeDefinition {
    node("directional_blur", "Directional blur", ATOMIC)
        .desc("Blurs along one direction.")
        .atomic()
        .any_in("input", "Input")
        .float("intensity", "Intensity", 10.0, 0.0, 64.0)
        .angle("angle", "Angle", 0.0)
        .any_out()
        .cook(directional_blur)
}

fn directional_blur(context: &CookContext) -> CookResult {
    blur(context, context.f("angle"))
}

pub(super) fn blur(context: &CookContext, angle: f32) -> CookResult {
    let intensity = context.f("intensity");
    let (input, source_format) = input_resize::blur_input(context, intensity);
    const BASE_RESOLUTION: f32 = 256.0;
    const MAX_HALF_SAMPLES: i32 = 512;
    let radius = intensity.max(0.0) * context.w.min(context.h) as f32 / BASE_RESOLUTION;
    if radius <= 0.5 {
        return Ok(vec![input]);
    }

    let tile = context.tiling_axes();
    let quarters = angle * 4.0;
    if quarters.fract() == 0.0 {
        let horizontal = quarters % 2.0 == 0.0;
        // Rotated axial footprints project their reach one float below unity before normalization.
        let projection = if angle == 0.0 { 1.0 } else { f32::from_bits(0x3f7f_ffff) };
        let reach = (radius - 0.5) * projection;
        if reach <= 2.0 {
            let step = if !horizontal {
                [0.0, -1.0]
            } else if reach <= 1.0 {
                [-1.0, 0.0]
            } else {
                [1.0, 0.0]
            };
            return Ok(vec![box_filter::short_line(&input, reach, step, tile, source_format)]);
        }
        return Ok(vec![box_filter::axis(&input, reach, horizontal, tile[usize::from(!horizontal)], source_format, context.format)]);
    }

    let (dx, dy) = direction(angle);
    // Image-space rotation follows increasing rows.
    let dy = -dy;
    let major = dx.abs().max(dy.abs());
    let reach = (radius - 0.5) * major;
    if reach <= 1.0 {
        // Normalize the offsets in texture coordinates so the longer side keeps its texel scale.
        let u = dx / context.w as f32;
        let v = dy / context.h as f32;
        // Tap order affects the fused sum. A rectangular image can change which UV axis dominates.
        let dominant = if u.abs() > v.abs() { u } else { v };
        let span = -dominant;
        return Ok(vec![box_filter::short_line(&input, reach, [u / span, v / span], tile, source_format)]);
    }
    if tile == [true; 2] && context.w == input.width() && context.h == input.height() {
        // Compare the components before rounding the direction to the sample grid.
        let radians = angle * std::f32::consts::TAU;
        let (sine, cosine) = f64::from(radians).sin_cos();
        let horizontal = cosine.abs() / f64::from(context.w) > sine.abs() / f64::from(context.h);
        // An authored eighth turn has an exact unit slope even after radian conversion.
        // On a rectangle the dominant UV direction can have the smaller texel
        // component. The window spans the projection along the chosen texel axis.
        let (slope, along) = if (angle * 8.0).fract() == 0.0 {
            ((sine * cosine).signum() as f32, major)
        } else {
            let sine = projected_sine(radians.abs()) * radians.signum();
            let cosine = projected_sine(radians.abs() + std::f32::consts::FRAC_PI_2);
            let (across, along) = if horizontal { (sine, cosine) } else { (cosine, sine) };
            (across / along, along.abs())
        };
        if let Some(image) = box_filter::sheared(&input, (radius - 0.5) * along, horizontal, slope, source_format, context.format) {
            return Ok(vec![image]);
        }
    }
    let projected_radius = 0.5 + reach;
    let half_samples = (projected_radius + 0.5).floor().clamp(1.0, MAX_HALF_SAMPLES as f32) as i32;

    // Bound oblique sampling cost while preserving the extent of large kernels
    let step = ((projected_radius - 0.5) / half_samples as f32).max(1.0);
    let weight = step / (2.0 * projected_radius);
    let edge_weight = (0.5 - (half_samples as f32 - 0.5) * weight).max(0.0);
    let (inverse_width, inverse_height) = (1.0 / context.w as f32, 1.0 / context.h as f32);

    Ok(vec![Image::generate_uv(context.w, context.h, input.kind(), |u, v| {
        let mut pixel = [0.0; 4];
        for i in -half_samples..=half_samples {
            let sample_weight = if i.abs() == half_samples { edge_weight } else { weight };
            if sample_weight == 0.0 {
                continue;
            }

            let offset = i as f32 * step / major;
            let sample = input.sample_axes(u + dx * offset * inverse_width, v + dy * offset * inverse_height, tile);
            for channel in 0..4 {
                pixel[channel] += sample[channel] * sample_weight;
            }
        }
        pixel
    })])
}

/// The wide filter stores both direction components before deriving the window and shear.
fn projected_sine(radians: f32) -> f32 {
    use std::f64::consts::{FRAC_PI_2, PI, TAU};

    let phase = f64::from(radians) % TAU;
    let folded = if phase <= FRAC_PI_2 {
        phase
    } else if phase <= 3.0 * FRAC_PI_2 {
        PI - phase
    } else {
        phase - TAU
    };
    let x = folded as f32;
    let square = x * x;
    // Evaluate the odd sine series on the central half turn. Each product rounds before the next sum.
    let mut tail = -1.0 / 39_916_800.0;
    for coefficient in [1.0 / 362_880.0, -1.0 / 5_040.0, 1.0 / 120.0, -1.0 / 6.0] {
        tail = tail * square + coefficient;
    }
    x + (tail * square) * x
}
