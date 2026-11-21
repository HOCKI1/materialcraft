use materialcraft_core::{PatternNodeKind, Value};
use std::collections::{BTreeMap, BTreeSet};

use super::{catalog, footprint::SampleGrid, pattern_stamp::PatternStamp, pattern_visit::PatternVisit, raster, variables::PatternVariables};
use crate::function::{EvaluationContext, EvaluationState, compile_with_literal_inputs};
use crate::random::FunctionRandom;
use crate::{CookContext, CookResult};

// A 512-by-512 quadtree takes 349,525 visits. Each discarded stamp can fund one extra visit,
// up to this same limit again. Retained stamps, raster copies and pixel work stay bounded.
const MAX_VISITS: usize = 1_048_576;

pub(crate) fn cook(context: &CookContext) -> CookResult {
    let Some(Value::PatternGraph(graph)) = context.params.get("pattern") else {
        return Err("missing pattern graph".into());
    };

    graph.validate()?;
    let mut nodes = BTreeMap::new();
    let mut functions = BTreeMap::new();
    let mut fixed_switches = BTreeSet::new();
    for node in &graph.nodes {
        let mut fixed = node.kind == PatternNodeKind::Switch && node.children.contains(&None);
        let mut literal_geometry = true;
        for (name, value) in &node.parameters {
            if let Value::Function(function) = value {
                let program = compile_with_literal_inputs(function, context.seed, context.vars.literal_inputs())?;
                if matches!(name.as_str(), "patternsize" | "patternrotation") {
                    literal_geometry &= program.output_is_literal();
                }
                fixed = fixed && name == "switch" && program.depends_only_on_image_size();
                let mut names = BTreeMap::new();
                super::variables::declarations(&program, &mut names);
                functions.insert((node.id, name.as_str()), (program, names));
            }
        }
        if fixed {
            fixed_switches.insert(node.id);
        }
        nodes.insert(node.id, (node, literal_geometry));
    }

    let image_slots: Vec<_> =
        context.inputs.iter().skip(1).take(crate::MAX_IMAGE_INPUTS).enumerate().filter_map(|(i, image)| image.as_ref().map(|_| i as i32)).collect();
    let mut pending = Vec::new();
    if let Some(root) = graph.root {
        pending.push((PatternVisit { node: root, center: [0.5; 2], extent: 1.0, depth: 0, number: 0, seed: context.seed }, 0_u32));
    }

    let mut stamps = Vec::new();
    let mut root_children_pending = 0;
    let mut branch_starts = [None; 4];

    let mut variables = PatternVariables::new(context.vars.graph_values());
    let mut max_depth = 0;
    let mut state = EvaluationState::default();
    let mut remaining = MAX_VISITS;
    let mut discard_allowance = MAX_VISITS;
    let grid = SampleGrid::new(context, graph.nodes.len());
    let mut parameter_values = Vec::new();
    let mut children = Vec::new();
    let mut empty_switches = BTreeSet::new();
    let grayscale = !context.b("color_mode");
    while let Some((mut visit, repeats_left)) = pending.pop() {
        // Returning below the previous stack size starts the next child of the root.
        if pending.len() < root_children_pending {
            root_children_pending = pending.len();
            branch_starts[root_children_pending] = Some(stamps.len());
        }
        variables.resume(pending.len());
        max_depth = max_depth.max(visit.depth);
        if empty_switches.contains(&visit.node) {
            continue;
        }
        remaining = remaining.checked_sub(1).ok_or("pattern graph exceeded its execution budget")?;
        if repeats_left != 0 {
            // Resume the previous draw's scope before scheduling its sibling. Descendants of
            // this draw run first and inherit its own number and seed.
            let next = PatternVisit { number: visit.number + 1, seed: crate::random::iteration_child(visit.seed), ..visit.clone() };
            pending.push((next, repeats_left - 1));
        }
        let &(node, literal_geometry) = nodes.get(&visit.node).ok_or("missing pattern node")?;
        if node.kind == PatternNodeKind::Dot {
            if let Some(Some(child)) = node.children.first() {
                visit.node = *child;
                pending.push((visit, 0));
            }
            continue;
        }

        let definition = catalog::definition(node.kind).ok_or("missing pattern definition")?;
        for (name, value) in [("$depth", visit.depth as f32), ("$depthpow2", visit.extent), ("$number", visit.number as f32)] {
            if let Some(bound) = variables.drawing.get_mut(name) {
                *bound = Value::Float(value);
            } else {
                variables.drawing.insert(name.into(), Value::Float(value));
            }
        }
        let mut declared = Vec::new();

        let mut evaluate = |id: &str, random: &mut FunctionRandom| -> Result<Option<Value>, String> {
            let Some((program, names)) = functions.get(&(node.id, id)) else { return Ok(node.parameters.get(id).cloned()) };
            let mode =
                node.kind == PatternNodeKind::Quadrant && matches!(id, "patterntype" | "blendingmode" | "imagefiltering" | "imagepremul" | "randominherited");
            if !mode {
                variables.enter(names, &mut declared);
            }
            let inputs = if mode { &variables.modes } else { &variables.drawing };
            // An unbound direct link leaves the field at its default. A getter inside an expression still evaluates to its typed zero.
            if program.input_identifier().is_some_and(|name| !name.starts_with('$') && !inputs.contains_key(name)) {
                return Ok(None);
            }
            let evaluation = EvaluationContext {
                image_formats: context.input_formats.get(1..).unwrap_or(&[]),
                scope: crate::function::FunctionScope::FxMap,
                position: if mode { [0.0; 2] } else { visit.center },
                size: [context.w as f32, context.h as f32],
                images: context.inputs.get(1..).unwrap_or(&[]),
                tile: context.tile,
                variables: inputs,
            };
            let value = program.evaluate_with_random(&evaluation, &mut state, random)?;
            // A quadrant's mode expressions keep their own writes. A later stamp property can read writes from the draw expressions.
            if !mode {
                for (name, value) in state.assignments() {
                    if let Some(bound) = variables.drawing.get_mut(name) {
                        *bound = value.clone();
                    }
                }
            }
            Ok(Some(value))
        };
        // The seed expression reads the incoming stream before its offset applies to this stamp and its descendants.
        let offset = evaluate("randomseed", &mut FunctionRandom::uniform(visit.seed))?.and_then(|v| v.as_i32()).unwrap_or(0) as u32;
        let inherits = evaluate("randominherited", &mut FunctionRandom::image(visit.seed))?.and_then(|v| v.as_bool()).unwrap_or(true);
        visit.seed = if inherits { visit.seed.wrapping_add(offset) } else { offset };
        parameter_values.clear();
        let mut random = FunctionRandom::image(visit.seed);
        let mut pattern = if node.kind == PatternNodeKind::Quadrant { evaluate("patterntype", &mut random)?.and_then(|v| v.as_i32()).unwrap_or(0) } else { 0 };
        let image_pattern = pattern == 1 || (256..256 + crate::MAX_IMAGE_INPUTS as i32).contains(&pattern);
        // An unused stamp property cannot publish a Set to a child. The branch offset still positions that child.
        let active = |id: &str| match id {
            "patterntype" | "randomseed" | "randominherited" => false,
            "branchoffset" => true,
            "imageindex" | "imagefiltering" | "imagepremul" => image_pattern,
            _ => node.kind != PatternNodeKind::Quadrant || pattern != 0,
        };
        for parameter in definition.params.iter().filter(|p| active(p.id)) {
            let mut stream = FunctionRandom::pattern(visit.seed, parameter.id);
            let value = evaluate(parameter.id, stream.as_mut().unwrap_or(&mut random))?.unwrap_or_else(|| parameter.default.clone());
            parameter_values.push((parameter.id, value));
        }

        let field = |name: &str| parameter_values.iter().find_map(|(id, value)| (*id == name).then_some(value));
        let scalar = |name: &str| field(name).and_then(Value::as_f32).unwrap_or(0.0);
        let integer = |name: &str| field(name).and_then(Value::as_i32).unwrap_or(0);
        let vector = |name: &str| field(name).and_then(Value::as_vec4).map(|value| [value[0], value[1]]).unwrap_or([0.0; 2]);
        children.clear();
        let mut repeated = None;
        match node.kind {
            PatternNodeKind::Dot => {}
            PatternNodeKind::Quadrant => {
                let offset = vector("branchoffset");
                visit.center = [visit.center[0] + offset[0], visit.center[1] + offset[1]];
                if image_pattern {
                    let requested = integer("imageindex");
                    let slot = if functions.get(&(node.id, "imageindex")).is_some_and(|(program, _)| !program.uses_fixed_image_slot(&variables.drawing)) {
                        // A runtime selector advances across disconnected slots and stops at the last wired image.
                        let pos = image_slots.partition_point(|&i| i < requested);
                        image_slots.get(pos).or_else(|| image_slots.last()).copied()
                    } else {
                        // A fixed zero preserves the packed slot. The generic image pattern starts at slot zero.
                        (requested >= 0).then_some(if requested == 0 { pattern.max(256) - 256 } else { requested })
                    };
                    pattern = slot.map_or(0, |i| i.saturating_add(256));
                }
                if pattern != 0 {
                    let offset = vector("frameoffset");
                    let tint = field("opacity").unwrap_or(&Value::Float4([1.0; 4]));
                    let rgba = tint.as_vec4().ok_or("pattern color must be a float or float4")?;
                    if !rgba.iter().all(|v| v.is_finite()) {
                        return Err("pattern color must be finite".into());
                    }
                    let color = match tint {
                        Value::Float(luminosity) => [1.0, 1.0, 1.0, *luminosity],
                        _ if grayscale => {
                            // A color expression supplies its alpha. A literal swatch supplies its mean RGB.
                            let luminosity = if functions.contains_key(&(node.id, "opacity")) { rgba[3] } else { (rgba[0] + rgba[1] + rgba[2]) / 3.0 };
                            [1.0, 1.0, 1.0, luminosity]
                        }
                        _ => rgba,
                    };

                    let stamp = PatternStamp {
                        center: [visit.center[0] + offset[0], visit.center[1] + offset[1]],
                        size: vector("patternsize").map(|size| size * visit.extent),
                        rotation: scalar("patternrotation"),
                        literal_geometry,
                        color,
                        depth: visit.depth,
                        pattern,
                        variation: scalar("patternsuppl"),
                        blend: integer("blendingmode"),
                        filtering: integer("imagefiltering"),
                        premultiplied: field("imagepremul").and_then(Value::as_bool).unwrap_or(false),
                    };
                    // Evaluate every property before discarding a stamp. Its assignments can reach
                    // later visits. Keep the first stamp so raster setup still validates depth weights.
                    if !stamps.is_empty() && grid.as_ref().is_some_and(|grid| grid.misses(&stamp)) {
                        if discard_allowance > 0 {
                            discard_allowance -= 1;
                            remaining += 1;
                        }
                    } else {
                        stamps.push(stamp);
                    }
                }

                let mut child_random = visit.seed;
                for (child, center) in node.children.iter().zip(visit.quadrant_centers()) {
                    let child_seed = crate::random::quadrant_child(&mut child_random, visit.depth.saturating_add(1));
                    if let Some(child) = child {
                        let mut child_visit = visit.clone();
                        child_visit.node = *child;
                        child_visit.depth = visit.depth.saturating_add(1);
                        child_visit.extent *= 0.5;
                        child_visit.seed = child_seed;
                        child_visit.center = center;
                        children.push(child_visit);
                    }
                }
            }
            PatternNodeKind::Switch => {
                let slot = usize::from(field("switch").and_then(Value::as_bool).unwrap_or(false));
                if let Some(Some(child)) = node.children.get(slot) {
                    visit.node = *child;
                    children.push(visit);
                } else if fixed_switches.contains(&node.id) {
                    empty_switches.insert(node.id);
                }
            }
            PatternNodeKind::Iterate => {
                let count = u32::try_from(integer("numberadded")).map_err(|_| "iteration count cannot be negative")?;

                if let Some(Some(child)) = node.children.first() {
                    let mut first = visit.clone();
                    first.node = *child;
                    first.number = 0;
                    children.push(first);
                }
                if let Some(repeats_left) = count.checked_sub(1)
                    && let Some(Some(child)) = node.children.get(1)
                    && !empty_switches.contains(child)
                {
                    // Each connected visit costs at least one unit, even if its stamp is discarded.
                    // A fixed switch can discover an empty branch on its first visit and skip the rest.
                    if count as usize > remaining + discard_allowance && !fixed_switches.contains(child) {
                        return Err("pattern iteration count exceeds the execution budget".into());
                    }
                    let first = PatternVisit { node: *child, number: 0, seed: crate::random::iteration_child(visit.seed), ..visit };
                    repeated = Some((first, repeats_left));
                }
            }
        }

        if !empty_switches.is_empty() {
            children.retain(|child| {
                if empty_switches.contains(&child.node) {
                    // Empty branches still contribute their visited depth to the roughness weights.
                    max_depth = max_depth.max(child.depth);
                    false
                } else {
                    true
                }
            });
        }
        if children.len() + pending.len() + usize::from(repeated.is_some()) > remaining {
            return Err("pattern graph exceeded its execution budget".into());
        }

        variables.keep_for_children(pending.len(), declared);
        if grayscale && node.kind == PatternNodeKind::Quadrant && Some(node.id) == graph.root {
            root_children_pending = children.len();
        }
        pending.extend(repeated);
        pending.extend(children.drain(..).rev().map(|visit| (visit, 0)));
    }

    let mut groups = [&[][..]; 5];
    let mut prefix = stamps.as_slice();
    if max_depth == 6 {
        // At six subdivisions the root draws first, then its child streams composite
        // in reverse slot order. Within a stream the original draw order is retained.
        for (group, start) in groups[1..].iter_mut().zip(branch_starts.into_iter().flatten()) {
            let (earlier, branch) = prefix.split_at(start);
            *group = branch;
            prefix = earlier;
        }
    }
    groups[0] = prefix;
    Ok(vec![raster::render(context, groups, max_depth)?])
}
