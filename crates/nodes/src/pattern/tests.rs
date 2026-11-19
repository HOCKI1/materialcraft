use crate::{CookContext, find, resolve_params};
use materialcraft_core::{FunctionGraph, PatternGraph, PatternNodeKind, Value};
use std::collections::BTreeMap;

#[test]
fn radial_stamps_keep_the_measured_center_and_skirt() {
    for (x, y, expected) in [
        (0xbf7c0000, 0x3c7fffbf, [0x3cfc0000, 0x35762800, 0x36cecb1c]),
        (0xbeb80001, 0xbef80001, [0x3f22e000, 0x3e27c967, 0x3c90589e]),
        (0x3c7fffbf, 0x3c7fffbf, [0x3f7fe000, 0x3f7f8018, 0x3f7e9cc6]),
    ] {
        for (shape, bits) in (4..=6).zip(expected) {
            let value = super::shape::coverage(shape, f32::from_bits(x), f32::from_bits(y), 0.0).unwrap();
            assert_eq!(value.to_bits(), bits, "shape {shape} at ({x:08x}, {y:08x})");
        }
    }
}

#[test]
fn beveled_square_multiplies_the_two_edge_profiles() {
    for (x, y, variation, expected) in [
        (-0.984375, 0.0, 0.0625, 0x3e200000),
        (-0.984375, -0.984375, 0.0625, 0x3cc80000),
        (-0.984375, -0.984375, 0.125, 0x3af20000),
        (-0.984375, -0.984375, -0.25, 0x3f800000),
    ] {
        assert_eq!(super::shape::coverage(9, x, y, variation).unwrap().to_bits(), expected);
    }
}

#[test]
fn gradient_variation_attenuates_the_horizontal_ramp() {
    for (variation, expected) in [(-0.25, 0.2578125), (0.0625, 0.241_699_22), (0.5, 0.12890625), (1.0, 0.0), (1.25, -0.064453125)] {
        assert_eq!(super::shape::coverage(10, -0.484375, 0.75, variation).unwrap(), expected);
    }
}

#[test]
fn wave_frequency_and_radial_falloff_follow_stamp_coordinates() {
    let x = f32::from_bits(0x3d5fffef);
    let y = f32::from_bits(0xbe080003);
    for (variation, expected) in [(-0.25, 0x3f6a8659), (0.0, 0x3f6a8659), (0.1875, 0x3f4b2e24), (0.25, 0x3f3dd911), (1.0, 0x3d9ac833)] {
        assert_eq!(super::shape::coverage(11, x, y, variation).unwrap().to_bits(), expected);
    }
    assert_eq!(super::shape::coverage(11, 0.0, 0.5, 0.25).unwrap(), 0.75);
}

#[test]
fn stamp_profiles_follow_the_radial_and_horizontal_falloffs() {
    // Two samples on the low-coverage skirt distinguish the signed quartic from a clamped falloff.
    for (shape, x, y, variation, bits) in [
        (7, 0xbf0c0000, 0xbd40000f, 0.0, 0x3d29a3c2),
        (12, 0xbeb80001, 0xbef80001, 0.25, 0x3e27c967),
        (12, 0x3ec7fffd, 0xbec80001, 0.25, 0x3ddfc34c),
        (12, 0x3f3bfffe, 0x3f2bfffe, 0.25, 0xadf38c4f),
        (13, 0xbf3c0000, 0x3f2bfffe, 0.25, 0xb1cb84a4),
        (13, 0x3ec7fffd, 0xbec80001, 0.25, 0x3db1415d),
        (14, 0xbeb80001, 0xbef80001, 0.0, 0x3f40da6b),
        (14, 0x3ec7fffd, 0xbec80001, 0.25, 0x3c774318),
        (15, 0x3e4ffffb, 0x3c7fffbf, 0.0, 0x3e2e002d),
        (15, 0x3e4ffffb, 0x3c7fffbf, 0.25, 0x3f7e2c09),
        (16, 0xbe880001, 0xbf740000, 0.0, 0x3c2ce9c0),
        (17, 0x3ec7fffd, 0xbec80001, 0.5, 0x3f990000),
        (18, 0x3ec7fffd, 0xbec80001, 0.25, 0x3f8ab222),
        (18, 0xbf7c0000, 0xbf7c0000, 0.25, 0x00000000),
    ] {
        let value = super::shape::coverage(shape, f32::from_bits(x), f32::from_bits(y), variation).unwrap();
        assert_eq!(value.to_bits(), bits, "shape {shape} at ({x:08x}, {y:08x})");
    }
}

fn cook(graph: PatternGraph) -> crate::CookResult {
    cook_with_variables(graph, &BTreeMap::new(), 7)
}

fn cook_with_variables(graph: PatternGraph, variables: &BTreeMap<String, Value>, seed: u32) -> crate::CookResult {
    let definition = find("fx_map").unwrap();

    // These traversal fixtures measure unattenuated grayscale stamps
    let parameters = resolve_params(
        definition,
        &BTreeMap::from([
            ("pattern".into(), Value::PatternGraph(Box::new(graph))),
            ("roughness".into(), Value::Float(0.0)),
            ("color_mode".into(), Value::Bool(false)),
        ]),
    );
    let context = CookContext {
        input_formats: &[],
        format: materialcraft_core::OutputFormat::F32,
        w: 16,
        h: 16,
        seed,
        tile: true,
        inputs: &[],
        params: &parameters,
        resource: &|_| None,
        vars: variables,
    };

    (definition.cook)(&context)
}

#[test]
fn pattern_functions_keep_host_time_and_quadrant_depth() {
    for (name, expected) in [("$time", 0.75), ("$depth", 1.0), ("$depthpow2", 0.5)] {
        let mut graph = PatternGraph::with_root(PatternNodeKind::Quadrant);
        let child = graph.add(PatternNodeKind::Quadrant).unwrap();
        graph.nodes[0].parameters.insert("patterntype".into(), Value::Int(0));
        graph.nodes[0].children[0] = Some(child);
        graph.nodes[1].parameters.insert("patterntype".into(), Value::Int(2));
        graph.nodes[1].parameters.insert("patternsize".into(), Value::Float2([2.0; 2]));
        let mut function = FunctionGraph::default();
        function.output = Some(function.add("get_float1", &[], Some(Value::Text(name.into()))));
        graph.nodes[1].parameters.insert("opacity".into(), Value::Function(Box::new(function)));

        let image = cook_with_variables(graph, &BTreeMap::from([("$time".into(), Value::Float(0.75))]), 7).unwrap().remove(0);

        assert!((image.mean()[0] - expected).abs() < 1e-6, "{name}: {}", image.mean()[0]);
    }
}

#[test]
fn iterate_sets_number_and_switch_selects_one_child() {
    let mut graph = PatternGraph::default();
    let iterate = graph.add(PatternNodeKind::Iterate).unwrap();
    let switch = graph.add(PatternNodeKind::Switch).unwrap();
    let quadrant = graph.add(PatternNodeKind::Quadrant).unwrap();
    graph.root = Some(iterate);
    graph.nodes[0].children = vec![None, Some(switch)];
    graph.nodes[0].parameters.insert("numberadded".into(), Value::Int(3));
    graph.nodes[1].children = vec![Some(quadrant), None];
    graph.nodes[2].parameters.insert("patterntype".into(), Value::Int(2));
    let mut function = FunctionGraph::default();
    function.output = Some(function.add("get_float1", &[], Some(Value::Text("$number".into()))));
    graph.nodes[2].parameters.insert("opacity".into(), Value::Function(Box::new(function)));
    let image = cook(graph.clone()).unwrap().remove(0);

    assert!((image.mean()[0] - 3.0).abs() < 1e-6);

    let dot = graph.add(PatternNodeKind::Dot).unwrap();
    graph.nodes[1].children[0] = Some(dot);
    graph.nodes[3].children[0] = Some(quadrant);

    assert_eq!(cook(graph.clone()).unwrap()[0].data(), image.data());

    graph.nodes[1].parameters.insert("switch".into(), Value::Bool(true));

    assert_eq!(cook(graph).unwrap()[0].mean()[0], 0.0);
}

#[test]
fn iterate_preserves_assignments_from_previous_iterations() {
    let mut graph = PatternGraph::with_root(PatternNodeKind::Iterate);
    let child = graph.add(PatternNodeKind::Quadrant).unwrap();
    graph.nodes[0].children = vec![None, Some(child)];
    let mut initialize = FunctionGraph::default();
    let zero = initialize.add_constant(Value::Float(0.0));
    let count = initialize.add_constant(Value::Int(3));
    let set = initialize.add("set", &[zero], Some(Value::Text("counter".into())));
    initialize.output = Some(initialize.add("sequence", &[set, count], None));
    graph.nodes[0].parameters.insert("numberadded".into(), Value::Function(Box::new(initialize)));
    let mut increment = FunctionGraph::default();
    let value = increment.add("get_float1", &[], Some(Value::Text("counter".into())));
    let one = increment.add_constant(Value::Float(1.0));
    let next = increment.add("add", &[value, one], None);
    increment.output = Some(increment.add("set", &[next], Some(Value::Text("counter".into()))));
    graph.nodes[1].parameters.insert("patterntype".into(), Value::Int(2));
    graph.nodes[1].parameters.insert("opacity".into(), Value::Function(Box::new(increment)));
    let image = cook(graph).unwrap().remove(0);

    assert!(image.data().iter().all(|value| *value == 6.0), "expected 1 + 2 + 3, got {:?}", image.mean());
}

#[test]
fn an_iterate_keeps_the_initial_seed_and_advances_each_repetition() {
    for (slot, count, expected) in [(0, 1, 0x3f042a5e), (1, 1, 0x3f1abd9c), (1, 2, 0x3f2b7424)] {
        let mut graph = PatternGraph::with_root(PatternNodeKind::Iterate);
        let child = graph.add(PatternNodeKind::Quadrant).unwrap();
        graph.nodes[0].children[slot] = Some(child);
        graph.nodes[0].parameters.insert("numberadded".into(), Value::Int(count));
        let params = &mut graph.nodes[1].parameters;
        params.insert("patterntype".into(), Value::Int(2));
        params.insert("blendingmode".into(), Value::Int(2));
        let mut draw = FunctionGraph::default();
        let one = draw.add_constant(Value::Float(1.0));
        draw.output = Some(draw.add("rand", &[one], None));
        params.insert("opacity".into(), Value::Function(Box::new(draw)));
        let image = cook(graph).unwrap().remove(0);
        assert_eq!(image.get(8, 8, false)[0].to_bits(), expected);
    }
}

#[test]
fn a_pattern_seed_expression_runs_before_the_stamp_properties() {
    let mut seed = FunctionGraph::default();
    let one = seed.add_constant(Value::Float(1.0));
    let draw = seed.add("rand", &[one], None);
    let save = seed.add("set", &[draw], Some(Value::Text("seed_draw".into())));
    let local = seed.add_constant(Value::Int(43));
    seed.output = Some(seed.add("sequence", &[save, local], None));
    for (inherits, scope_value, expected) in [(true, false, 0x3f4fd146), (false, false, 0x3e98dd54), (true, true, 0x3ed93158), (false, true, 0x3ed93158)] {
        let mut graph = PatternGraph::with_root(PatternNodeKind::Quadrant);
        let params = &mut graph.nodes[0].parameters;
        params.insert("randominherited".into(), Value::Bool(inherits));
        params.insert("randomseed".into(), Value::Function(Box::new(seed.clone())));
        params.insert("patterntype".into(), Value::Int(2));
        let mut opacity = FunctionGraph::default();
        let one = opacity.add_constant(Value::Float(1.0));
        opacity.output =
            Some(if scope_value { opacity.add("get_float1", &[], Some(Value::Text("seed_draw".into()))) } else { opacity.add("rand", &[one], None) });
        params.insert("opacity".into(), Value::Function(Box::new(opacity)));
        let image = cook_with_variables(graph, &BTreeMap::new(), 7).unwrap().remove(0);
        assert_eq!(image.get(8, 8, false)[0].to_bits(), expected);
    }
}

#[test]
fn a_pattern_uses_the_seed_without_exposing_a_variable_for_it() {
    let mut graph = PatternGraph::with_root(PatternNodeKind::Quadrant);
    let params = &mut graph.nodes[0].parameters;
    params.insert("patterntype".into(), Value::Int(2));
    params.insert("randomseed".into(), Value::Int(43));
    let mut opacity = FunctionGraph::default();
    let variable = Value::Text("$randomseed".into());
    let exists = opacity.add("isdefined", &[], Some(variable.clone()));
    let value = opacity.add("get_integer1", &[], Some(variable));
    let numeric = opacity.add("tofloat", &[value], None);
    let fallback = opacity.add_constant(Value::Float(0.25));
    opacity.output = Some(opacity.add("ifelse", &[exists, numeric, fallback], None));
    params.insert("opacity".into(), Value::Function(Box::new(opacity)));
    let image = cook_with_variables(graph, &BTreeMap::new(), 7).unwrap().remove(0);
    assert_eq!(image.get(8, 8, false)[0], 0.25);
}

#[test]
fn quadrant_properties_draw_from_independent_streams() {
    for (property, setting, expected) in [
        ("branchoffset", Value::Float2([0.0; 2]), 0x3ee082ec),
        ("frameoffset", Value::Float2([0.0; 2]), 0x3f11adbc),
        ("patternsize", Value::Float2([0.25; 2]), 0x3f0db18c),
        ("patternrotation", Value::Float(0.0), 0x3f30ca04),
        ("patternsuppl", Value::Float(0.5), 0x3ad62400),
    ] {
        let mut graph = PatternGraph::with_root(PatternNodeKind::Quadrant);
        let child = graph.add(PatternNodeKind::Quadrant).unwrap();
        graph.nodes[0].children[0] = Some(child);
        let parent = &mut graph.nodes[0].parameters;
        parent.insert("patterntype".into(), Value::Int(2));
        parent.insert("opacity".into(), Value::Float(0.0));
        let mut write = FunctionGraph::default();
        let one = write.add_constant(Value::Float(1.0));
        let draw = write.add("rand", &[one], None);
        let store = write.add("set", &[draw], Some(Value::Text("measured".into())));
        let setting = write.add_constant(setting);
        write.output = Some(write.add("sequence", &[store, setting], None));
        parent.insert(property.into(), Value::Function(Box::new(write)));
        let mut read = FunctionGraph::default();
        read.output = Some(read.add("get_float1", &[], Some(Value::Text("measured".into()))));
        let leaf = &mut graph.nodes[1].parameters;
        leaf.insert("patterntype".into(), Value::Int(2));
        leaf.insert("opacity".into(), Value::Function(Box::new(read)));
        let image = cook_with_variables(graph, &BTreeMap::new(), 42).unwrap().remove(0);
        assert_eq!(image.get(4, 4, false)[0].to_bits(), expected, "{property}");
    }
}

#[test]
fn patterns_reject_cycles_and_bound_exponential_traversal() {
    let mut graph = PatternGraph::default();
    let root = graph.add(PatternNodeKind::Quadrant).unwrap();
    graph.nodes[0].children[0] = Some(root);
    assert!(cook(graph).unwrap_err().contains("cycle"));

    let mut graph = PatternGraph::default();
    for _ in 0..12 {
        graph.add(PatternNodeKind::Quadrant).unwrap();
    }
    for index in 0..11 {
        graph.nodes[index].children = vec![Some(graph.nodes[index + 1].id); 4];
    }
    assert!(cook(graph).unwrap_err().contains("budget"));
}

#[test]
fn image_indices_outside_the_supported_range_return_an_error() {
    let mut graph = PatternGraph::with_root(PatternNodeKind::Quadrant);
    graph.nodes[0].parameters.insert("patterntype".into(), Value::Int(1));
    graph.nodes[0].parameters.insert("imageindex".into(), Value::Int(i32::MAX));
    assert!(cook(graph).unwrap_err().contains("unsupported pattern"));
}

fn roughness_sample(roughness: f32, depth: usize, blend: i32, color: bool, repeat: bool) -> crate::CookResult {
    let mut graph = PatternGraph::with_root(PatternNodeKind::Quadrant);
    for index in 0..depth {
        let child = graph.add(PatternNodeKind::Quadrant).unwrap();
        graph.nodes[index].children[0] = Some(child);
    }
    let leaf = depth;
    let leaf_id = graph.nodes[leaf].id;
    graph.nodes[leaf].parameters.insert("patterntype".into(), Value::Int(2));
    graph.nodes[leaf].parameters.insert("opacity".into(), Value::Float4([0.8, 0.4, 0.2, 1.0]));
    graph.nodes[leaf].parameters.insert("blendingmode".into(), Value::Int(blend));
    if repeat {
        let dot = graph.add(PatternNodeKind::Dot).unwrap();
        graph.nodes.last_mut().unwrap().children[0] = Some(leaf_id);
        let switch = graph.add(PatternNodeKind::Switch).unwrap();
        graph.nodes.last_mut().unwrap().children[0] = Some(dot);
        let iterate = graph.add(PatternNodeKind::Iterate).unwrap();
        graph.nodes.last_mut().unwrap().children = vec![None, Some(switch)];
        graph.nodes.last_mut().unwrap().parameters.insert("numberadded".into(), Value::Int(2));
        if leaf == 0 {
            graph.root = Some(iterate);
        } else {
            graph.nodes[leaf - 1].children[0] = Some(iterate);
        }
    }
    let definition = find("fx_map").unwrap();
    let parameters = resolve_params(
        definition,
        &BTreeMap::from([
            ("pattern".into(), Value::PatternGraph(Box::new(graph))),
            ("roughness".into(), Value::Float(roughness)),
            ("opacity".into(), Value::Float(0.5)),
            ("color_mode".into(), Value::Bool(color)),
            ("background".into(), Value::Color([0.1; 4])),
        ]),
    );
    let context = CookContext {
        input_formats: &[],
        format: materialcraft_core::OutputFormat::F32,
        w: 16,
        h: 16,
        seed: 7,
        tile: true,
        inputs: &[],
        params: &parameters,
        resource: &|_| None,
        vars: &BTreeMap::new(),
    };

    (definition.cook)(&context)
}

#[test]
fn roughness_attenuates_quadrants_by_depth_and_keeps_background() {
    for color in [false, true] {
        for (roughness, depth, weight) in [(0.0, 2, 1.0), (1.0, 0, 1.0), (1.0, 1, 0.5), (0.5, 2, 1.0 / 7.0), (2.0, 2, 4.0 / 7.0)] {
            for blend in 0..=2 {
                let image = roughness_sample(roughness, depth, blend, color, false).unwrap().remove(0);
                let pixel = image.get(0, 0, false);
                for (channel, foreground) in [0.8_f32, 0.4, 0.2, 1.0].into_iter().take(image.channels()).enumerate() {
                    let value = foreground * 0.5 * weight;
                    let expected = match blend {
                        1 if !color => value.max(0.1),
                        1 => value + 0.1 * (1.0 - 0.5 * weight),
                        2 => value.max(0.1),
                        _ => value + 0.1,
                    };

                    assert!((pixel[channel] - expected).abs() < 1e-6, "roughness={roughness}, depth={depth}, blend={blend}, channel={channel}: {pixel:?}");
                }
                if depth > 0 {
                    assert!((image.get(15, 15, false)[0] - 0.1).abs() < 1e-6);
                }
            }
        }
    }
}

#[test]
fn repetition_switches_and_dots_do_not_add_roughness_depth() {
    let image = roughness_sample(1.0, 2, 0, false, true).unwrap().remove(0);

    assert!((image.get(0, 0, false)[0] - (0.1 + 0.8 / 3.0)).abs() < 1e-6);
    assert!((image.get(15, 15, false)[0] - 0.1).abs() < 1e-6);
}

#[test]
fn negative_roughness_reports_an_error() {
    assert!(roughness_sample(-32.0, 5, 0, false, false).unwrap_err().contains("roughness must be finite and nonnegative"));
}

#[test]
fn a_full_nine_level_quadtree_renders_one_pattern_per_pixel() {
    let mut graph = PatternGraph::with_root(PatternNodeKind::Quadrant);
    for index in 0..8 {
        let child = graph.add(PatternNodeKind::Quadrant).unwrap();
        graph.nodes[index].children = vec![Some(child); 4];
    }
    graph.nodes.last_mut().unwrap().parameters.insert("patterntype".into(), Value::Int(2));
    let definition = find("fx_map").unwrap();
    let parameters = resolve_params(
        definition,
        &BTreeMap::from([
            ("pattern".into(), Value::PatternGraph(Box::new(graph))),
            ("roughness".into(), Value::Float(0.0)),
            ("color_mode".into(), Value::Bool(false)),
        ]),
    );
    let context = CookContext {
        input_formats: &[],
        format: materialcraft_core::OutputFormat::F32,
        w: 256,
        h: 256,
        seed: 7,
        tile: true,
        inputs: &[],
        params: &parameters,
        resource: &|_| None,
        vars: &BTreeMap::new(),
    };

    let images = (definition.cook)(&context).unwrap();

    assert!(images[0].data().iter().all(|value| (*value - 1.0).abs() < 1e-6));
}

fn square(graph: &mut PatternGraph, node: usize) {
    graph.nodes[node].parameters.insert("patterntype".into(), Value::Int(2));
}

#[test]
fn an_iteration_reads_the_variables_the_previous_iteration_set() {
    let mut graph = PatternGraph::default();
    let iterate = graph.add(PatternNodeKind::Iterate).unwrap();
    let quadrant = graph.add(PatternNodeKind::Quadrant).unwrap();
    graph.root = Some(iterate);
    graph.nodes[0].parameters.insert("numberadded".into(), Value::Int(3));
    graph.nodes[0].children[1] = Some(quadrant);
    square(&mut graph, 1);

    let mut function = FunctionGraph::default();
    let total = function.add("get_float1", &[], Some(Value::Text("total".into())));
    let step = function.add_constant(Value::Float(0.25));
    let next = function.add("add", &[total, step], None);
    function.output = Some(function.add("set", &[next], Some(Value::Text("total".into()))));
    graph.nodes[1].parameters.insert("opacity".into(), Value::Function(Box::new(function)));

    let image = cook_with_variables(graph, &BTreeMap::from([("total".into(), Value::Float(0.0))]), 7).unwrap().remove(0);

    // 0.25, 0.5 and 0.75 add up; three separate copies of the variable would add 0.25 three times
    assert!((image.mean()[0] - 1.5).abs() < 1e-5, "{}", image.mean()[0]);
}

#[test]
fn the_render_region_is_the_part_of_pattern_space_the_image_shows() {
    let mut graph = PatternGraph::with_root(PatternNodeKind::Quadrant);
    square(&mut graph, 0);
    let definition = find("fx_map").unwrap();
    let parameters = resolve_params(
        definition,
        &BTreeMap::from([("pattern".into(), Value::PatternGraph(Box::new(graph))), ("render_region".into(), Value::Float4([0.0, 2.0, 0.0, 2.0]))]),
    );
    let context = CookContext {
        input_formats: &[],
        format: materialcraft_core::OutputFormat::F32,
        w: 16,
        h: 16,
        seed: 7,
        tile: false,
        inputs: &[],
        params: &parameters,
        resource: &|_| None,
        vars: &BTreeMap::new(),
    };

    let image = (definition.cook)(&context).unwrap().remove(0);

    // The unit square fills the top-left quarter of a region twice as wide and tall
    assert!((image.mean()[0] - 0.25).abs() < 1e-5, "{}", image.mean()[0]);
    assert_eq!(image.get(3, 3, false)[0], 1.0);
    assert_eq!(image.get(12, 12, false)[0], 0.0);
}

#[test]
fn every_visit_draws_its_own_random_numbers() {
    let mut graph = PatternGraph::with_root(PatternNodeKind::Quadrant);
    graph.nodes[0].parameters.insert("patterntype".into(), Value::Int(0));
    let child = graph.add(PatternNodeKind::Quadrant).unwrap();
    for slot in 0..4 {
        graph.nodes[0].children[slot] = Some(child);
    }
    square(&mut graph, 1);
    let mut function = FunctionGraph::default();
    let range = function.add_constant(Value::Float(1.0));
    function.output = Some(function.add("rand", &[range], None));
    graph.nodes[1].parameters.insert("opacity".into(), Value::Function(Box::new(function)));

    let image = cook(graph).unwrap().remove(0);

    let quarters = [image.get(4, 4, false)[0], image.get(12, 4, false)[0], image.get(4, 12, false)[0], image.get(12, 12, false)[0]];

    assert!(quarters.windows(2).all(|pair| (pair[0] - pair[1]).abs() > 1e-4), "{quarters:?}");
}

#[test]
fn a_pixel_on_the_seam_of_two_patterns_is_drawn_once() {
    let mut graph = PatternGraph::with_root(PatternNodeKind::Quadrant);
    graph.nodes[0].parameters.insert("patterntype".into(), Value::Int(0));
    let child = graph.add(PatternNodeKind::Quadrant).unwrap();
    let grandchild = graph.add(PatternNodeKind::Quadrant).unwrap();
    for slot in 0..4 {
        graph.nodes[0].children[slot] = Some(child);
        graph.nodes[1].children[slot] = Some(grandchild);
    }
    graph.nodes[1].parameters.insert("patterntype".into(), Value::Int(0));
    square(&mut graph, 2);
    let definition = find("fx_map").unwrap();
    let parameters = resolve_params(
        definition,
        &BTreeMap::from([
            ("pattern".into(), Value::PatternGraph(Box::new(graph))),
            ("roughness".into(), Value::Float(0.0)),
            ("color_mode".into(), Value::Bool(false)),
        ]),
    );
    let variables = BTreeMap::new();

    // Two pixels a side put every pixel center on the corner of four of the sixteen squares
    let context = CookContext {
        input_formats: &[],
        format: materialcraft_core::OutputFormat::F32,
        w: 2,
        h: 2,
        seed: 7,
        tile: true,
        inputs: &[],
        params: &parameters,
        resource: &|_| None,
        vars: &variables,
    };

    let image = (definition.cook)(&context).unwrap().remove(0);

    assert_eq!(image.get(0, 0, false)[0], 1.0);
    assert_eq!(image.get(1, 1, false)[0], 1.0);
}
