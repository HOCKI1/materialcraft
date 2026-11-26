pub const CATEGORY: &str = "Filters/Adjustments";

mod auto_levels;
mod clamp;
mod color_temperature;
mod contrast_luminosity;
mod highpass;
mod histogram_range;
mod histogram_scan;
mod invert;
mod min_max;
mod quantize;
mod replace_color;
mod threshold;

use crate::NodeDefinition;

pub(crate) fn defs() -> Vec<NodeDefinition> {
    vec![
        auto_levels::definition(),
        clamp::definition(),
        contrast_luminosity::definition(),
        highpass::definition(),
        histogram_range::definition(),
        histogram_scan::definition(),
        invert::definition(),
        quantize::definition(),
        replace_color::definition(),
        threshold::definition(),
        min_max::definition(),
        color_temperature::definition(),
    ]
}

#[cfg(test)]
mod tests;
