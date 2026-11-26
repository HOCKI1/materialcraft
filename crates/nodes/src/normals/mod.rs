pub const CATEGORY: &str = "Filters/Normal map";

mod ambient_occlusion;
mod curvature;
mod normal_combine;
mod normal_invert;
mod normal_normalize;
mod normal_to_height;

use crate::NodeDefinition;

pub(crate) fn defs() -> Vec<NodeDefinition> {
    vec![
        ambient_occlusion::definition(),
        curvature::definition(),
        normal_combine::definition(),
        normal_invert::definition(),
        normal_normalize::definition(),
        normal_to_height::definition(),
    ]
}

#[cfg(test)]
mod tests;
