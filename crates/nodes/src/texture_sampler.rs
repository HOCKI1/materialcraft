use craft_texture::Image;
use materialcraft_core::OutputFormat;

/// Resample one stored input level at destination pixel centers.
pub fn resize_input_level(image: &Image, width: u32, height: u32, format: OutputFormat, tiling: [bool; 2]) -> Image {
    Image::generate_uv(width, height, image.kind(), |u, v| sample(image, [u, v], tiling, format, None))
}

/// Bilinear texture filtering. Untiled axes clamp unless the caller supplies a border color.
pub(crate) fn sample(image: &Image, uv: [f32; 2], tiling: [bool; 2], format: OutputFormat, matte: Option<[f32; 4]>) -> [f32; 4] {
    if !uv.iter().all(|v| v.is_finite()) {
        return matte.unwrap_or([0.0, 0.0, 0.0, 1.0]);
    }
    let taps = taps(image, uv, tiling);
    if image.integer_sampling() {
        return sample_normalized(image, taps, tiling, matte);
    }
    if let Some((x, y, _)) = taps.iter().find(|(_, _, weight)| *weight == 256) {
        return texel(image, *x, *y, tiling, matte);
    }
    let pixels = taps.map(|(x, y, weight)| (texel(image, x, y, tiling, matte), weight));
    std::array::from_fn(|channel| filter_float(&pixels, channel, format))
}

fn taps(image: &Image, [u, v]: [f32; 2], tiling: [bool; 2]) -> [(i64, i64, u32); 4] {
    let (x, dx) = axis(u, image.width(), tiling[0]);
    let (y, dy) = axis(v, image.height(), tiling[1]);
    // Round the diagonal coefficient first. The four tap weights must still sum to 256.
    let diagonal = (dx * dy + 128) / 256;
    let bottom = dy - diagonal;
    [(x, y, 256 - dx - bottom), (x + 1, y, dx - diagonal), (x, y + 1, bottom), (x + 1, y + 1, diagonal)]
}

fn texel(image: &Image, x: i64, y: i64, tiling: [bool; 2], matte: Option<[f32; 4]>) -> [f32; 4] {
    if let Some(matte) = matte
        && ((!tiling[0] && !(0..i64::from(image.width())).contains(&x)) || (!tiling[1] && !(0..i64::from(image.height())).contains(&y)))
    {
        return matte;
    }
    image.get_axes(x, y, tiling)
}

fn sample_normalized(image: &Image, taps: [(i64, i64, u32); 4], tiling: [bool; 2], matte: Option<[f32; 4]>) -> [f32; 4] {
    let mut sum = [0.0; 4];
    for (x, y, weight) in taps {
        if weight != 0 {
            for (total, channel) in sum.iter_mut().zip(texel(image, x, y, tiling, matte)) {
                // Stored 16-bit levels times the eight-bit weights sum exactly in f32.
                *total += (channel.clamp(0.0, 1.0) * 65535.0).round() * weight as f32;
            }
        }
    }
    sum.map(|value| (value / 256.0).round() / 65535.0)
}

fn filter_float(taps: &[([f32; 4], u32); 4], channel: usize, format: OutputFormat) -> f32 {
    let mut maximum = 0.0_f32;
    for (pixel, weight) in taps {
        if *weight != 0 {
            let value = pixel[channel];
            if !value.is_finite() {
                return taps.iter().filter(|(_, w)| *w != 0).map(|(p, w)| p[channel] * (*w as f32 / 256.0)).sum();
            }
            maximum = maximum.max(value.abs());
        }
    }
    if maximum == 0.0 {
        return 0.0;
    }

    // Align participating texels to their largest exponent, retaining four extra significand bits.
    // The fixed-point products fit in 36 bits even for the largest finite float32 inputs.
    let (fraction_bits, smallest_exponent) = if format == OutputFormat::F16 { (10, -24) } else { (23, -149) };
    let exponent = (f64::from(maximum).to_bits() >> 52) as i32 - 1023;
    let unit_exponent = exponent - fraction_bits - 4;
    let scale = f64::from_bits(((1023 - unit_exponent) as u64) << 52);
    let sum: i64 = taps.iter().filter(|(_, w)| *w != 0).map(|(p, w)| (f64::from(p[channel]) * scale) as i64 * i64::from(*w)).sum();
    let magnitude = sum.unsigned_abs();
    if magnitude == 0 {
        return 0.0;
    }
    // Round the weighted sum once at the input storage precision, including ties away from zero.
    let shift = (63 - magnitude.leading_zeros() as i32 - fraction_bits).max(smallest_exponent - (unit_exponent - 8)).max(0);
    if shift >= 63 {
        return 0.0;
    }
    let rounded = if shift == 0 { magnitude } else { ((magnitude + (1 << (shift - 1))) >> shift) << shift };
    let value = (rounded as f64 / (256.0 * scale)) as f32;
    if sum < 0 { -value } else { value }
}

fn axis(uv: f32, size: u32, tile: bool) -> (i64, u32) {
    // Reduce before converting to a texel so even extreme finite coordinates have bounded neighbors.
    // One extra image width on either side retains outside taps for a transform's matte.
    let uv = if tile { uv.rem_euclid(1.0) } else { uv.clamp(-1.0, 2.0) };
    let texel = uv * size as f32 - 0.5;
    let first = texel.floor();
    (first as i64, ((texel - first) * 256.0).round() as u32)
}

#[cfg(test)]
mod tests {
    use crate::function::{EvaluationContext, EvaluationState, FunctionScope, compile};
    use craft_texture::{Image, Kind};
    use materialcraft_core::{FunctionGraph, Value};
    use std::collections::BTreeMap;

    #[test]
    fn pixel_sampler_rounds_the_four_integer_texel_weights_together() {
        // RGBA identifies the four corners independently. At (1,128)/256 the diagonal weight is halfway to one.
        let image = Image::generate(2, 2, Kind::Color, |x, y| {
            let mut pixel = [0.0; 4];
            pixel[(2 * y + x) as usize] = 1.0;
            pixel
        })
        .with_integer_sampling(true);
        let variables = BTreeMap::new();
        let mut context = EvaluationContext {
            image_formats: &[],
            scope: FunctionScope::PixelProcessor,
            position: [0.25 + 1.0 / 512.0, 0.5],
            size: [2.0; 2],
            images: &[Some(image)],
            tile: true,
            variables: &variables,
        };
        let mut graph = FunctionGraph::default();
        let pos = graph.add("get_float2", &[], Some(Value::Text("$pos".into())));
        graph.output = Some(graph.add("samplecol", &[pos], Some(Value::Int2([0, 1]))));
        let program = compile(&graph, 0).unwrap();
        let pixel = |context: &EvaluationContext<'_>| program.evaluate(context, &mut EvaluationState::default()).unwrap();
        let expected = Value::Float4([32768.0, 0.0, 32512.0, 256.0].map(|n| n / 65535.0));
        assert_eq!(pixel(&context), expected);
        context.position[0] -= 1.0;
        assert_eq!(pixel(&context), expected);
        context.tile = false;
        assert_eq!(pixel(&context), expected);
        graph.nodes.last_mut().unwrap().value = Some(Value::Int2([0, 0]));
        let nearest = compile(&graph, 0).unwrap();
        assert_eq!(nearest.evaluate(&context, &mut EvaluationState::default()).unwrap(), Value::Float4([0.0, 0.0, 1.0, 0.0]));
    }

    #[test]
    fn pixel_sampler_uses_the_storage_precision_of_the_selected_slot() {
        use materialcraft_core::OutputFormat;

        let bits = [0x3fd0e000, 0xbfa46000, 0xc02d8000, 0x407f0000];
        let image = Image::gray(2, 2, |x, y| f32::from_bits(bits[(2 * y + x) as usize]));
        let inputs = [Some(image.clone()), Some(image)];
        let variables = BTreeMap::new();
        let context = EvaluationContext {
            scope: FunctionScope::PixelProcessor,
            position: [0.375, 0.4375],
            size: [2.0; 2],
            images: &inputs,
            image_formats: &[OutputFormat::F32, OutputFormat::F16],
            tile: true,
            variables: &variables,
        };
        for (index, expected) in [(0, 0x3e339000), (1, 0x3e33a000)] {
            let mut graph = FunctionGraph::default();
            let pos = graph.add("get_float2", &[], Some(Value::Text("$pos".into())));
            graph.output = Some(graph.add("samplelum", &[pos], Some(Value::Int2([index, 1]))));
            let program = compile(&graph, 0).unwrap();
            assert_eq!(program.evaluate(&context, &mut EvaluationState::default()).unwrap(), Value::Float(f32::from_bits(expected)));
        }
    }
}
