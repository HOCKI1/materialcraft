use crate::{function::pixel_sqrt, ops::gray_from};
use craft_texture::Image;
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

pub(super) fn render(mask: &Image, radius: f32, tile: [bool; 2]) -> Result<Image, String> {
    let (width, height) = (mask.width(), mask.height());
    // Reserve the final coordinate pair as the missing-site marker.
    if width == 0 || height == 0 || width > u32::from(u16::MAX) || height > u32::from(u16::MAX) {
        return Err("distance mask dimensions exceed the supported coordinate range".into());
    }
    let w = width as usize;
    let mut sites: Vec<Site> = mask.data().iter().enumerate().map(|(i, &v)| if v >= 0.5 { [(i % w) as u16, (i / w) as u16] } else { EMPTY }).collect();
    if radius == 0.0 || sites.iter().all(|s| *s == EMPTY) || sites.iter().all(|s| *s != EMPTY) {
        return gray_from(width, height, sites.into_iter().map(|s| if s == EMPTY { 0.0 } else { 1.0 }).collect());
    }
    let columns = Axis::new(width, tile[0]);
    let rows = Axis::new(height, tile[1]);
    let squared_distance = |x, y, site: Site| {
        let dx = columns.delta(x, site[0]);
        let dy = rows.delta(y, site[1]);
        dy.mul_add(dy, dx * dx)
    };

    // At each jump every pixel reads the previous pass. An extra unit jump changes the winners
    // on sparse masks where a nearer site was never propagated into this pixel's neighborhood.
    let largest = (radius as u32).min(width.max(height)).max(1);
    let mut step = 1 << largest.ilog2();
    let mut next = vec![EMPTY; sites.len()];
    while step > 0 {
        let xs = columns.neighbors(step);
        let ys = rows.neighbors(step);
        next.par_chunks_mut(w).enumerate().for_each(|(y, output)| {
            for (x, dest) in output.iter_mut().enumerate() {
                let mut best = sites[y * w + x];
                if best != [x as u16, y as u16] {
                    let mut nearest = if best == EMPTY { f32::INFINITY } else { squared_distance(x, y, best) };
                    for &sy in ys[y].iter().flatten() {
                        for &sx in xs[x].iter().flatten() {
                            let candidate = sites[sy * w + sx];
                            if candidate != EMPTY && candidate != best {
                                let d = squared_distance(x, y, candidate);
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
        step >>= 1;
    }
    drop(next);

    let inverse_radius = radius.recip();
    let values = sites
        .into_par_iter()
        .enumerate()
        .map(|(i, site)| {
            let (x, y) = (i % w, i / w);
            if site == [x as u16, y as u16] {
                1.0
            } else if site == EMPTY || radius <= 0.0 {
                0.0
            } else {
                (-pixel_sqrt(squared_distance(x, y, site))).mul_add(inverse_radius, 1.0).clamp(0.0, 1.0)
            }
        })
        .collect();
    gray_from(width, height, values)
}
