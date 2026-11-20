use super::{coordinates::PixelPlanes, pattern_stamp::PatternStamp, region::Region};
use crate::CookContext;

pub(super) struct Footprint {
    pub sine: f32,
    pub cosine: f32,
    pub radius: [f32; 2],
}

impl Footprint {
    pub fn new(stamp: &PatternStamp) -> Result<Self, String> {
        if stamp.rotation == 0.0 {
            return Ok(Self { sine: stamp.rotation, cosine: 1.0, radius: stamp.size.map(|s| s.abs() * 0.5) });
        }
        let angle = stamp.rotation * std::f32::consts::TAU;
        if !angle.is_finite() {
            return Err("pattern rotation exceeds the finite angle range".into());
        }
        // Extended quarter turns can retain a cross-axis component at the stored phase.
        let (sine, cosine) = crate::function::pixel_sin_cos(angle);
        // Exact quarter turns have axis-aligned draw bounds even when the sampling phase retains a residual.
        let (bounds_sine, bounds_cosine) = match stamp.rotation.fract().abs() {
            0.0 | 0.5 => (0.0, 1.0),
            0.25 | 0.75 => (1.0, 0.0),
            _ => (sine, cosine),
        };
        let radius = [
            (stamp.size[0] * bounds_cosine).abs() * 0.5 + (stamp.size[1] * bounds_sine).abs() * 0.5,
            (stamp.size[0] * bounds_sine).abs() * 0.5 + (stamp.size[1] * bounds_cosine).abs() * 0.5,
        ];
        Ok(Self { sine, cosine, radius })
    }

    pub fn local(&self, point: [f32; 2], center: [f32; 2], size: [f32; 2]) -> Option<[f32; 2]> {
        let delta = [point[0] - center[0], point[1] - center[1]];
        let local = [(delta[0] * self.cosine + delta[1] * self.sine) * 2.0 / size[0], (-delta[0] * self.sine + delta[1] * self.cosine) * 2.0 / size[1]];
        // For axis-aligned stamps the preceding interval owns a sample on the shared edge.
        let outside = if self.sine == 0.0 || self.cosine == 0.0 {
            // Subtract the radius from the center before testing the open edge at pixel precision.
            (0..2).any(|axis| !self.contains_axis(axis, point[axis], center[axis]))
        } else {
            local.iter().any(|v| *v <= -1.0 || *v > 1.0)
        };
        (!outside).then_some(local)
    }

    pub(super) fn contains_axis(&self, axis: usize, point: f32, center: f32) -> bool {
        point > center - self.radius[axis] && point - center <= self.radius[axis]
    }

    pub(super) fn is_axis_aligned(&self) -> bool {
        (self.sine == 0.0 && self.cosine.abs() == 1.0) || (self.cosine == 0.0 && self.sine.abs() == 1.0)
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
        let center = self.region.copy_origin(stamp.center, self.axes);
        let Ok(repeats) = self.region.repeats(&self.region, center, frame.radius, self.axes) else { return false };
        let columns = repeats[0].centers();
        for y_center in repeats[1].centers() {
            for x_center in columns.clone() {
                let center = [x_center, y_center];
                let bounds = self.region.bounds(center, frame.radius, self.dimensions);
                let [[left, right], [top, bottom]] = bounds;
                if left == right || top == bottom {
                    continue;
                }
                if frame.is_axis_aligned() {
                    // Skip the copy when even the final sample precedes the snapped triangle
                    // and lies outside the ordinary footprint used for extreme vertex spans.
                    if (0..2).any(|axis| {
                        let low = center[axis] - frame.radius[axis];
                        let end = bounds[axis][1];
                        let pixels = self.dimensions[axis];
                        f64::from(low * pixels as f32) + 0.5 >= f64::from(end) && self.region.pixel_center(axis, end - 1, pixels) <= low
                    }) {
                        continue;
                    }
                    if let Some(planes) = PixelPlanes::new(&frame, &self.region, center, stamp.size, self.dimensions) {
                        let [[x0, x1], [y0, y1]] = planes.clip(bounds);
                        if x0 < x1 && y0 < y1 {
                            return false;
                        }
                        continue;
                    }
                }
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
