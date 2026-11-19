use super::{EvaluationContext, EvaluationState, evaluation_step::EvaluationStep, instruction::Instruction, operation, value_type};
use crate::random::FunctionRandom;
use materialcraft_core::Value;

/// Validated graph with input ids resolved to instruction indices
pub struct Program {
    pub(crate) instructions: Vec<Instruction>,
    pub(crate) output: usize,
    pub(crate) seed: u32,
}

const MAX_EVALUATION_STEPS: usize = 100_000;

impl Program {
    pub fn output_type(&self) -> u32 {
        self.instructions.get(self.output).map_or(0, |instruction| instruction.type_code)
    }

    /// The variable read by a direct link. Disconnected editor nodes have no instructions.
    pub fn input_identifier(&self) -> Option<&str> {
        let [op] = self.instructions.as_slice() else { return None };
        match &op.value {
            Some(Value::Text(name)) if op.definition.id.starts_with("get_") => Some(name),
            _ => None,
        }
    }

    /// Whether the result depends only on literals and pure operations.
    pub fn output_is_literal(&self) -> bool {
        self.instructions.get(self.output).is_some_and(|instruction| instruction.literal)
    }

    /// A pattern can reuse a size predicate across positions, random streams and variable assignments.
    pub(crate) fn depends_only_on_image_size(&self) -> bool {
        self.instructions.iter().all(|op| {
            op.definition.id.starts_with("const_")
                || op.depends_only_on_operands()
                || (op.definition.id == "get_float2" && matches!(&op.value, Some(Value::Text(name)) if matches!(name.as_str(), "$size" | "$sizelog2")))
        })
    }

    pub fn node_type(&self, id: u32) -> Option<u32> {
        self.instructions.iter().find(|instruction| instruction.id == id).map(|instruction| instruction.type_code)
    }

    fn instruction(&self, index: usize) -> Result<&Instruction, String> {
        self.instructions.get(index).ok_or_else(|| "invalid function instruction".into())
    }

    fn input(&self, index: usize, slot: usize) -> Result<usize, String> {
        self.instruction(index)?.inputs.get(slot).copied().ok_or_else(|| "missing function input".into())
    }

    pub fn evaluate(&self, context: &EvaluationContext<'_>, state: &mut EvaluationState) -> Result<Value, String> {
        let mut random = self.random_stream(context);
        self.evaluate_with_random(context, state, &mut random)
    }

    /// Calls within an expression consume its selected random stream.
    pub(crate) fn evaluate_with_random(
        &self,
        context: &EvaluationContext<'_>,
        state: &mut EvaluationState,
        random: &mut FunctionRandom,
    ) -> Result<Value, String> {
        let value = self.run_with_random(context, state, random)?;
        // Callers may clamp undefined intermediates; completed functions must be finite
        if value.as_vec4().is_some_and(|components| components.iter().any(|value| !value.is_finite())) {
            return Err("function produced a non-finite output".into());
        }
        Ok(value)
    }

    /// Evaluate an integer texture's channel control at its normalized storage boundary.
    pub fn evaluate_normalized_color(&self, context: &EvaluationContext<'_>, state: &mut EvaluationState) -> Result<Value, String> {
        let mut random = self.random_stream(context);
        let channels = match self.run_with_random(context, state, &mut random)? {
            Value::Float(v) => [v, v, v, 1.0],
            Value::Float3([r, g, b]) => [r, g, b, 1.0],
            Value::Float4(rgba) | Value::Color(rgba) => rgba,
            _ => return Err("channel control requires a gray level, RGB or RGBA".into()),
        };
        Ok(Value::Float4(channels.map(|v| if v.is_nan() { 0.0 } else { v.clamp(0.0, 1.0) })))
    }

    fn run_with_random(&self, context: &EvaluationContext<'_>, state: &mut EvaluationState, random: &mut FunctionRandom) -> Result<Value, String> {
        state.variables.clone_from(context.variables);
        state.random = *random;
        let mut remaining = MAX_EVALUATION_STEPS;
        let result = self.run(context, state, &mut remaining);
        *random = state.random;
        result
    }

    fn random_stream(&self, context: &EvaluationContext<'_>) -> FunctionRandom {
        let seed = context.variables.get("$randomseed").and_then(Value::as_i32).map_or(self.seed, |seed| seed as u32);
        if context.scope == super::FunctionScope::PixelProcessor {
            FunctionRandom::pixel(seed, context.position)
        } else if context.scope == super::FunctionScope::FxMap {
            FunctionRandom::image(seed)
        } else {
            FunctionRandom::uniform(seed)
        }
    }

    // Pixel assignments are local; only the general evaluator exports the complete environment
    pub(super) fn evaluate_pixel(&self, context: &EvaluationContext<'_>, state: &mut EvaluationState) -> Result<Value, String> {
        state.variables.clear();
        state.random = self.random_stream(context);
        let mut remaining = MAX_EVALUATION_STEPS;
        self.run(context, state, &mut remaining)
    }

    fn run(&self, context: &EvaluationContext<'_>, state: &mut EvaluationState, remaining_steps: &mut usize) -> Result<Value, String> {
        use EvaluationStep::*;

        state.steps.clear();
        state.values.clear();
        state.assignments.clear();
        state.registers.clear();
        state.registers.resize(self.instructions.len(), None);
        state.steps.push(Enter(self.output));

        while let Some(step) = state.steps.pop() {
            *remaining_steps = remaining_steps.checked_sub(1).ok_or("function exceeded its execution budget; check loop termination")?;
            match step {
                Enter(index) => {
                    let instruction = self.instruction(index)?;
                    if let Some(Some(value)) = state.registers.get(index) {
                        state.values.push(value.clone());
                        continue;
                    }

                    state.steps.push(Store(index));

                    match instruction.definition.id {
                        "ifelse" => {
                            state.steps.push(Branch(index));
                            state.steps.push(Enter(self.input(index, 0)?));
                        }
                        "sequence" => {
                            state.steps.push(Sequence(index));
                            state.steps.push(Enter(self.input(index, 0)?));
                        }
                        "while" => {
                            state.steps.push(LoopStart(index));
                            state.steps.push(Enter(self.input(index, 0)?));
                        }
                        _ => {
                            state.steps.push(Apply { instruction: index, base: state.values.len() });
                            let inputs = match &instruction.pixel_product_sum {
                                Some(product) if context.scope == super::FunctionScope::PixelProcessor => product.inputs.as_slice(),
                                _ => instruction.inputs.as_slice(),
                            };
                            state.steps.extend(inputs.iter().rev().map(|&index| Enter(index)));
                        }
                    }
                }
                Apply { instruction: index, base } => {
                    let instruction = self.instruction(index)?;
                    let arguments = state.values.get(base..).ok_or("invalid function operands")?;
                    let result = if context.scope == super::FunctionScope::PixelProcessor
                        && let Some(value) = instruction.pixel_interval.as_deref().and_then(|interval| interval.saturate(state))
                    {
                        Ok(value)
                    } else if let Some(product) = &instruction.pixel_product_sum
                        && context.scope == super::FunctionScope::PixelProcessor
                    {
                        product.evaluate(arguments, instruction.definition.id == "sub", state)
                    } else if let Some(call) = &instruction.call {
                        let factors = if let Some([a, b]) = call.argument_factors.filter(|_| context.scope == super::FunctionScope::PixelProcessor) {
                            let factor = |i| state.scalar_register(i).ok_or("missing scalar argument factor");
                            Some([factor(a)?, factor(b)?])
                        } else {
                            None
                        };
                        let storage = state.calls.entry(index).or_default();
                        storage.argument_factors = factors;
                        storage.variables.clone_from(&state.variables);
                        storage.variables.extend(call.defaults.clone());
                        for (name, value) in call.inputs.iter().zip(arguments) {
                            storage.variables.insert(name.clone(), value.clone());
                        }

                        // A function call consumes its caller's stream instead of restarting it
                        storage.random = state.random;
                        let result = call.program.run(context, storage, remaining_steps);
                        state.random = storage.random;
                        if call.exports && result.is_ok() {
                            // Set publishes into the caller's scope; arguments and defaults stay local
                            state.variables.extend(storage.assignments.clone());
                            state.assignments.extend(storage.assignments.clone());
                        }

                        result
                    } else {
                        operation::evaluate(instruction, arguments, &mut state.variables, &mut state.random, context)
                    }
                    .map_err(|error| format!("{} ({}): {error}", instruction.definition.label, instruction.id))?;

                    if instruction.definition.id == "set"
                        && let Some(Value::Text(name)) = &instruction.value
                    {
                        state.assignments.insert(name.clone(), result.clone());
                    }

                    state.values.truncate(base);
                    state.values.push(result);
                }
                Store(index) => {
                    let value = state.values.last().ok_or("missing function result")?;
                    let register = state.registers.get_mut(index).ok_or("invalid function register")?;
                    *register = Some(value.clone());
                }
                Branch(index) => {
                    let condition = state.values.pop().ok_or("missing branch condition")?;
                    let slot = if value_type::boolean(&condition)? { 1 } else { 2 };

                    state.steps.push(Enter(self.input(index, slot)?));
                    if context.scope == super::FunctionScope::PixelProcessor {
                        for &draw in self.instruction(index)?.control_registers.iter().rev() {
                            if state.registers.get(draw).is_some_and(Option::is_none) {
                                state.steps.push(Discard);
                                state.steps.push(Enter(draw));
                            }
                        }
                    }
                }
                Discard => {
                    state.values.pop().ok_or("missing shared branch draw")?;
                }
                Sequence(index) => {
                    state.values.pop().ok_or("missing sequence result")?;
                    state.steps.push(Enter(self.input(index, 1)?));
                }
                LoopStart(index) => {
                    let initializer = state.values.pop().ok_or("missing loop initialization")?;
                    // The initializer may assign a different type. A skipped body has its own typed zero.
                    let last = materialcraft_core::ValueType::from_code(self.instruction(index)?.type_code)
                        .map_or(initializer, materialcraft_core::ValueType::default_value);
                    self.clear_loop_results(index, state)?;
                    state.steps.push(LoopCondition { instruction: index, iteration: 0, last });
                    state.steps.push(Enter(self.input(index, 1)?));
                }
                LoopCondition { instruction: index, iteration, last } => {
                    let exit = value_type::boolean(&state.values.pop().ok_or("missing loop condition")?)?;
                    let maximum = self.instruction(index)?.value.as_ref().and_then(Value::as_i32).unwrap_or(super::DEFAULT_LOOP_LIMIT);
                    if exit || (maximum >= 0 && iteration >= maximum as u32) {
                        state.values.push(last);
                    } else {
                        self.clear_loop_results(index, state)?;
                        state.steps.push(LoopBody { instruction: index, iteration: iteration.saturating_add(1) });
                        state.steps.push(Enter(self.input(index, 2)?));
                    }
                }
                LoopBody { instruction: index, iteration } => {
                    let last = state.values.pop().ok_or("missing loop body result")?;
                    self.clear_loop_results(index, state)?;
                    state.steps.push(LoopCondition { instruction: index, iteration, last });
                    state.steps.push(Enter(self.input(index, 1)?));
                }
            }
        }

        let result = state.values.pop().ok_or("function returned no value")?;
        let expected = self.instruction(self.output)?.type_code;
        if value_type::type_code(&result) != expected {
            return Err(format!("function returned {}, expected native type {expected}", result.type_name()));
        }

        Ok(result)
    }

    fn clear_loop_results(&self, index: usize, state: &mut EvaluationState) -> Result<(), String> {
        // Outgoing wires share a node value across reads, branches and effects
        // Only the active loop condition and body run again
        for repeated in &self.instruction(index)?.control_registers {
            if let Some(register) = state.registers.get_mut(*repeated) {
                *register = None;
            }
        }

        Ok(())
    }
}
