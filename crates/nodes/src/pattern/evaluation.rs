use materialcraft_core::{PatternNodeKind, Value};
use std::collections::{BTreeMap, BTreeSet};

use super::{catalog, pattern_stamp::PatternStamp, pattern_visit::PatternVisit, raster};
use crate::function::{EvaluationContext, EvaluationState, compile};
use crate::random::FunctionRandom;
use crate::{CookContext, CookResult};

// A 512-by-512 quadtree needs 349,525 visits including internal quadrants
// Cap deeper trees; the rasterizer separately bounds pixel work
const MAX_VISITS: usize = 1_048_576;

pub(crate) fn cook(context: &CookContext) -> CookResult {
    let Some(Value::PatternGraph(graph)) = context.params.get("pattern") else {
        return Err("missing pattern graph".into());
    };

    graph.validate()?;
    let nodes: BTreeMap<_, _> = graph.nodes.iter().map(|node| (node.id, node)).collect();
    let mut functions = BTreeMap::new();
    let mut fixed_switches = BTreeSet::new();
    for node in &graph.nodes {
        let mut fixed = node.kind == PatternNodeKind::Switch && node.children.contains(&None);
        for (name, value) in &node.parameters {
            if let Value::Function(function) = value {
                let program = compile(function, context.seed)?;
                fixed = fixed && name == "switch" && program.depends_only_on_image_size();
                functions.insert((node.id, name.as_str()), program);
            }
        }
        if fixed {
            fixed_switches.insert(node.id);
        }
    }

    let mut pending = Vec::new();
    if let Some(root) = graph.root {
        pending.push(PatternVisit { node: root, center: [0.5; 2], extent: 1.0, depth: 0, number: 0, seed: context.seed });
    }

    let mut stamps = Vec::new();

    // Keep Set assignments across iterations within this FX-Map cook
    let mut variables = context.vars.clone();
    variables.remove("$randomseed");
    let mut max_depth = 0;
    let mut state = EvaluationState::default();
    let mut remaining = MAX_VISITS;
    let mut empty_switches = BTreeSet::new();
    while let Some(mut visit) = pending.pop() {
        max_depth = max_depth.max(visit.depth);
        if empty_switches.contains(&visit.node) {
            continue;
        }
        remaining = remaining.checked_sub(1).ok_or("pattern graph exceeded its execution budget")?;
        let node = nodes.get(&visit.node).ok_or("missing pattern node")?;
        if node.kind == PatternNodeKind::Dot {
            if let Some(Some(child)) = node.children.first() {
                visit.node = *child;
                pending.push(visit);
            }
            continue;
        }

        let definition = catalog::definition(node.kind).ok_or("missing pattern definition")?;
        variables.insert("$depth".into(), Value::Float(visit.depth as f32));
        variables.insert("$depthpow2".into(), Value::Float(visit.extent));
        variables.insert("$number".into(), Value::Float(visit.number as f32));

        let mut evaluate = |id: &str, random: &mut FunctionRandom| -> Result<Option<Value>, String> {
            let Some(program) = functions.get(&(node.id, id)) else { return Ok(node.parameters.get(id).cloned()) };
            let evaluation = EvaluationContext {
                image_formats: context.input_formats.get(1..).unwrap_or(&[]),
                scope: crate::function::FunctionScope::FxMap,
                position: visit.center,
                size: [context.w as f32, context.h as f32],
                images: context.inputs.get(1..).unwrap_or(&[]),
                tile: context.tile,
                variables: &variables,
            };
            let value = program.evaluate_with_random(&evaluation, &mut state, random)?;
            variables.extend(state.assignments().clone());
            Ok(Some(value))
        };
        // The seed expression reads the incoming stream before its offset applies to this stamp and its descendants.
        let offset = evaluate("randomseed", &mut FunctionRandom::uniform(visit.seed))?.and_then(|v| v.as_i32()).unwrap_or(0) as u32;
        let inherits = evaluate("randominherited", &mut FunctionRandom::image(visit.seed))?.and_then(|v| v.as_bool()).unwrap_or(true);
        visit.seed = if inherits { visit.seed.wrapping_add(offset) } else { offset };
        let mut parameters = BTreeMap::new();
        let mut random = FunctionRandom::image(visit.seed);
        for parameter in definition.params.iter().filter(|p| !matches!(p.id, "randomseed" | "randominherited")) {
            let mut stream = FunctionRandom::pattern(visit.seed, parameter.id);
            let value = evaluate(parameter.id, stream.as_mut().unwrap_or(&mut random))?.unwrap_or_else(|| parameter.default.clone());
            parameters.insert(parameter.id, value);
        }

        let scalar = |name: &str| parameters.get(name).and_then(Value::as_f32).unwrap_or(0.0);
        let integer = |name: &str| parameters.get(name).and_then(Value::as_i32).unwrap_or(0);
        let vector = |name: &str| parameters.get(name).and_then(Value::as_vec4).map(|value| [value[0], value[1]]).unwrap_or([0.0; 2]);
        let mut children = Vec::new();
        match node.kind {
            PatternNodeKind::Dot => {}
            PatternNodeKind::Quadrant => {
                let offset = vector("branchoffset");
                visit.center = [visit.center[0] + offset[0], visit.center[1] + offset[1]];
                let pattern = integer("patterntype");
                let pattern = if pattern == 1 { integer("imageindex").max(1).saturating_add(255) } else { pattern };
                if pattern != 0 {
                    let offset = vector("frameoffset");
                    let color = match parameters.get("opacity") {
                        Some(Value::Float(value)) => [*value, *value, *value, 1.0],
                        Some(value) => value.as_vec4().ok_or("pattern color must be a float or float4")?,
                        None => [1.0; 4],
                    };

                    stamps.push(PatternStamp {
                        center: [visit.center[0] + offset[0], visit.center[1] + offset[1]],
                        size: vector("patternsize").map(|size| size * visit.extent),
                        rotation: scalar("patternrotation"),
                        color,
                        scalar_color: matches!(parameters.get("opacity"), Some(Value::Float(_))),
                        depth: visit.depth,
                        pattern,
                        variation: scalar("patternsuppl"),
                        blend: integer("blendingmode"),
                        filtering: integer("imagefiltering"),
                        premultiplied: parameters.get("imagepremul").and_then(Value::as_bool).unwrap_or(false),
                    });
                }

                let mut child_random = visit.seed;
                for (slot, child) in node.children.iter().enumerate() {
                    let child_seed = crate::random::quadrant_child(&mut child_random, visit.depth.saturating_add(1));
                    if let Some(child) = child {
                        let mut child_visit = visit.clone();
                        child_visit.node = *child;
                        child_visit.depth = visit.depth.saturating_add(1);
                        child_visit.extent *= 0.5;
                        child_visit.seed = child_seed;
                        child_visit.center = [
                            visit.center[0] + (if slot & 1 == 0 { -0.25 } else { 0.25 }) * visit.extent,
                            visit.center[1] + (if slot & 2 == 0 { -0.25 } else { 0.25 }) * visit.extent,
                        ];
                        children.push(child_visit);
                    }
                }
            }
            PatternNodeKind::Switch => {
                let slot = usize::from(parameters.get("switch").and_then(Value::as_bool).unwrap_or(false));
                if let Some(Some(child)) = node.children.get(slot) {
                    visit.node = *child;
                    children.push(visit);
                } else if fixed_switches.contains(&node.id) {
                    empty_switches.insert(node.id);
                }
            }
            PatternNodeKind::Iterate => {
                let count = usize::try_from(integer("numberadded")).map_err(|_| "iteration count cannot be negative")?;
                if count > remaining || count > 10_000 {
                    return Err("pattern iteration count exceeds the execution budget".into());
                }

                if let Some(Some(child)) = node.children.first() {
                    let mut first = visit.clone();
                    first.node = *child;
                    first.number = 0;
                    children.push(first);
                }
                if let Some(Some(child)) = node.children.get(1)
                    && !empty_switches.contains(child)
                {
                    let mut seed = visit.seed;
                    for number in 0..count {
                        let mut repeated = visit.clone();
                        repeated.node = *child;
                        repeated.number = number as u32;
                        seed = crate::random::iteration_child(seed);
                        repeated.seed = seed;
                        children.push(repeated);
                    }
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
        if children.len().saturating_add(pending.len()) > remaining {
            return Err("pattern graph exceeded its execution budget".into());
        }

        pending.extend(children.into_iter().rev());
    }

    Ok(vec![raster::render(context, &stamps, max_depth)?])
}
