use super::{Program, vector::PatternComponent};
use materialcraft_core::Value;
use std::collections::{BTreeMap, BTreeSet};

pub(super) struct PatternDraws {
    shared: Vec<usize>,
    true_count: usize,
}

impl PatternDraws {
    // The true arm occurs first. A skipped arm still consumes any draw needed outside it.
    pub(super) fn skipped(&self, take_true: bool) -> &[usize] {
        let (through_true, through_false_only) = self.shared.split_at(self.true_count);
        if take_true { through_false_only } else { through_true }
    }
}

/// Schedule draws shared by an image branch and another part of its expression. An arm's
/// private draws stay lazy. The condition itself always runs first.
pub(super) fn identify(program: &mut Program) {
    let instructions = &program.instructions;
    if !instructions.iter().any(|op| op.definition.id == "ifelse")
        || !instructions.iter().any(|op| op.definition.id == "rand")
        || instructions.iter().any(|op| op.definition.id == "while" || op.call.as_ref().is_some_and(|call| call.exports))
    {
        return;
    }

    let written: BTreeSet<_> = instructions
        .iter()
        .filter_map(|op| match (&op.value, op.definition.id) {
            (Some(Value::Text(name)), "set") => Some(name.as_str()),
            _ => None,
        })
        .collect();
    // A range depending on an assignment or image sample must stay at its original read.
    let mut simple = Vec::with_capacity(instructions.len());
    let mut pattern_range = Vec::with_capacity(instructions.len());
    for op in instructions {
        simple.push(op.type_code == 256 && (op.definition.id == "const_float1" || (op.definition.id == "rand" && op.inputs.iter().all(|&i| simple[i]))));
        let fixed_input = op.definition.id == "get_float1" && matches!(&op.value, Some(Value::Text(name)) if !written.contains(name.as_str()));
        pattern_range.push(op.type_code == 256 && (op.literal || fixed_input || (op.definition.id == "rand" && op.inputs.iter().all(|&i| pattern_range[i]))));
    }
    let pixel = if written.is_empty() { shared_draws(program, &simple, None) } else { BTreeMap::new() };
    // A called graph can observe the caller's assignments without listing them among its inputs.
    if written.is_empty() || instructions.iter().all(|op| op.call.is_none()) {
        program.pattern_draws = shared_draws(program, &pattern_range, Some(&program.pattern_components));
    }
    for (index, draws) in pixel {
        program.instructions[index].control_registers = draws.shared;
    }
}

fn shared_draws(program: &Program, ranges: &[bool], components: Option<&BTreeMap<usize, PatternComponent>>) -> BTreeMap<usize, PatternDraws> {
    let instructions = &program.instructions;
    let mut paths = vec![0_u8; instructions.len()];
    let mut stack = Vec::new();
    let mut draws = Vec::new();
    let mut plans = BTreeMap::new();
    for (branch, op) in instructions.iter().enumerate().filter(|(_, op)| op.definition.id == "ifelse") {
        let &[_, yes, no] = op.inputs.as_slice() else { continue };
        paths.fill(0);
        draws.clear();
        // The first visit through the true arm sets the order when both arms use multiple draws.
        for (root, path) in [(yes, 1), (no, 2), (program.output, 4)] {
            if path == 4 && draws.is_empty() {
                break;
            }
            stack.push(root);
            while let Some(index) = stack.pop() {
                if index == branch || paths[index] & path != 0 {
                    continue;
                }
                let node = &instructions[index];
                if paths[index] == 0 && path != 4 && ranges[index] && node.definition.id == "rand" && !node.inputs.is_empty() {
                    draws.push(index);
                }
                paths[index] |= path;
                match components.and_then(|components| components.get(&index)) {
                    Some(PatternComponent::Scalar(source)) => stack.push(*source),
                    Some(PatternComponent::Zero) => {}
                    None => {
                        let inputs = node
                            .call
                            .as_ref()
                            .and_then(|call| call.pattern_arguments.as_ref())
                            .filter(|plan| components.is_some() && matches!(plan.target, super::function_call::PatternTarget::ReturnedChannels))
                            .map_or(node.inputs.as_slice(), |plan| plan.registers.as_slice());
                        stack.extend(inputs.iter().rev().copied());
                    }
                }
            }
        }
        let shared: Vec<_> = draws.iter().copied().filter(|&i| paths[i].count_ones() > 1).collect();
        if !shared.is_empty() {
            let true_count = shared.iter().take_while(|&&i| paths[i] & 1 != 0).count();
            plans.insert(branch, PatternDraws { shared, true_count });
        }
    }
    plans
}
