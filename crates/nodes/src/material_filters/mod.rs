pub const CATEGORY: &str = "Material Filters/Effects";

mod height_blend;
mod material_blend;
mod water_drops;

use crate::NodeDefinition;

pub(crate) fn defs() -> Vec<NodeDefinition> {
    vec![height_blend::definition(), material_blend::definition(), water_drops::definition()]
}

#[cfg(test)]
mod tests;
