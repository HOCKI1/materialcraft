use super::{FunctionDefinition, Program};
use materialcraft_core::Value;
use std::collections::BTreeMap;

pub(crate) const DEFINITION: FunctionDefinition = FunctionDefinition { id: "instance", label: "Function instance", inputs: &[], category: "Graph" };

pub(crate) struct FunctionCall {
    pub program: Program,
    pub inputs: Vec<String>,
    pub defaults: BTreeMap<String, Value>,
    /// Parent registers for an argument used only in scalar sums within the call.
    pub argument_factors: Option<[usize; 2]>,
    /// The called graph sets variables, directly or in a graph it calls; the caller reads them after the call
    pub exports: bool,
}

/// Wire presence is local to a call. Free variables can be defined by an executed Set in a caller.
pub(super) struct CallBindings<'a> {
    pub call: &'a FunctionCall,
    pub caller_assignments: &'a BTreeMap<String, Value>,
    pub parent: Option<&'a CallBindings<'a>>,
}

impl CallBindings<'_> {
    pub(super) fn is_defined(&self, name: &str, assignments: &BTreeMap<String, Value>) -> bool {
        if self.call.defaults.contains_key(name) {
            return self.call.inputs.iter().any(|input| input == name);
        }
        if assignments.contains_key(name) {
            return true;
        }
        let mut frame = Some(self);
        while let Some(current) = frame {
            if current.caller_assignments.contains_key(name) && current.parent.is_none_or(|parent| !parent.call.defaults.contains_key(name)) {
                return true;
            }
            frame = current.parent;
        }
        false
    }
}
