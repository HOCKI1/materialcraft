pub(crate) mod multi_directional_warp;

#[cfg(test)]
mod tests;

pub(crate) const CATEGORY: &str = "Filters/Effects";

pub(crate) fn defs() -> Vec<crate::NodeDefinition> {
    vec![multi_directional_warp::definition()]
}
