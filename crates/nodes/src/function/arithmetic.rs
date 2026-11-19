use super::value_type::{dimensions, floats, integer_value, integers, intermediate_float_value as float_value};
use materialcraft_core::Value;

/// Runtime arithmetic treats subnormals as signed zero at its inputs and rounded results.
pub(super) fn flush_subnormal(value: f32) -> f32 {
    if value.is_subnormal() { 0.0_f32.copysign(value) } else { value }
}

pub(crate) fn evaluate(instruction: &super::instruction::Instruction, args: &[Value], random: &mut u32, scope: super::FunctionScope) -> Result<Value, String> {
    let op = instruction.definition.id;
    let pixel_math = scope == super::FunctionScope::PixelProcessor && !instruction.literal;
    let angle_component = |angle, cosine| super::trigonometry::component(angle, cosine, scope, instruction.literal);
    let first = args.first().ok_or("missing arithmetic operand")?;
    let count = dimensions(first);
    let integer = matches!(first, Value::Int(_) | Value::Int2(_) | Value::Int3(_) | Value::Int4(_));
    if integer {
        let left = integers(first)?;
        let right = args.get(1).map(integers).transpose()?.unwrap_or([0; 4]);
        if args.get(1).is_some_and(|value| dimensions(value) != count) {
            return Err("integer operands must have the same dimensions".into());
        }

        let mut result = [0; 4];
        for ((result, left), right) in result.iter_mut().zip(left).zip(right).take(count) {
            *result = match op {
                "add" => left.checked_add(right),
                "sub" => left.checked_sub(right),
                "mul" => left.checked_mul(right),
                "div" => left.checked_div(right),
                "mod" => left.checked_rem(right),
                "neg" => left.checked_neg(),
                "abs" => left.checked_abs(),
                "min" => Some(left.min(right)),
                "max" => Some(left.max(right)),
                _ => return Err(format!("{op} requires floating-point operands")),
            }
            .ok_or_else(|| format!("{op}: integer overflow or division by zero"))?;
        }

        return integer_value(result.get(..count).ok_or("invalid integer dimensions")?);
    }

    let left = floats(first)?;
    let right = args.get(1).map(floats).transpose()?.unwrap_or([0.0; 4]);
    let factor = args.get(2).map(floats).transpose()?.unwrap_or([0.0; 4]);
    if args.get(1).is_some_and(|value| dimensions(value) != count && !(op == "mulscalar" && dimensions(value) == 1)) {
        return Err("float operands must have the same dimensions".into());
    }

    if args.get(2).is_some_and(|value| dimensions(value) != count && dimensions(value) != 1) {
        return Err("interpolation factor must be a scalar or match its operands".into());
    }

    if op == "dot" {
        if pixel_math && count >= 2 {
            // The second product rounds first. Each remaining channel contributes one fused operation.
            let left = left.map(flush_subnormal);
            let right = right.map(flush_subnormal);
            let xy = flush_subnormal(left[0].mul_add(right[0], flush_subnormal(left[1] * right[1])));
            let sum = left.iter().zip(right).take(count).skip(2).fold(xy, |sum, (a, b)| flush_subnormal(a.mul_add(b, sum)));
            return float_value(&[sum]);
        }
        return float_value(&[left.iter().zip(right).take(count).map(|(a, b)| a * b).sum()]);
    }

    if op == "atan2" {
        if count != 2 {
            return Err("arc tangent requires a float2".into());
        }

        return float_value(&[if pixel_math { pixel_atan2(left[0], left[1]) } else { left[1].atan2(left[0]) }]);
    }

    if op == "cartesian" {
        if count != 1 {
            return Err("Cartesian requires scalar radius and angle".into());
        }

        let angle = right[0];
        return float_value(&[left[0] * angle_component(angle, true), left[0] * angle_component(angle, false)]);
    }

    let flush_arithmetic = !instruction.literal && matches!(op, "add" | "sub" | "mul" | "mulscalar" | "div");
    let mut result = [0.0; 4];
    for (((result, left), right), factor) in result.iter_mut().zip(left).zip(right).zip(factor).take(count) {
        let (left, right) = if flush_arithmetic { (flush_subnormal(left), flush_subnormal(right)) } else { (left, right) };
        let value = match op {
            "add" => left + right,
            "sub" => left - right,
            "mul" | "mulscalar" => left * right,
            "div" if pixel_math => super::reciprocal::divide_by(left, right, instruction.literal_divisor),
            "div" => left / right,
            // Floored modulo wraps negative coordinates; remainder keeps their sign
            "mod" => left - right * (left / right).floor(),
            "neg" => -left,
            "abs" => left.abs(),
            "min" => left.min(right),
            "max" => left.max(right),
            "floor" => left.floor(),
            "ceil" => left.ceil(),
            "sqrt" if pixel_math => super::square_root::evaluate(left),
            "sqrt" => left.sqrt(),
            "exp" => super::binary_exponential::exp(left, pixel_math),
            "log" if pixel_math => super::binary_exponential::ln(left),
            "log" => super::binary_exponential::uniform_ln(left),
            "log2" if pixel_math => super::binary_exponential::log2(left),
            "log2" => super::binary_exponential::uniform_ln(left) / std::f32::consts::LN_2,
            "pow" => super::binary_exponential::power(left, right, pixel_math),
            "pow2" if pixel_math => super::binary_exponential::exp2(left),
            "pow2" => super::binary_exponential::uniform_exp2(left),
            "sin" => angle_component(left, false),
            "cos" => angle_component(left, true),
            "tan" if instruction.literal => left.tan(),
            "tan" => {
                let sine = angle_component(left, false);
                let cosine = angle_component(left, true);
                if pixel_math { super::reciprocal::divide(sine, cosine) } else { sine / cosine }
            }
            "lerp" if pixel_math => (right - left).mul_add(factor, left),
            "lerp" => left + (right - left) * factor,
            "rand" if matches!(scope, super::FunctionScope::Node | super::FunctionScope::ValueProcessor) => crate::random::node_function(random) * left,
            "rand" => crate::random::image_function(random) * left,
            _ => return Err(format!("unknown arithmetic operation `{op}`")),
        };
        *result = if flush_arithmetic { flush_subnormal(value) } else { value };
    }

    float_value(result.get(..count).ok_or("invalid float dimensions")?)
}

fn pixel_atan2(x: f32, y: f32) -> f32 {
    use std::f32::consts::{FRAC_PI_2, PI};
    let flush = |v: f32| if v.is_subnormal() { 0.0 } else { v };
    let (x, y) = (flush(x), flush(y));
    if x == 0.0 {
        return if y > 0.0 {
            FRAC_PI_2
        } else if y < 0.0 {
            -FRAC_PI_2
        } else {
            0.0
        };
    }
    let (ax, ay) = (x.abs(), y.abs());
    let horizontal = ax >= ay;
    let (small, large) = if horizontal { (ay, ax) } else { (ax, ay) };
    let ratio = flush(small * flush(super::reciprocal::divide(1.0, large)));
    // A quartic fitted to the measured first octant. Its four terms follow the pixel dot-product order.
    let square = ratio * ratio;
    let linear = ratio.mul_add(1.00249, square * -0.0123825);
    let cubic = (ratio * square).mul_add(-0.347313, linear);
    let octant = (square * square).mul_add(0.142604, cubic);
    let angle = match (horizontal, x < 0.0) {
        (true, false) => octant,
        (true, true) => PI - octant,
        (false, false) => FRAC_PI_2 - octant,
        (false, true) => FRAC_PI_2 + octant,
    };
    if y < 0.0 && angle > 0.0 { -angle } else { angle }
}
