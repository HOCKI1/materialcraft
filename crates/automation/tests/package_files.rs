use materialcraft_automation::{AuthorizedWorkspace, Filesystem, Headless};
use materialcraft_engine::{Session, core::Package};
use serde_json::json;

#[test]
fn reopening_relative_paths_activates_the_existing_document_and_preserves_edits() {
    let temporary = tempfile::tempdir().unwrap();
    let root = temporary.path().to_path_buf();
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
}

#[test]
fn both_formats_save_and_reopen_using_separate_workspace_capabilities() {
    let temporary = tempfile::tempdir().unwrap();
    let root = temporary.path().to_path_buf();
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
}

#[test]
fn scoped_publication_resolves_linked_graphs_and_images_using_the_granted_read_root() {
    use materialcraft_engine::core::{Graph, GraphInstance, GraphPort, Value};
    use materialcraft_engine::{formats, samples};
    let temporary = tempfile::tempdir().unwrap();
    let dir = temporary.path().to_path_buf();
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
            assert!(pkg.graphs.iter().any(|graph| graph.name == "Finish"));
            let calls: Vec<_> = pkg.graph(entry).unwrap().instances().collect();
            assert_eq!(calls.len(), 1);
            assert!(calls[0].package.is_empty());
            assert!(pkg.graph_by_name(&calls[0].graph).is_some());

            // Saving under a different granted root rebases links from that actual
            // destination, not from the process working directory or a relative path.
            app.save(Some("saved.sbs")).unwrap();
            let xml = std::fs::read_to_string(output.join("saved.sbs")).unwrap();
            assert!(xml.contains("../input/parts/finish.sbs"), "{xml}");
            assert_eq!(app.session.path.as_deref(), Some("saved.sbs"));
            let workspace = AuthorizedWorkspace::new(Some(&dir), Some(&output)).unwrap();
            let mut reopened = Headless::new(Session::new(Package::new("Empty")), Filesystem::Workspace(workspace));
            reopened.load("published/saved.sbs").unwrap();
            let helper_path = input.join("parts/finish.sbs").canonicalize().unwrap().to_string_lossy().into_owned();
            assert!(reopened.session.pkg.dependencies.contains_key(&helper_path));
            assert!(reopened.session.pkg.dependencies.values().any(|dependency| dependency.name == "Helpers"));
            app.save(Some("new/sub/saved.sbs")).unwrap();
            let xml = std::fs::read_to_string(output.join("new/sub/saved.sbs")).unwrap();
            assert!(xml.contains("../../../input/parts/finish.sbs"), "{xml}");
            reopened.load("published/new/sub/saved.sbs").unwrap();
            assert!(reopened.session.pkg.dependencies.contains_key(&helper_path));
            #[cfg(unix)]
            {
                std::fs::create_dir_all(output.join("nested/deep")).unwrap();
                std::os::unix::fs::symlink("nested/deep", output.join("alias")).unwrap();
                app.save(Some("alias/new/linked.sbs")).unwrap();
                let xml = std::fs::read_to_string(output.join("nested/deep/new/linked.sbs")).unwrap();
                assert!(xml.contains("../../../../input/parts/finish.sbs"), "{xml}");
                reopened.load("published/nested/deep/new/linked.sbs").unwrap();
                assert!(reopened.session.pkg.dependencies.values().any(|dependency| dependency.name == "Helpers"));
            }
        } else {
            let error = result.unwrap_err().to_string();
            assert!(error.contains("the instanced graph does not exist"), "{error}");
            let outside = dir.join("outside.sbs").canonicalize().unwrap().to_string_lossy().into_owned();
            assert!(!app.session.pkg.dependencies.contains_key(&outside));
            assert!(!app.session.pkg.dependencies.values().any(|dependency| dependency.name == "Helpers"));
            assert!(!output.join("missing.sbs.mcmat").exists());
        }
    }
}
