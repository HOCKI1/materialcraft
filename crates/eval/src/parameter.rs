use crate::{ResolvedBaseParameters, parameter_function::ChannelMode};
use materialcraft_core::{Node, OutputFormat, Package, Value, ValueType};
use materialcraft_nodes::{NodeDefinition, ParameterDescriptor, clamp_params, resolve_params};
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

    for (parameter, function) in &node.functions {
        let current = parameters.get(parameter).ok_or_else(|| format!("unknown function parameter `{parameter}`"))?;
        let expected = ValueType::of(current).ok_or_else(|| format!("parameter `{parameter}` cannot have a function"))?;
        let channels = match (node.def.as_str(), parameter.as_str(), base.format) {
            ("levels", "in_low" | "in_high" | "out_low" | "out_high", OutputFormat::U8 | OutputFormat::U16 | OutputFormat::Inherit) => ChannelMode::Normalized,
            ("fx_map", "background", _) if matches!(current, Value::Float4(_)) => ChannelMode::Broadcast,
            _ => ChannelMode::Opaque,
        };
        let (value, literal) =
            crate::parameter_function::evaluate(function, expected, variables.to_mut(), base.seed, base.size, channels, Some(current), literal_inputs)
                .map_err(|error| format!("function of `{parameter}`: {error}"))?;
        if node.def == "graph_instance" {
            if literal {
                dynamic.remove(parameter);
            } else {
                dynamic.insert(parameter);
            }
        }
        let value = current.coerce_json(&serde_json::to_value(value).unwrap_or_default()).ok_or_else(|| format!("invalid function value for `{parameter}`"))?;
        parameters.insert(parameter.clone(), value);
    }

    // Catalog limits keep values from files and functions within safe cook bounds
    if let Some(definition) = definition
        && node.def != "graph_instance"
    {
        clamp_params(definition, &mut parameters);
    }

    let fixed = if node.def == "graph_instance" {
        parameters.iter().filter(|(name, value)| matches!(value, Value::Int(_)) && !dynamic.contains(*name)).map(|(name, _)| name.clone()).collect()
    } else {
        Vec::new()
    };
    Ok((parameters, fixed))
}

#[cfg(test)]
mod tests {
    use crate::{Cache, Options, cook};
    use materialcraft_core::{FunctionGraph, Graph, Package, Value, ValueType};
    use std::sync::Mutex;

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
