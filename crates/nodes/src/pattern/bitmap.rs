use craft_texture::{Image, Limits};
use materialcraft_core::OutputFormat;
use std::collections::BTreeMap;

use super::coordinates::PixelPlanes;
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

pub(super) struct Sampler<'a> {
    source: &'a Image,
    slot: usize,
    format: OutputFormat,
    filtering: i32,
    fallback_level: usize,
    level: usize,
    image: Image,
    last_quad: Option<[u32; 2]>,
}

impl<'a> Sampler<'a> {
    pub fn new(source: &'a Image, slot: usize, format: OutputFormat, filtering: i32, fallback: [[f32; 2]; 2]) -> Self {
        let fallback_level = if filtering == 0 { mip_level(source, fallback) } else { 0 };
        Self { source, slot, format, filtering, fallback_level, level: 0, image: source.clone(), last_quad: None }
    }

    pub fn start_copy(&mut self) {
        self.last_quad = None;
    }

    pub fn sample(&mut self, local: [f32; 2], pixel: [u32; 2], planes: Option<&PixelPlanes>, mipmaps: &mut Mipmaps) -> Result<[f32; 4], String> {
        let quad = pixel.map(|v| v & !1);
        if self.filtering == 0 && self.last_quad != Some(quad) {
            self.last_quad = Some(quad);
            let level = planes.map_or(self.fallback_level, |p| mip_level(self.source, p.quad_gradients(quad)));
            if level != self.level {
                self.image = mipmaps.image(self.slot, self.source, self.format, level)?;
                self.level = level;
            }
        }
        Ok(sample(&self.image, local, self.format, self.filtering == 2))
    }
}

fn mip_level(image: &Image, gradients: [[f32; 2]; 2]) -> usize {
    let [dx, dy] = gradients.map(|[u, v]| [u * image.width() as f32, v * image.height() as f32].map(|g| f32::from_bits(g.to_bits() & !0x3fff)));
    // The footprint uses nine fraction bits per derivative. The smaller component
    // contributes 11/32 at its own exponent before alignment with the larger one.
    let footprint = |[x, y]: [f32; 2]| {
        let minor = x.abs().min(y.abs()).to_bits();
        let mantissa = 512 + ((minor >> 14) & 511);
        let weighted = (11 * mantissa + 4) >> 5;
        let major = f32::from_bits(x.abs().max(y.abs()).to_bits() & !0x3fff);
        major + f32::from_bits(minor & 0x7f80_0000) * (weighted as f32 / 512.0)
    };
    let add = [dx[0] + dy[0], dx[1] + dy[1]];
    let sub = [dx[0] - dy[0], dx[1] - dy[1]];
    let diagonal = footprint(add).max(footprint(sub)) * (11.0 / 16.0);
    let bits = footprint(dx).max(footprint(dy)).max(diagonal).max(1.0).to_bits();
    // The first measured half-level boundary is 1 + 53/128 and repeats at each power of two.
    let exponent = (bits >> 23).saturating_sub(127);
    (exponent + u32::from(bits & 0x7f_ffff >= 53 << 16)).min(13) as usize
}

fn sample(image: &Image, local: [f32; 2], format: OutputFormat, nearest: bool) -> [f32; 4] {
    let uv = local.map(|axis| axis * 0.5 + 0.5);
    if nearest {
        if uv.iter().all(|v| (0.0..1.0).contains(v)) { image.sample_nearest_axes(uv[0], uv[1], [false; 2]) } else { [0.0; 4] }
    } else {
        texture_sampler::sample(image, uv, [false; 2], format, Some([0.0; 4]))
    }
}
