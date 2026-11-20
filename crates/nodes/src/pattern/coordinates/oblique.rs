use super::{Footprint, Triangle, gradient_product, offset_product};
use crate::function::{pixel_divide, truncate_float};

pub(super) struct Plane {
    base: f64,
    step: [f64; 2],
    negative_bias: f64,
}

impl Plane {
    pub(super) fn y_term(&self, dy: f64) -> f64 {
        dy * self.step[1]
    }

    pub(super) fn sample(&self, dx: f64, y_term: f64) -> f32 {
        let value = self.base + dx * self.step[0] + y_term;
        truncate_float(if value < 0.0 { value + self.negative_bias } else { value })
    }
}

pub(super) fn planes(triangle: &Triangle, frame: &Footprint, size: [f32; 2], [[left, right], [top, bottom]]: [[f32; 2]; 2]) -> Option<[Plane; 2]> {
    let scale = size.map(|s| pixel_divide(2.0, s));
    let vertices = [[left, top], [right, top], [left, bottom]]
        .map(|[x, y]| [y.mul_add(frame.sine, x * frame.cosine) * scale[0], y.mul_add(frame.cosine, -x * frame.sine) * scale[1]]);
    if vertices.iter().flatten().any(|v| !v.is_finite()) {
        return None;
    }
    let anchor = triangle.far_axis.map_or(0, |axis| axis + 1);
    Some(std::array::from_fn(|component| {
        let gradient: [f32; 2] = std::array::from_fn(|axis| {
            let difference = aligned_sum([f64::from(vertices[axis + 1][component]), -f64::from(vertices[0][component])], 1);
            let span = truncate_float(difference * 0.5);
            let basis = triangle.basis[axis];
            let exponents = (span.abs().to_bits() >> 23) + (basis.to_bits() >> 23);
            // The partial-product path assumes a normal, finite result at the stored exponent.
            if !span.is_normal() || !(128..=380).contains(&exponents) {
                return truncate_float(f64::from(basis) * f64::from(span));
            }
            gradient_product(basis, span.abs()).copysign(span)
        });
        let units = gradient.map(|v| f64::from(v.abs().next_up()) - f64::from(v.abs()));
        let terms: [f64; 2] = std::array::from_fn(|i| {
            let magnitude = offset_product(triangle.offset[i], gradient[i].abs(), units[i]);
            if gradient[i].is_sign_negative() { -magnitude } else { magnitude }
        });
        let base = truncate_float(aligned_sum([terms[0], terms[1], f64::from(vertices[anchor][component])], 5));
        Plane { base: f64::from(base), step: gradient.map(f64::from), negative_bias: units[0].max(units[1]) / 64.0 }
    }))
}

/// Align signed operands at a shared exponent before cancellation in the interpolation setup.
fn aligned_sum<const N: usize>(terms: [f64; N], guard_bits: u32) -> f64 {
    let largest = terms.iter().map(|v| v.abs()).fold(0.0_f64, f64::max);
    if largest == 0.0 {
        return 0.0;
    }
    let power = f64::from_bits(largest.to_bits() & (0x7ff_u64 << 52));
    let quantum = power / f64::from(1_u32 << (23 + guard_bits));
    terms.into_iter().map(|v| (v / quantum).trunc() * quantum).sum()
}
