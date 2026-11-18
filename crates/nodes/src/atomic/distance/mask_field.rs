use crate::function::pixel_sqrt;
use craft_texture::{Image, Kind};
use rayon::prelude::*;

type Site = [u16; 2];
const EMPTY: Site = [u16::MAX; 2];

struct Axis {
    positions: Vec<f32>,
    tiled: bool,
}

impl Axis {
    fn new(size: u32, tiled: bool) -> Self {
        let extent = size as f32;
        let positions = (0..size)
            .map(|i| {
                let center = (i as f32 + 0.5) / extent;
                (center * f32::from(u16::MAX)).round_ties_even() / f32::from(u16::MAX) * extent
            })
            .collect();
        Self { positions, tiled }
    }

    fn delta(&self, pixel: usize, site: u16) -> f32 {
        let distance = (pixel as f32 + 0.5 - self.positions[usize::from(site)]).abs();
        if self.tiled { distance.min(self.positions.len() as f32 - distance) } else { distance }
    }

    fn neighbors(&self, step: i32) -> Vec<[Option<usize>; 3]> {
        let extent = self.positions.len() as i32;
        (0..extent)
            .map(|i| {
                [i - step, i, i + step].map(|n| if self.tiled { Some(n.rem_euclid(extent) as usize) } else { (0..extent).contains(&n).then_some(n as usize) })
            })
            .collect()
    }
}

struct Geometry {
    columns: Axis,
    rows: Axis,
    metric: i32,
}

impl Geometry {
    fn squared_distance(&self, x: usize, y: usize, site: Site) -> f32 {
        let dx = self.columns.delta(x, site[0]);
        let dy = self.rows.delta(y, site[1]);
        match self.metric {
            1 => (dx + dy) * (dx + dy),
            2 => dx.max(dy) * dx.max(dy),
            _ => dy.mul_add(dy, dx * dx),
        }
    }

    fn propagate(&self, mut sites: Vec<Site>, first_jump: u32) -> Vec<Site> {
        let width = self.columns.positions.len();
        let mut next = vec![EMPTY; sites.len()];
        let jumps = (0..=first_jump.ilog2()).rev().map(|power| 1i32 << power);
        // A final unit step fills missed sites on open fields. It changes measured winners on tiled fields.
        let refine = (!self.columns.tiled && !self.rows.tiled).then_some(1);
        for step in jumps.chain(refine) {
            let xs = self.columns.neighbors(step);
            let ys = self.rows.neighbors(step);
            next.par_chunks_mut(width).enumerate().for_each(|(y, output)| {
                for (x, dest) in output.iter_mut().enumerate() {
                    let mut best = sites[y * width + x];
                    if best != [x as u16, y as u16] {
                        let mut nearest = if best == EMPTY { f32::INFINITY } else { self.squared_distance(x, y, best) };
                        for &sx in xs[x].iter().flatten() {
                            for &sy in ys[y].iter().flatten() {
                                let candidate = sites[sy * width + sx];
                                if candidate != EMPTY && candidate != best {
                                    let d = self.squared_distance(x, y, candidate);
                                    if d < nearest {
                                        nearest = d;
                                        best = candidate;
                                    }
                                }
                            }
                        }
                    }
                    *dest = best;
                }
            });
            std::mem::swap(&mut sites, &mut next);
        }
        sites
    }
}

fn attenuate(mut pixel: [f32; 4], gray: bool, weight: f32) -> [f32; 4] {
    pixel[if gray { 0 } else { 3 }] *= weight;
    pixel
}

pub(super) fn render(mask: &Image, source: Option<&Image>, radius: f32, tile: [bool; 2], metric: i32, combine: bool) -> Result<Image, String> {
    let (width, height) = (mask.width(), mask.height());
    // Reserve the final coordinate pair as the missing-site marker.
    if width == 0 || height == 0 || width > u32::from(u16::MAX) || height > u32::from(u16::MAX) {
        return Err("distance mask dimensions exceed the supported coordinate range".into());
    }
    let empty = !mask.data().iter().any(|v| *v >= 0.5);
    let tiled = tile[0] || tile[1];
    if empty || radius == 0.0 {
        let result = match source {
            None => Image::filled(width, height, Kind::Gray, [0.0; 4]),
            Some(src) if empty && !tiled => Image::filled(width, height, src.kind(), src.get(0, 0, false)),
            Some(src) if combine => Image::generate(width, height, src.kind(), |x, y| attenuate(src.get(x.into(), y.into(), false), src.is_gray(), 0.0)),
            Some(src) => src.clone(),
        };
        return Ok(result);
    }
    if mask.data().iter().all(|v| *v >= 0.5) {
        return Ok(source.cloned().unwrap_or_else(|| Image::filled(width, height, Kind::Gray, [1.0; 4])));
    }
    // With a subpixel radius no site can reach a neighboring texel. This also keeps a
    // tiny positive radius from overflowing its reciprocal at a site of distance zero.
    if radius < 0.5 {
        let weight = |x: u32, y: u32| if mask.get_gray(x.into(), y.into(), false) >= 0.5 { 1.0 } else { 0.0 };
        return Ok(match source {
            None => Image::gray(width, height, weight),
            Some(src) if combine => {
                Image::generate(width, height, src.kind(), |x, y| attenuate(src.get(x.into(), y.into(), false), src.is_gray(), weight(x, y)))
            }
            Some(src) => src.clone(),
        });
    }

    let w = width as usize;
    let initial = mask.data().iter().enumerate().map(|(i, &v)| if v >= 0.5 { [(i % w) as u16, (i / w) as u16] } else { EMPTY }).collect();
    let geometry = Geometry { columns: Axis::new(width, tile[0]), rows: Axis::new(height, tile[1]), metric };
    let first_jump = (radius.ceil() as u32).clamp(1, width.max(height)).next_power_of_two();
    let sites = geometry.propagate(initial, first_jump);
    let inverse_radius = radius.recip();
    let site_distance = |x: u32, y: u32| {
        let site = sites[y as usize * w + x as usize];
        let distance = if site == [x as u16, y as u16] {
            0.0
        } else if site == EMPTY {
            f32::INFINITY
        } else {
            pixel_sqrt(geometry.squared_distance(x as usize, y as usize, site))
        };
        (site, distance)
    };
    let mask_weight = |distance: f32| (-distance).mul_add(inverse_radius, 1.0).clamp(0.0, 1.0);
    let Some(source) = source else {
        return Ok(Image::gray(width, height, |x, y| mask_weight(site_distance(x, y).1)));
    };
    Ok(Image::generate(width, height, source.kind(), |x, y| {
        let (site, distance) = site_distance(x, y);
        let pixel = if distance <= radius { source.get(site[0].into(), site[1].into(), false) } else { source.get(x.into(), y.into(), false) };
        if !combine {
            return pixel;
        }
        // The tiled path stores the complement after rounding the mask gradient.
        let weight = if tiled { 1.0 - (1.0 - mask_weight(distance)) } else { (1.0 - distance * inverse_radius).clamp(0.0, 1.0) };
        attenuate(pixel, source.is_gray(), weight)
    }))
}
