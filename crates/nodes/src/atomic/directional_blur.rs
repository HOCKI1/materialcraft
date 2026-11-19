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
        // The first tap lies in the positive direction of the dominant image axis.
        let u = dx / context.w as f32;
        let v = dy / context.h as f32;
        let dominant = if dx.abs() > dy.abs() { dx } else { dy };
        let span = -u.abs().max(v.abs()).copysign(dominant);
        return Ok(vec![box_filter::short_line(&input, reach, [u / span, v / span], tile, source_format)]);
    }
    if tile == [true; 2] && context.w == input.width() && context.h == input.height() {
        // Compare the components before rounding the direction to the sample grid.
        let (sine, cosine) = f64::from(angle * std::f32::consts::TAU).sin_cos();
        let horizontal = cosine.abs() > sine.abs();
        let slope = if horizontal { sine / cosine } else { cosine / sine };
        // An authored eighth turn has an exact unit slope even after radian conversion.
        let slope = if (angle * 8.0).fract() == 0.0 { slope.signum() as f32 } else { slope as f32 };
        if let Some(image) = box_filter::sheared(&input, reach, horizontal, slope, source_format, context.format) {
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
