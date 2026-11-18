use std::collections::BTreeMap;

use craft_texture::{Image, Kind};
use materialcraft_core::Value;
use materialcraft_nodes::{CookContext, find, resolve_params};

fn cook(mask: Image, source: Option<Image>, tile: bool, combine: bool, distance: f32) -> Image {
    let def = find("distance").unwrap();
    let params = resolve_params(def, &BTreeMap::from([("distance".into(), Value::Float(distance)), ("combine".into(), Value::Bool(combine))]));
    let (w, h) = (mask.width(), mask.height());
    let inputs = [Some(mask), source];
    let vars = BTreeMap::new();
    let resource = |_: &str| None;
    let ctx = CookContext {
        input_formats: &[],
        format: materialcraft_core::OutputFormat::F32,
        w,
        h,
        seed: 0,
        tile,
        inputs: &inputs,
        params: &params,
        resource: &resource,
        vars: &vars,
    };
    (def.cook)(&ctx).unwrap().remove(0)
}

#[test]
fn tiled_distance_spreads_source_across_stationary_gradient() {
    let mask = Image::gray(32, 32, |x, y| if x == 0 && y == 0 { 1.0 } else { 0.0 });
    let output = cook(mask.clone(), Some(mask), true, false, 256.0);

    // A zero slope at the only mask pixel must not stop the walk on background
    assert_eq!(output.get_gray(16, 16, false), 1.0);
    assert!(output.data().iter().all(|&v| v == 1.0));
}

#[test]
fn distance_spreads_source_beyond_the_old_walk_limit() {
    let mask = Image::gray(1024, 1, |x, _| if x == 0 { 1.0 } else { 0.0 });
    let output = cook(mask.clone(), Some(mask), false, false, 256.0);
    assert_eq!(output.get_gray(1023, 0, false), 1.0);
}

#[test]
fn distance_preserves_source_alpha_range_and_combine() {
    for tile in [false, true] {
        let mask = Image::gray(9, 3, |x, y| if x == 0 && y == 1 { 1.0 } else { 0.0 });
        let source = Image::generate(9, 3, Kind::Color, |x, y| if x == 0 && y == 1 { [0.8, 0.4, 0.2, 0.6] } else { [0.1, 0.1, 0.1, 1.0] });
        for combine in [false, true] {
            let output = cook(mask.clone(), Some(source.clone()), tile, combine, 512.0 / 9.0); // two pixels
            assert_eq!(output.get(0, 1, false), [0.8, 0.4, 0.2, 0.6]);
            let expected = if combine { [0.4, 0.2, 0.1, 0.6] } else { [0.8, 0.4, 0.2, 0.6] };
            assert_eq!(output.get(1, 1, false), expected);
            assert_eq!(output.get(4, 1, false), [0.0; 4]);
            assert_eq!(output.get(8, 1, false), if tile { expected } else { [0.0; 4] });
        }
    }
}

#[test]
fn distance_handles_empty_full_masks_and_no_source() {
    for tile in [false, true] {
        let empty = Image::filled(7, 5, Kind::Gray, [0.0; 4]);
        let full = Image::filled(7, 5, Kind::Gray, [1.0; 4]);
        let source = Image::generate(7, 5, Kind::Color, |x, y| [x as f32 / 7.0, y as f32 / 5.0, 0.4, 0.7]);

        assert!(cook(empty.clone(), Some(source.clone()), tile, false, 256.0).data().iter().all(|&v| v == 0.0));
        assert_eq!(cook(full, Some(source), tile, true, 256.0).get(3, 2, false), [3.0 / 7.0, 2.0 / 5.0, 0.4, 0.7]);
        assert!(cook(empty, None, tile, true, 256.0).data().iter().all(|&v| v == 0.0));

        let mask = Image::gray(32, 32, |x, y| if (x, y) == (16, 16) { 0.5 } else { 0.499 });
        let ramp = cook(mask, None, tile, false, 16.0);
        assert_eq!(ramp.get_gray(16, 16, false), 1.0);
        assert_eq!(ramp.get_gray(17, 16, false).to_bits(), 0x3eff_f080);
        assert_eq!(ramp.get_gray(19, 16, false), 0.0);
    }
}

#[test]
fn mask_distance_uses_stored_site_centers_and_the_shorter_dimension() {
    let mask = Image::gray(128, 64, |x, y| if (x, y) == (64, 32) { 1.0 } else { 0.0 });
    let ramp = cook(mask, None, true, true, 256.0 * std::f32::consts::SQRT_2);
    for (x, y, bits) in [(65, 32, 0x3f7d_2b39), (64, 33, 0x3f7d_2b93), (65, 33, 0x3f7b_ff42), (63, 32, 0x3f7d_2ca0)] {
        assert_eq!(ramp.get_gray(x, y, false).to_bits(), bits);
    }
}

#[test]
fn sparse_mask_distance_preserves_the_sites_selected_during_propagation() {
    let mask = Image::gray(128, 128, |x, y| if (13 * x + 23 * y + 11 * x * y) % 109 == 0 { 1.0 } else { 0.0 });
    let ramp = cook(mask, None, true, true, 17.375);
    for (x, y, bits) in [(99, 0, 0x3e86_a510), (64, 30, 0x3e26_0066), (39, 55, 0x3e25_ccdc)] {
        assert_eq!(ramp.get_gray(x, y, false).to_bits(), bits);
    }
}
