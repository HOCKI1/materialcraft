use crate::{CookContext, find, resolve_params};
use materialcraft_core::{FunctionGraph, PatternGraph, PatternNodeKind, Value};
use std::collections::BTreeMap;

fn coverage(shape: i32, x: f32, y: f32, variation: f32) -> f32 {
    let sample = super::shape::prepare(shape, variation);
    let (base, offset) = sample(x, y).unwrap();
    base + offset
}

#[test]
fn gaussian_stamps_stop_at_the_disc_boundary() {
    let x = 0.6;
    let inside = 0.79999;
    let outside = 0.80001;
    assert_eq!(coverage(3, x, inside, 0.0), 1.0);
    assert!(coverage(6, x, inside, 0.0) > 0.0);
    assert_eq!(coverage(3, x, outside, 0.0), 0.0);
    assert_eq!(coverage(6, x, outside, 0.0), 0.0);
}

#[test]
fn radial_stamps_keep_the_measured_center_and_skirt() {
    for (x, y, expected) in [
        (0xbf7c0000, 0x3c7fffbf, [0x3cfc0000, 0x35762800, 0x36cecb1c]),
        (0xbeb80001, 0xbef80001, [0x3f22e000, 0x3e27c967, 0x3c90589e]),
        (0x3c7fffbf, 0x3c7fffbf, [0x3f7fe000, 0x3f7f8018, 0x3f7e9cc6]),
    ] {
        for (shape, bits) in (4..=6).zip(expected) {
            let value = coverage(shape, f32::from_bits(x), f32::from_bits(y), 0.0);
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
        assert_eq!(coverage(9, x, y, variation).to_bits(), expected);
    }
}

#[test]
fn beveled_square_uses_the_pixel_reciprocal_for_its_edge_width() {
    // Pixel (1, 31) in a centered 64 by 64 stamp lies 3/64 inside the left edge.
    for (width, expected) in [(0.072_187_5, 0x3f37_a51f), (0.718_97, 0x3c47_d97b)] {
        assert_eq!(coverage(9, -0.953125, -0.015625, width).to_bits(), expected);
    }
}

#[test]
fn horizontal_stamp_fades_use_the_pixel_reciprocal_for_the_remaining_width() {
    let coordinate = f32::from_bits(0x3f03_fffe);
    let variation = 86.0 / 1021.0;
    assert_eq!(coverage(12, coordinate, coordinate, variation).to_bits(), 0x3c9f_e0f0);
    assert_eq!(coverage(14, -0.265625, 0.109375, variation).to_bits(), 0x3e9b_3a66);
}

#[test]
fn gradient_variation_attenuates_the_horizontal_ramp() {
    for (variation, expected) in [(-0.25, 0.2578125), (0.0625, 0.241_699_22), (0.5, 0.12890625), (1.0, 0.0), (1.25, -0.064453125)] {
        assert_eq!(coverage(10, -0.484375, 0.75, variation), expected);
    }
}

#[test]
fn wave_frequency_and_radial_falloff_follow_stamp_coordinates() {
    let x = f32::from_bits(0x3d5fffef);
    let y = f32::from_bits(0xbe080003);
    for (variation, expected) in [(-0.25, 0x3f6a8659), (0.0, 0x3f6a8659), (0.1875, 0x3f4b2e24), (0.25, 0x3f3dd911), (1.0, 0x3d9ac833)] {
        assert_eq!(coverage(11, x, y, variation).to_bits(), expected);
    }
    assert_eq!(coverage(11, 0.0, 0.5, 0.25), 0.75);
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
        let value = coverage(shape, f32::from_bits(x), f32::from_bits(y), variation);
        assert_eq!(value.to_bits(), bits, "shape {shape} at ({x:08x}, {y:08x})");
    }
}

fn cook(graph: PatternGraph) -> crate::CookResult {
    cook_with_variables(graph, &BTreeMap::new(), 7)
}

#[test]
fn stamp_tint_preserves_the_authored_type_and_scales_before_coverage() {
    let rgba = Value::Float4([0.9, 0.11, 0.57, 0.732]);
    let constant = |value| Value::Function(Box::new(FunctionGraph::constant(value)));
    for (shape, color, tint, gain, expected) in [
        (2, false, rgba.clone(), 1.0, [0x3f06_d3a0; 4]),
        (2, false, constant(rgba.clone()), 1.0, [0x3f3b_645a; 4]),
        (5, true, constant(Value::Float(0.732)), 0.6137, [0x3ee4_3699; 4]),
        (5, true, rgba.clone(), 0.6137, [0x3ecd_6456, 0x3d48_d3e4, 0x3e82_14e2, 0x3ee4_3699]),
        (17, true, constant(rgba), 327.125, [0x42de_3ea0, 0x4159_4e4d, 0x428c_c144, 0x42f6_f041]),
    ] {
        assert_stamp_pixel(shape, color, tint, gain, 0.0, expected);
    }
}

#[test]
fn pattern_opacity_flushes_subnormals_at_their_arithmetic_boundaries() {
    let normal = f32::MIN_POSITIVE;
    let tiny = f32::from_bits(0x0037_1245);
    let large = f32::from_bits(0x5d80_0000);
    for (tint, gain, background, expected) in [
        ([large, large, large, 0.25], normal, 0.0, [0; 4]),
        ([1.0, 1.0, 1.0, tiny], large, 0.0, [0; 4]),
        ([tiny, tiny, tiny, 1.0], large, 0.0, [0, 0, 0, large.to_bits()]),
        ([1.0, 1.0, 1.0, -1.0], f32::from_bits(0x0080_0001), normal, [0x8000_0000; 4]),
        ([1.0; 4], normal, 0.0, [normal.to_bits(); 4]),
    ] {
        assert_stamp_pixel(2, true, Value::Float4(tint), gain, background, expected);
    }
    for color in [false, true] {
        for shape in [0, 2] {
            assert_stamp_pixel(shape, color, Value::Float4([1.0; 4]), 0.0, -tiny, [0x8000_0000; 4]);
        }
    }
}

#[test]
fn solid_stamps_round_the_alpha_blend_at_accumulator_precision() {
    for (background, expected) in
        [(0x393f_a080, [0x3e81_77f4, 0x3e0f_efe8, 0x3eba_f7f4, 0x3ee6_0d32]), (0xbb89_bb22, [0x3e7e_7227, 0x3e0b_7227, 0x3eb8_b913, 0x3ee4_d090])]
    {
        assert_stamp_pixel(2, true, Value::Float4([0.5625, 0.3125, 0.8125, 0.71875]), 0.625, f32::from_bits(background), expected);
    }
}

fn assert_stamp_pixel(shape: i32, color: bool, tint: Value, gain: f32, background: f32, expected: [u32; 4]) {
    let definition = find("fx_map").unwrap();
    let mut graph = PatternGraph::with_root(PatternNodeKind::Quadrant);
    graph.nodes[0].parameters = BTreeMap::from([("patterntype".into(), Value::Int(shape)), ("opacity".into(), tint), ("blendingmode".into(), Value::Int(0))]);
    let params = resolve_params(
        definition,
        &BTreeMap::from([
            ("pattern".into(), Value::PatternGraph(Box::new(graph))),
            ("roughness".into(), Value::Float(0.0)),
            ("color_mode".into(), Value::Bool(color)),
            ("opacity".into(), Value::Float(gain)),
            ("background".into(), Value::Color([background; 4])),
        ]),
    );
    let context = CookContext {
        input_formats: &[],
        format: materialcraft_core::OutputFormat::F32,
        w: 32,
        h: 32,
        seed: 7,
        tile: true,
        inputs: &[],
        params: &params,
        resource: &|_| None,
        vars: &BTreeMap::new(),
    };
    let image = (definition.cook)(&context).unwrap().remove(0);
    let pixel = image.get(16, 16, false).map(f32::to_bits);
    assert_eq!(&pixel[..image.channels()], &expected[..image.channels()], "shape {shape}, color {color}, opacity {gain}");
}

#[test]
fn gradient_stamp_offsets_apply_to_the_tinted_channels() {
    let background = [0.5, 0.375, 0.75, 0.875];
    for (shape, variation, blend, background, position, expected) in [
        (17, 0.25, 0, [0.0; 4], 16, [0x3ebde000, 0x3e9ef000, 0x3ed28000, 0x3ed28000]),
        (17, -0.125, 0, [0.0; 4], 16, [0xbb880000, 0xbd844000, 0x3d140000, 0x3d140000]),
        (2, 0.0, 0, background, 16, [0x3f3c0000, 0x3efc0000, 0x3f880000, 0x3f6a0000]),
        (2, 0.0, 2, background, 16, [0x3f140000, 0x3ec00000, 0x3f540000, 0x3f6a0000]),
        (18, -0.125, 1, background, 0, background.map(f32::to_bits)),
    ] {
        let mut graph = PatternGraph::with_root(PatternNodeKind::Quadrant);
        graph.nodes[0].parameters = BTreeMap::from([
            ("patterntype".into(), Value::Int(shape)),
            ("patternsuppl".into(), Value::Float(variation)),
            ("opacity".into(), Value::Float4([0.75, 0.375, 1.0, 0.625])),
            ("blendingmode".into(), Value::Int(blend)),
        ]);
        let definition = find("fx_map").unwrap();
        let parameters = resolve_params(
            definition,
            &BTreeMap::from([
                ("pattern".into(), Value::PatternGraph(Box::new(graph))),
                ("color_mode".into(), Value::Bool(true)),
                ("roughness".into(), Value::Float(0.0)),
                ("opacity".into(), Value::Float(0.5)),
                ("background".into(), Value::Color(background)),
            ]),
        );
        let context = CookContext {
            input_formats: &[],
            format: materialcraft_core::OutputFormat::F32,
            w: 32,
            h: 32,
            seed: 468,
            tile: true,
            inputs: &[],
            params: &parameters,
            resource: &|_| None,
            vars: &BTreeMap::new(),
        };
        let image = (definition.cook)(&context).unwrap().remove(0);
        assert_eq!(image.get(position, position, false).map(f32::to_bits), expected);
    }
}

fn cook_with_variables(graph: PatternGraph, variables: &BTreeMap<String, Value>, seed: u32) -> crate::CookResult {
    cook_at_size(graph, variables, seed, 16)
}

fn cook_at_size(graph: PatternGraph, variables: &BTreeMap<String, Value>, seed: u32, side: u32) -> crate::CookResult {
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
        w: side,
        h: side,
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
fn stamp_gradients_keep_the_phase_of_extended_turns() {
    for (turns, x, y, bits) in [(1.5, 30, 6, 0x3f05_ffff), (1.75, 6, 30, 0x3f05_ffff), (-3.0, 19, 26, 0x3e9b_fffe)] {
        let mut graph = PatternGraph::with_root(PatternNodeKind::Quadrant);
        graph.nodes[0].parameters = BTreeMap::from([("patterntype".into(), Value::Int(10)), ("patternrotation".into(), Value::Float(turns))]);
        let image = cook_at_size(graph, &BTreeMap::new(), 43, 64).unwrap().remove(0);
        assert_eq!(image.get_gray(x, y, false).to_bits(), bits, "{turns} turns at ({x},{y})");
    }
}

#[test]
fn fractional_stamp_edges_set_both_coordinate_gradients() {
    for (size, shift, turn, x, y, expected) in [
        ([0.375, 0.23046875], [0.0, 0.0], 0.0, 21, 32, 0x3d80_0004),
        ([0.375, 0.23828125], [0.0, 0.00390625], 0.0, 35, 32, 0x3f25_5554),
        ([0.625, 0.234375], [0.0078125, 0.0], 0.0, 37, 32, 0x3f1f_ffff),
        ([0.23828125, 0.375], [0.0, 0.001953125], 0.25, 20, 34, 0x3f27_de6c),
        ([0.2578125, 0.375], [0.0, 0.001953125], 0.25, 20, 33, 0x3f15_5554),
    ] {
        let mut graph = PatternGraph::with_root(PatternNodeKind::Quadrant);
        graph.nodes[0].parameters = BTreeMap::from([
            ("patterntype".into(), Value::Int(10)),
            ("patternsize".into(), Value::Float2(size)),
            ("frameoffset".into(), Value::Float2(shift)),
            ("patternrotation".into(), Value::Float(turn)),
        ]);
        let image = cook_at_size(graph, &BTreeMap::new(), 428, 64).unwrap().remove(0);
        assert_eq!(image.get_gray(x, y, false).to_bits(), expected, "size {size:?}, turn {turn}, pixel ({x},{y})");
    }
}

#[test]
fn wrapped_beveled_stamp_retains_its_coordinate_at_the_crop() {
    let mut graph = PatternGraph::with_root(PatternNodeKind::Quadrant);
    graph.nodes[0].parameters = BTreeMap::from([
        ("patterntype".into(), Value::Int(9)),
        ("patternsuppl".into(), Value::Float(0.7265625)),
        ("patternsize".into(), Value::Float2([0.984375, 0.484375])),
        ("frameoffset".into(), Value::Float2([0.5, 0.25])),
    ]);
    let image = cook_at_size(graph, &BTreeMap::new(), 261312, 128).unwrap().remove(0);
    assert_eq!(image.get_gray(48, 88, false).to_bits(), 0x3e73_2b04);
}

#[test]
fn clipped_stamp_gradients_use_one_triangle_vertex_for_both_axes() {
    let region = super::region::Region::new([0.0, 1.0, 0.0, 1.0]).unwrap();
    for (canvas, center, size, expected) in [
        ([128, 64], [-0.2734375, 0.3125], [0.734375_f32, 0.734375], 0x3f60_ae4e),
        ([128, 64], [-0.2734375, 0.3125], [-0.734375, 0.734375], 0x3dfa_8d8c),
        ([128, 128], [-0.3046875, -0.265625], [0.7578125, 0.6953125], 0x3f68_3f56),
    ] {
        let frame = super::footprint::Footprint { sine: 0.0, cosine: 1.0, radius: size.map(|s| s.abs() * 0.5) };
        let planes = super::coordinates::PixelPlanes::new(&frame, &region, center, size, canvas).unwrap();
        let [x, _] = planes.row(0)(0);
        assert_eq!(((x + 1.0) * 0.5).to_bits(), expected, "center {center:?}, size {size:?}");
    }
}

#[test]
fn gaussian_stamp_coordinates_follow_the_pixel_footprint() {
    let definition = find("fx_map").unwrap();
    let mut graph = PatternGraph::with_root(PatternNodeKind::Quadrant);
    graph.nodes[0].parameters = BTreeMap::from([
        ("patterntype".into(), Value::Int(6)),
        ("blendingmode".into(), Value::Int(1)),
        ("patternsize".into(), Value::Float2([3.0, 1.5])),
        ("frameoffset".into(), Value::Float2([-0.25; 2])),
    ]);
    let params = resolve_params(
        definition,
        &BTreeMap::from([
            ("pattern".into(), Value::PatternGraph(Box::new(graph))),
            ("color_mode".into(), Value::Bool(false)),
            ("roughness".into(), Value::Float(0.0)),
            ("opacity".into(), Value::Float(0.995)),
        ]),
    );
    for (w, h, x, y, expected) in [
        (16, 16, 7, 1, 0x3ef8_8006),
        (16, 32, 0, 14, 0x3eb2_37e3),
        (32, 32, 0, 14, 0x3eac_19c4),
        (8, 64, 0, 30, 0x3e9b_8a3c),
        (64, 128, 2, 62, 0x3e85_65a0),
        (128, 256, 0, 126, 0x3e69_16dd),
    ] {
        let context = CookContext {
            w,
            h,
            seed: 223,
            format: materialcraft_core::OutputFormat::F32,
            input_formats: &[],
            tile: false,
            inputs: &[],
            params: &params,
            resource: &|_| None,
            vars: &BTreeMap::new(),
        };
        let image = (definition.cook)(&context).unwrap().remove(0);
        assert_eq!(image.get_gray(x, y, false).to_bits(), expected, "{w}x{h} at ({x},{y})");
    }
}

#[test]
fn oblique_square_keeps_the_last_sample_inside_its_edge() {
    let mut graph = PatternGraph::with_root(PatternNodeKind::Quadrant);
    graph.nodes[0].parameters = BTreeMap::from([
        ("patterntype".into(), Value::Int(2)),
        ("patternsize".into(), Value::Float2([0.755, 0.391])),
        ("patternrotation".into(), Value::Float(0.13125)),
        ("frameoffset".into(), Value::Float2([-0.1113, 0.1437])),
    ]);
    let image = cook_at_size(graph, &BTreeMap::new(), 1108, 512).unwrap().remove(0);
    assert_eq!(image.get_gray(307, 492, false), 1.0);
    assert_eq!(image.get_gray(308, 492, false), 0.0);
    assert_eq!(image.get_gray(307, 493, false), 0.0);
}

#[test]
fn quarter_turns_keep_stamp_edge_ownership_in_image_space() {
    // The sample centers on all four rectangle edges distinguish image-space ownership after rotation.
    let size = [0.625, 0.25];
    for (angle, size, first, last) in [
        (0.0, size, [13, 25], [52, 40]),
        (0.25, size, [25, 13], [40, 52]),
        (0.5, size, [13, 25], [52, 40]),
        (0.75, size, [25, 13], [40, 52]),
        (-0.25, size, [25, 13], [40, 52]),
        (1.25, size, [25, 13], [40, 52]),
        (0.0, [-0.625, 0.25], [13, 25], [52, 40]),
        (0.25, [-0.625, -0.25], [25, 13], [40, 52]),
    ] {
        let mut graph = PatternGraph::with_root(PatternNodeKind::Quadrant);
        graph.nodes[0].parameters = BTreeMap::from([
            ("patterntype".into(), Value::Int(2)),
            ("patternsize".into(), Value::Float2(size)),
            ("patternrotation".into(), Value::Float(angle)),
            ("frameoffset".into(), Value::Float2([1.0 / 128.0; 2])),
        ]);
        let image = cook_at_size(graph, &BTreeMap::new(), 943, 64).unwrap().remove(0);
        for y in 0..64 {
            for x in 0..64 {
                let covered = (first[0]..=last[0]).contains(&x) && (first[1]..=last[1]).contains(&y);
                assert_eq!(image.get_gray(x, y, false), f32::from(covered), "turn {angle} at ({x},{y})");
            }
        }
    }
}

#[test]
fn a_sample_on_the_stored_lower_stamp_edge_is_excluded() {
    let size = [0.4, 0.3];
    let frame = super::footprint::Footprint { sine: 0.0, cosine: 1.0, radius: size.map(|v| v * 0.5) };
    let sample = [14.5 / 32.0, 9.5 / 16.0];
    for axis in 0..2 {
        let mut center = sample;
        center[axis] += frame.radius[axis];
        assert!(frame.local(sample, center, size).is_none());
        let mut inside = sample;
        inside[axis] += 1.0 / [32.0, 16.0][axis];
        assert!(frame.local(inside, center, size).is_some());
    }
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
fn repeated_empty_size_switches_leave_the_visit_budget_for_drawing() {
    let mut graph = PatternGraph::with_root(PatternNodeKind::Iterate);
    let mark = graph.add(PatternNodeKind::Quadrant).unwrap();
    let repeat = graph.add(PatternNodeKind::Iterate).unwrap();
    let gate = graph.add(PatternNodeKind::Switch).unwrap();
    let bright = graph.add(PatternNodeKind::Quadrant).unwrap();
    graph.nodes[0].children = vec![Some(mark), Some(repeat)];
    graph.nodes[0].parameters.insert("numberadded".into(), Value::Int(1024));
    graph.nodes[1].parameters.insert("patterntype".into(), Value::Int(2));
    graph.nodes[1].parameters.insert("opacity".into(), Value::Float(0.5));
    graph.nodes[2].children[1] = Some(gate);
    graph.nodes[2].parameters.insert("numberadded".into(), Value::Int(1024));
    graph.nodes[3].children[1] = Some(bright);
    let mut condition = FunctionGraph::default();
    let log_size = condition.add("get_float2", &[], Some(Value::Text("$sizelog2".into())));
    let width = condition.add("swizzle1", &[log_size], Some(Value::Int(0)));
    let threshold = condition.add_constant(Value::Float(6.0));
    condition.output = Some(condition.add("gteq", &[width, threshold], None));
    graph.nodes[3].parameters.insert("switch".into(), Value::Function(Box::new(condition)));
    graph.nodes[4].parameters.insert("patterntype".into(), Value::Int(2));
    assert_eq!(cook(graph).unwrap()[0].get_gray(4, 4, false), 0.5);
}

#[test]
fn empty_switches_at_later_depths_still_affect_roughness() {
    let mut graph = PatternGraph::with_root(PatternNodeKind::Quadrant);
    let empty = graph.add(PatternNodeKind::Switch).unwrap();
    let deeper = graph.add(PatternNodeKind::Quadrant).unwrap();
    graph.nodes[0].parameters.insert("patterntype".into(), Value::Int(2));
    graph.nodes[0].children = vec![Some(empty), None, None, Some(deeper)];
    graph.nodes[2].children[3] = Some(empty);
    let definition = find("fx_map").unwrap();
    let parameters = resolve_params(
        definition,
        &BTreeMap::from([
            ("pattern".into(), Value::PatternGraph(Box::new(graph))),
            ("roughness".into(), Value::Float(1.0)),
            ("color_mode".into(), Value::Bool(false)),
        ]),
    );
    let context = CookContext {
        input_formats: &[],
        format: materialcraft_core::OutputFormat::F32,
        w: 16,
        h: 16,
        seed: 0,
        tile: true,
        inputs: &[],
        params: &parameters,
        resource: &|_| None,
        vars: &BTreeMap::new(),
    };
    assert_eq!((definition.cook)(&context).unwrap()[0].get_gray(8, 8, false), 1.0 / 3.0);
}

#[test]
fn missing_direct_pattern_inputs_use_the_field_default() {
    use materialcraft_core::ValueType;

    let direct = FunctionGraph::input("strength", ValueType::Float);
    let mut derived = direct.clone();
    let read = derived.output.unwrap();
    let offset = derived.add_constant(Value::Float(0.375));
    derived.output = Some(derived.add("add", &[read, offset], None));
    for (expression, binding, expected) in [
        (direct.clone(), None, 1.0),
        (direct.clone(), Some(0.0), 0.0),
        (direct, Some(0.375), 0.375),
        (derived, None, 0.375),
        (FunctionGraph::input("tint", ValueType::Float4), None, 1.0),
    ] {
        let mut graph = PatternGraph::with_root(PatternNodeKind::Quadrant);
        graph.nodes[0].parameters = BTreeMap::from([("patterntype".into(), Value::Int(2)), ("opacity".into(), Value::Function(Box::new(expression)))]);
        let vars = binding.map(|v| ("strength".into(), Value::Float(v))).into_iter().collect();
        let image = cook_with_variables(graph, &vars, 7).unwrap().remove(0);
        assert_eq!(image.get_gray(8, 8, false), expected);
    }

    let mut graph = PatternGraph::with_root(PatternNodeKind::Iterate);
    let square = graph.add(PatternNodeKind::Quadrant).unwrap();
    graph.nodes[0].children[1] = Some(square);
    graph.nodes[0].parameters.insert("numberadded".into(), Value::Function(Box::new(FunctionGraph::input("count", ValueType::Integer))));
    graph.nodes[1].parameters.insert("patterntype".into(), Value::Int(2));
    graph.nodes[1].parameters.insert("patternsize".into(), Value::Function(Box::new(FunctionGraph::input("extent", ValueType::Float2))));
    assert!(cook(graph).unwrap()[0].data().iter().all(|&value| value == 1.0));
}

#[test]
fn a_sibling_can_update_a_binding_declared_by_the_parent() {
    use materialcraft_core::ValueType;

    for (published, initialized, writer_first, expected) in
        [(None, false, true, 1.0), (Some(0.25), false, true, 0.25), (Some(0.25), true, true, 0.75), (None, true, true, 0.75), (Some(0.25), true, false, 0.5625)]
    {
        let mut graph = PatternGraph::with_root(PatternNodeKind::Iterate);
        let writer = graph.add(PatternNodeKind::Quadrant).unwrap();
        let reader = graph.add(PatternNodeKind::Quadrant).unwrap();
        graph.nodes[0].children = if writer_first { vec![Some(writer), Some(reader)] } else { vec![Some(reader), Some(writer)] };
        if initialized {
            graph.nodes[0].parameters.insert("numberadded".into(), set_then_return("tint", Value::Float(0.5625), Value::Int(1)));
        }
        graph.nodes[1].parameters.insert("patterntype".into(), Value::Int(2));
        graph.nodes[1].parameters.insert("opacity".into(), set_then_return("tint", Value::Float(0.75), Value::Float(0.0)));
        graph.nodes[2].parameters.insert("patterntype".into(), Value::Int(2));
        graph.nodes[2].parameters.insert("opacity".into(), Value::Function(Box::new(FunctionGraph::input("tint", ValueType::Float))));
        let variables = published.map(|v| ("tint".into(), Value::Float(v))).into_iter().collect();
        let image = cook_with_variables(graph, &variables, 17).unwrap().remove(0);
        assert_eq!(image.get_gray(8, 8, false), expected, "published {published:?}, initialized {initialized}, writer first {writer_first}");
    }
}

fn set_then_return(name: &str, value: Value, result: Value) -> Value {
    let mut function = FunctionGraph::default();
    let value = function.add_constant(value);
    let set = function.add("set", &[value], Some(Value::Text(name.into())));
    let result = function.add_constant(result);
    function.output = Some(function.add("sequence", &[set, result], None));
    Value::Function(Box::new(function))
}

#[test]
fn quadrant_modes_read_published_inputs_and_shadow_outer_pattern_bindings() {
    use materialcraft_core::ValueType;

    let direct = FunctionGraph::input("form", ValueType::Integer);
    let mut add_two = direct.clone();
    let read = add_two.output.unwrap();
    let two = add_two.add_constant(Value::Int(2));
    add_two.output = Some(add_two.add("add", &[read, two], None));
    for (shadowed, expression, expected_corner) in [(false, direct.clone(), 1.0), (true, direct, 0.0), (true, add_two, 1.0)] {
        let mut graph = PatternGraph::with_root(PatternNodeKind::Iterate);
        let child = graph.add(PatternNodeKind::Quadrant).unwrap();
        graph.nodes[0].children[0] = Some(child);
        if shadowed {
            graph.nodes[0].parameters.insert("numberadded".into(), set_then_return("form", Value::Int(3), Value::Int(0)));
        }
        graph.nodes[1].parameters.insert("patterntype".into(), Value::Function(Box::new(expression)));
        let image = cook_with_variables(graph, &BTreeMap::from([("form".into(), Value::Int(2))]), 17).unwrap().remove(0);
        assert_eq!(image.get_gray(0, 0, false), expected_corner, "shadowed {shadowed}");
        if expected_corner == 0.0 {
            assert_eq!(image.get_gray(8, 8, false), 0.0);
        }
    }
}

#[test]
fn a_quadrant_mode_evaluates_before_it_has_a_traversal_depth() {
    let mut graph = PatternGraph::with_root(PatternNodeKind::Quadrant);
    let child = graph.add(PatternNodeKind::Quadrant).unwrap();
    graph.nodes[0].children[0] = Some(child);
    let mut shape = FunctionGraph::default();
    let extent = shape.add("get_float1", &[], Some(Value::Text("$depthpow2".into())));
    let threshold = shape.add_constant(Value::Float(0.25));
    let test = shape.add("gt", &[extent, threshold], None);
    let disc = shape.add_constant(Value::Int(3));
    let square = shape.add_constant(Value::Int(2));
    shape.output = Some(shape.add("ifelse", &[test, disc, square], None));
    graph.nodes[1].parameters.insert("patterntype".into(), Value::Function(Box::new(shape)));

    let image = cook(graph).unwrap().remove(0);
    assert_eq!(image.get_gray(0, 0, false), 1.0);
}

#[test]
fn a_quadrant_only_publishes_writes_from_active_stamp_properties() {
    for (pattern, property, result, expected) in [
        (0, "opacity", Value::Float(0.0), 0.125),
        (0, "branchoffset", Value::Float2([0.0; 2]), 0.75),
        (2, "opacity", Value::Float(0.0), 0.75),
        (2, "imageindex", Value::Int(0), 0.125),
        (2, "patterntype", Value::Int(2), 0.125),
        (2, "blendingmode", Value::Int(0), 0.125),
        (1, "imagefiltering", Value::Int(1), 0.125),
        (1, "imagepremul", Value::Bool(true), 0.125),
        (2, "randominherited", Value::Bool(true), 0.125),
    ] {
        let mut graph = PatternGraph::with_root(PatternNodeKind::Quadrant);
        let child = graph.add(PatternNodeKind::Quadrant).unwrap();
        graph.nodes[0].children[0] = Some(child);
        let parent = &mut graph.nodes[0].parameters;
        parent.insert("patterntype".into(), Value::Int(pattern));
        parent.insert("opacity".into(), Value::Float(0.0));
        let mut expression = FunctionGraph::default();
        let value = expression.add_constant(Value::Float(0.75));
        let write = expression.add("set", &[value], Some(Value::Text("marker".into())));
        let result = expression.add_constant(result);
        expression.output = Some(expression.add("sequence", &[write, result], None));
        parent.insert(property.into(), Value::Function(Box::new(expression)));
        graph.nodes[1].parameters.insert("patterntype".into(), Value::Int(2));
        graph.nodes[1].parameters.insert("opacity".into(), Value::Function(Box::new(FunctionGraph::input("marker", materialcraft_core::ValueType::Float))));
        let image = cook_with_variables(graph, &BTreeMap::from([("marker".into(), Value::Float(0.125))]), 19).unwrap().remove(0);
        assert_eq!(image.get_gray(3, 3, false), expected, "pattern {pattern}, property {property}");
    }
}

#[test]
fn an_empty_switch_still_runs_its_seed_assignment() {
    let mut graph = PatternGraph::with_root(PatternNodeKind::Quadrant);
    let repeat = graph.add(PatternNodeKind::Iterate).unwrap();
    let empty = graph.add(PatternNodeKind::Switch).unwrap();
    let mark = graph.add(PatternNodeKind::Quadrant).unwrap();
    graph.nodes[0].children = vec![Some(repeat), Some(repeat), Some(mark), None];
    graph.nodes[1].parameters.insert("numberadded".into(), Value::Int(2));
    graph.nodes[1].children[1] = Some(empty);
    let mut advance = FunctionGraph::default();
    let count = advance.add("get_float1", &[], Some(Value::Text("count".into())));
    let quarter = advance.add_constant(Value::Float(0.25));
    let next = advance.add("add", &[count, quarter], None);
    let set = advance.add("set", &[next], Some(Value::Text("count".into())));
    let zero = advance.add_constant(Value::Int(0));
    advance.output = Some(advance.add("sequence", &[set, zero], None));
    graph.nodes[2].parameters.insert("randomseed".into(), Value::Function(Box::new(advance)));
    graph.nodes[3].parameters.insert("patterntype".into(), Value::Int(2));
    graph.nodes[3].parameters.insert("opacity".into(), Value::Function(Box::new(FunctionGraph::input("count", materialcraft_core::ValueType::Float))));
    assert_eq!(cook(graph).unwrap()[0].get_gray(3, 11, false), 1.0);
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
fn deep_quadrants_draw_only_the_leaves_covering_output_samples() {
    let mut graph = PatternGraph::with_root(PatternNodeKind::Quadrant);
    for depth in 0..10 {
        let child = graph.add(PatternNodeKind::Quadrant).unwrap();
        graph.nodes[depth].children.fill(Some(child));
    }
    let increment = 1.0 / 1_048_576.0;
    graph.nodes[10].parameters = BTreeMap::from([("patterntype".into(), Value::Int(2)), ("opacity".into(), Value::Float(increment))]);
    let image = cook_at_size(graph, &BTreeMap::new(), 107185, 4).unwrap().remove(0);
    assert_eq!(image.data(), &[increment; 16]);
}

#[test]
fn image_selectors_use_zero_based_slots_and_override_packed_patterns() {
    use craft_texture::{Image, Kind};

    let gray = |v| Some(Image::filled(4, 4, Kind::Gray, [v; 4]));
    let inputs = [None, gray(0.25), gray(0.75), None, gray(0.5)];
    let definition = find("fx_map").unwrap();
    for (pattern, slot, computed, expected) in [
        (1, None, false, 0.25),
        (1, Some(0), false, 0.25),
        (1, Some(1), false, 0.75),
        (1, Some(2), false, 0.0),
        (256, Some(1), false, 0.75),
        (257, None, false, 0.75),
        (258, Some(1), false, 0.75),
        (257, Some(-1), false, 0.0),
        (1, Some(2), true, 0.5),
        (1, Some(12), true, 0.5),
        (257, Some(-1), true, 0.25),
        (257, Some(0), true, 0.25),
    ] {
        let mut graph = PatternGraph::with_root(PatternNodeKind::Quadrant);
        let node = &mut graph.nodes[0];
        node.parameters.insert("patterntype".into(), Value::Int(pattern));
        let mut vars = BTreeMap::new();
        if let Some(slot) = slot {
            let selector = if computed {
                vars.insert("requested".into(), Value::Int(slot));
                let mut function = FunctionGraph::default();
                function.output = Some(function.add("get_integer1", &[], Some(Value::Text("requested".into()))));
                Value::Function(Box::new(function))
            } else {
                Value::Int(slot)
            };
            node.parameters.insert("imageindex".into(), selector);
        }
        let parameters =
            resolve_params(definition, &BTreeMap::from([("pattern".into(), Value::PatternGraph(Box::new(graph))), ("color_mode".into(), Value::Bool(false))]));
        let context = CookContext {
            input_formats: &[],
            format: materialcraft_core::OutputFormat::F32,
            w: 4,
            h: 4,
            seed: 0,
            tile: false,
            inputs: &inputs,
            params: &parameters,
            resource: &|_| None,
            vars: &vars,
        };
        let image = (definition.cook)(&context).unwrap().remove(0);
        assert_eq!(image.get(2, 2, false)[0], expected, "pattern {pattern}, slot {slot:?}");
    }
}

#[test]
fn image_indices_outside_the_supported_range_return_an_error() {
    let mut graph = PatternGraph::with_root(PatternNodeKind::Quadrant);
    graph.nodes[0].parameters.insert("patterntype".into(), Value::Int(1));
    graph.nodes[0].parameters.insert("imageindex".into(), Value::Int(i32::MAX));
    assert!(cook(graph).unwrap_err().contains("unsupported pattern"));
}

fn render_bitmap(
    input: craft_texture::Image,
    format: materialcraft_core::OutputFormat,
    size: u32,
    stamp: &[(&str, Value)],
    controls: &[(&str, Value)],
) -> craft_texture::Image {
    let mut graph = PatternGraph::with_root(PatternNodeKind::Quadrant);
    graph.nodes[0].parameters.insert("patterntype".into(), Value::Int(1));
    graph.nodes[0].parameters.extend(stamp.iter().map(|(k, v)| ((*k).into(), v.clone())));
    let definition = find("fx_map").unwrap();
    let mut params = BTreeMap::from([("pattern".into(), Value::PatternGraph(Box::new(graph)))]);
    params.extend(controls.iter().map(|(k, v)| ((*k).into(), v.clone())));
    let context = CookContext {
        w: size,
        h: size,
        seed: 0,
        format: materialcraft_core::OutputFormat::F32,
        tile: false,
        input_formats: &[materialcraft_core::OutputFormat::F32, format],
        inputs: &[None, Some(input)],
        params: &resolve_params(definition, &params),
        resource: &|_| None,
        vars: &BTreeMap::new(),
    };
    (definition.cook)(&context).unwrap().remove(0)
}

#[test]
fn an_unbound_image_selector_keeps_the_fixed_slot() {
    use materialcraft_core::{OutputFormat, ValueType};

    let source = craft_texture::Image::gray(4, 4, |_, _| 0.75);
    for (pattern, offset, expected) in [(259, None, 0.0), (259, Some(0), 0.0), (1, Some(0), 0.75), (1, Some(2), 0.0)] {
        let mut selector = FunctionGraph::input("missing", ValueType::Integer);
        if let Some(offset) = offset {
            let index = selector.output.unwrap();
            let step = selector.add_constant(Value::Int(offset));
            selector.output = Some(selector.add("add", &[index, step], None));
        }
        let stamp = [("patterntype", Value::Int(pattern)), ("imageindex", Value::Function(Box::new(selector)))];
        let image = render_bitmap(source.clone(), OutputFormat::F32, 4, &stamp, &[("color_mode", Value::Bool(false))]);
        assert_eq!(image.get_gray(2, 2, false), expected, "pattern {pattern}, offset {offset:?}");
    }
}

#[test]
fn bitmap_stamp_filters_use_the_selected_source_level() {
    use materialcraft_core::OutputFormat;

    let input = craft_texture::Image::gray(64, 64, |x, y| ((47 * x + 31 * y + 7 * x * y + 13) % 256) as f32 / 255.0).with_integer_sampling(true);
    for (mode, expected) in
        [(None, [0x3f4ea0cf, 0x3ecc32cc]), (Some(0), [0x3f4ea0cf, 0x3ecc32cc]), (Some(1), [0x3f5e5ade, 0x3e2cf0ac]), (Some(2), [0x3f62e2e3, 0x3e109090])]
    {
        let mut stamp = vec![("patternsize", Value::Float2([0.375; 2]))];
        if let Some(mode) = mode {
            stamp.push(("imagefiltering", Value::Int(mode)));
        }
        let image = render_bitmap(input.clone(), OutputFormat::U8, 64, &stamp, &[("color_mode", Value::Bool(false))]);
        for (x, bits) in [32, 33].into_iter().zip(expected) {
            assert_eq!(image.get_gray(x, 32, false).to_bits(), bits, "filter {mode:?}, x {x}");
        }
    }
    for (angle, expected) in [(0.0625, 0x3f2f57af), (0.125, 0x3f0b1f8b)] {
        let stamp =
            [("patternsize", Value::Float2([0.17, 1.0])), ("frameoffset", Value::Float2([0.07125, -0.0390625])), ("patternrotation", Value::Float(angle))];
        let image = render_bitmap(input.clone(), OutputFormat::U8, 64, &stamp, &[("color_mode", Value::Bool(false))]);
        assert_eq!(image.get_gray(36, 29, false).to_bits(), expected, "angle {angle}");
    }
}

#[test]
fn bitmap_stamp_requires_an_input_matching_the_output_kind() {
    use craft_texture::{Image, Kind};
    use materialcraft_core::OutputFormat;

    let background = [0.125, 0.5, 0.875, 0.75];
    for (kind, color_output) in [(Kind::Color, false), (Kind::Gray, true)] {
        let controls = [("color_mode", Value::Bool(color_output)), ("background", Value::Color(background))];
        let image = render_bitmap(Image::filled(4, 4, kind, [1.0; 4]), OutputFormat::F32, 4, &[], &controls);
        let expected = if color_output { background } else { [0.5, 0.5, 0.5, 1.0] };
        assert_eq!(image.get(2, 2, false), expected);
    }
}

#[test]
fn connected_pattern_backgrounds_prepare_small_samples_before_filtering() {
    use craft_texture::{Image, Kind};
    use materialcraft_core::OutputFormat;

    let smallest = f32::MIN_POSITIVE;
    let largest_subnormal = f32::from_bits(0x007f_ffff);
    let gray = Image::gray(2, 2, |x, _| if x == 0 { largest_subnormal } else { 2.0 * smallest });
    let underflow = Image::gray(2, 2, |x, _| if x == 0 { smallest } else { 0.0 });
    let mixed = [-0.0_f32, -0.375, 0.5, -0.0];
    let negative = [-0.0_f32, -0.375, -0.5, -0.0];
    for (input, size, source, expected) in [
        (gray, 1, OutputFormat::F32, [smallest.to_bits(); 4]),
        (underflow, 1, OutputFormat::F32, [0; 4]),
        (Image::filled(2, 2, Kind::Gray, [-largest_subnormal; 4]), 2, OutputFormat::F32, [0; 4]),
        (
            Image::filled(2, 2, Kind::Color, [-largest_subnormal, -0.0, largest_subnormal, -smallest]),
            2,
            OutputFormat::F32,
            [0x8000_0000, 0x8000_0000, 0, 0x8080_0000],
        ),
        (Image::filled(2, 2, Kind::Color, mixed), 2, OutputFormat::F32, mixed.map(f32::to_bits)),
        (Image::filled(2, 2, Kind::Color, mixed), 4, OutputFormat::F32, [0, mixed[1].to_bits(), mixed[2].to_bits(), 0]),
        (Image::filled(2, 2, Kind::Color, mixed), 4, OutputFormat::F16, [0, mixed[1].to_bits(), mixed[2].to_bits(), 0]),
        (Image::filled(2, 2, Kind::Color, negative), 4, OutputFormat::F32, negative.map(f32::to_bits)),
    ] {
        let kind = input.kind();
        let graph = PatternGraph::with_root(PatternNodeKind::Quadrant);
        let definition = find("fx_map").unwrap();
        let params = resolve_params(
            definition,
            &BTreeMap::from([("pattern".into(), Value::PatternGraph(Box::new(graph))), ("color_mode".into(), Value::Bool(kind == Kind::Color))]),
        );
        let context = CookContext {
            w: size,
            h: size,
            seed: 100237,
            format: OutputFormat::F32,
            tile: true,
            input_formats: &[source],
            inputs: &[Some(input)],
            params: &params,
            resource: &|_| None,
            vars: &BTreeMap::new(),
        };
        let image = (definition.cook)(&context).unwrap().remove(0);
        let pixel = image.get(0, 0, false).map(f32::to_bits);
        assert_eq!(&pixel[..image.channels()], &expected[..image.channels()]);
    }
}

#[test]
fn bitmap_stamp_tint_and_coverage_compose_straight_and_premultiplied_colors() {
    use craft_texture::{Image, Kind};
    use materialcraft_core::OutputFormat;

    let source = [0x3f12d2d3, 0xbe67e7e8, 0x3ef57575, 0x3f97b7b8].map(f32::from_bits);
    let controls = [("color_mode", Value::Bool(true)), ("background", Value::Color([0.25, 0.375, 0.625, 0.8125])), ("opacity", Value::Float(0.375))];
    for (blend, premultiplied, expected) in [
        (0, false, [0x3ebd2e9e, 0x3eb3eba3, 0x3f42183a, 0x3f5d55a6]),
        (1, false, [0x3e999f8f, 0x3e7d2a19, 0x3f15a567, 0x3f5d55a6]),
        (2, false, [0x3e999f8f, 0x3e7d2a19, 0x3f15a567, 0x3f5d55a6]),
        (0, true, [0x3eb39e1e, 0x3eb5cf0f, 0x3f3cc3c4, 0x3f5d55a6]),
    ] {
        let stamp = [
            ("opacity", Value::Float4([0.75, 0.375, 1.0, 0.625])),
            ("blendingmode", Value::Int(blend)),
            ("imagepremul", Value::Bool(premultiplied)),
            ("imagefiltering", Value::Int(2)),
        ];
        let image = render_bitmap(Image::filled(1, 1, Kind::Color, source), OutputFormat::F32, 1, &stamp, &controls);
        assert_eq!(image.get(0, 0, false).map(f32::to_bits), expected, "blend {blend}, premultiplied {premultiplied}");
    }
}

#[test]
fn bitmap_stamp_blend_preserves_guard_bits_during_cancellation() {
    use craft_texture::{Image, Kind};
    use materialcraft_core::OutputFormat;

    let source = [0xbaf97631, 0xc061d2bc, 0xbc4a2f47, 0xbe8c58b1].map(f32::from_bits);
    let background = [0x3ac3d70f, 0x4031485b, 0x3c1eb9a8, 0x3e5c5bb0].map(f32::from_bits);
    let controls = [("color_mode", Value::Bool(true)), ("background", Value::Color(background))];
    for (mode, expected) in [
        (0, [0xb9d67c88, 0xbf422984, 0xbb2dd67c, 0x388c6000]),
        (1, [0x34f98400, 0x3a61d400, 0x364a3400, 0x388c5c00]),
        (2, [0x34f98400, 0x3a61d400, 0x364a3400, 0x388c5c00]),
    ] {
        let stamp = [("imagefiltering", Value::Int(2)), ("imagepremul", Value::Bool(true)), ("blendingmode", Value::Int(mode))];
        let image = render_bitmap(Image::filled(1, 1, Kind::Color, source), OutputFormat::F32, 1, &stamp, &controls);
        assert_eq!(image.get(0, 0, false).map(f32::to_bits), expected, "blend {mode}");
    }
    let controls = [("color_mode", Value::Bool(true)), ("background", Value::Color([-0.0; 4]))];
    for mode in 0..=2 {
        let stamp = [("imagefiltering", Value::Int(2)), ("blendingmode", Value::Int(mode))];
        let image = render_bitmap(Image::filled(1, 1, Kind::Color, [0.0; 4]), OutputFormat::F32, 1, &stamp, &controls);
        assert_eq!(image.get(0, 0, false).map(f32::to_bits), [(-0.0_f32).to_bits(); 4]);
    }
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
    let tint = if color { [0.8, 0.4, 0.2, 1.0] } else { [0.8, 0.8, 0.8, 1.0] };
    graph.nodes[leaf].parameters.insert("opacity".into(), Value::Float4(tint));
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
fn single_square_preserves_the_stored_opacity_at_each_depth() {
    let definition = find("fx_map").unwrap();
    let gain = f32::from_bits(0x4020_0150);
    for (last, draw, roughness, opacity, blend, bits) in [
        (5, 5, 0.95, gain, 1, 0x3eba_f23c),
        (1, 0, 0.99, 1.0, 1, 0x3f00_a4b1),
        (2, 2, 0.99, 1.0, 1, 0x3ea8_f445),
        (2, 2, 1.5, 0.6137, 1, 0x3e94_d6a2),
        (2, 1, 1.0, gain, 1, 0x3f55_5716),
        (5, 5, 4_194_304.0, 1.0, 1, 0),
        (0, 0, 0.0, -1.0, 0, 0xbf80_0000),
        (0, 0, 0.95, 327.125, 0, 0x43a3_9000),
    ] {
        let mut graph = PatternGraph::with_root(PatternNodeKind::Quadrant);
        for depth in 0..last {
            let child = graph.add(PatternNodeKind::Quadrant).unwrap();
            graph.nodes[depth].children[0] = Some(child);
        }
        graph.nodes[draw].parameters = BTreeMap::from([
            ("patterntype".into(), Value::Int(2)),
            ("patternsize".into(), Value::Float2([(1_u32 << draw) as f32; 2])),
            ("blendingmode".into(), Value::Int(blend)),
        ]);
        let params = resolve_params(
            definition,
            &BTreeMap::from([
                ("pattern".into(), Value::PatternGraph(Box::new(graph))),
                ("color_mode".into(), Value::Bool(false)),
                ("roughness".into(), Value::Float(roughness)),
                ("opacity".into(), Value::Float(opacity)),
            ]),
        );
        let vars = BTreeMap::new();
        let context = CookContext {
            input_formats: &[],
            format: materialcraft_core::OutputFormat::F32,
            w: 16,
            h: 16,
            seed: 7,
            tile: false,
            inputs: &[],
            params: &params,
            resource: &|_| None,
            vars: &vars,
        };
        let image = (definition.cook)(&context).unwrap().remove(0);
        assert_eq!(image.get_gray(0, 0, false).to_bits(), bits, "drawing depth {draw} of {last}, roughness {roughness}, opacity {opacity}");
    }
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
                    let expected = match (color, blend) {
                        (false, 1 | 2) => value.max(0.1),
                        (true, 1 | 2) => value + 0.1 * (1.0 - 0.5 * weight),
                        (true, 0) if channel == 3 => value + 0.1 * (1.0 - value),
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

    for published in [0.0, 0.125] {
        let image = cook_with_variables(graph.clone(), &BTreeMap::from([("total".into(), Value::Float(published))]), 7).unwrap().remove(0);
        // The pattern declares a local accumulator. Its three visits add 0.25, 0.5 and 0.75.
        assert_eq!(image.mean()[0], 1.5, "published {published}");
    }
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
fn reversing_a_render_interval_selects_the_full_extent_of_that_axis() {
    for (bounds, first_center) in [
        ([0.8125, 0.1875, 0.25, 0.75], [0.0078125, 0.25390625]),
        ([0.1875, 0.8125, 0.75, 0.25], [0.19238281, 0.0078125]),
        ([0.8125, 0.1875, 0.75, 0.25], [0.0078125; 2]),
    ] {
        let region = super::region::Region::new(bounds).unwrap();
        assert_eq!(region.point([0, 0], [64, 64]), first_center);
    }
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
