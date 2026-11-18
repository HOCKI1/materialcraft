use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Deserialize, Serialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct MaterialExportParams {
    /// Forward-slash path relative to the write root (`.mcmat` is appended if missing). The
    /// parent folder must exist
    pub path: String,
    /// Graph index or full SBS path; defaults to the open graph.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub graph: Option<Value>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn material_publication_rejects_legacy_bake_size() {
        for size in [json!(256), json!(null)] {
            let error = serde_json::from_value::<MaterialExportParams>(json!({"path":"material", "size":size})).unwrap_err();
            assert!(error.to_string().contains("unknown field `size`"));
        }
        let params = serde_json::from_value::<MaterialExportParams>(json!({"path":"material", "graph":"Materials/Oak"})).unwrap();
        assert_eq!(params.graph, Some(json!("Materials/Oak")));
    }
}
