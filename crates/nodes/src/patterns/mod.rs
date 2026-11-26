pub const CATEGORY: &str = "Texture generators/Patterns";

mod brick;
mod checker;
mod fibers;
mod gaussian;
mod gradient_axial;
mod gradient_circular;
mod gradient_linear;
mod hexagon;
mod mesh;
mod polygon;
mod scratches;
mod shape;
mod splatter_circular;
mod star;
mod stripes;
mod tile_generator;
mod tile_random;
mod tile_sampler;
mod triangle_grid;
mod weave;

use crate::NodeDefinition;

pub(crate) fn defs() -> Vec<NodeDefinition> {
    vec![
        shape::definition(),
        gradient_linear::definition(),
        gradient_linear::definition_1(),
        gradient_linear::definition_2(),
        gradient_linear::definition_3(),
        gradient_axial::definition(),
        gradient_circular::definition(),
        checker::definition(),
        checker::definition_1(),
        stripes::definition(),
        hexagon::definition(),
        scratches::definition(),
        tile_generator::definition(),
        tile_sampler::definition(),
        weave::definition(),
        weave::definition_1(),
        weave::definition_2(),
        splatter_circular::definition(),
        brick::definition(),
        brick::definition_1(),
        brick::definition_2(),
        polygon::definition(),
        polygon::definition_1(),
        polygon::definition_2(),
        star::definition(),
        tile_random::definition(),
        triangle_grid::definition(),
        fibers::definition_1(),
        fibers::definition_2(),
        gaussian::definition_1(),
        gaussian::definition_2(),
        mesh::definition_1(),
        mesh::definition_2(),
    ]
}

#[cfg(test)]
mod tests;
