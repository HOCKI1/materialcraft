use crate::function::Program;
use materialcraft_core::{Value, ValueType};
use std::collections::BTreeMap;

pub(super) fn declarations(program: &Program, names: &mut BTreeMap<String, ValueType>) {
    for op in &program.instructions {
        if op.definition.id == "set"
            && let Some(Value::Text(name)) = &op.value
            && let Some(kind) = ValueType::from_code(op.type_code)
        {
            names.insert(name.clone(), kind);
        }
        if let Some(call) = &op.call
            && call.exports
        {
            declarations(&call.program, names);
        }
    }
}

pub(super) struct PatternVariables<'a> {
    published: &'a BTreeMap<String, Value>,
    pub(super) drawing: BTreeMap<String, Value>,
    pub(super) modes: BTreeMap<String, Value>,
    saved: BTreeMap<String, Value>,
    scopes: Vec<(usize, Vec<String>)>,
}

impl<'a> PatternVariables<'a> {
    pub(super) fn new(published: &'a BTreeMap<String, Value>) -> Self {
        let mut drawing = published.clone();
        drawing.remove("$randomseed");
        let mut modes = drawing.clone();
        // Quadrant modes resolve before the traversal provides its position, depth and iteration.
        for name in ["$depth", "$depthpow2", "$number"] {
            modes.insert(name.into(), Value::Float(0.0));
        }
        Self { modes, drawing, published, saved: BTreeMap::new(), scopes: Vec::new() }
    }

    pub(super) fn enter(&mut self, names: &BTreeMap<String, ValueType>, declared: &mut Vec<String>) {
        for (name, kind) in names {
            if self.modes.remove(name).is_some() || !self.drawing.contains_key(name) {
                // A property that declares a name reopens its last pattern value across visits.
                let value = self.saved.remove(name).unwrap_or_else(|| kind.default_value());
                self.drawing.insert(name.clone(), value);
                declared.push(name.clone());
            }
        }
    }

    pub(super) fn keep_for_children(&mut self, pending: usize, declared: Vec<String>) {
        if !declared.is_empty() {
            self.scopes.push((pending, declared));
        }
    }

    pub(super) fn resume(&mut self, pending_after_pop: usize) {
        // Once a waiting sibling is popped, the declaring node and all its descendants have finished.
        while self.scopes.last().is_some_and(|(pending, _)| pending_after_pop < *pending) {
            if let Some((_, declared)) = self.scopes.pop() {
                for name in declared {
                    if let Some(value) = self.drawing.remove(&name) {
                        self.saved.insert(name.clone(), value);
                    }
                    if let Some(value) = self.published.get(&name) {
                        self.drawing.insert(name.clone(), value.clone());
                        self.modes.insert(name, value.clone());
                    }
                }
            }
        }
    }
}
