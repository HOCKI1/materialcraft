pub(crate) mod bevel;
pub(crate) mod edge_detect;

#[cfg(test)]
mod tests;

pub(crate) const CATEGORY: &str = "Filters/Morphology";

pub(crate) fn defs() -> Vec<crate::NodeDefinition> {
    vec![bevel::definition(), edge_detect::definition()]
}
