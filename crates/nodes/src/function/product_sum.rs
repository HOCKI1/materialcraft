use super::{arithmetic::flush_subnormal, instruction::Instruction, value_type};
use materialcraft_core::Value;

pub(super) struct ProductSum {
    // Visit operands in their original order, including reads with side effects.
    pub(super) inputs: Vec<usize>,
    product_on_right: bool,
    negated: bool,
    operation: &'static str,
}

impl ProductSum {
    pub(super) fn evaluate(&self, arguments: &[Value], subtract: bool) -> Result<Value, String> {
        let [first, second, third, rest @ ..] = arguments else { return Err("missing product-sum operands".into()) };
        let (a, b, c) = if self.product_on_right { (second, third, first) } else { (first, second, third) };
        let width = value_type::dimensions(a);
        if width != value_type::dimensions(c) || (width != value_type::dimensions(b) && !(self.operation == "mulscalar" && value_type::dimensions(b) == 1)) {
            return Err("float operands must have the same dimensions".into());
        }
        let blend = rest.first();
        if blend.is_some_and(|v| value_type::dimensions(v) != width && value_type::dimensions(v) != 1) {
            return Err("interpolation factor must be a scalar or match its operands".into());
        }
        let blend = blend.map(value_type::floats).transpose()?;
        let a = value_type::floats(a)?.map(flush_subnormal);
        let b = value_type::floats(b)?.map(flush_subnormal);
        let c = value_type::floats(c)?.map(flush_subnormal);
        let negate_product = self.negated ^ (subtract && self.product_on_right);
        let negate_addend = (subtract && !self.product_on_right) || blend.is_some();
        let result: [f32; 4] = std::array::from_fn(|i| {
            let factor = if negate_product { -a[i] } else { a[i] };
            let addend = if negate_addend { -c[i] } else { c[i] };
            let sum = if self.operation == "div" { super::reciprocal::divide_add(factor, b[i], addend) } else { factor.mul_add(b[i], addend) };
            blend.map_or(sum, |t| flush_subnormal(sum).mul_add(t[i], c[i]))
        })
        .map(flush_subnormal);
        value_type::intermediate_float_value(result.get(..width).ok_or("invalid product-sum dimensions")?)
    }
}

#[derive(Clone, Copy)]
struct ProductOperand {
    index: usize,
    negated: bool,
}

#[derive(Clone, Copy, Default)]
struct ProductUsage {
    count: usize,
    rounded: bool,
}

pub(super) fn identify(instructions: &mut [Instruction]) {
    // A sign change can be applied to a factor inside the fused operation. Follow inputs in
    // compilation order so even a long chain of negations takes a single bounded pass.
    let mut operands: Vec<Option<ProductOperand>> = Vec::with_capacity(instructions.len());
    for (index, op) in instructions.iter().enumerate() {
        let product = match (op.definition.id, op.inputs.as_slice()) {
            ("mul" | "mulscalar" | "div", [_, _]) if !op.literal && matches!(op.type_code, 256 | 512 | 1024 | 2048) => {
                Some(ProductOperand { index, negated: false })
            }
            ("neg", [input]) => operands.get(*input).copied().flatten().map(|p| ProductOperand { negated: !p.negated, ..p }),
            _ => None,
        };
        operands.push(product);
    }
    let operand = |index: usize| operands.get(index).copied().flatten();
    let mut usage = vec![ProductUsage::default(); instructions.len()];
    let has_loop = instructions.iter().any(|op| op.definition.id == "while");
    for op in instructions.iter() {
        if op.definition.id == "neg" {
            continue;
        }
        let sum = match (op.definition.id, op.inputs.as_slice()) {
            ("add" | "sub", [a, b]) => operand(*a).zip(operand(*b)).is_none_or(|(p, q)| p.index != q.index),
            _ => false,
        };
        for (slot, &input) in op.inputs.iter().enumerate() {
            if let Some(p) = operand(input) {
                usage[p.index].count += 1;
                // In lerp(a,b,t), b can contract into b-a. Both uses of a need its rounded value.
                usage[p.index].rounded |= !(sum || (op.definition.id == "lerp" && slot == 1));
            }
        }
    }

    // If a sum consumes a single-use product, the shared product on its other side is a
    // rounded addend. Its other readers must see the same value.
    for op in instructions.iter().filter(|op| matches!(op.definition.id, "add" | "sub")) {
        if let [left, right] = op.inputs.as_slice()
            && let (Some(left), Some(right)) = (operand(*left), operand(*right))
        {
            for (single, shared) in [(left.index, right.index), (right.index, left.index)] {
                if usage[single].count == 1 && !usage[single].rounded && usage[shared].count > 1 {
                    usage[shared].rounded = true;
                }
            }
        }
    }

    for index in 0..instructions.len() {
        let Some(op) = instructions.get(index) else { continue };
        if !matches!(op.type_code, 256 | 512 | 1024 | 2048) {
            continue;
        }
        let (left, right, blend) = match (op.definition.id, op.inputs.as_slice()) {
            ("add" | "sub", &[left, right]) => (left, right, None),
            ("lerp", &[left, right, factor]) => (left, right, Some(factor)),
            _ => continue,
        };
        let product = [(left, right, false), (right, left, true)]
            .into_iter()
            .filter(|(_, _, on_right)| blend.is_none() || *on_right)
            .filter_map(|(source, other, product_on_right)| {
                let source = operand(source)?;
                let product = instructions.get(source.index)?;
                let used = usage[source.index];
                if used.rounded || product.type_code != op.type_code {
                    return None;
                }
                // Loop iterations can invalidate an operand independently of a shared product.
                if has_loop && used.count > 1 {
                    return None;
                }
                let &[a, b] = product.inputs.as_slice() else { return None };
                let mut inputs = if product_on_right { vec![other, a, b] } else { vec![a, b, other] };
                inputs.extend(blend);
                Some((used.count, ProductSum { inputs, product_on_right, negated: source.negated, operation: product.definition.id }))
            })
            .min_by_key(|(count, _)| *count)
            .map(|(_, sum)| sum);
        if let Some(op) = instructions.get_mut(index) {
            op.pixel_product_sum = product;
        }
    }
}
