pub const CATEGORY: &str = "Filters/Normal map";

mod ambient_occlusion;
mod curvature;
mod normal_combine;
mod normal_invert;

use crate::NodeDefinition;

pub(crate) fn defs() -> Vec<NodeDefinition> {
    vec![ambient_occlusion::definition(), curvature::definition(), normal_combine::definition(), normal_invert::definition()]
}

#[cfg(test)]
mod tests;
