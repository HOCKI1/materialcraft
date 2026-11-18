use materialcraft_core::{FunctionGraph, Value, ValueType};
use materialcraft_nodes::function::{EvaluationContext, EvaluationState, compile};
use std::collections::BTreeMap;

#[derive(Clone, Copy)]
pub(crate) enum ChannelMode {
    Opaque,
    Broadcast,
    Normalized,
}

pub(crate) fn evaluate(
    function: &FunctionGraph,
    expected: ValueType,
    variables: &mut BTreeMap<String, Value>,
    seed: u32,
    size: [u32; 2],
    channels: ChannelMode,
) -> Result<Value, String> {
    let program = compile(function, seed)?;
    let widened = expected == ValueType::Float4 && [ValueType::Float as u32, ValueType::Float3 as u32].contains(&program.output_type());
    if program.output_type() != expected as u32 && !widened {
        return Err(format!("output must be {}", expected.label()));
    }

    let context = EvaluationContext {
        image_formats: &[],
        scope: Default::default(),
        position: [0.5; 2],
        size: size.map(|side| side as f32),
        images: &[],
        tile: true,
        variables,
    };
    let mut state = EvaluationState::default();
    let value = match channels {
        ChannelMode::Normalized => program.evaluate_normalized_color(&context, &mut state)?,
        _ => program.evaluate(&context, &mut state)?,
    };
    let value = match value {
        Value::Float(level) if widened && matches!(channels, ChannelMode::Broadcast) => Value::Float4([level; 4]),
        Value::Float(level) if widened => Value::Float4([level, level, level, 1.0]),
        Value::Float3([red, green, blue]) if widened => Value::Float4([red, green, blue, 1.0]),
        value => value,
    };

    variables.extend(state.assignments().clone());

    Ok(value)
}
