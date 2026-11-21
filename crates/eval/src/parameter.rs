use crate::{ResolvedBaseParameters, parameter_function::ChannelMode};
use materialcraft_core::{Node, OutputFormat, Package, Value, ValueType};
use materialcraft_nodes::{NodeDefinition, ParameterDescriptor, accepts_scalar_function, clamp_params, resolve_params};
use std::collections::BTreeMap;

pub(crate) fn resolve_parameters(
    package: &Package,
    node: &Node,
    definition: Option<&NodeDefinition>,
    variables: &mut std::borrow::Cow<'_, BTreeMap<String, Value>>,
    base: ResolvedBaseParameters,
    literal_inputs: &[String],
) -> Result<(BTreeMap<String, Value>, Vec<String>), String> {
    let mut parameters = match definition {
        Some(definition) if node.def != "graph_instance" => resolve_params(definition, &node.params),
        _ => {
            let mut parameters = node.instance.as_ref().map(|instance| instance.defaults.clone()).unwrap_or_default();
            parameters.extend(node.params.clone());
            parameters
        }
    };

    for parameter in node.bindings.keys().chain(node.functions.keys()) {
        if !parameters.contains_key(parameter)
            && let Some(descriptor) = ParameterDescriptor::for_node(package, node, parameter)
        {
            parameters.insert(parameter.clone(), descriptor.default);
        }
    }

    let mut dynamic = std::collections::BTreeSet::new();
    for (parameter, exposed) in &node.bindings {
        if node.def == "graph_instance" && !literal_inputs.contains(exposed) {
            dynamic.insert(parameter);
        }
        if let (Some(value), Some(current)) = (variables.get(exposed), parameters.get(parameter))
            && let Some(coerced) = current.coerce_json(&serde_json::to_value(value).unwrap_or_default())
        {
            parameters.insert(parameter.clone(), coerced);
        }
    }

    let mut gray_pattern_opacity = None;
    for (parameter, function) in &node.functions {
        let current = parameters.get(parameter).ok_or_else(|| format!("unknown function parameter `{parameter}`"))?;
        let expected = ValueType::of(current).ok_or_else(|| format!("parameter `{parameter}` cannot have a function"))?;
        let channels = match (node.def.as_str(), parameter.as_str(), base.format) {
            ("levels", name, format) if accepts_scalar_function("levels", name) => {
                ChannelMode::Levels { normalized: name != "in_mid" && matches!(format, OutputFormat::U8 | OutputFormat::U16 | OutputFormat::Inherit) }
            }
            ("fx_map", "background", _) if matches!(current, Value::Float4(_)) => ChannelMode::Broadcast,
            ("fx_map", "opacity", _) if parameters.get("color_mode") == Some(&Value::Bool(false)) => ChannelMode::GrayPatternOpacity,
            _ => ChannelMode::Opaque,
        };
        let (value, literal) =
            crate::parameter_function::evaluate(function, expected, variables.to_mut(), base.seed, base.size, channels, Some(current), literal_inputs)
                .map_err(|error| format!("function of `{parameter}`: {error}"))?;
        if matches!(channels, ChannelMode::GrayPatternOpacity) {
            // Keep the result in floating point until blending. JSON and slider limits lose signed infinities.
            gray_pattern_opacity = Some(value);
            continue;
        }
        if node.def == "graph_instance" {
            if literal {
                dynamic.remove(parameter);
            } else {
                dynamic.insert(parameter);
            }
        }
        let value = if matches!((&channels, &value), (ChannelMode::Levels { .. }, Value::Float(_))) {
            value
        } else {
            current.coerce_json(&serde_json::to_value(value).unwrap_or_default()).ok_or_else(|| format!("invalid function value for `{parameter}`"))?
        };
        parameters.insert(parameter.clone(), value);
    }

    // Catalog limits keep values from files and functions within safe cook bounds
    if let Some(definition) = definition
        && node.def != "graph_instance"
    {
        clamp_params(definition, &mut parameters);
    }
    if let Some(opacity) = gray_pattern_opacity {
        parameters.insert("opacity".into(), opacity);
    }

    let fixed = if node.def == "graph_instance" {
        parameters.iter().filter(|(name, value)| matches!(value, Value::Int(_)) && !dynamic.contains(*name)).map(|(name, _)| name.clone()).collect()
    } else {
        Vec::new()
    };
    Ok((parameters, fixed))
}

pub(crate) fn inspection_parameters(node: &Node, parameters: &mut BTreeMap<String, Value>) {
    parameters.retain(|name, value| {
        if let Value::Float(level) = value
            && node.def == "levels"
            && accepts_scalar_function("levels", name)
        {
            *value = Value::Float4([*level, *level, *level, 1.0]);
        }
        ValueType::of(value).is_some()
    });
}

#[cfg(test)]
mod tests {
    use crate::{Cache, Options, cook};
    use materialcraft_core::{FunctionGraph, Graph, Package, Value, ValueType};
    use std::sync::Mutex;

    #[test]
    fn gray_levels_keep_scalar_endpoints_distinct_from_equal_rgb_controls() {
        use craft_texture::Image;
        use materialcraft_core::OutputFormat;

        let mut graph = Graph::new("Endpoint precision");
        graph.size = [2, 2];
        graph.format = OutputFormat::F32;
        let source = graph.add_node("input_grayscale", [0.0; 2]);
        let levels = graph.add_node("levels", [0.0; 2]);
        graph.connect(source, "output", levels, "input").unwrap();
        let mut package = Package::new("Endpoint precision");
        package.graphs.push(graph);
        let mut options = Options::new(&|_| None);
        options.inputs.insert("input".into(), Image::gray(4, 4, |_, _| 1.0));
        let cache = Mutex::new(Cache::default());

        let scalar = f32::from_bits(0x3ec0c0c2);
        let rgba = [scalar, scalar, scalar, 1.0];
        let vector_result = f32::from_bits(0x3ec0c0c3);
        for (value, expected) in [(Value::Float4(rgba), vector_result), (Value::Float(scalar), scalar), (Value::Float4(rgba), vector_result)] {
            package.graphs[0].node_mut(levels).unwrap().functions.insert("out_high".into(), FunctionGraph::constant(value));
            let cooked = cook(&package, 0, &options, &cache);
            assert_eq!(cooked.nodes[&levels].parameters["out_high"], Value::Float4(rgba));
            assert_eq!(cooked.image(levels, 0).unwrap().get_gray(0, 0, false).to_bits(), expected.to_bits());
        }
    }

    #[test]
    fn missing_direct_level_link_keeps_the_property_value() {
        let mut graph = Graph::new("Matte");
        graph.size = [2; 2];
        let levels = graph.add_node("levels", [0.0; 2]);
        let mut package = Package::new("Links");
        package.graphs.push(graph);
        let cache = Mutex::new(Cache::default());
        let reader = FunctionGraph::input("%amount", ValueType::Float);
        let mut disconnected = reader.clone();
        disconnected.add_constant(Value::Float(0.75));
        let mut computed = reader.clone();
        let value = computed.output.unwrap();
        let offset = computed.add_constant(Value::Float(0.1875));
        computed.output = Some(computed.add("add", &[value, offset], None));

        for (function, bound, expected) in [
            (reader.clone(), None, [1.0; 4]),
            (disconnected, None, [1.0; 4]),
            (reader, Some(0.0), [0.0, 0.0, 0.0, 1.0]),
            (computed, None, [0.1875, 0.1875, 0.1875, 1.0]),
        ] {
            package.graphs[0].node_mut(levels).unwrap().functions.insert("out_high".into(), function);
            let mut options = Options::new(&|_| None);
            if let Some(level) = bound {
                options.overrides.insert("amount".into(), Value::Float(level));
            }
            let cooked = cook(&package, 0, &options, &cache);
            assert_eq!(cooked.nodes[&levels].error, None);
            assert_eq!(cooked.nodes[&levels].parameters.get("out_high"), Some(&Value::Float4(expected)));
        }
    }
}
