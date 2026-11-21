use materialcraft_core::Value;

use super::evaluation_state::LoopCheckpoint;

pub(crate) enum EvaluationStep {
    Enter(usize),
    Store(usize),
    Apply { instruction: usize, base: usize },
    Branch(usize),
    Discard,
    Sequence(usize),
    LoopStart(usize),
    LoopCondition { instruction: usize, iteration: u32, last: Value, checkpoint: Option<Box<LoopCheckpoint>> },
    LoopBody { instruction: usize, iteration: u32, checkpoint: Option<Box<LoopCheckpoint>> },
}
