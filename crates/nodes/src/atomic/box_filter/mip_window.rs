use crate::{output_precision, texture_sampler};
use craft_texture::Image;
use materialcraft_core::OutputFormat;

enum Read {
    Texel { level: usize, position: i64 },
    Filtered(f32),
}

struct Tap {
    read: Read,
    weight: f32,
}

impl Tap {
    fn texel(position: i64, width: u32, weight: f32) -> Self {
        Self { read: Read::Texel { level: width.ilog2() as usize, position: position.div_euclid(i64::from(width)) }, weight }
    }
}

fn window(position: u32, length: u32, radius: f32, tile: bool) -> Vec<Tap> {
    let extent = (radius - 0.5).ceil() as i64;
    let fraction = radius - (extent as f32 - 0.5);
    let pair_weight = 1.0 + fraction;
    let center = (position as f32 + 0.5) / length as f32;
    let filtered = |offset| Tap { read: Read::Filtered(center + offset / length as f32), weight: pair_weight };
    let mut left = i64::from(position) - extent;
    let mut right = i64::from(position) + extent;
    let mut taps = Vec::new();
    if !tile && left < 0 {
        // Clamped coverage contributes one weighted read of the original border texel.
        taps.push(Tap::texel(0, 1, fraction + (-left - 1) as f32));
        left = 0;
    } else if left % 2 == 0 {
        taps.push(filtered(-extent as f32 + 1.0 / pair_weight));
        left += 2;
    } else {
        taps.push(Tap::texel(left, 1, fraction));
        left += 1;
    }
    let last = if !tile && right >= i64::from(length) {
        let weight = fraction + (right - i64::from(length)) as f32;
        right = i64::from(length) - 1;
        Tap::texel(right, 1, weight)
    } else if right % 2 != 0 {
        right -= 2;
        filtered((extent - 1) as f32 + fraction / pair_weight)
    } else {
        right -= 1;
        Tap::texel(right + 1, 1, fraction)
    };
    while left <= right {
        // An aligned run of 2^k texels is a single stored texel at level k.
        let available = (right - left + 1).min(i64::from(length)) as u32;
        let width = 1 << available.ilog2().min(left.trailing_zeros());
        taps.push(Tap::texel(left, width, width as f32));
        left += i64::from(width);
    }
    taps.push(last);
    taps
}

pub(super) fn filter(input: &Image, radius: f32, horizontal: bool, tile: bool, source: OutputFormat, destination: OutputFormat) -> Image {
    let length = if horizontal { input.width() } else { input.height() };
    let mut levels = vec![input.clone()];
    let mut format = source;
    let mut size = length;
    let max_width = ((radius * 2.0).ceil() as u32).min(length);
    for _ in 0..max_width.ilog2() {
        let Some(previous) = levels.last() else { break };
        size /= 2;
        let (w, h) = if horizontal { (size, input.height()) } else { (input.width(), size) };
        let reduced = texture_sampler::resize_input_level(previous, w, h, format, [true; 2]);
        levels.push(output_precision::apply(reduced, destination));
        format = destination;
    }
    let windows: Vec<_> = (0..length)
        .map(|position| {
            let mut taps = window(position, length, radius, tile);
            if horizontal {
                taps.reverse();
            }
            taps
        })
        .collect();
    let scale = 0.5 / radius;
    Image::generate(input.width(), input.height(), input.kind(), |x, y| {
        let (position, across, other_size) = if horizontal { (x, y, input.height()) } else { (y, x, input.width()) };
        let other_uv = (across as f32 + 0.5) / other_size as f32;
        let read = |tap: &Tap| match tap.read {
            Read::Texel { level, position } => {
                let (x, y) = if horizontal { (position, i64::from(across)) } else { (i64::from(across), position) };
                levels.get(level).unwrap_or(input).get(x, y, true)
            }
            Read::Filtered(uv) => {
                let xy = if horizontal { [uv, other_uv] } else { [other_uv, uv] };
                texture_sampler::sample(input, xy, [tile; 2], source, None)
            }
        };
        let Some((first, rest)) = windows.get(position as usize).and_then(|taps| taps.split_first()) else {
            return input.get(i64::from(x), i64::from(y), false);
        };
        let mut sum = read(first).map(|v| v * first.weight);
        for tap in rest {
            for (total, v) in sum.iter_mut().zip(read(tap)) {
                *total = v.mul_add(tap.weight, *total);
            }
        }
        sum.map(|v| v * scale)
    })
}
