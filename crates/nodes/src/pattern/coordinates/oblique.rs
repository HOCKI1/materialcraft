use super::{Footprint, Triangle, gradient_product, offset_product};
use crate::function::{pixel_divide, truncate_float};

pub(super) struct Plane {
    base: f64,
    step: [f64; 2],
    pair_offset: [[f64; 2]; 2],
    quantum: f64,
    inverse_quantum: f64,
}

impl Plane {
    pub(super) fn y_term(&self, y: u32, dy: f64) -> f64 {
        dy * self.step[1] + self.pair_offset[1][(y & 1) as usize]
    }

    pub(super) fn sample(&self, x: u32, dx: f64, y_term: f64) -> f32 {
        let value = self.base + dx * self.step[0] + self.pair_offset[0][(x & 1) as usize] + y_term;
        let fixed = (value * self.inverse_quantum).floor();
        truncate_float((if fixed < 0.0 { fixed + 1.0 } else { fixed }) * self.quantum)
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
    let planes: [Plane; 2] = std::array::from_fn(|component| {
        let gradient: [f32; 2] = std::array::from_fn(|axis| {
            let start = f64::from(vertices[0][component]);
            let end = f64::from(vertices[axis + 1][component]);
            let difference = aligned_sum([end, -start], leading_power(start.abs().max(end.abs())), 1);
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
        let anchor_value = f64::from(vertices[anchor][component]);
        // Align each offset product using the exponents of its factors, before a
        // carry in the product can shift away another bit during cancellation.
        let base_scale = triangle
            .offset
            .into_iter()
            .zip(gradient)
            .fold(leading_power(anchor_value), |scale, (offset, slope)| scale.max(leading_power(f64::from(offset)) * leading_power(f64::from(slope))));
        let base = truncate_float(aligned_sum([terms[0], terms[1], anchor_value], base_scale, 5));
        let step = gradient.map(f64::from);
        let unit = units[0].max(units[1]);
        // The two samples of a pair share the value at their midpoint. Its signed
        // half-step discards gradient bits below a quarter of the larger gradient's ULP.
        let quarter = unit * 0.25;
        let pair_offset = step.map(|v| {
            let half_remainder = (v - (v / quarter).trunc() * quarter) * 0.5;
            [half_remainder, -half_remainder]
        });
        let quantum = unit / 64.0;
        Plane { base: f64::from(base), step, pair_offset, quantum, inverse_quantum: quantum.recip() }
    });
    planes.iter().all(|p| p.quantum.is_finite()).then_some(planes)
}

fn leading_power(value: f64) -> f64 {
    f64::from_bits(value.to_bits() & (0x7ff_u64 << 52))
}

/// Align signed operands at a shared exponent before cancellation in the interpolation setup.
fn aligned_sum<const N: usize>(terms: [f64; N], scale: f64, guard_bits: u32) -> f64 {
    if scale == 0.0 {
        return 0.0;
    }
    let quantum = scale / f64::from(1_u32 << (23 + guard_bits));
    terms.into_iter().map(|v| (v / quantum).trunc() * quantum).sum()
}
