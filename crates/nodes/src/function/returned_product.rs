use super::{EvaluationState, Program, arithmetic::flush_subnormal, instruction::Instruction, product_sum::ProductSum, value_type};
use materialcraft_core::Value;
use std::collections::BTreeMap;

/// Registers and components of the factors in the call that evaluated a product.
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord)]
pub(super) struct Source {
    calls: Vec<usize>,
    factors: [usize; 2],
    components: [usize; 2],
    operation: Operation,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum Operation {
    Multiply,
    Divide { literal_divisor: bool },
}

impl Source {
    pub(super) fn local(factors: [usize; 2]) -> Self {
        Self { calls: Vec::new(), factors, components: [0, 0], operation: Operation::Multiply }
    }

    pub(super) fn factors<'a>(&self, mut state: &'a EvaluationState) -> Result<(&'a Value, &'a Value), String> {
        for call in &self.calls {
            state = state.calls.get(call).ok_or("missing evaluated product call")?;
        }
        let read = |index| state.registers.get(index).and_then(Option::as_ref).ok_or("missing evaluated product factor");
        let [a, b] = self.factors;
        Ok((read(a)?, read(b)?))
    }
}

pub(super) struct Term {
    source: Source,
    on_right: bool,
}

pub(super) type Sum = [Option<Term>; 4];

pub(super) fn evaluate_sum(terms: &Sum, arguments: &[Value], subtract: bool, state: &EvaluationState) -> Result<Value, String> {
    let [left, right] = arguments else { return Err("missing sum operands".into()) };
    let width = value_type::dimensions(left);
    if width != value_type::dimensions(right) {
        return Err("float operands must have the same dimensions".into());
    }
    let left = value_type::floats(left)?.map(flush_subnormal);
    let right = value_type::floats(right)?.map(flush_subnormal);
    let mut result = [0.0; 4];
    for (((output, term), left), right) in result.iter_mut().zip(terms).zip(left).zip(right).take(width) {
        let sum = if let Some(term) = term {
            let (a, b) = term.source.factors(state)?;
            let [i, j] = term.source.components;
            let a = flush_subnormal(*value_type::floats(a)?.get(i).ok_or("invalid product component")?);
            let b = flush_subnormal(*value_type::floats(b)?.get(j).ok_or("invalid product component")?);
            let a = if subtract && term.on_right { -a } else { a };
            let addend = if term.on_right {
                left
            } else if subtract {
                -right
            } else {
                right
            };
            match term.source.operation {
                Operation::Multiply => a.mul_add(b, addend),
                Operation::Divide { literal_divisor } => super::reciprocal::divide_add(a, b, addend, literal_divisor),
            }
        } else if subtract {
            left - right
        } else {
            left + right
        };
        *output = flush_subnormal(sum);
    }
    value_type::intermediate_float_value(result.get(..width).ok_or("invalid sum dimensions")?)
}

pub(super) type Lanes = [Option<Source>; 4];
type Origins = [Option<usize>; 4];

pub(super) fn float_width(code: u32) -> usize {
    match code {
        256 => 1,
        512 => 2,
        1024 => 3,
        2048 => 4,
        _ => 0,
    }
}

/// Packing and component selection forward a lane without observing its rounded value.
pub(super) fn forwarded<T: Copy>(op: &Instruction, instructions: &[Instruction], origins: &[[Option<T>; 4]], padding: Option<T>) -> Option<[Option<T>; 4]> {
    let mut out = [None; 4];
    match (op.definition.id, op.inputs.as_slice()) {
        ("vector2" | "vector3" | "vector4", [left, right]) => {
            let left_width = float_width(instructions.get(*left)?.type_code);
            let right_width = float_width(instructions.get(*right)?.type_code);
            if left_width == 0 || right_width == 0 || left_width + right_width != float_width(op.type_code) {
                return None;
            }
            let lanes = op.inputs.iter().flat_map(|i| {
                let width = instructions.get(*i).map_or(0, |v| float_width(v.type_code));
                origins.get(*i).into_iter().flatten().take(width)
            });
            for (dst, src) in out.iter_mut().zip(lanes) {
                *dst = *src;
            }
        }
        ("swizzle1" | "swizzle2" | "swizzle3" | "swizzle4", [input]) => {
            let indices = value_type::integers(op.value.as_ref()?).ok()?;
            let width = float_width(instructions.get(*input)?.type_code);
            if width == 0 {
                return None;
            }
            let input = origins.get(*input)?;
            for (dst, i) in out.iter_mut().take(float_width(op.type_code)).zip(indices) {
                let i = usize::try_from(i).ok()?;
                let source = *input.get(i)?;
                *dst = if i < width { source } else { padding };
            }
        }
        _ => return None,
    }
    Some(out)
}

pub(super) fn identify(program: &mut Program, mut returned: BTreeMap<usize, Lanes>) -> Lanes {
    let empty = || std::array::from_fn(|_| None);
    // Loop iterations invalidate individual registers. Only retain factors for straight-line lifetimes.
    if program.instructions.iter().any(|op| op.definition.id == "while")
        || (returned.is_empty() && !program.instructions.iter().any(|op| op.product_operation().is_some() && float_width(op.type_code) > 0 && !op.literal))
    {
        return empty();
    }

    let mut origins: Vec<Origins> = Vec::with_capacity(program.instructions.len());
    let mut products: Vec<(Source, usize)> = Vec::new();
    let mut sums = Vec::new();
    for (index, op) in program.instructions.iter().enumerate() {
        if let Some(lanes) = forwarded(op, &program.instructions, &origins, None) {
            origins.push(lanes);
            continue;
        }

        for source in op.inputs.iter().filter_map(|i| origins.get(*i)).flatten().flatten() {
            if let Some((_, uses)) = products.get_mut(*source) {
                *uses += 1;
            }
        }
        let width = float_width(op.type_code);
        if matches!(op.definition.id, "add" | "sub")
            && width > 0
            && op.pixel_product_sum.is_none()
            && let &[left, right] = op.inputs.as_slice()
        {
            let sides = [left, right].map(|i| origins.get(i).copied().unwrap_or([None; 4]));
            if sides.iter().flatten().any(Option::is_some) {
                sums.push((index, sides));
            }
        }

        let mut lanes = [None; 4];
        if let Some(operation) = op.product_operation()
            && !op.literal
            && let &[a, b] = op.inputs.as_slice()
        {
            let operation = if operation == "div" { Operation::Divide { literal_divisor: op.literal_divisor } } else { Operation::Multiply };
            for (i, lane) in lanes.iter_mut().take(width).enumerate() {
                *lane = Some(products.len());
                let j = if op.definition.id == "mulscalar" { 0 } else { i };
                products.push((Source { components: [i, j], operation, ..Source::local([a, b]) }, 0));
            }
        } else if let Some(exports) = returned.remove(&index) {
            // Two returned lanes can reference the same product. Count their downstream reads together.
            let mut distinct = BTreeMap::new();
            for (lane, source) in lanes.iter_mut().zip(exports) {
                if let Some(mut source) = source {
                    source.calls.insert(0, index);
                    *lane = Some(*distinct.entry(source.clone()).or_insert_with(|| {
                        let id = products.len();
                        products.push((source, 0));
                        id
                    }));
                }
            }
        }
        origins.push(lanes);
    }

    let output = origins.get(program.output).copied().unwrap_or([None; 4]);
    let exported = output.map(|id| {
        let (source, uses) = products.get(id?)?;
        (*uses == 0).then(|| source.clone())
    });
    for id in output.into_iter().flatten() {
        if let Some((_, uses)) = products.get_mut(id) {
            *uses += 1;
        }
    }
    for (index, sides) in sums {
        let terms: Sum = std::array::from_fn(|i| {
            // A shared lane has already rounded. When both products are free, the left one contracts.
            sides.iter().enumerate().find_map(|(side, lanes)| {
                let id = lanes.get(i).copied().flatten()?;
                let (source, uses) = products.get(id)?;
                (*uses == 1).then(|| Term { source: source.clone(), on_right: side == 1 })
            })
        });
        if terms.iter().any(Option::is_some)
            && let Some(op) = program.instructions.get_mut(index)
        {
            op.pixel_product_sum = Some(ProductSum::returned(&op.inputs, terms));
        }
    }
    exported
}
