//! Fixed-point reciprocal used by runtime pixel division.

use super::fixed_point::square_fraction;

#[derive(Clone, Copy)]
struct Polynomial {
    quarter_constant: i32,
    half_slope: i32,
    curvature: i32,
}

const fn nearest(value: f64) -> i32 {
    (value + if value < 0.0 { -0.5 } else { 0.5 }) as i32
}

// Derive the coefficients from 1/x. The normalized mantissa spans 128 equal intervals.
// In units of the output significand, the function on interval i is 2^31 / (128+i+t).
const fn polynomials() -> [Polynomial; 128] {
    const SCALE: f64 = 2_147_483_648.0;
    let mut result = [Polynomial { quarter_constant: 0, half_slope: 0, curvature: 0 }; 128];
    let mut i = 0;
    while i < result.len() {
        let start = 128.0 + i as f64;
        let mid = start + 0.5;
        let mid_squared = mid * mid;
        // On [-1/2,1/2], the linear minimax approximation of t³ is 3t/16.
        let center_slope = -SCALE / mid_squared - (3.0 / 16.0) * SCALE / (mid_squared * mid_squared);
        let half_slope = nearest((center_slope - SCALE / (mid_squared * mid)) * 0.5);
        let slope = 2 * half_slope;
        let curvature = nearest(center_slope) - slope;

        // Center the quarter-unit constant between the extrema of the residual.
        let mut low = SCALE;
        let mut high = 0.0;
        let mut step = 0;
        while step <= 64 {
            let t = step as f64 / 64.0;
            let residual = SCALE / (start + t) - slope as f64 * t - curvature as f64 * t * t;
            if residual < low {
                low = residual;
            }
            if residual > high {
                high = residual;
            }
            step += 1;
        }
        result[i] = Polynomial { quarter_constant: nearest(2.0 * (low + high)), half_slope, curvature };
        i += 1;
    }
    result
}

const POLYNOMIALS: [Polynomial; 128] = polynomials();

pub(crate) fn divide(numerator: f32, denominator: f32) -> f32 {
    let bits = denominator.to_bits();
    let exponent = (bits >> 23) & 255;
    if !(1..=252).contains(&exponent) {
        // At the limits, an intermediate reciprocal can overflow or underflow a finite quotient.
        let reciprocal = denominator.recip();
        return if reciprocal.is_normal() { numerator * reciprocal } else { numerator / denominator };
    }
    let Some(bin) = POLYNOMIALS.get(((bits >> 16) & 127) as usize) else {
        return numerator / denominator;
    };
    let fraction = bits & 0xffff;
    let polynomial = (i64::from(bin.quarter_constant) << 13)
        + i64::from(bin.half_slope) * i64::from(fraction)
        + i64::from(bin.curvature) * i64::from(square_fraction(fraction));
    // The observed rounding threshold includes a bias of 2023/32768 of an output unit.
    let significand = ((polynomial + 16_384 + 2_023) >> 15) as u32;
    let magnitude = ((252 - exponent) << 23) + significand;
    numerator * f32::from_bits((bits & 0x8000_0000) | magnitude)
}
