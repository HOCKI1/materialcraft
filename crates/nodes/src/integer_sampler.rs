use craft_texture::Image;

/// Bilinear reads from normalized integer textures with independent wrap axes.
pub(crate) fn sample(image: &Image, [u, v]: [f32; 2], tiling: [bool; 2]) -> [f32; 4] {
    if !u.is_finite() || !v.is_finite() {
        return [0.0, 0.0, 0.0, 1.0];
    }
    let (x, dx) = axis(u, image.width(), tiling[0]);
    let (y, dy) = axis(v, image.height(), tiling[1]);
    // Round the diagonal coefficient first. The four tap weights must still sum to 256.
    let diagonal = (dx * dy + 128) / 256;
    let bottom = dy - diagonal;
    let taps = [(x, y, 256 - dx - bottom), (x + 1, y, dx - diagonal), (x, y + 1, bottom), (x + 1, y + 1, diagonal)];
    let mut sum = [0.0; 4];
    for (x, y, weight) in taps {
        if weight != 0 {
            for (total, channel) in sum.iter_mut().zip(image.get_axes(x, y, tiling)) {
                // Stored 16-bit levels times the eight-bit weights sum exactly in f32.
                *total += (channel.clamp(0.0, 1.0) * 65535.0).round() * weight as f32;
            }
        }
    }
    sum.map(|value| (value / 256.0).round() / 65535.0)
}

fn axis(uv: f32, size: u32, tile: bool) -> (i64, u32) {
    // Reduce before converting to a texel so even extreme finite coordinates have bounded neighbors.
    let uv = if tile { uv.rem_euclid(1.0) } else { uv.clamp(0.0, 1.0) };
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
        assert_eq!(pixel(&context), Value::Float4([32768.0 / 65535.0, 0.0, 32768.0 / 65535.0, 0.0]));
    }
}
