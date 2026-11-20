use super::{footprint::Footprint, region::Region};
use crate::function::truncate_float;

/// The two signed coordinate gradients for a stamp whose edges land on whole pixels.
pub(super) struct PixelPlanes {
    axes: [Axis; 2],
    swap: bool,
}

struct Axis {
    base: f64,
    step: f64,
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
        let padded = extents.map(|extent| (extent + 0.5) as f32);
        // Fractional edges and extents too large to retain the half pixel use the ordinary footprint calculation.
        if bounds.iter().flatten().any(|edge| edge.fract() != 0.0)
            || extents.iter().zip(padded).any(|(&extent, pad)| !(1.0..8_388_608.0).contains(&extent) || f64::from(pad) != extent + 0.5)
        {
            return None;
        }
        let reciprocal = area_reciprocal(truncate_float(f64::from(padded[0]) * f64::from(padded[1])));
        let axes = std::array::from_fn(|axis| {
            let basis = truncate_float(f64::from(padded[1 - axis]) * f64::from(reciprocal));
            let span = (2.0 + extents[axis].recip()) as f32;
            let step = f64::from(gradient_product(basis, span));
            let left = bounds[axis][0];
            let origin = (left.max(0.0) * 0.25).floor() * 4.0;
            let base = truncate_float((origin - left).mul_add(step, -1.0));
            let sign = f64::from(signs[axis]);
            Axis { base: ((0.5 - origin) * step + f64::from(base)) * sign, step: step * sign }
        });
        Some(Self { axes, swap })
    }

    pub fn sample(&self, x: u32, y: u32) -> [f32; 2] {
        let mut p = [self.axes[0].sample(x), self.axes[1].sample(y)];
        if self.swap {
            p.swap(0, 1);
        }
        p
    }
}

impl Axis {
    fn sample(&self, pixel: u32) -> f32 {
        let value = f64::from(pixel).mul_add(self.step, self.base);
        let stored = truncate_float(value);
        if value < 0.0 && f64::from(stored) == value { stored.next_up() } else { stored }
    }
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
