use super::{EvaluationState, Program, instruction::Instruction, product_sum::ProductSum, value_type};
use materialcraft_core::Value;
use std::collections::BTreeMap;

/// Registers of the factors before the scalar product rounded, in the call that evaluated it.
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord)]
pub(super) struct Source {
    calls: Vec<usize>,
    factors: [usize; 2],
}

impl Source {
    pub(super) fn local(factors: [usize; 2]) -> Self {
        Self { calls: Vec::new(), factors }
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

pub(super) type Lanes = [Option<Source>; 4];
type Origins = [Option<usize>; 4];

fn float_width(code: u32) -> usize {
    match code {
        256 => 1,
        512 => 2,
        1024 => 3,
        2048 => 4,
        _ => 0,
    }
}

/// Packing and component selection forward a lane without observing its rounded value.
fn forwarded(op: &Instruction, instructions: &[Instruction], origins: &[Origins]) -> Option<Origins> {
    let mut out = [None; 4];
    match (op.definition.id, op.inputs.as_slice()) {
        ("vector2" | "vector3" | "vector4", [_, _]) => {
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
            let input = origins.get(*input)?;
            for (dst, i) in out.iter_mut().take(float_width(op.type_code)).zip(indices) {
                *dst = usize::try_from(i).ok().and_then(|i| input.get(i)).copied().flatten();
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
        || (returned.is_empty() && !program.instructions.iter().any(|op| op.definition.id == "mul" && op.type_code == 256 && !op.literal))
    {
        return empty();
    }

    let mut origins: Vec<Origins> = Vec::with_capacity(program.instructions.len());
    let mut products: Vec<(Source, usize)> = Vec::new();
    let mut sums = Vec::new();
    for (index, op) in program.instructions.iter().enumerate() {
        if let Some(lanes) = forwarded(op, &program.instructions, &origins) {
            origins.push(lanes);
            continue;
        }

        for source in op.inputs.iter().filter_map(|i| origins.get(*i)).flatten().flatten() {
            if let Some((_, uses)) = products.get_mut(*source) {
                *uses += 1;
            }
        }
        if op.definition.id == "add" && op.type_code == 256 && op.pixel_product_sum.is_none() {
            let mut candidates = op.inputs.iter().enumerate().filter_map(|(slot, input)| {
                let input_op = program.instructions.get(*input)?;
                if !matches!(input_op.definition.id, "swizzle1" | "instance") {
                    return None;
                }
                Some((index, origins.get(*input)?[0]?, slot == 1))
            });
            if let Some(sum) = candidates.next()
                && candidates.next().is_none()
            {
                sums.push(sum);
            }
        }

        let mut lanes = [None; 4];
        if let ("mul", 256, false, &[a, b]) = (op.definition.id, op.type_code, op.literal, op.inputs.as_slice()) {
            lanes[0] = Some(products.len());
            products.push((Source::local([a, b]), 0));
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
    for (index, id, on_right) in sums {
        if let Some((source, 1)) = products.get(id)
            && let Some(op) = program.instructions.get_mut(index)
        {
            op.pixel_product_sum = Some(ProductSum::returned(&op.inputs, on_right, source.clone()));
        }
    }
    exported
}
