//! Runtime pixel sine and cosine from a fixed-point quarter wave.

use super::fixed_point::square_fraction;
use std::sync::LazyLock;

const QUARTER: u32 = 1 << 23;

struct Polynomial {
    constant: i64,
    slope: i64,
    curvature: i64,
}

// Derive each quadratic from the sine function over one 64th of the quadrant.
static QUARTER_WAVE: LazyLock<[Polynomial; 64]> = LazyLock::new(|| std::array::from_fn(quarter_polynomial));

fn quarter_polynomial(index: usize) -> Polynomial {
    let width = std::f64::consts::PI / 128.0;
    let (sine, cosine) = ((index as f64 + 0.5) * width).sin_cos();
    // The linear minimax approximation of the centered cubic on [-1/2, 1/2] is 3t/16.
    let center_slope = 4_194_304.0 * (width * cosine - width.powi(3) * cosine / 32.0);
    let slope = ((center_slope + 4_194_304.0 * width * width * sine * 0.5) / 4.0).round() as i64;
    let curvature = center_slope.round() as i64 - 4 * slope;
    let mut low = f64::INFINITY;
    let mut high = f64::NEG_INFINITY;
    for step in 0..=64 {
        let t = f64::from(step) / 64.0;
        let residual = 33_554_432.0 * ((index as f64 + t) * width).sin() - 32.0 * slope as f64 * t - 8.0 * curvature as f64 * t * t;
        low = low.min(residual);
        high = high.max(residual);
    }
    let midpoint = ((low + high) * 0.5).round() as i64;
    let constant = match index {
        0 => 0,
        // These three measured intervals start one constant unit below the rounded midpoint.
        2 | 18 | 46 => midpoint - 1,
        _ => midpoint,
    };
    Polynomial { constant, slope, curvature }
}

// One turn spans 25 bits. Retaining the product word before the exponent shift
// reproduces the loss of phase resolution at large angles.
fn phase(bits: u32) -> u32 {
    let exponent = ((bits >> 23) & 255) as i32 - 127;
    let mantissa = (bits & 0x7f_ffff) | (1 << 23);
    let word = ((u64::from(mantissa) * 10_680_707) >> 16) as u32;
    let shift = exponent - 8;
    let shifted = if shift >= 0 { word.checked_shl(shift as u32) } else { word.checked_shr((-shift) as u32) };
    shifted.unwrap_or(0) & (4 * QUARTER - 1)
}

pub(super) fn component(angle: f32, cosine: bool) -> f32 {
    if !angle.is_finite() {
        return f32::NAN;
    }
    let phase = phase(angle.to_bits()) + if cosine { QUARTER } else { 0 };
    let half = phase & (2 * QUARTER - 1);
    let quarter = if half < QUARTER { half } else { 2 * QUARTER - 1 - half };
    let bin = &QUARTER_WAVE[(quarter >> 17) as usize];
    let fraction = quarter & 0x1_ffff;
    let fixed = (bin.constant << 12) + bin.slope * i64::from(fraction) + bin.curvature * i64::from(square_fraction(fraction >> 1));
    // The quarter-wave magnitude truncates to the next representable float below the polynomial.
    let exact = fixed as f64 / 137_438_953_472.0;
    let nearest = exact as f32;
    let magnitude = if f64::from(nearest) > exact { f32::from_bits(nearest.to_bits().saturating_sub(1)) } else { nearest };
    if (phase & (2 * QUARTER) != 0) ^ (!cosine && angle.is_sign_negative()) { -magnitude } else { magnitude }
}
