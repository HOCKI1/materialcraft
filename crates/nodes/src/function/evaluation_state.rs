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
