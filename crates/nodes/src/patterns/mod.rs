pub const CATEGORY: &str = "Texture generators/Patterns";

mod brick;
mod checker;
mod gradient_axial;
mod gradient_circular;
mod gradient_linear;
mod hexagon;
mod polygon;
mod scratches;
mod shape;
mod splatter_circular;
mod star;
mod stripes;
mod tile_generator;
mod tile_sampler;
mod weave;

use crate::NodeDefinition;

pub(crate) fn defs() -> Vec<NodeDefinition> {
    vec![
        shape::definition(),
        gradient_linear::definition(),
        gradient_axial::definition(),
        gradient_circular::definition(),
        checker::definition(),
        stripes::definition(),
        hexagon::definition(),
        scratches::definition(),
        tile_generator::definition(),
        tile_sampler::definition(),
        weave::definition(),
        splatter_circular::definition(),
        brick::definition(),
        polygon::definition(),
        star::definition(),
    ]
}

#[cfg(test)]
mod tests;
