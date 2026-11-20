use super::{footprint::Footprint, region::Region};
use crate::function::{pixel_divide, truncate_float};

mod oblique;

/// Interpolated signed coordinates of a stamp's padded triangle.
pub(super) struct PixelPlanes {
    first_pixel: [u32; 2],
    mapping: Mapping,
}

enum Mapping {
    Aligned { axes: [Axis; 2], swap: bool },
    Oblique { planes: [oblique::Plane; 2], origin: [f64; 2] },
}

struct Axis {
    base: f64,
    step: f64,
    negative_bias: f64,
}

struct Triangle {
    near: [f64; 2],
    far: [f64; 2],
    origin: [f64; 2],
    basis: [f32; 2],
    offset: [i32; 2],
    far_axis: Option<usize>,
}

impl PixelPlanes {
    pub fn new(frame: &Footprint, region: &Region, center: [f32; 2], size: [f32; 2], dimensions: [u32; 2]) -> Option<Self> {
        let pixel_size: [f32; 2] = std::array::from_fn(|i| region.period()[i] / dimensions[i] as f32);
        let projected: [_; 2] = std::array::from_fn(|i| region.stamp_vertices(i, center[i], frame.radius[i], dimensions[i]));
        // Even a subpixel stamp interpolates its signed coordinates. Extents too large
        // to resolve half pixels use the ordinary footprint.
        if (0..2).any(|i| {
            !(0.0..8_388_608.0).contains(&(2.0 * f64::from(frame.radius[i]) / f64::from(pixel_size[i]))) || projected[i].iter().any(|v| !v.is_finite())
        }) {
            return None;
        }
        let triangle = Triangle::new(projected);
        // Move each stored vertex by its snapping displacement in pattern space.
        let displacements: [[f32; 2]; 2] = std::array::from_fn(|axis| {
            let low_delta = (triangle.near[axis] - f64::from(projected[axis][0])) as f32 * pixel_size[axis];
            let high_delta = (triangle.far[axis] - f64::from(projected[axis][1])) as f32 * pixel_size[axis];
            [low_delta - frame.radius[axis], high_delta + 3.0 * frame.radius[axis]]
        });
        let first_pixel = triangle.near.map(|n| n as u32);
        let swap = frame.cosine == 0.0;
        let signs = if swap && frame.sine.abs() == 1.0 {
            [-frame.sine * size[1].signum(), frame.sine * size[0].signum()]
        } else if frame.sine == 0.0 && frame.cosine.abs() == 1.0 {
            [frame.cosine * size[0].signum(), frame.cosine * size[1].signum()]
        } else {
            let planes = oblique::planes(&triangle, frame, size, displacements)?;
            return Some(Self { first_pixel, mapping: Mapping::Oblique { planes, origin: triangle.origin } });
        };
        let [Some(horizontal), Some(vertical)] = std::array::from_fn(|axis| {
            let scale = pixel_divide(2.0, size[if swap { 1 - axis } else { axis }].abs());
            let [low, high] = displacements[axis].map(|v| v * scale);
            let span = truncate_float(f64::from(high) - f64::from(low)) * 0.5;
            let basis = triangle.basis[axis];
            let exponents = (basis.to_bits() >> 23) + (span.to_bits() >> 23);
            // Extremely small stamps in large render windows can overflow the vertex span.
            if !span.is_normal() || span <= 0.0 || !basis.is_normal() || !(128..=380).contains(&exponents) {
                return None;
            }
            let gradient = gradient_product(basis, span);
            let unit = f64::from(gradient.next_up() - gradient);
            let vertex = if triangle.far_axis == Some(axis) { high } else { low };
            let base = truncate_float(f64::from(vertex) + offset_product(triangle.offset[axis], gradient, unit));
            let sign = f64::from(signs[axis]);
            let step = f64::from(gradient);
            Some(Axis { base: ((0.5 - triangle.origin[axis]) * step + f64::from(base)) * sign, step: step * sign, negative_bias: unit / 64.0 })
        }) else {
            return None;
        };
        Some(Self { first_pixel, mapping: Mapping::Aligned { axes: [horizontal, vertical], swap } })
    }

    pub fn clip(&self, bounds: [[u32; 2]; 2]) -> [[u32; 2]; 2] {
        std::array::from_fn(|axis| {
            let [mut start, mut end] = bounds[axis];
            // The triangle starts at the snapped vertex. Its interpolated coordinates include both stamp edges.
            start = start.max(self.first_pixel[axis]).min(end);
            let Mapping::Aligned { axes, .. } = &self.mapping else { return [start, end] };
            let contains = |pixel| axes[axis].sample(pixel).abs() <= 1.0;
            while start < end && !contains(start) {
                start += 1;
            }
            while start < end && !contains(end - 1) {
                end -= 1;
            }
            [start, end]
        })
    }

    pub fn covers_any(&self, bounds: [[u32; 2]; 2]) -> bool {
        let [[left, right], [top, bottom]] = self.clip(bounds);
        if left >= right || top >= bottom {
            return false;
        }
        if matches!(self.mapping, Mapping::Aligned { .. }) {
            return true;
        }
        (top..bottom).any(|y| {
            let row = self.row(y);
            (left..right).any(|x| row(x).is_some())
        })
    }

    pub fn row(&self, y: u32) -> impl Fn(u32) -> Option<[f32; 2]> + '_ {
        let vertical = match &self.mapping {
            Mapping::Aligned { axes, .. } => [f64::from(axes[1].sample(y)), 0.0],
            Mapping::Oblique { planes, origin } => planes.each_ref().map(|plane| plane.y_term(y, f64::from(y) + 0.5 - origin[1])),
        };
        move |x| match &self.mapping {
            Mapping::Aligned { axes, swap } => {
                let u = axes[0].sample(x);
                let v = vertical[0] as f32;
                Some(if *swap { [v, u] } else { [u, v] })
            }
            Mapping::Oblique { planes, origin } => {
                let dx = f64::from(x) + 0.5 - origin[0];
                let uv: [f32; 2] = std::array::from_fn(|i| planes[i].sample(x, dx, vertical[i]));
                uv.iter().all(|v| v.abs() <= 1.0).then_some(uv)
            }
        }
    }
}

impl Triangle {
    fn new(projected: [[f32; 2]; 2]) -> Self {
        let near = projected.map(|[v, _]| (f64::from(v) + 0.5).floor());
        let far = projected.map(|[_, v]| (f64::from(v) + 0.5).floor() + 1.0);
        let padded: [f32; 2] = std::array::from_fn(|i| ((far[i] - near[i]) * 0.5) as f32);
        let reciprocal = area_reciprocal(truncate_float(f64::from(padded[0]) * f64::from(padded[1])));
        let basis = std::array::from_fn(|i| truncate_float(f64::from(padded[1 - i]) * f64::from(reciprocal)));
        let origin = near.map(|n| (n.max(0.0) * 0.25).floor() * 4.0);
        let near_offset: [i32; 2] = std::array::from_fn(|i| (origin[i] - near[i]) as i32);
        let far_offset: [i32; 2] = std::array::from_fn(|i| (origin[i] - far[i]) as i32);
        // The horizontal and vertical far corners are distinct. Prefer the vertical corner on a tie.
        let far_axis = (0..2).rev().find(|&i| far_offset[i].unsigned_abs().leading_zeros() >= near_offset[i].unsigned_abs().leading_zeros());
        let offset = std::array::from_fn(|i| if far_axis == Some(i) { far_offset[i] } else { near_offset[i] });
        Self { near, far, origin, basis, offset, far_axis }
    }
}

impl Axis {
    fn sample(&self, pixel: u32) -> f32 {
        let value = f64::from(pixel) * self.step + self.base;
        truncate_float(if value < 0.0 { value + self.negative_bias } else { value })
    }
}

fn offset_product(offset: i32, gradient: f32, unit: f64) -> f64 {
    if !gradient.is_normal() {
        return f64::from(offset) * f64::from(gradient);
    }
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
    if offset < 0 { -product } else { product }
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
    // A power of two only shifts the significand. Every discarded column is zero.
    if basis.to_bits() & FRACTION == 0 || span.to_bits() & FRACTION == 0 {
        return basis * span;
    }
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
