/// Geometric coverage and the signed channel offset applied after tint and opacity.
pub(super) fn profile(pattern: i32, x: f32, y: f32, variation: f32) -> Result<(f32, f32), String> {
    let radius_squared = y.mul_add(y, x * x);
    let radial = (1.0 - radius_squared).max(0.0);
    let value = match pattern - 1 {
        1 => 1.0,
        2 => {
            if radius_squared <= 1.0 {
                1.0
            } else {
                0.0
            }
        }
        3 => radial,
        4 => quartic_bell(radius_squared).max(0.0),
        5 => {
            const FALLOFF: f32 = 11.1 * std::f32::consts::LOG2_E;
            (crate::function::pixel_exp2(-FALLOFF * radius_squared) - 0.000_015_1).max(0.0)
        }
        6 => (1.0 - crate::function::pixel_sqrt(radius_squared)).max(0.0).powi(4),
        7 => (1.0 - x.abs().max(y.abs())).max(0.0),
        8 if variation <= 0.0 => 1.0,
        8 => {
            // Each edge fades independently, including at a corner.
            let inverse_width = variation.max(f32::MIN_POSITIVE).recip();
            let fade = |coordinate: f32| {
                let t = ((1.0 - coordinate.abs()) * inverse_width).clamp(0.0, 1.0);
                let square = t * t;
                (-2.0_f32).mul_add(square * t, 3.0 * square)
            };
            fade(x) * fade(y)
        }
        9 => (x + 1.0) * 0.5 * (1.0 - variation.max(0.0)),
        10 => {
            let cycles = 1.5 + 6.0 * f64::from(variation.max(0.0));
            let phase = f64::from(x) * cycles;
            let triangle = (1.0 - 4.0 * (phase - phase.round()).abs()) as f32;
            let parabola = (-triangle.abs()).mul_add(triangle, 2.0 * triangle);
            // The measured wave uses a quadratic correction before the radial fade.
            const CORRECTION: f32 = 0.223_936_68;
            let wave = parabola * CORRECTION.mul_add(parabola.abs(), 1.0 - CORRECTION);
            radial * (0.5 * wave + 0.5)
        }
        11 => quartic_bell(radius_squared) * descending_fade(x, variation),
        12 => quartic_bell(radius_squared) * (1.0 - x.abs()).powi(2),
        13 => {
            let t = radius_squared.min(1.0);
            // Normalize t(1-t)² to one at t=1/3.
            let ring = 6.75_f32.mul_add(t, -13.5).mul_add(t, 6.75) * t;
            ring * descending_fade((x + 1.0) * 0.5, variation)
        }
        14 => {
            let tip = (x.abs() - 0.77 * variation.max(0.0)).max(0.0);
            let distance_squared = y.mul_add(y, tip * tip);
            (-20.0_f32).mul_add(distance_squared, 1.0).max(0.0)
        }
        15 => (1.0 - crate::function::pixel_sqrt(radius_squared)).max(0.0),
        16 => return Ok(((x + 1.0) * 0.5, variation)),
        17 if radius_squared <= 1.0 => return Ok((crate::function::pixel_sqrt(radial), variation)),
        17 => 0.0,
        _ => return Err(format!("unknown built-in pattern {pattern}")),
    };

    Ok((value, 0.0))
}

fn quartic_bell(radius_squared: f32) -> f32 {
    // The expanded quartic retains the signed rounding of the low-coverage skirt.
    let t = radius_squared.min(1.0);
    (t - 4.0).mul_add(t, 6.0).mul_add(t, -4.0).mul_add(t, 1.0)
}

fn descending_fade(position: f32, variation: f32) -> f32 {
    let width = 1.0 - variation.max(0.0);
    let t = if width == 0.0 { f32::from(position > 0.0) } else { (position * width.recip()).clamp(0.0, 1.0) };
    (t * t).mul_add(2.0_f32.mul_add(t, -3.0), 1.0)
}
