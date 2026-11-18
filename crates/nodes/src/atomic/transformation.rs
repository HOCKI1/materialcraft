use super::ATOMIC;
use crate::{CookContext, CookResult, NodeDefinition, node};
use craft_texture::{Image, Kind};
use materialcraft_core::OutputFormat;

pub(super) fn definition() -> NodeDefinition {
    node("transformation", "Transformation 2D", ATOMIC)
        .desc("Transforms the input with an offset and a 2 by 2 matrix.")
        .atomic()
        .any_in("input", "Input")
        .float2("offset", "Offset", [0.0; 2], -1.0, 1.0)
        .float4("matrix", "Transform matrix", [1.0, 0.0, 0.0, 1.0], -10.0, 10.0)
        .choice("filtering", "Filtering", &["Bilinear", "Nearest"], 0)
        .choice("mip_mode", "Mipmap mode", &["Automatic", "Manual"], 0)
        .int("mip_level", "Mipmap level", 0, 0, 13)
        .color("matte", "Matte color", [0.0; 4])
        .any_out()
        .cook(transformation)
}

fn transformation(context: &CookContext) -> CookResult {
    // Preserve source texels so nearest sampling does not blur the input
    let input = context.input(0).cloned().unwrap_or_else(|| Image::filled(context.w, context.h, Kind::Gray, [0.0; 4]));
    let offset = context.v2("offset");
    let matrix = context.v4("matrix");
    const MIN_DETERMINANT: f32 = 1e-8;
    let determinant = matrix[0] * matrix[3] - matrix[1] * matrix[2];
    if !determinant.is_finite() || determinant.abs() < MIN_DETERMINANT {
        return Err("transform matrix is singular".into());
    }

    let nearest = context.i("filtering") == 1;
    let levels = mip_level(&input, matrix, context);
    let source_format = context.input_formats.first().copied().unwrap_or(if input.integer_sampling() { OutputFormat::U16 } else { OutputFormat::F32 });
    let sample_format = if !nearest && levels > 0 && (input.width() > 1 || input.height() > 1) { context.format } else { source_format };
    let input = reduce_mip(input, levels, nearest, context.format);
    let matte = crate::output_precision::store_pixel(context.v4("matte"), sample_format);

    // Keep independent wrap axes for panoramic images
    let tiling = context.tiling_axes();

    Ok(vec![Image::generate_uv(context.w, context.h, input.kind(), |u, v| {
        let x = u - 0.5;
        let y = v - 0.5;

        // SBS stores matrix columns consecutively; interpreting rows reverses rotations
        let source_u = matrix[0] * x + matrix[2] * y + 0.5 + offset[0];
        let source_v = matrix[1] * x + matrix[3] * y + 0.5 + offset[1];
        if nearest {
            if (!tiling[0] && !(0.0..1.0).contains(&source_u)) || (!tiling[1] && !(0.0..1.0).contains(&source_v)) {
                matte
            } else {
                input.sample_nearest_axes(source_u, source_v, tiling)
            }
        } else {
            crate::texture_sampler::sample(&input, [source_u, source_v], tiling, sample_format, Some(matte))
        }
    })])
}

fn mip_level(input: &Image, matrix: [f32; 4], context: &CookContext) -> u32 {
    // Thirteen reductions cover the maximum supported side of 8192 texels.
    if context.i("mip_mode") == 1 {
        return context.i("mip_level").clamp(0, 13) as u32;
    }
    let width = context.w.max(1) as f32;
    let height = context.h.max(1) as f32;
    let dx = (matrix[0] * input.width() as f32 / width).hypot(matrix[1] * input.height() as f32 / width);
    let dy = (matrix[2] * input.width() as f32 / height).hypot(matrix[3] * input.height() as f32 / height);

    dx.max(dy).max(1.0).log2().round().clamp(0.0, 13.0) as u32
}

fn reduce_mip(mut input: Image, levels: u32, nearest: bool, format: OutputFormat) -> Image {
    let nearest_maximum = match format {
        OutputFormat::U8 => Some(255.0),
        OutputFormat::Inherit | OutputFormat::U16 => Some(65535.0),
        OutputFormat::F16 | OutputFormat::F32 => None,
    };

    for _ in 0..levels {
        if input.width() <= 1 && input.height() <= 1 {
            break;
        }

        // Bilinear samples of either normalized source depth return a 16-bit value.
        let maximum = if nearest { nearest_maximum } else { input.integer_sampling().then_some(65535.0) };
        input = Image::generate(input.width().div_ceil(2), input.height().div_ceil(2), input.kind(), |x, y| {
            let (x, y) = (i64::from(x) * 2, i64::from(y) * 2);
            let pixels = [input.get(x, y, false), input.get(x + 1, y, false), input.get(x, y + 1, false), input.get(x + 1, y + 1, false)];
            std::array::from_fn(|channel| {
                if let Some(maximum) = maximum {
                    let sum: u32 = pixels.iter().map(|pixel| (pixel[channel].clamp(0.0, 1.0) * maximum).round() as u32).sum();
                    ((sum + 2) / 4) as f32 / maximum
                } else {
                    // Average in f64 so finite HDR texels cannot overflow the sum
                    (pixels.iter().map(|pixel| f64::from(pixel[channel])).sum::<f64>() * 0.25) as f32
                }
            })
        });
        if !nearest {
            // Each reduction is stored at the transform's output precision before the next read.
            input = crate::output_precision::apply(input, format);
        }
    }

    input
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::resolve_params;
    use materialcraft_core::Value;
    use std::collections::BTreeMap;

    #[test]
    fn bilinear_transform_filters_the_matte_at_each_untiled_edge() {
        let params = resolve_params(&definition(), &BTreeMap::from([("offset".into(), Value::Float2([0.25 / 4.0, 0.375 / 4.0]))]));
        let inputs = [Some(Image::filled(4, 4, Kind::Color, [1.0; 4]).with_integer_sampling(true))];
        let vars = BTreeMap::new();
        let context = CookContext {
            input_formats: &[],
            w: 4,
            h: 4,
            seed: 0,
            format: OutputFormat::F32,
            tile: false,
            inputs: &inputs,
            params: &params,
            resource: &|_| None,
            vars: &vars,
        };
        let result = transformation(&context).unwrap();
        for (x, y, level) in [(0, 0, 65535.0), (3, 0, 49151.0), (0, 3, 40959.0), (3, 3, 30720.0)] {
            assert_eq!(result[0].get(x, y, false), [level / 65535.0; 4], "texel ({x}, {y})");
        }
    }

    #[test]
    fn filtered_float_texels_round_at_the_source_precision() {
        let params = resolve_params(&definition(), &BTreeMap::from([("offset".into(), Value::Float2([0.25 / 2.0, 0.375 / 2.0]))]));
        let vars = BTreeMap::new();
        for (format, bits, expected) in [
            (OutputFormat::F16, [0x3fd0e000, 0xbfa46000, 0xc02d8000, 0x407f0000], 0x3e33a000),
            (OutputFormat::F32, [0x3fc50790, 0xc0781fc9, 0x3f8389d2, 0xc029c3cd], 0x3e1fdd4b),
        ] {
            let inputs = [Some(Image::gray(2, 2, |x, y| f32::from_bits(bits[(2 * y + x) as usize])))];
            let context = CookContext {
                w: 2,
                h: 2,
                seed: 0,
                format: OutputFormat::F32,
                tile: true,
                inputs: &inputs,
                input_formats: &[format],
                params: &params,
                resource: &|_| None,
                vars: &vars,
            };
            assert_eq!(transformation(&context).unwrap()[0].get_gray(0, 0, false).to_bits(), expected, "{format:?}");
        }
    }

    #[test]
    fn matte_uses_the_inputs_storage_even_with_a_float_output() {
        let matte = [0.5, 0.1, -0.312345, 1.144444];
        let params = resolve_params(&definition(), &BTreeMap::from([("offset".into(), Value::Float2([1.0, 1.0])), ("matte".into(), Value::Color(matte))]));
        let vars = BTreeMap::new();
        for (format, expected) in [
            (OutputFormat::U8, [127.0 / 255.0, 25.0 / 255.0, 0.0, 1.0]),
            (OutputFormat::U16, [32767.0 / 65535.0, 6553.0 / 65535.0, 0.0, 1.0]),
            (OutputFormat::F16, [0.5, 0.099975586, -0.31225586, 1.1435547]),
            (OutputFormat::F32, matte),
        ] {
            let integer = matches!(format, OutputFormat::U8 | OutputFormat::U16);
            let inputs = [Some(Image::filled(4, 4, Kind::Color, [1.0; 4]).with_integer_sampling(integer))];
            let context = CookContext {
                w: 4,
                h: 4,
                seed: 0,
                format: OutputFormat::F32,
                tile: false,
                inputs: &inputs,
                input_formats: &[format],
                params: &params,
                resource: &|_| None,
                vars: &vars,
            };
            assert_eq!(transformation(&context).unwrap()[0].get(2, 2, false), expected, "{format:?}");
        }
    }
}
