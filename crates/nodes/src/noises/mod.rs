pub(crate) mod anisotropic_noise;
pub(crate) mod blue_noise_fast;
pub(crate) mod bnw_spots;
pub(crate) mod caustics;
pub(crate) mod clouds;
pub(crate) mod cracks;
pub(crate) mod creased;
pub(crate) mod crystal;
pub(crate) mod directional_noise;
pub(crate) mod directional_scratches;
pub(crate) mod dirt;
pub(crate) mod fluid;
pub(crate) mod fractal_sum;
pub(crate) mod fur;
pub(crate) mod gaussian_noise;
pub(crate) mod grunge_map;
pub(crate) mod microscope_view;
pub(crate) mod moisture_noise;
pub(crate) mod noise_3d;
pub(crate) mod perlin;
pub(crate) mod plasma;
pub(crate) mod simplex;
pub(crate) mod turbulence;
pub(crate) mod voronoi;
pub(crate) mod voronoi_fractal;
pub(crate) mod waveform;
pub(crate) mod white_noise;
pub(crate) mod white_noise_fast;

#[cfg(test)]
mod tests;

pub(crate) const CATEGORY: &str = "Texture generators/Noises";

pub(crate) fn defs() -> Vec<crate::NodeDefinition> {
    vec![
        anisotropic_noise::definition(),
        blue_noise_fast::definition(),
        bnw_spots::definition(),
        caustics::definition(),
        clouds::definition(),
        cracks::definition(),
        creased::definition(),
        crystal::definition(),
        directional_noise::definition(),
        directional_scratches::definition(),
        dirt::definition(),
        fluid::definition(),
        fractal_sum::definition(),
        fur::definition(),
        gaussian_noise::definition(),
        grunge_map::definition(),
        microscope_view::definition(),
        moisture_noise::definition(),
        noise_3d::perlin_3d_definition(),
        noise_3d::voronoi_3d_definition(),
        perlin::definition(),
        plasma::definition(),
        simplex::definition(),
        turbulence::definition(),
        voronoi::definition(),
        voronoi_fractal::definition(),
        waveform::definition(),
        white_noise::definition(),
        white_noise_fast::definition(),
    ]
}
