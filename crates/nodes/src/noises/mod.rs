pub(crate) mod bnw_spots;
pub(crate) mod clouds;
pub(crate) mod cracks;
pub(crate) mod grunge_map;
pub(crate) mod perlin;
pub(crate) mod simplex;
pub(crate) mod turbulence;
pub(crate) mod voronoi;
pub(crate) mod white_noise;

#[cfg(test)]
mod tests;

pub(crate) const CATEGORY: &str = "Texture generators/Noises";

pub(crate) fn defs() -> Vec<crate::NodeDefinition> {
    vec![
        bnw_spots::definition(),
        clouds::definition(),
        cracks::definition(),
        grunge_map::definition(),
        perlin::definition(),
        simplex::definition(),
        turbulence::definition(),
        voronoi::definition(),
        white_noise::definition(),
    ]
}
