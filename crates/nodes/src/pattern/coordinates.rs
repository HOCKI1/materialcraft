use super::{footprint::Footprint, region::Region};
use crate::function::{pixel_divide, truncate_float};

/// The two signed coordinate gradients for an axis-aligned analytic stamp.
pub(super) struct PixelPlanes {
    axes: [Axis; 2],
    swap: bool,
}

struct Axis {
    base: f64,
    step: f64,
    negative_bias: f64,
}

impl PixelPlanes {
    pub fn new(frame: &Footprint, region: &Region, center: [f32; 2], size: [f32; 2], dimensions: [u32; 2]) -> Option<Self> {
        let swap = frame.cosine == 0.0;
        let signs = if swap && frame.sine.abs() == 1.0 {
            [-frame.sine * size[1].signum(), frame.sine * size[0].signum()]
        } else if frame.sine == 0.0 && frame.cosine.abs() == 1.0 {
            [frame.cosine * size[0].signum(), frame.cosine * size[1].signum()]
        } else {
            return None;
        };
        let bounds: [[f64; 2]; 2] = std::array::from_fn(|axis| region.pixel_interval(axis, center[axis], frame.radius[axis], dimensions[axis]));
        let extents = bounds.map(|[left, right]| right - left);
        // Smaller stamps and extents too large to resolve half pixels use the ordinary footprint.
        if extents.iter().any(|extent| !(1.0..8_388_608.0).contains(extent)) {
            return None;
        }
        // Snap the vertices of a right triangle spanning twice the stamp's bounding rectangle.
        let near = bounds.map(|[left, _]| (left + 0.5).floor());
        let far: [f64; 2] = std::array::from_fn(|i| (bounds[i][0] + 2.0 * extents[i] + 0.5).floor() + 1.0);
        let padded: [f32; 2] = std::array::from_fn(|i| ((far[i] - near[i]) * 0.5) as f32);
        let reciprocal = area_reciprocal(truncate_float(f64::from(padded[0]) * f64::from(padded[1])));
        let origin = near.map(|n| (n.max(0.0) * 0.25).floor() * 4.0);
        let near_offset: [i32; 2] = std::array::from_fn(|i| (origin[i] - near[i]) as i32);
        let far_offset: [i32; 2] = std::array::from_fn(|i| (origin[i] - far[i]) as i32);
        // The triangle has only one far vertex on each axis. When both offsets fit the same
        // number of bits, the vertical far vertex has priority, then the horizontal one.
        let far_axis = (0..2).rev().find(|&i| far_offset[i].unsigned_abs().leading_zeros() >= near_offset[i].unsigned_abs().leading_zeros());
        let axes = std::array::from_fn(|axis| {
            let basis = truncate_float(f64::from(padded[1 - axis]) * f64::from(reciprocal));
            let center = (bounds[axis][0] + bounds[axis][1]) * 0.5;
            let radius = extents[axis] * 0.5;
            let scale = pixel_divide(2.0, extents[axis] as f32);
            // Store the unsnapped vertices at pixel precision before applying each snap displacement.
            let near_vertex = (center - radius) as f32;
            let far_radius = f64::from((3.0 * radius) as f32);
            let far_vertex = (center + far_radius) as f32;
            let low = (near[axis] - f64::from(near_vertex) - radius) as f32 * scale;
            let high = (far[axis] - f64::from(far_vertex) + far_radius) as f32 * scale;
            let span = truncate_float(f64::from(high) - f64::from(low)) * 0.5;
            let gradient = gradient_product(basis, span);
            let unit = f64::from(gradient.next_up() - gradient);
            let (offset, vertex) = if far_axis == Some(axis) { (far_offset[axis], high) } else { (near_offset[axis], low) };
            let base = origin_coordinate(offset, gradient, unit, vertex);
            let sign = f64::from(signs[axis]);
            let step = f64::from(gradient);
            Axis { base: ((0.5 - origin[axis]) * step + f64::from(base)) * sign, step: step * sign, negative_bias: unit / 64.0 }
        });
        Some(Self { axes, swap })
    }

    pub fn clip(&self, frame: &Footprint, region: &Region, center: [f32; 2], size: [u32; 2], bounds: [[u32; 2]; 2]) -> [[u32; 2]; 2] {
        std::array::from_fn(|axis| {
            let [mut start, mut end] = bounds[axis];
            // A shared physical edge has one owner. A rounded coordinate beyond the stamp is discarded as well.
            let contains =
                |pixel| frame.contains_axis(axis, region.pixel_center(axis, pixel, size[axis]), center[axis]) && self.axes[axis].sample(pixel).abs() <= 1.0;
            while start < end && !contains(start) {
                start += 1;
            }
            while start < end && !contains(end - 1) {
                end -= 1;
            }
            [start, end]
        })
    }

    pub fn row(&self, y: u32) -> impl Fn(u32) -> [f32; 2] + '_ {
        let v = self.axes[1].sample(y);
        move |x| {
            let u = self.axes[0].sample(x);
            if self.swap { [v, u] } else { [u, v] }
        }
    }
}

impl Axis {
    fn sample(&self, pixel: u32) -> f32 {
        let value = f64::from(pixel) * self.step + self.base;
        truncate_float(if value < 0.0 { value + self.negative_bias } else { value })
    }
}

fn origin_coordinate(offset: i32, gradient: f32, unit: f64, vertex: f32) -> f32 {
    let magnitude = offset.unsigned_abs();
    // The integer offset times the 24-bit gradient retains 29 product columns. Each
    // partial row loses its low columns before the remaining rows are added.
    let low_columns = (u32::BITS - magnitude.leading_zeros()).saturating_sub(5);
    let significand = (gradient.to_bits() & 0x7f_ffff) | 0x80_0000;
    let mask = (1_u32 << low_columns) - 1;
    let mut discarded = 0_u64;
    for bit in 0..low_columns {
        if magnitude & (1 << bit) != 0 {
            discarded += u64::from(significand & (mask >> bit)) << bit;
        }
    }
    let product = f64::from(magnitude) * f64::from(gradient) - discarded as f64 * unit;
    truncate_float(f64::from(vertex) + if offset < 0 { -product } else { product })
}

// Interpolate 1/x from its two endpoints and midpoint in each normalized mantissa interval.
// The three samples use quarter units. The squared interpolation fraction retains sixteen bits.
fn area_reciprocal(area: f32) -> f32 {
    let bits = area.to_bits();
    let start = 2 * (128 + i64::from((bits >> 16) & 127));
    let [a, b, c] = [start, start + 1, start + 2].map(|denominator| (1_i64 << 34) / denominator);
    let t = i64::from(bits & 0xffff);
    let square = (t * t) >> 16;
    let value = (a << 16) + (4 * b - 3 * a - c) * t + (2 * a - 4 * b + 2 * c) * square;
    let significand = ((value + (1 << 17)) >> 18) as u32;
    f32::from_bits(((252 - (bits >> 23)) << 23) + significand)
}

fn gradient_product(basis: f32, span: f32) -> f32 {
    // Both inputs are positive normal values. Round the magnitude of each signed radix-four
    // partial product before accumulating it. The low bit of the span is a separate row.
    const FRACTION: u32 = (1 << 23) - 1;
    let significand = |value: f32| i64::from((value.to_bits() & FRACTION) | (1 << 23));
    let a = significand(span);
    let b = significand(basis);
    let rounding_error = |row: i64| {
        let magnitude = row.abs();
        let rounded = (magnitude + (1 << 16)) & !((1 << 17) - 1);
        row.signum() * (rounded - magnitude)
    };
    let mut product = a * b + rounding_error((a & 1) * b);
    let mut carry = 0;
    // Higher rows are already multiples of 2^17 and contribute no rounding error.
    for bit in (1..17).step_by(2) {
        let pair = (a >> bit) & 3;
        let high = pair >> 1;
        let digit = (pair & 1) - 2 * high + carry;
        product += rounding_error((digit * b) << bit);
        carry = high;
    }
    let product = (product + (1 << 20)) as u64;
    let overflow = u32::from(product >= 1 << 47);
    let exponent = (span.to_bits() >> 23) + (basis.to_bits() >> 23) - 127 + overflow;
    f32::from_bits((exponent << 23) | ((product >> (23 + overflow)) as u32 & FRACTION))
}
