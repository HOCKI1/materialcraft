use materialcraft_core::{PatternGraph, PatternNodeKind, Value};
use materialcraft_nodes::{CookContext, find, resolve_params};
use std::collections::BTreeMap;

fn render(region: [f32; 4], tiling: [f32; 4], tile: bool) -> materialcraft_nodes::CookResult {
    render_stamp(region, tiling, if tile { 3 } else { 0 }, [0.0; 2], [0.5; 2])
}

fn render_stamp(region: [f32; 4], tiling: [f32; 4], axes: i32, offset: [f32; 2], size: [f32; 2]) -> materialcraft_nodes::CookResult {
    let mut graph = PatternGraph::with_root(PatternNodeKind::Quadrant);
    graph.nodes[0].parameters.insert("patterntype".into(), Value::Int(2));
    graph.nodes[0].parameters.insert("patternsize".into(), Value::Float2(size));
    graph.nodes[0].parameters.insert("frameoffset".into(), Value::Float2(offset));
    let definition = find("fx_map").unwrap();
    let params = resolve_params(
        definition,
        &BTreeMap::from([
            ("pattern".into(), Value::PatternGraph(Box::new(graph))),
            ("render_region".into(), Value::Float4(region)),
            ("tiling_region".into(), Value::Float4(tiling)),
        ]),
    );
    (definition.cook)(&CookContext {
        input_formats: &[],
        format: materialcraft_core::OutputFormat::F32,
        w: 64,
        h: 64,
        seed: 0,
        tile: axes == 3,
        inputs: &[],
        params: &params,
        resource: &|_| None,
        vars: &BTreeMap::from([("$tiling".into(), Value::Int(axes))]),
    })
}

#[test]
fn render_region_samples_a_cropped_area() {
    let identity = [0.0, 1.0, 0.0, 1.0];
    let full = render(identity, identity, false).unwrap().remove(0);
    let crop = render([0.25, 0.75, 0.25, 0.75], identity, false).unwrap().remove(0);

    assert_eq!(full.mean()[0], 0.25);
    assert!(crop.data().iter().all(|value| *value == 1.0));
}

#[test]
fn expanding_the_render_window_preserves_the_tiling_region() {
    let region = [0.0, 2.0, 0.0, 1.0];
    let repeated = render(region, [0.0, 1.0, 0.0, 1.0], true).unwrap().remove(0);
    let wider = render(region, region, true).unwrap().remove(0);
    let untiled = render(region, [0.0, 1.0, 0.0, 1.0], false).unwrap().remove(0);

    assert_eq!(repeated.get(48, 32, false)[0], 0.0);
    assert_eq!(wider.get(48, 32, false)[0], 0.0);
    assert_eq!(wider.data(), untiled.data());
    assert_eq!(repeated.mean()[0], 0.125);
    assert_eq!(wider.mean()[0], 0.125);
}

#[test]
fn a_stamp_repeats_on_each_enabled_axis_inside_the_tiling_region() {
    let unit = [0.0, 1.0, 0.0, 1.0];
    for (region, tiling, counts) in [(unit, unit, [27, 324, 72, 864]), ([-0.25, 1.25, 0.0, 1.0], [0.125, 0.875, 0.25, 0.75], [117, 216, 312, 576])] {
        for (mask, count) in counts.into_iter().enumerate() {
            let image = render_stamp(region, tiling, mask as i32, [-0.734375, -0.546875], [0.5625, 0.375]).unwrap().remove(0);
            assert_eq!(image.mean()[0] * 4096.0, count as f32, "tiling mask {mask}");
        }
    }
}

#[test]
fn a_stamp_that_only_touches_the_tiling_interval_contributes_no_copy() {
    let render = [0.0, 1.0, 0.0, 1.0];
    let tile = [0.25, 0.75, 0.125, 0.875];
    for (offset, excluded, included) in [([-0.375, 0.375], [8, 56], [40, 56]), ([0.375, -0.375], [56, 8], [24, 8])] {
        let image = render_stamp(render, tile, 1, offset, [0.25, 0.5]).unwrap().remove(0);
        assert_eq!(image.get(excluded[0], excluded[1], false)[0], 0.0);
        assert_eq!(image.get(included[0], included[1], false)[0], 1.0);
    }
}

#[test]
fn degenerate_regions_and_excessive_repetition_return_errors() {
    let identity = [0.0, 1.0, 0.0, 1.0];

    assert!(render([0.0; 4], identity, true).is_err());
    assert!(render(identity, [0.0; 4], true).is_err());
    assert!(render(identity, [0.0, 1e-9, 0.0, 1e-9], true).is_err());
}
