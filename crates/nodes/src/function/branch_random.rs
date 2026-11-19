use super::Program;

/// Schedule draws shared by a pixel branch and another part of its expression. An arm's
/// private draws stay lazy. The condition itself always runs first.
pub(super) fn identify(program: &mut Program) {
    let instructions = &program.instructions;
    if !instructions.iter().any(|op| op.definition.id == "ifelse")
        || !instructions.iter().any(|op| op.definition.id == "rand")
        || instructions.iter().any(|op| matches!(op.definition.id, "while" | "set") || op.call.as_ref().is_some_and(|call| call.exports))
    {
        return;
    }

    // A range depending on assignment or image evaluation must stay at its original read.
    let mut simple = Vec::with_capacity(instructions.len());
    for op in instructions {
        simple.push(op.type_code == 256 && (op.definition.id == "const_float1" || (op.definition.id == "rand" && op.inputs.iter().all(|&i| simple[i]))));
    }
    let mut paths = vec![0_u8; instructions.len()];
    let mut stack = Vec::new();
    let mut draws = Vec::new();
    let mut plans = Vec::new();
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
                if paths[index] == 0 && path != 4 && simple[index] && node.definition.id == "rand" && !node.inputs.is_empty() {
                    draws.push(index);
                }
                paths[index] |= path;
                stack.extend(node.inputs.iter().rev().copied());
            }
        }
        let shared: Vec<_> = draws.iter().copied().filter(|&i| paths[i].count_ones() > 1).collect();
        if !shared.is_empty() {
            plans.push((branch, shared));
        }
    }
    for (index, shared) in plans {
        program.instructions[index].control_registers = shared;
    }
}
