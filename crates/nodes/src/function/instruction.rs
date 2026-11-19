use super::FunctionDefinition;
use materialcraft_core::Value;

pub(crate) struct Instruction {
    pub id: u32,
    pub definition: &'static FunctionDefinition,
    pub inputs: Vec<usize>,
    pub value: Option<Value>,
    pub type_code: u32,
    /// Pure expression with only literal inputs. Its arithmetic rounds before pixel evaluation.
    pub literal: bool,
    /// A constant denominator folds to its rounded reciprocal even with a varying numerator.
    pub(super) literal_divisor: bool,
    pub(super) pixel_product_sum: Option<super::product_sum::ProductSum>,
    /// For a loop: the instructions of its condition and body, evaluated again on every iteration
    pub repeated: Vec<usize>,
    pub call: Option<super::function_call::FunctionCall>,
}
