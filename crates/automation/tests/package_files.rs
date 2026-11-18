use materialcraft_automation::{AuthorizedWorkspace, Filesystem, Headless};
use materialcraft_engine::{Session, core::Package};
use serde_json::json;

#[test]
fn reopening_relative_paths_activates_the_existing_document_and_preserves_edits() {
    let root = std::env::temp_dir().join(format!("materialcraft-workspace-reopen-{}", std::process::id()));
    std::fs::create_dir_all(&root).unwrap();

    for extension in ["sbs", "mcraft"] {
        let workspace = AuthorizedWorkspace::new(Some(&root), Some(&root)).unwrap();
        let mut app = Headless::new(Session::new(Package::new("Source")), Filesystem::Workspace(workspace.clone()));
        let filename = format!("package.{extension}");
        app.save(Some(&filename)).unwrap();
        let mut reopened = Headless::new(Session::new(Package::new("Empty")), Filesystem::Workspace(workspace));
        reopened.open(&filename).unwrap();
        let document = reopened.session.document_id;
        reopened.command_run("folder.create", json!({"name": "Unsaved"})).unwrap();
        let history = reopened.session.history();

        reopened.open(&filename).unwrap();

        assert_eq!(reopened.session.document_id, document);
        assert_eq!(reopened.session.open_packages().len(), 2);
        assert_eq!(reopened.session.path.as_deref(), Some(filename.as_str()));
        assert!(reopened.session.modified);
        assert!(reopened.session.pkg.folder_paths().contains("Unsaved"));
        assert_eq!(reopened.session.history(), history);

        reopened.command_run("file.newPackage", json!({"name": "Other"})).unwrap();
        reopened.open(&filename).unwrap();

        assert_eq!(reopened.session.document_id, document);
        assert_eq!(reopened.session.open_packages().len(), 3);
        assert!(reopened.session.modified);
        assert!(reopened.session.pkg.folder_paths().contains("Unsaved"));
        assert_eq!(reopened.session.history(), history);

        reopened.save(None).unwrap();

        assert!(!reopened.session.modified);
        assert!(root.join(&filename).is_file());
        assert_eq!(reopened.session.path.as_deref(), Some(filename.as_str()));
    }

    std::fs::remove_dir_all(root).unwrap();
}

#[test]
fn both_formats_save_and_reopen_using_separate_workspace_capabilities() {
    let root = std::env::temp_dir().join(format!("materialcraft-workspace-packages-{}", std::process::id()));
    let input = root.join("input");
    let output = root.join("output");
    std::fs::create_dir_all(&input).unwrap();
    std::fs::create_dir_all(&output).unwrap();
    let svg = br#"<svg xmlns="http://www.w3.org/2000/svg" width="2" height="2"><rect width="2" height="2"/></svg>"#;
    std::fs::write(input.join("shape.svg"), svg).unwrap();
    let workspace = AuthorizedWorkspace::new(Some(&input), Some(&output)).unwrap();
    let mut app = Headless::new(Session::new(Package::new("Package")), Filesystem::Workspace(workspace));
    app.import_bitmap("shape.svg", None).unwrap();
    for extension in ["mcraft", "sbs"] {
        let filename = format!("package.{extension}");
        app.save(Some(&filename)).unwrap();
        app.save(None).unwrap();

        assert!(output.join(&filename).is_file());

        if extension == "mcraft" {
            assert_eq!(std::fs::read_dir(&output).unwrap().count(), 1);
        }
        let workspace = AuthorizedWorkspace::new(Some(&output), Some(&output)).unwrap();
        let mut reopened = Headless::new(Session::new(Package::new("Empty")), Filesystem::Workspace(workspace));
        reopened.open(&filename).unwrap();

        assert_eq!(reopened.session.pkg.resources[0].bytes.as_slice(), svg);

        reopened.save(None).unwrap();
    }

    assert_eq!(std::fs::read_dir(&input).unwrap().count(), 1);
    assert!(app.save(Some("../escape.sbs")).is_err());
    assert!(!root.join("escape.sbs").exists());

    // Release workspace directory handles before Windows removes the test tree
    drop(app);
    std::fs::remove_dir_all(root).unwrap();
}

#[test]
fn scoped_publication_resolves_linked_graphs_and_images_using_the_granted_read_root() {
    use materialcraft_engine::core::{Graph, GraphInstance, GraphPort, Value};
    use materialcraft_engine::{formats, samples};
    let dir = std::env::temp_dir().join(format!("materialcraft-publish-workspace-{}", std::process::id()));
    let input = dir.join("input");
    let output = dir.join("published");
    std::fs::create_dir_all(input.join("parts")).unwrap();
    std::fs::create_dir_all(&output).unwrap();
    let mut helpers = Package::new("Helpers");
    helpers.graphs.push(samples::template("pbr", "Finish", 4).unwrap());
    let helper = formats::sbs::write_sbs(&helpers).unwrap();
    std::fs::write(input.join("parts/finish.sbs"), &helper).unwrap();
    std::fs::write(dir.join("outside.sbs"), &helper).unwrap();
    for (project, dependency) in [("surface.sbs", "parts/finish.sbs"), ("missing.sbs", "../outside.sbs")] {
        let mut graph = Graph::new("Oak");
        let call = graph.add_node("graph_instance", [0.; 2]);
        graph.node_mut(call).unwrap().instance = Some(GraphInstance {
            package: dependency.into(),
            graph: "Finish".into(),
            inputs: vec![],
            outputs: vec![GraphPort { identifier: "basecolor".into(), type_code: 2 }],
            defaults: Default::default(),
            label: String::new(),
        });
        let node = graph.add_node("output", [200., 0.]);
        graph.node_mut(node).unwrap().params.extend([("identifier".into(), Value::Text("color".into())), ("usage".into(), Value::Enum(0))]);
        graph.connect(call, "basecolor", node, "input").unwrap();
        let mut package = Package::new("Surface");
        package.graphs.push(graph);
        std::fs::write(input.join(project), formats::sbs::write_sbs(&package).unwrap()).unwrap();
        let fs = AuthorizedWorkspace::new(Some(&input), Some(&output)).unwrap();
        let mut app = Headless::new(Session::new(Package::new("Empty")), Filesystem::Workspace(fs));
        app.load(project).unwrap();
        let result = app.export_material(&json!({"path":format!("{project}.mcmat")}));
        if project == "surface.sbs" {
            assert!(result.is_ok(), "{result:?}");
            let (pkg, entry) = formats::material::read_material(&std::fs::read(output.join("surface.sbs.mcmat")).unwrap()).unwrap();
            assert_eq!(pkg.graph(entry).unwrap().name, "Oak");
            assert_eq!(pkg.graphs.len(), 2);
        } else {
            assert!(result.is_err());
            assert!(!output.join("missing.sbs.mcmat").exists());
        }
    }
    std::fs::remove_dir_all(dir).unwrap();
}
