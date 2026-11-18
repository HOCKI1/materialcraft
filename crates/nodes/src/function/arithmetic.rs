use super::value_type::{dimensions, floats, integer_value, integers, intermediate_float_value as float_value};
use materialcraft_core::Value;

pub(crate) fn evaluate(instruction: &super::instruction::Instruction, args: &[Value], random: &mut u32, scope: super::FunctionScope) -> Result<Value, String> {
    let op = instruction.definition.id;
    let pixel_math = scope == super::FunctionScope::PixelProcessor && !instruction.literal;
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
            let xy = left[0].mul_add(right[0], left[1] * right[1]);
            let sum = left.iter().zip(right).take(count).skip(2).fold(xy, |sum, (a, b)| a.mul_add(b, sum));
            return float_value(&[sum]);
        }
        return float_value(&[left.iter().zip(right).take(count).map(|(a, b)| a * b).sum()]);
    }

    if op == "atan2" {
        if count != 2 {
            return Err("arc tangent requires a float2".into());
        }

        return float_value(&[left[1].atan2(left[0])]);
    }

    if op == "cartesian" {
        if count != 1 {
            return Err("Cartesian requires scalar radius and angle".into());
        }

        return float_value(&[left[0] * right[0].cos(), left[0] * right[0].sin()]);
    }

    let mut result = [0.0; 4];
    for (((result, left), right), factor) in result.iter_mut().zip(left).zip(right).zip(factor).take(count) {
        *result = match op {
            "add" => left + right,
            "sub" => left - right,
            "mul" | "mulscalar" => left * right,
            // Outside the normal reciprocal range, an intermediate overflow or underflow can lose a finite quotient.
            "div" if pixel_math && right.recip().is_normal() => left * right.recip(),
            "div" => left / right,
            // Floored modulo wraps negative coordinates; remainder keeps their sign
            "mod" => left - right * (left / right).floor(),
            "neg" => -left,
            "abs" => left.abs(),
            "min" => left.min(right),
            "max" => left.max(right),
            "floor" => left.floor(),
            "ceil" => left.ceil(),
            "sqrt" => left.sqrt(),
            "exp" => left.exp(),
            "log" => left.ln(),
            "log2" => left.log2(),
            "pow" => left.powf(right),
            "pow2" => left.exp2(),
            "sin" => left.sin(),
            "cos" => left.cos(),
            "tan" => left.tan(),
            "lerp" if pixel_math => (right - left).mul_add(factor, left),
            "lerp" => left + (right - left) * factor,
            "rand" if scope == super::FunctionScope::Node => crate::random::node_function(random) * left,
            "rand" if scope == super::FunctionScope::FxMap => crate::random::pattern_function(random) * left,
            "rand" => {
                *random = crate::random::hash(random.wrapping_add(1));
                (*random >> 8) as f32 / 16777216.0 * left
            }
            _ => return Err(format!("unknown arithmetic operation `{op}`")),
        };
    }

    float_value(result.get(..count).ok_or("invalid float dimensions")?)
}
