pub(crate) mod slope_blur;

#[cfg(test)]
mod tests;

pub(crate) const CATEGORY: &str = "Filters/Blurs";

pub(crate) fn defs() -> Vec<crate::NodeDefinition> {
    vec![slope_blur::definition()]
}
