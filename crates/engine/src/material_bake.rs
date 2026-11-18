//! Inspect and evaluate the published graph through the same CPU cook as an authoring document.
use std::collections::BTreeSet;

use craft_texture::Kind;
use materialcraft_core::Graph;
use materialcraft_eval::{Cooked, outputs};
use materialcraft_formats::{Depth, encode};
use serde_json::{Value as J, json};

use crate::commands::{CommandSpec, bad, has_graph};
use crate::{ExportRequest, Exported, MaterialOptions, Result, Session, cmd, graph_cmds::graph_index};

pub(crate) fn specs() -> Vec<CommandSpec> {
    vec![
        cmd!(
            "material.info",
            "Inspect material",
            [],
            None,
            "{graph?} returns the material interface, published values, image inputs, outputs, presets and supported sizes",
            has_graph,
            |s, p| info(s, p)
        ),
        cmd!(
            "material.cook",
            "Cook material",
            [],
            None,
            "{graph?, size?, preset?, values?, inputs?} evaluates a variation and returns its output dimensions and means",
            has_graph,
            |s, p| cook(s, p, read_input)
        ),
    ]
}

#[cfg(not(target_arch = "wasm32"))]
pub fn read_input(path: &str) -> Result<Vec<u8>> {
    crate::linked_resource::read_regular(std::path::Path::new(path)).map_err(|error| bad("material.cook", format!("{path}: {error}")))
}

#[cfg(target_arch = "wasm32")]
pub fn read_input(_: &str) -> Result<Vec<u8>> {
    Err(bad("material.cook", "bind an image resource in the browser"))
}

pub fn info(s: &Session, p: &J) -> Result<J> {
    let index = graph_index(s, p)?;
    let graph = s.pkg.graph(index)?;
    let parameters: Vec<_> = graph
        .exposed
        .iter()
        .map(|param| {
            let mut json = json!(param);
            json["value"] = json!(graph.input_value(&param.id));
            json
        })
        .collect();
    Ok(json!({"version": materialcraft_formats::material::VERSION, "graph":graph.sbs_path(), "name":graph.display_name(),
        "size":graph.size_px(), "supportedSizes":(materialcraft_core::MIN_SIZE_LOG2..=materialcraft_core::MAX_SIZE_LOG2).map(|v| 1u32 << v).collect::<Vec<_>>(),
        "parameters":parameters,"presets":graph.presets,"inputs":crate::material_cmds::image_inputs(graph),
        "outputs":graph.nodes.iter().filter(|n|n.def=="output").map(|n|json!({"identifier":materialcraft_eval::identifier(n),"usage":materialcraft_eval::output_usage(n)})).collect::<Vec<_>>()
    }))
}

pub(crate) fn check_publish(s: &Session, index: usize) -> Result<()> {
    let graph = s.pkg.graph(index)?;
    if graph.function.is_some() {
        return Err(bad("file.exportMaterial", "select a material graph"));
    }
    let cooked = MaterialOptions::default().cook(s, index, Some(16))?;
    if !outputs(graph, &cooked).into_iter().any(|(id, _, _, image)| image.is_some() && graph.connections.iter().any(|wire| wire.to == id)) {
        return Err(bad("file.exportMaterial", format!("graph `{}` has no connected, cookable material outputs", graph.sbs_path())));
    }
    Ok(())
}

pub fn cook(s: &Session, p: &J, read: impl Fn(&str) -> Result<Vec<u8>>) -> Result<J> {
    let index = graph_index(s, p)?;
    let graph = s.pkg.graph(index)?;
    let cooked = MaterialOptions::from_params(graph, p, read)?.cook(s, index, None)?;
    Ok(json!({"graph":graph.sbs_path(),"ms":cooked.ms,
        "outputs":outputs(graph,&cooked).into_iter().filter_map(|(_,usage,id,image)| {
            let image = image?;
            Some(json!({"identifier":id,"usage":usage,"size":[image.width(),image.height()],"mean":image.mean()}))
        }).collect::<Vec<_>>() }))
}

/// Encode every file before the caller writes to its destination. Paths in the report are relative.
pub fn export(s: &Session, p: &J, read: impl Fn(&str) -> Result<Vec<u8>>) -> Result<(Vec<Exported>, J)> {
    let request = ExportRequest::from_params(s, p)?;
    let graph = s.pkg.graph(request.graph)?;
    let options = MaterialOptions::from_params(graph, p, read)?;
    let cooked = options.cook(s, request.graph, None)?;
    match p.get("profile").filter(|v| !v.is_null()) {
        None => Ok((crate::file_cmds::encode_outputs(graph, &cooked, &request)?, json!({}))),
        Some(profile) if profile == "maps" || profile == "materialx" => {
            let scale = match p.get("displacementScale").filter(|v| !v.is_null()) {
                Some(v) => {
                    v.as_f64().map(|x| x as f32).filter(|x| x.is_finite() && *x >= 0.0).ok_or_else(|| bad("file.export", "invalid displacementScale"))?
                }
                None => 1.0,
            };
            let (files, mut report) = encode_maps(graph, &cooked, &request, profile == "materialx", scale)?;
            let mut interface = info(s, p)?;
            if let Some(params) = interface["parameters"].as_array_mut() {
                for param in params {
                    if let Some(value) = param["id"].as_str().and_then(|id| options.values.get(id)) {
                        param["value"] = json!(value);
                    }
                }
            }
            report["material"] = interface;
            Ok((files, report))
        }
        Some(value) => Err(bad("file.export", format!("unknown export profile {value}"))),
    }
}

fn shader_input(usage: &str) -> Option<&'static str> {
    Some(match usage {
        "baseColor" | "diffuse" => "base_color",
        "roughness" => "specular_roughness",
        "metallic" => "base_metalness",
        "normal" => "geometry_normal",
        "emissive" => "emission_color",
        "opacity" => "geometry_opacity",
        "height" => "height",
        "coatOpacity" => "coat_weight",
        "coatRoughness" => "coat_roughness",
        "transmissive" => "transmission_weight",
        "scattering" => "subsurface_weight",
        "sheenOpacity" => "fuzz_weight",
        "sheenColor" => "fuzz_color",
        "sheenRoughness" => "fuzz_roughness",
        "anisotropyLevel" => "specular_roughness_anisotropy",
        _ => return None,
    })
}

fn encode_maps(graph: &Graph, cooked: &Cooked, request: &ExportRequest, materialx: bool, scale: f32) -> Result<(Vec<Exported>, J)> {
    let mut files = Vec::new();
    let mut maps = Vec::new();
    let mut textures = Vec::new();
    let mut unmapped = Vec::new();
    let mut names = BTreeSet::new();
    let mut shader_inputs = BTreeSet::new();
    for (id, usage, identifier, image) in outputs(graph, cooked) {
        let Some(image) = image.filter(|_| request.includes(&identifier) && graph.connections.iter().any(|wire| wire.to == id)) else { continue };
        let height = materialx && usage == "height";
        // The shader's displacement input uses signed scene units; a mid-gray source height is neutral.
        let signed;
        let image = if height {
            signed = image.map(Kind::Gray, |pixel| [(pixel[0] - 0.5) * scale, 0.0, 0.0, 1.0]);
            &signed
        } else {
            image
        };
        let float = height || request.depth == Some(Depth::F32) || image.data().iter().any(|value| !(0.0..=1.0).contains(value));
        let (ext, depth) = if float {
            ("exr", Depth::F32)
        } else {
            (
                "png",
                request.depth.unwrap_or(if matches!(usage, "normal" | "height") || !materialcraft_formats::material::srgb_usage(usage) {
                    Depth::U16
                } else {
                    Depth::U8
                }),
            )
        };
        let file = format!("{}.{}", materialcraft_formats::safe_file_name(&identifier), ext);
        if !names.insert(file.to_lowercase()) {
            return Err(bad("file.export", format!("duplicate output filename `{file}`; give each output a unique identifier")));
        }
        let srgb = !float && request.color_space(&identifier, usage).encodes_srgb();
        let bytes = encode(image, ext, depth, srgb)?;
        maps.push(json!({"identifier":identifier,"usage":usage,"file":file,"size":[image.width(),image.height()],"colorSpace":if srgb {"srgb"}else{"linear"}}));
        if materialx {
            if let Some(input) = shader_input(usage).filter(|i| shader_inputs.insert(*i)) {
                textures.push(craft_materialx::Texture {
                    input: input.into(),
                    file: file.clone(),
                    colorspace: if srgb { "srgb_texture" } else { "lin_rec709" }.into(),
                });
            } else {
                unmapped.push(identifier);
            }
        }
        files.push(Exported { file, bytes });
    }
    if files.is_empty() {
        return Err(bad("file.export", "no connected image outputs were selected"));
    }
    if materialx {
        if textures.is_empty() {
            return Err(bad("file.export", format!("no selected output maps to OpenPBR: {}", unmapped.join(", "))));
        }
        let doc = craft_materialx::write_open_pbr(&graph.name, &textures, None);
        files.push(Exported { file: "material.mtlx".into(), bytes: doc.into_bytes() });
    }
    Ok((files, json!({"maps":maps,"unmapped":unmapped,"normalOrientation":"Y+","displacementScale":scale})))
}
