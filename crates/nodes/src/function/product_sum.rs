use super::{instruction::Instruction, value_type};
use materialcraft_core::Value;

pub(super) struct ProductSum {
    // Visit operands in their original order, including reads with side effects.
    pub(super) inputs: [usize; 3],
    product_on_right: bool,
    scale_vector: bool,
}

impl ProductSum {
    pub(super) fn evaluate(&self, arguments: &[Value], subtract: bool) -> Result<Value, String> {
        let [first, second, third] = arguments else { return Err("expected three product-sum operands".into()) };
        let (a, b, c) = if self.product_on_right { (second, third, first) } else { (first, second, third) };
        let width = value_type::dimensions(a);
        if width != value_type::dimensions(c) || (width != value_type::dimensions(b) && !(self.scale_vector && value_type::dimensions(b) == 1)) {
            return Err("float operands must have the same dimensions".into());
        }
        let a = value_type::floats(a)?;
        let b = value_type::floats(b)?;
        let c = value_type::floats(c)?;
        let result: [f32; 4] = std::array::from_fn(|i| match (subtract, self.product_on_right) {
            (true, true) => (-a[i]).mul_add(b[i], c[i]),
            (true, false) => a[i].mul_add(b[i], -c[i]),
            (false, _) => a[i].mul_add(b[i], c[i]),
        });
        value_type::intermediate_float_value(result.get(..width).ok_or("invalid product-sum dimensions")?)
    }
}

pub(super) fn identify(instructions: &mut [Instruction]) {
    let mut uses = vec![0usize; instructions.len()];
    let mut needs_rounded = vec![false; instructions.len()];
    let has_loop = instructions.iter().any(|op| op.definition.id == "while");
    for op in instructions.iter() {
        let sum = matches!(op.definition.id, "add" | "sub") && matches!(op.inputs.as_slice(), [a, b] if a != b);
        for &input in &op.inputs {
            if let Some(count) = uses.get_mut(input) {
                *count += 1;
            }
            if !sum && let Some(rounded) = needs_rounded.get_mut(input) {
                *rounded = true;
            }
        }
    }

    for index in 0..instructions.len() {
        let Some(op) = instructions.get(index) else { continue };
        if !matches!(op.definition.id, "add" | "sub") || !matches!(op.type_code, 256 | 512 | 1024 | 2048) {
            continue;
        }
        let &[left, right] = op.inputs.as_slice() else { continue };
        let product = [(left, right, false), (right, left, true)].into_iter().find_map(|(source, other, product_on_right)| {
            let product = instructions.get(source)?;
            if product.literal || product.type_code != op.type_code || !matches!(product.definition.id, "mul" | "mulscalar") || *needs_rounded.get(source)? {
                return None;
            }
            // Loop iterations can invalidate an operand independently of a shared product.
            if has_loop && *uses.get(source)? > 1 {
                return None;
            }
            let &[a, b] = product.inputs.as_slice() else { return None };
            let inputs = if product_on_right { [other, a, b] } else { [a, b, other] };
            Some(ProductSum { inputs, product_on_right, scale_vector: product.definition.id == "mulscalar" })
        });
        if let Some(op) = instructions.get_mut(index) {
            op.pixel_product_sum = product;
        }
    }
}
