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
) -> Result<BTreeMap<String, Value>, String> {
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

    for (parameter, exposed) in &node.bindings {
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
        let (value, _) = crate::parameter_function::evaluate(function, expected, variables.to_mut(), base.seed, base.size, channels)
            .map_err(|error| format!("function of `{parameter}`: {error}"))?;
        let value = current.coerce_json(&serde_json::to_value(value).unwrap_or_default()).ok_or_else(|| format!("invalid function value for `{parameter}`"))?;
        parameters.insert(parameter.clone(), value);
    }

    // Catalog limits keep values from files and functions within safe cook bounds
    if let Some(definition) = definition
        && node.def != "graph_instance"
    {
        clamp_params(definition, &mut parameters);
    }

    Ok(parameters)
}
