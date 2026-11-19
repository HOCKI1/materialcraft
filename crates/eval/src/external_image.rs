use craft_texture::{Image, Kind, Limits, MAX_SIDE};
use materialcraft_core::OutputFormat;

/// Non-power-of-two external rasters filter each axis directly, storing the result between passes.
pub(super) fn resize(image: &Image, size: (u32, u32), kind: Kind, source: OutputFormat, output: OutputFormat) -> Option<Image> {
    let size = craft_texture::clamp_size(size.0, size.1);
    let (width, height) = (image.width(), image.height());
    if (width.is_power_of_two() && height.is_power_of_two()) || (width, height) == size || width.max(height) > MAX_SIDE || image.kind() != kind {
        return None;
    }
    let input_levels: Option<f32> = match (image.integer_sampling(), source) {
        (true, OutputFormat::U8) => Some(255.0),
        (true, OutputFormat::U16) => Some(65535.0),
        (false, OutputFormat::F32) => None,
        _ => return None,
    };
    let output_levels = match output {
        OutputFormat::U8 => Some(255.0),
        OutputFormat::Inherit | OutputFormat::U16 => Some(65535.0),
        OutputFormat::F32 => None,
        OutputFormat::F16 => return None,
    };
    // Keep the lower channel depth through both filter passes. A byte raster sent to
    // a word or float output still filters at byte precision.
    let maximum = match (input_levels, output_levels) {
        (Some(source), Some(output)) => Some(source.min(output)),
        (source, output) => source.or(output),
    };
    let mut filtered = match (input_levels, maximum) {
        (Some(source), Some(working)) if source == working => image.map(kind, |pixel| pixel.map(|v| (v * source).round())),
        (_, Some(working)) => dither(image, working, input_levels.is_some())?,
        _ => image.clone(),
    };
    let (w, h) = size;
    let passes = if w <= width { [(true, w), (false, h)] } else { [(false, h), (true, w)] };
    for (horizontal, end) in passes {
        filtered = filter_axis(filtered, horizontal, end, maximum.is_some());
    }
    Some(if let Some(maximum) = maximum { filtered.map(kind, |p| p.map(|v| v / maximum)).with_integer_sampling(output_levels.is_some()) } else { filtered })
}

fn filter_axis(image: Image, horizontal: bool, end: u32, integer: bool) -> Image {
    let start = if horizontal { image.width() } else { image.height() };
    if start == end {
        return image;
    }
    let scale = f64::from(end) / f64::from(start);
    let radius = 1.0 / scale.min(1.0);
    let taps: Vec<_> = (0..end)
        .map(|index| {
            let center = (f64::from(index) + 0.5) / scale - 0.5;
            let first = (center - radius).ceil().max(0.0) as u32;
            let last = ((center + radius).floor() as u32).min(start - 1);
            let mut weights: Vec<_> = (first..=last).map(|i| (1.0 - (f64::from(i) - center).abs() / radius).max(0.0)).collect();
            let sum: f64 = weights.iter().sum();
            for w in &mut weights {
                *w /= sum;
            }
            (first, weights)
        })
        .collect();
    let (w, h) = if horizontal { (end, image.height()) } else { (image.width(), end) };
    Image::generate(w, h, image.kind(), |x, y| {
        let index = if horizontal { x } else { y };
        let (first, weights) = &taps[index as usize];
        let mut sum = [0.0_f64; 4];
        for (offset, &weight) in weights.iter().enumerate() {
            if weight == 0.0 {
                continue;
            }
            let pos = i64::from(*first) + offset as i64;
            let pixel = if horizontal { image.get(pos, i64::from(y), false) } else { image.get(i64::from(x), pos, false) };
            for (value, channel) in sum.iter_mut().zip(pixel) {
                *value += f64::from(channel) * weight;
            }
        }
        sum.map(|v| if integer { v.round() as f32 } else { v as f32 })
    })
}

fn dither(image: &Image, maximum: f32, word_to_byte: bool) -> Option<Image> {
    let width = usize::try_from(image.width()).ok()?;
    let channels = image.channels();
    let mut samples = Vec::new();
    samples.try_reserve_exact(image.data().len()).ok()?;
    samples.extend_from_slice(image.data());
    let mut errors: [_; 3] = std::array::from_fn(|_| vec![[0.0_f32; 4]; width + 4]);
    let mut state = 1_u64;
    let unit = maximum.recip();
    let limit = f32::MAX / (4.0 * maximum);
    for (y, row) in samples.chunks_exact_mut(width * channels).enumerate() {
        let current = y % 3;
        for (x, pixel) in row.chunks_exact_mut(channels).enumerate() {
            let mut error = [0.0; 4];
            for (c, value) in pixel.iter_mut().enumerate() {
                state = (48_271 * state) % 2_147_483_647;
                let random = (state as f64 / 2_147_483_647.0) as f32;
                let original = if value.is_finite() { value.clamp(-limit, limit) } else { f32::from(*value > 0.0) };
                let received = errors[current][x + 2][c];
                let stored = if word_to_byte {
                    // One full byte level expands to 257 word levels. The noise spans one high-byte step.
                    let word = (original * 65535.0).round();
                    ((word + received * 257.0 + (random - 0.5) * 256.0) / 256.0).floor()
                } else {
                    let noise = if maximum == 255.0 { (random * maximum).round() / maximum } else { random };
                    ((original + (received + (noise - 0.5)) * unit) * maximum).round()
                }
                .clamp(0.0, maximum);
                error[c] = original * maximum - stored;
                *value = stored;
            }
            // Two extra columns on each end absorb error outside the image. Propagate the
            // original quantization error once, with the three-row Sierra filter.
            for (dy, column, weight) in [
                (0, x + 3, 5),
                (0, x + 4, 3),
                (1, x, 2),
                (1, x + 1, 4),
                (1, x + 2, 5),
                (1, x + 3, 4),
                (1, x + 4, 2),
                (2, x + 1, 2),
                (2, x + 2, 3),
                (2, x + 3, 2),
            ] {
                let dst = &mut errors[(current + dy) % 3][column];
                for (v, e) in dst.iter_mut().zip(error).take(channels) {
                    *v += e * (weight as f32 / 32.0);
                }
            }
        }
        errors[current].fill([0.0; 4]);
    }
    Image::from_data(image.width(), image.height(), image.kind(), samples, &Limits::default()).ok()
}
