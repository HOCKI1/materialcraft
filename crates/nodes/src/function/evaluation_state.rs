use super::evaluation_step::EvaluationStep;
use materialcraft_core::Value;
use std::collections::BTreeMap;

/// Reusable execution storage for one worker
#[derive(Default)]
pub struct EvaluationState {
    pub(crate) steps: Vec<EvaluationStep>,
    pub(crate) values: Vec<Value>,
    pub(crate) registers: Vec<Option<Value>>,
    pub(crate) variables: BTreeMap<String, Value>,
    pub(crate) assignments: BTreeMap<String, Value>,
    pub(crate) random: crate::random::FunctionRandom,
    pub(super) argument_factors: Option<[f32; 2]>,
    pub(crate) calls: BTreeMap<usize, EvaluationState>,
}

impl EvaluationState {
    pub(super) fn scalar_register(&self, index: usize) -> Option<f32> {
        match self.registers.get(index)? {
            Some(Value::Float(value)) => Some(*value),
            _ => None,
        }
    }

    pub fn variables(&self) -> &BTreeMap<String, Value> {
        &self.variables
    }

    /// Explicit Set results, excluding inherited values and function arguments.
    pub fn assignments(&self) -> &BTreeMap<String, Value> {
        &self.assignments
    }
}

/// A loop can change the random stream, the variables it writes and the assignments its caller sees.
pub(crate) struct LoopCheckpoint {
    random: crate::random::FunctionRandom,
    writes: Vec<(String, Value, Option<Value>)>,
}

impl LoopCheckpoint {
    pub(super) fn capture(state: &EvaluationState) -> Option<Box<Self>> {
        let mut writes = Vec::with_capacity(state.assignments.len());
        for (name, value) in &state.assignments {
            let visible = state.variables.get(name);
            materialcraft_core::ValueType::of(value)?;
            if let Some(visible) = visible {
                materialcraft_core::ValueType::of(visible)?;
            }
            writes.push((name.clone(), value.clone(), visible.cloned()));
        }
        Some(Box::new(Self { random: state.random, writes }))
    }

    pub(super) fn unchanged(&self, state: &EvaluationState) -> bool {
        self.random == state.random
            && self.writes.len() == state.assignments.len()
            && self.writes.iter().zip(&state.assignments).all(|((name, written, visible), (current, value))| {
                name == current && same_loop_value(Some(written), Some(value)) && same_loop_value(visible.as_ref(), state.variables.get(name))
            })
    }
}

fn same_loop_value(left: Option<&Value>, right: Option<&Value>) -> bool {
    match (left, right) {
        (None, None) => true,
        (Some(a), Some(b)) if std::mem::discriminant(a) == std::mem::discriminant(b) => match a {
            // A later operation can distinguish signed zero or inspect a float's bits.
            Value::Float(_) | Value::Float2(_) | Value::Float3(_) | Value::Float4(_) | Value::Color(_) => {
                a.as_vec4().map(|v| v.map(f32::to_bits)) == b.as_vec4().map(|v| v.map(f32::to_bits))
            }
            Value::Bool(_) | Value::Int(_) | Value::Int2(_) | Value::Int3(_) | Value::Int4(_) | Value::Text(_) | Value::Enum(_) => a == b,
            _ => false,
        },
        _ => false,
    }
}
