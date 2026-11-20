use super::{pattern_stamp::PatternStamp, region::Region};
use crate::CookContext;

pub(super) struct Footprint {
    pub sine: f32,
    pub cosine: f32,
    pub radius: [f32; 2],
}

impl Footprint {
    pub fn new(stamp: &PatternStamp) -> Result<Self, String> {
        let angle = stamp.rotation * std::f32::consts::TAU;
        if !angle.is_finite() {
            return Err("pattern rotation exceeds the finite angle range".into());
        }
        // Extended quarter turns can retain a cross-axis component at the stored phase.
        let (sine, cosine) = crate::function::pixel_sin_cos(angle);
        let radius = [
            (stamp.size[0] * cosine).abs() * 0.5 + (stamp.size[1] * sine).abs() * 0.5,
            (stamp.size[0] * sine).abs() * 0.5 + (stamp.size[1] * cosine).abs() * 0.5,
        ];
        Ok(Self { sine, cosine, radius })
    }

    pub fn local(&self, point: [f32; 2], center: [f32; 2], size: [f32; 2]) -> Option<[f32; 2]> {
        let delta = [point[0] - center[0], point[1] - center[1]];
        let local = [(delta[0] * self.cosine + delta[1] * self.sine) * 2.0 / size[0], (-delta[0] * self.sine + delta[1] * self.cosine) * 2.0 / size[1]];
        // For axis-aligned stamps the preceding interval owns a sample on the shared edge.
        let outside = if self.sine == 0.0 || self.cosine == 0.0 {
            // Subtract the radius from the center before testing the open edge at pixel precision.
            (0..2).any(|axis| point[axis] <= center[axis] - self.radius[axis] || delta[axis] > self.radius[axis])
        } else {
            local.iter().any(|v| *v <= -1.0 || *v > 1.0)
        };
        (!outside).then_some(local)
    }
}

pub(super) struct SampleGrid {
    region: Region,
    dimensions: [u32; 2],
    axes: [bool; 2],
}

impl SampleGrid {
    pub fn new(context: &CookContext, graph_nodes: usize) -> Option<Self> {
        let unit = [0.0, 1.0, 0.0, 1.0];
        let roughness = context.f("roughness");
        // Retain ordinary raster validation for nonunit regions and depth weights that might overflow.
        if context.v4("render_region") != unit
            || context.v4("tiling_region") != unit
            || !(roughness >= 0.0 && roughness.is_finite() && context.f("opacity").abs() <= 1.0)
            || (roughness > 1.0 && !roughness.powf(graph_nodes as f32).is_finite())
        {
            return None;
        }
        let (width, height) = craft_texture::clamp_size(context.w, context.h);
        Some(Self { region: Region::new(unit).ok()?, dimensions: [width, height], axes: context.tiling_axes() })
    }

    pub fn misses(&self, stamp: &PatternStamp) -> bool {
        if !(2..=18).contains(&stamp.pattern)
            || !(0..=2).contains(&stamp.blend)
            || !stamp.center.iter().chain(&stamp.size).chain(&stamp.color).chain([&stamp.variation]).all(|v| v.is_finite())
        {
            return false;
        }
        let Ok(frame) = Footprint::new(stamp) else { return false };
        // Limit this check to footprints spanning at most two sample columns and rows per copy.
        if frame.radius.iter().zip(self.dimensions).any(|(&r, n)| r * n as f32 > 0.5) {
            return false;
        }
        let center = std::array::from_fn(|i| if self.axes[i] { stamp.center[i].rem_euclid(1.0) } else { stamp.center[i] });
        let Ok(repeats) = self.region.repeats(&self.region, center, frame.radius, self.axes) else { return false };
        for row in repeats[1][0]..=repeats[1][1] {
            for column in repeats[0][0]..=repeats[0][1] {
                let center = [center[0] + column as f32, center[1] + row as f32];
                let [[left, right], [top, bottom]] = self.region.bounds(center, frame.radius, self.dimensions);
                for y in top..bottom {
                    for x in left..right {
                        if frame.local(self.region.point([x, y], self.dimensions), center, stamp.size).is_some() {
                            return false;
                        }
                    }
                }
            }
        }
        true
    }
}
