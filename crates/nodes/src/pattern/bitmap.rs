use craft_texture::{Image, Limits};
use materialcraft_core::OutputFormat;
use std::collections::BTreeMap;

use crate::texture_sampler;

pub(super) struct Mipmaps {
    levels: BTreeMap<usize, Vec<Image>>,
    remaining_bytes: u64,
}

impl Mipmaps {
    pub fn new() -> Self {
        Self { levels: BTreeMap::new(), remaining_bytes: Limits::default().max_bytes }
    }

    pub fn image(&mut self, slot: usize, source: &Image, format: OutputFormat, level: usize) -> Result<Image, String> {
        if level == 0 {
            return Ok(source.clone());
        }
        let levels = self.levels.entry(slot).or_default();
        while levels.len() < level {
            let previous = levels.last().unwrap_or(source);
            if previous.width() == 1 && previous.height() == 1 {
                return Ok(previous.clone());
            }
            let texels = u64::from(previous.width().div_ceil(2)) * u64::from(previous.height().div_ceil(2));
            let bytes = texels * previous.channels() as u64 * size_of::<f32>() as u64;
            // Stamps share cached levels. Bound their combined storage across distinct image slots.
            self.remaining_bytes = self.remaining_bytes.checked_sub(bytes).ok_or("pattern mipmaps exceeded their memory budget")?;
            levels.push(texture_sampler::reduce_level(previous, format, format));
        }
        levels.get(level - 1).cloned().ok_or_else(|| "missing pattern mipmap".into())
    }
}

pub(super) fn mip_level(image: &Image, size: [f32; 2], pixel: [f32; 2], sine: f32, cosine: f32) -> usize {
    let u = image.width() as f32 / size[0];
    let v = image.height() as f32 / size[1];
    let dx = [cosine * u * pixel[0], -sine * v * pixel[0]];
    let dy = [sine * u * pixel[1], cosine * v * pixel[1]];
    // Estimate the source footprint along both screen axes and both diagonals.
    // Thin rotated stamps can cross a mip boundary on the diagonal first.
    let footprint = |x: f32, y: f32| x.abs().max(y.abs()) + x.abs().min(y.abs()) / 3.0;
    let axes = footprint(dx[0], dx[1]).max(footprint(dy[0], dy[1]));
    let diagonals = footprint(dx[0] + dy[0], dx[1] + dy[1]).max(footprint(dx[0] - dy[0], dx[1] - dy[1]));
    axes.max(diagonals * (11.0 / 16.0)).max(1.0).log2().round().clamp(0.0, 13.0) as usize
}

pub(super) fn sample(image: &Image, local: [f32; 2], format: OutputFormat, nearest: bool) -> [f32; 4] {
    let uv = local.map(|axis| axis * 0.5 + 0.5);
    if nearest {
        if uv.iter().all(|v| (0.0..1.0).contains(v)) { image.sample_nearest_axes(uv[0], uv[1], [false; 2]) } else { [0.0; 4] }
    } else {
        texture_sampler::sample(image, uv, [false; 2], format, Some([0.0; 4]))
    }
}
