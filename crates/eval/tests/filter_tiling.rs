use craft_texture::Image;
use materialcraft_core::{Graph, OutputFormat, Package, ParameterInheritance, Value};
use materialcraft_eval::{Cache, Options, cook};
use std::sync::Mutex;

#[test]
fn atomic_filters_wrap_only_the_requested_axes_and_invalidate_cache() {
    for operation in ["blur", "directional_blur", "sharpen", "distance"] {
        let mut graph = Graph::new("Filter boundaries");
        graph.size = [4; 2];
        graph.format = OutputFormat::F32;
        graph.tiling_mode = ParameterInheritance::Absolute;
        let input = graph.add_node("input_grayscale", [0.0; 2]);
        graph.node_mut(input).unwrap().params.insert("identifier".into(), Value::Text("source".into()));
        let filter = graph.add_node(operation, [100.0, 0.0]);
        let (parameter, port, center, background) = if operation == "distance" { ("distance", "mask", 1.0, 0.0) } else { ("intensity", "input", 0.5625, 0.5) };
        let strength = match operation {
            "distance" => 48.0,
            "sharpen" => 1.0,
            _ => 16.0,
        };
        graph.node_mut(filter).unwrap().params.insert(parameter.into(), Value::Float(strength));
        graph.connect(input, "output", filter, port).unwrap();
        let mut package = Package::new("Filter boundaries");
        package.graphs.push(graph);
        let resource = |_: &str| None;
        let mut options = Options::new(&resource);
        options.inputs.insert("source".into(), Image::gray(16, 16, |x, y| if x == 0 && y == 0 { center } else { background }));
        let cache = Mutex::new(Cache::default());

        for tiling in [[false, false], [true, false], [false, true], [true, true]] {
            package.graphs[0].tiling = tiling;

            let cooked = cook(&package, 0, &options, &cache);
            let image = cooked.image(filter, 0).unwrap();

            assert_eq!(cooked.nodes[&filter].error, None);
            assert!(!cooked.nodes[&filter].cached);
            assert_eq!(image.get_gray(15, 0, false) != background, tiling[0], "{operation}, x, {tiling:?}");
            assert_eq!(image.get_gray(0, 15, false) != background, tiling[1] && operation != "directional_blur", "{operation}, y, {tiling:?}");

            let cached = cook(&package, 0, &options, &cache);

            assert!(cached.nodes[&filter].cached);
        }
    }
}

#[test]
fn directional_warp_uses_each_graph_tiling_axis() {
    let mut graph = Graph::new("Warp boundaries");
    graph.size = [4; 2];
    graph.format = OutputFormat::F32;
    graph.tiling_mode = ParameterInheritance::Absolute;
    let input = graph.add_node("input_grayscale", [0.0; 2]);
    graph.node_mut(input).unwrap().params.insert("identifier".into(), Value::Text("field".into()));
    let mask = graph.add_node("input_grayscale", [0.0, 50.0]);
    graph.node_mut(mask).unwrap().params.insert("identifier".into(), Value::Text("mask".into()));
    let warp = graph.add_node("directional_warp", [100.0, 0.0]);
    graph.node_mut(warp).unwrap().params.insert("intensity".into(), Value::Float(16.0));
    graph.connect(input, "output", warp, "input").unwrap();
    graph.connect(mask, "output", warp, "intensity_input").unwrap();
    let mut package = Package::new("Warp boundaries");
    package.graphs.push(graph);
    let resource = |_: &str| None;
    let mut options = Options::new(&resource);
    options.inputs.insert("field".into(), Image::gray(16, 16, |x, y| f32::from(x == 0 && y == 0)));
    options.inputs.insert("mask".into(), Image::gray(16, 16, |_, _| 1.0));
    let cache = Mutex::new(Cache::default());

    for tiling in [[false, false], [true, false], [false, true], [true, true]] {
        package.graphs[0].tiling = tiling;
        for (axis, angle, x, y) in [(0, 0.0, 15, 0), (1, 0.25, 0, 15)] {
            package.graphs[0].node_mut(warp).unwrap().params.insert("angle".into(), Value::Float(angle));
            let cooked = cook(&package, 0, &options, &cache);
            assert_eq!(cooked.nodes[&warp].error, None);
            let image = cooked.image(warp, 0).unwrap();
            assert_eq!(image.get_gray(x, y, false), f32::from(tiling[axis]), "tiling {tiling:?}, angle {angle}");
        }
    }
}
