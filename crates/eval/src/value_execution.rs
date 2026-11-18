use craft_texture::Image;
use materialcraft_core::{OutputFormat, Value};
use std::collections::BTreeMap;

pub(crate) fn evaluate_value(
    parameters: &BTreeMap<String, Value>,
    variables: &BTreeMap<String, Value>,
    seed: u32,
    size: [u32; 2],
    images: &[Option<Image>],
    image_formats: &[OutputFormat],
    tile: bool,
) -> Result<Value, String> {
    use materialcraft_nodes::function::{EvaluationContext, EvaluationState, FunctionScope, compile};

    let Some(Value::Function(graph)) = parameters.get("function") else {
        return Err("missing value function".into());
    };

    let program = compile(graph, seed)?;
    let context = EvaluationContext {
        image_formats,
        scope: FunctionScope::ValueProcessor,
        position: [0.0; 2],
        size: size.map(|side| side as f32),
        images,
        tile,
        variables,
    };

    program.evaluate(&context, &mut EvaluationState::default())
}
