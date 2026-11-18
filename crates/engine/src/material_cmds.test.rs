use super::*;
use crate::{MaterialOptions, execute, file_cmds, material_bake};
use craft_texture::{Image, Kind};
use materialcraft_core::{ComponentUsage, GraphInstance, GraphPort, Package, Preset, Resource, ResourceBytes, Value};
use materialcraft_formats::{Depth, encode};

fn node(graph: &mut Graph, def: &str, params: &[(&str, Value)]) -> materialcraft_core::NodeId {
    let id = graph.add_node(def, [0.0; 2]);
    graph.node_mut(id).unwrap().params.extend(params.iter().map(|(k, v)| (k.to_string(), v.clone())));
    id
}

fn output(graph: &mut Graph, source: materialcraft_core::NodeId, port: &str, id: &str, usage: &str) {
    let out = node(graph, "output", &[("identifier", Value::Text(id.into())), ("usage", Value::Enum(ComponentUsage::index(usage)))]);
    graph.connect(source, port, out, "input").unwrap();
}

fn fixture() -> Session {
    let mut project = Package::new("Studio");
    project.graphs.push(Graph::new("Unused"));
    let mut graph = Graph::new("Oak");
    graph.folder = "Materials".into();
    graph.size = [5, 4];
    for (owner, usage, gray) in [("a.sbs", "height", 0.22), ("b.sbs", "metallic", 0.75)] {
        let mut helper = Graph::new("Grain");
        let image = node(&mut helper, "bitmap", &[("resource", Value::Text("grain".into()))]);
        output(&mut helper, image, "output", "grain", "height");
        let mut pkg = Package::new("Helpers");
        pkg.graphs.push(helper);
        let data = encode(&Image::filled(16, 16, Kind::Gray, [gray; 4]), "png", Depth::U16, false).unwrap();
        pkg.resources.push(Resource::new("grain", "image.png", ResourceBytes::new(data)));
        project.dependencies.insert(owner.into(), std::sync::Arc::new(pkg));
        let inst = node(&mut graph, "graph_instance", &[]);
        graph.node_mut(inst).unwrap().instance = Some(GraphInstance {
            package: owner.into(),
            graph: "Grain".into(),
            inputs: vec![],
            outputs: vec![GraphPort { identifier: "grain".into(), type_code: 1 }],
            defaults: Default::default(),
            label: String::new(),
        });
        output(&mut graph, inst, "grain", usage, usage);
    }
    let source = node(&mut graph, "uniform", &[]);
    output(&mut graph, source, "output", "albedo", "baseColor");
    let mesh = node(&mut graph, "input_grayscale", &[("identifier", Value::Text("curvature".into()))]);
    output(&mut graph, mesh, "output", "mesh", "roughness");
    project.graphs.push(graph);
    let mut session = Session::new(project);
    session.graph = 1;
    execute(&mut session, "graph.exposed.add", &json!({"id":"tint","bind":{"node":source,"param":"color"}})).unwrap();
    execute(&mut session, "graph.exposed.set", &json!({"id":"tint","value":[0.4,0.2,0.1,1.0]})).unwrap();
    session.graph_mut().unwrap().presets.push(Preset { name: "Dark".into(), values: [("tint".into(), Value::Color([0.15, 0.1, 0.05, 1.0]))].into() });
    session
}

#[test]
fn published_entry_cooks_independently_with_values_sizes_and_image_inputs() {
    let source = fixture();
    let bytes = material_bytes(&source, source.graph).unwrap();
    let (pkg, entry) = materialcraft_formats::material::read_material(&bytes).unwrap();
    assert_eq!(pkg.graphs.len(), 3);
    assert_eq!(pkg.resources.len(), 2);
    assert!(pkg.dependencies.is_empty());
    assert_eq!(pkg.graph(entry).unwrap().sbs_path(), "Oak");
    assert!(pkg.graphs.iter().all(|g| g.name != "Unused"));
    let mut opened = Session::new(Package::new("other"));
    file_cmds::open_bytes(&mut opened, &bytes, Some("Oak.mcmat".into())).unwrap();
    let info = material_bake::info(&opened, &J::Null).unwrap();
    assert_eq!(info["graph"], "Oak");
    assert_eq!(info["parameters"][0]["default"], json!(source.graph().unwrap().exposed[0].default));
    assert_eq!(info["parameters"][0]["value"], json!(source.graph().unwrap().input_value("tint")));
    assert_eq!(info["presets"][0]["name"], "Dark");
    let mut variations = Vec::new();
    for size in [32, 64] {
        for wear in [0.1, 0.7] {
            for gray in [0.2, 0.8] {
                let mesh = encode(&Image::filled(16, 16, Kind::Gray, [gray; 4]), "png", Depth::U16, false).unwrap();
                let p = json!({"profile":"maps","size":size,"preset":"Dark","values":{"tint":[wear,0.1,0.05,1.0]},"inputs":{"curvature":"mesh.png"}});
                let bake = |s: &Session| {
                    material_bake::export(s, &p, |path| {
                        assert_eq!(path, "mesh.png");
                        Ok(mesh.clone())
                    })
                    .unwrap()
                };
                let (original, _) = bake(&source);
                let (packed, report) = bake(&opened);
                assert_eq!(original.len(), 4);
                assert_eq!(report["maps"][0]["size"], json!([size, size / 2]));
                for (a, b) in original.iter().zip(&packed) {
                    assert_eq!(a.file, b.file);
                    assert_eq!(a.bytes, b.bytes, "{}", a.file);
                }
                let sample = |name: &str, linear: bool| {
                    let map = packed.iter().find(|f| f.file == name).unwrap();
                    materialcraft_formats::decode(&map.bytes, linear).unwrap().get(0, 0, false)[0]
                };
                assert!((f64::from(sample("albedo.png", false)) - wear).abs() < 0.01);
                assert!((sample("mesh.png", true) - gray).abs() < 0.01);
                variations.push(report["material"]["parameters"][0]["value"].clone());
            }
        }
    }
    assert_ne!(variations[0], variations[2]);
    assert_eq!(info, material_bake::info(&opened, &J::Null).unwrap(), "cook overrides leaked");
    assert!(
        material_bake::cook(&opened, &json!({"values":{"missing":0}}), |path| Err(crate::EngineError::Other(format!("unexpected file read: {path}")))).is_err()
    );
    assert!(
        material_bake::cook(&opened, &json!({"inputs":{"missing":"x"}}), |path| Err(crate::EngineError::Other(format!("unexpected file read: {path}"))))
            .is_err()
    );
}

#[test]
fn rejects_missing_ambiguous_and_uncookable_materials() {
    let mut session = fixture();
    assert!(material_bytes(&session, 0).is_err());
    session.pkg.dependencies.remove("b.sbs");
    assert!(material_bytes(&session, 1).is_err());
    let empty = Package::new("Empty");
    assert!(materialcraft_formats::material::entry_graph(&empty).is_err());
    let mut ambiguous = empty;
    ambiguous.graphs.extend([Graph::new("A"), Graph::new("B")]);
    assert!(materialcraft_formats::material::entry_graph(&ambiguous).is_err());
}

#[test]
fn preset_values_apply_before_explicit_overrides() {
    let s = fixture();
    let options = MaterialOptions::from_params(s.graph().unwrap(), &json!({"preset":"Dark"}), |path| {
        Err(crate::EngineError::Other(format!("unexpected file read: {path}")))
    })
    .unwrap();
    assert_eq!(options.values["tint"], Value::Color([0.15, 0.1, 0.05, 1.0]));
}

#[test]
fn publishing_sbs_and_project_archives_survives_removing_the_entire_source_tree() {
    let dir = std::env::temp_dir().join(format!("materialcraft-source-closure-{}", std::process::id()));
    let authored = dir.join("authored");
    std::fs::create_dir_all(&authored).unwrap();
    let source = fixture();
    for filename in ["a.sbs", "b.sbs"] {
        let dependency = &source.pkg.dependencies[filename];
        let files = materialcraft_formats::project::ProjectFiles::build(dependency, filename, false).unwrap();
        materialcraft_formats::project::write_files(&authored.join(filename), &files).unwrap();
    }
    for ext in ["sbs", "mcraft"] {
        let path = authored.join(format!("studio.{ext}"));
        // Save the independent project beside its library packages and their original bitmaps.
        let files = materialcraft_formats::project::ProjectFiles::build(&source.pkg, &format!("studio.{ext}"), ext == "mcraft").unwrap();
        if ext == "mcraft" {
            std::fs::write(&path, files.zip().unwrap()).unwrap();
        } else {
            materialcraft_formats::project::write_files(&path, &files).unwrap();
        }
        let mut editor = Session::new(Package::new("Empty"));
        execute(&mut editor, "file.open", &json!({"path":path})).unwrap();
        execute(&mut editor, "file.exportMaterial", &json!({"graph":"Materials/Oak","path":dir.join(format!("{ext}.mcmat"))})).unwrap();
    }
    std::fs::remove_dir_all(&authored).unwrap();
    for ext in ["sbs", "mcraft"] {
        let mut viewer = Session::new(Package::new("Empty"));
        execute(&mut viewer, "file.open", &json!({"path":dir.join(format!("{ext}.mcmat"))})).unwrap();
        let settings = json!({"profile":"maps","preset":"Dark","size":32});
        let bake = |s: &Session| {
            material_bake::export(s, &settings, |path| Err(crate::EngineError::Other(format!("unexpected file read: {path}"))))
                .unwrap()
                .0
                .into_iter()
                .map(|f| (f.file, f.bytes))
                .collect::<std::collections::BTreeMap<_, _>>()
        };
        assert_eq!(bake(&source), bake(&viewer));
    }
    std::fs::remove_dir_all(dir).unwrap();
}

#[test]
fn snapshots_transitive_function_calls_and_rejects_recursion_or_missing_assets() {
    use materialcraft_core::FunctionGraph;
    let mut s = fixture();
    let call = |owner: &str, name: &str| {
        let mut f = FunctionGraph::default();
        f.output = Some(f.add("instance", &[], None));
        f.nodes[0].type_code = 256;
        f.nodes[0].instance = Some(GraphInstance {
            package: owner.into(),
            graph: name.into(),
            inputs: vec![],
            outputs: vec![],
            defaults: Default::default(),
            label: String::new(),
        });
        f
    };
    let mut utils = Package::new("Functions");
    let mut constant = Graph::new("Amount");
    constant.function_type = 256;
    constant.function = Some(FunctionGraph::constant(Value::Float(0.4)));
    utils.graphs.push(constant);
    let mut wrapper = Graph::new("Wrapper");
    wrapper.function_type = 256;
    wrapper.function = Some(call("", "Amount"));
    utils.graphs.push(wrapper);
    s.pkg.dependencies.insert("math.sbs".into(), std::sync::Arc::new(utils));
    let graph = s.graph_mut().unwrap();
    let fx = graph.nodes.iter_mut().find(|n| n.def == "uniform").unwrap();
    fx.functions.insert("color_mode".into(), call("math.sbs", "Wrapper"));
    let bytes = materialcraft_formats::material::write_material(&s.pkg, s.graph).unwrap();
    let (p, root) = materialcraft_formats::material::read_material(&bytes).unwrap();
    assert_eq!(p.graphs.iter().filter(|g| g.function.is_some()).count(), 2);
    assert_eq!(p.graph(root).unwrap().name, "Oak");
    for reference in p.graphs.iter().flat_map(|g| g.instances()) {
        assert!(reference.package.is_empty() && p.graph_by_name(&reference.graph).is_some());
    }
    let lib = std::sync::Arc::make_mut(s.pkg.dependencies.get_mut("math.sbs").unwrap());
    lib.graphs[0].function = Some(call("", "Wrapper"));
    assert!(materialcraft_formats::material::write_material(&s.pkg, s.graph).unwrap_err().to_string().contains("cyclic"));
    std::sync::Arc::make_mut(s.pkg.dependencies.get_mut("a.sbs").unwrap()).resources.clear();
    assert!(materialcraft_formats::material::write_material(&s.pkg, s.graph).unwrap_err().to_string().contains("missing resource"));
}
