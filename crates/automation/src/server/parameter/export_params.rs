use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, JsonSchema)]
pub struct ExportParams {
    /// Output folder relative to the write root (created if missing; `.` = the root)
    pub dir: String,
    /// png (default), jpg, tga, bmp, webp, exr, hdr
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub format: Option<String>,
    /// File name pattern with $(graph), $(identifier) and $(usage) (default `$(graph)_$(identifier)`)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pattern: Option<String>,
    /// 8, 16 or 32 bits (default: the graph's output format)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub depth: Option<u32>,
    /// Pixels of the larger side, rounded down to a power of two, 16 to 8192 (default: the graph's size)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub size: Option<u32>,
    /// Graph index or full SBS path. Defaults to the open graph.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub graph: Option<serde_json::Value>,
    /// Output identifiers to write (default: every connected output)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub outputs: Option<Vec<String>>,
    /// Color space per output identifier: srgb, linear or raw (default: sRGB for color usages, raw for data)
    #[serde(default, rename = "colorSpaces", skip_serializing_if = "Option::is_none")]
    pub color_spaces: Option<std::collections::BTreeMap<String, String>>,
    /// Export profile: materialx for OpenPBR with baked textures, maps for labeled textures.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub profile: Option<String>,
    /// Optional named preset, applied before explicit values.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub preset: Option<String>,
    /// Height range in scene units for MaterialX displacement.
    #[serde(default, rename = "displacementScale", skip_serializing_if = "Option::is_none")]
    pub displacement_scale: Option<f32>,
    /// Typed exposed parameter overrides for this cook.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[schemars(schema_with = "values_schema")]
    pub values: Option<std::collections::BTreeMap<String, serde_json::Value>>,
    /// Image input identifier to path below the read root.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub inputs: Option<std::collections::BTreeMap<String, String>>,
}

fn values_schema(_: &mut schemars::SchemaGenerator) -> schemars::Schema {
    schemars::json_schema!({"type":["object","null"],"additionalProperties":{}})
}
