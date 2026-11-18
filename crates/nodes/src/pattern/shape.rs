pub(super) fn coverage(pattern: i32, x: f32, y: f32, variation: f32) -> Result<f32, String> {
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
        4 => {
            // The expanded quartic preserves the rounding of the low-coverage skirt.
            let t = radius_squared.min(1.0);
            let quadratic = (t - 4.0).mul_add(t, 6.0);
            quadratic.mul_add(t, -4.0).mul_add(t, 1.0).max(0.0)
        }
        5 => {
            const FALLOFF: f32 = 11.1 * std::f32::consts::LOG2_E;
            (crate::function::pixel_exp2(-FALLOFF * radius_squared) - 0.000_015_1).max(0.0)
        }
        6 => (1.0 - radius_squared.sqrt()).max(0.0).powi(4),
        7 => (1.0 - x.abs().max(y.abs())).max(0.0),
        8 => {
            let bevel = variation.clamp(0.0, 1.0);
            if bevel <= f32::EPSILON { 1.0 } else { ((1.0 - x.abs().max(y.abs())) / bevel).clamp(0.0, 1.0) }
        }
        9 => (x + 1.0) * 0.5,
        10 => (x * std::f32::consts::PI).cos() * 0.5 + 0.5,
        11 => radial * radial * if x >= 0.0 { 1.0 } else { 0.0 },
        12 => (1.0 - x.abs()).max(0.0).powi(2),
        13 => {
            if radius_squared <= 1.0 && (x - variation.max(0.05)).powi(2) + y * y > 1.0 {
                1.0
            } else {
                0.0
            }
        }
        14 => {
            if (x.abs() - 0.5).max(0.0).powi(2) + y * y <= 0.25 {
                1.0
            } else {
                0.0
            }
        }
        15 => (1.0 - radius_squared.sqrt()).max(0.0),
        16 => radial.sqrt(),
        17 => ((x + 1.0) * 0.5 + variation).fract(),
        _ => return Err(format!("unknown built-in pattern {pattern}")),
    };

    Ok(value)
}
