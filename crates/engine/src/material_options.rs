use craft_texture::Image;
use materialcraft_core::{Graph, Value};
use materialcraft_eval::{Cooked, Options};
use serde_json::Value as J;
use std::collections::BTreeMap;

use crate::commands::bad;
use crate::{ExportSize, Result, Session};

/// Per-invocation inputs. The authoring graph and the source archive retain their own values.
#[derive(Clone, Debug, Default)]
pub struct MaterialOptions {
    pub size: Option<ExportSize>,
    pub values: BTreeMap<String, Value>,
    pub inputs: BTreeMap<String, Image>,
}

impl MaterialOptions {
    pub fn from_params(graph: &Graph, p: &J, read: impl Fn(&str) -> Result<Vec<u8>>) -> Result<Self> {
        let invalid = |msg| bad("material.cook", msg);
        let mut options = Self { size: crate::export_request::export_size(p.get("size"))?, ..Default::default() };
        if let Some(value) = p.get("preset").filter(|v| !v.is_null()) {
            let name = value.as_str().ok_or_else(|| invalid("preset must be a name".into()))?;
            let preset = graph.presets.iter().find(|preset| preset.name == name).ok_or_else(|| invalid(format!("unknown preset `{name}`")))?;
            options.values.clone_from(&preset.values);
        }
        if let Some(value) = p.get("values").filter(|v| !v.is_null()) {
            for (id, v) in value.as_object().ok_or_else(|| invalid("values must map parameter identifiers to values".into()))? {
                let param = graph.exposed.iter().find(|e| e.id == *id).ok_or_else(|| invalid(format!("unknown exposed parameter `{id}`")))?;
                let value = param.default.coerce_json(v).ok_or_else(|| invalid(format!("`{id}` takes a {}", param.default.type_name())))?;
                options.values.insert(id.clone(), value);
            }
        }
        if let Some(value) = p.get("inputs").filter(|v| !v.is_null()) {
            let declared = crate::material_cmds::image_inputs(graph);
            let mut remaining = 1usize << 30;
            for (id, path) in value.as_object().ok_or_else(|| invalid("inputs must map image identifiers to file paths".into()))? {
                if !declared.iter().any(|input| input.identifier == *id) {
                    return Err(invalid(format!("unknown image input `{id}`")));
                }
                let path = path.as_str().ok_or_else(|| invalid(format!("inputs.{id} must be an image path")))?;
                let image = materialcraft_formats::decode(&read(path)?, !materialcraft_formats::material::srgb_usage(id))?;
                remaining = remaining.checked_sub(std::mem::size_of_val(image.data())).ok_or_else(|| invalid("decoded image inputs exceed 1 GiB".into()))?;
                options.inputs.insert(id.clone(), image);
            }
        }
        Ok(options)
    }

    pub fn cook(&self, session: &Session, index: usize, max_size: Option<u32>) -> Result<Cooked> {
        let mut package = session.pkg.clone();
        let graph = package.graph_mut(index)?;
        if let Some(size) = self.size {
            graph.size = size.log2_size(graph.size);
        }
        let decoder = session.resource_decoder();
        let mut options = Options::new(&decoder);
        options.environment = session.environment;
        options.max_size = max_size;
        options.only = Some(graph.nodes.iter().filter(|node| node.def == "output").map(|node| node.id).collect());
        options.inputs.clone_from(&self.inputs);
        options.overrides.clone_from(&self.values);
        let cooked = materialcraft_eval::cook(&package, index, &options, &session.cache);
        if let Some((id, error)) = cooked.nodes.iter().find_map(|(id, r)| Some((id, r.error.as_ref()?))) {
            return Err(bad("material.cook", format!("graph `{}`, node {}: {error}", session.pkg.graph(index)?.sbs_path(), id.0)));
        }
        Ok(cooked)
    }
}
