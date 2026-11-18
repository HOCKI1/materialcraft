use craft_texture::{Image, Kind};
use materialcraft_core::{Gradient, GradientKey, OutputFormat, Value};
use materialcraft_nodes::{CookContext, find, resolve_params};
use std::collections::BTreeMap;

fn cook(id: &str, inputs: &[Option<Image>], values: &[(&str, Value)], size: [u32; 2], tile: bool) -> Image {
    cook_format(id, inputs, values, size, tile, OutputFormat::F32)
}

fn cook_format(id: &str, inputs: &[Option<Image>], values: &[(&str, Value)], size: [u32; 2], tile: bool, format: OutputFormat) -> Image {
    let definition = find(id).unwrap();
    let values = values.iter().map(|(key, value)| ((*key).into(), value.clone())).collect();
    let params = resolve_params(definition, &values);
    let resource = |_: &str| None;
    let vars = BTreeMap::new();
    let context = CookContext { input_formats: &[], w: size[0], h: size[1], seed: 0, format, tile, inputs, params: &params, resource: &resource, vars: &vars };
    (definition.cook)(&context).unwrap().remove(0)
}

#[test]
fn grayscale_uses_all_driven_weights_and_flattens_signed_alpha() {
    let px = [0xbfd95555, 0xbf9aaaab, 0xbf380000, 0xbe6aaaab].map(f32::from_bits);
    let inp = Image::generate(2, 1, Kind::Color, |x, _| if x == 0 { [-0.0; 4] } else { px });
    let weights = ("weights", Value::Float4([0.125, 0.25, 0.5, 0.125]));
    let gray = cook("grayscale_conversion", &[Some(inp.clone())], std::slice::from_ref(&weights), [2, 1], false);
    assert_eq!(gray.get_gray(0, 0, false).to_bits(), 0);
    let params = [weights, ("flatten_alpha", Value::Bool(true)), ("matte", Value::Float(0.3125))];
    let flattened = cook("grayscale_conversion", &[Some(inp)], &params, [2, 1], false);
    assert_eq!(flattened.get_gray(1, 0, false).to_bits(), 0x3f174555);
}

#[test]
fn levels_applies_signed_gamma_before_mapping_the_output_range() {
    let input = Image::generate(2, 1, Kind::Color, |x, _| if x == 0 { [-0.5; 4] } else { [3.0; 4] });
    let params = [("clamp_intermediate", Value::Bool(false)), ("in_mid", Value::Float4([0.25, 0.375, 0.625, 0.75]))];
    let output = cook("levels", &[Some(input)], &params, [2, 1], false);
    assert_eq!(output.get(0, 0, false).map(f32::to_bits), [0xbf5edc67, 0xbf4b2ff5, 0xbe000003, 0xbd000003]);
    assert_eq!(output.get(1, 0, false).map(f32::to_bits), [0x3f9f741c, 0x3fb89ba3, 0x41d80001, 0x4372fffe]);
}

#[test]
fn integer_levels_bounds_the_four_endpoints_before_applying_gamma() {
    let params = [
        ("in_low", Value::Float4([-1.75, -0.625, -0.125, -0.5])),
        ("in_mid", Value::Float4([0.469, 0.44, 0.51, 0.25])),
        ("in_high", Value::Float4([2.375, 2.75, 1.5, 2.875])),
        ("out_low", Value::Float4([-0.5, -1.0, 0.25, 0.125])),
        ("out_high", Value::Float4([1.875, 2.25, 1.375, 0.875])),
        ("clamp_intermediate", Value::Bool(false)),
    ];
    let input = [Some(Image::filled(1, 1, Kind::Color, [0.125; 4]))];
    for (format, max, expected) in [(OutputFormat::U8, 255.0, [64, 88, 81, 158]), (OutputFormat::U16, 65535.0, [16323, 22684, 20789, 40620])] {
        let image = cook_format("levels", &input, &params, [1, 1], false, format);
        let stored = materialcraft_nodes::output_precision::apply(image, format);
        assert_eq!(stored.get(0, 0, false).map(|v| (v * max).round() as u16), expected);
    }
}

#[test]
fn additive_blend_bounds_global_opacity_and_fuses_the_background_weight() {
    let inputs = [Some(Image::gray(1, 1, |_, _| 9830.0 / 65535.0)), Some(Image::gray(1, 1, |_, _| 2753.0 / 65535.0))];
    for (opacity, bits) in [(0.55, 0x3dfefcff), (1.3, 0x3e449cc5)] {
        let params = [("mode", Value::Enum(1)), ("opacity", Value::Float(opacity))];
        let image = cook("blend", &inputs, &params, [1, 1], false);
        assert_eq!(image.get_gray(0, 0, false).to_bits(), bits);
    }
}

#[test]
fn levels_scales_narrow_and_coincident_ranges_independently() {
    let pixel = [0xbfd95555, 0xbf9aaaab, 0xbf380000, 0xbe6aaaab].map(f32::from_bits);
    let input = Image::filled(1, 1, Kind::Color, pixel);
    let params = [
        ("in_low", Value::Float4([0.5; 4])),
        ("in_high", Value::Float4([0.5, 0.50000036, 0.49999964, 0.50000477])),
        ("out_low", Value::Float4([0.0, 0.125, 0.875, 0.25])),
        ("out_high", Value::Float4([1.0, 0.875, 0.125, 0.75])),
        ("clamp_intermediate", Value::Bool(false)),
    ];
    let output = cook("levels", &[Some(input)], &params, [1, 1], false);
    assert_eq!(output.get(0, 0, false).map(f32::to_bits), [0xc6abb656, 0xca5aaaa0, 0xca1bfffb, 0xc795552d]);
}

#[test]
fn gradient_keeps_the_first_stop_at_a_shared_position() {
    let input = Image::gray(1, 1, |_, _| 640.0 / 1023.0);
    let keys = vec![
        GradientKey::new(0.125, [0.375, 0.75, 0.875, 0.6875]),
        GradientKey::new(0.5, [0.875, 0.0625, 0.3125, 0.25]),
        GradientKey::new(0.5, [0.1875, 0.75, 0.25, 0.8125]),
        GradientKey::new(0.75, [0.25, 0.5625, 0.75, 0.375]),
    ];
    let output = cook("gradient_map", &[Some(input)], &[("gradient", Value::Gradient(Gradient { keys }))], [1, 1], false);
    assert_eq!(output.get(0, 0, false).map(f32::to_bits), [0x3f0f9be7, 0x3ea0a028, 0x3f084612, 0x3ea0280a]);
}

#[test]
fn grayscale_gradient_averages_stops_before_interpolating() {
    let input = Image::gray(1, 1, |_, _| 640.0 / 1023.0);
    let keys = vec![
        GradientKey::new(0.0625, [0.1875, 0.25, 0.4375, 0.8125]),
        GradientKey::new(0.34375, [0.59375, 0.9375, 0.3125, 0.5625]),
        GradientKey::new(0.890625, [0.8125, 0.125, 0.75, 0.375]),
    ];
    for (mode, bits) in [(0, 0x3f167617), (1, 0x3f165bd5)] {
        let params = [("gradient", Value::Gradient(Gradient { keys: keys.clone() })), ("interpolation", Value::Enum(mode)), ("color_mode", Value::Bool(false))];
        let output = cook("gradient_map", &[Some(input.clone())], &params, [1, 1], false);
        assert_eq!(output.kind(), Kind::Gray);
        assert_eq!(output.get_gray(0, 0, false).to_bits(), bits);
    }
}

#[test]
fn gradient_filter_preserves_linear_and_biased_smooth_segments() {
    let input = Image::gray(1, 1, |_, _| f32::from_bits(0x3e414c15));
    let keys = vec![
        GradientKey::new(0.0625, [0.1875, 0.25, 0.4375, 0.8125]),
        GradientKey::new(0.34375, [0.59375, 0.9375, 0.3125, 0.5625]),
        GradientKey::new(0.890625, [0.8125, 0.125, 0.75, 0.375]),
    ];
    let params = [("gradient", Value::Gradient(Gradient { keys }))];
    let output = cook("gradient_map", &[Some(input)], &params, [1, 1], false);
    assert_eq!(output.get(0, 0, false).map(f32::to_bits), [0x3ebd61ca, 0x3f0f03fb, 0x3ec34460, 0x3f334460]);

    let mut gradient = Gradient::default();
    gradient.keys[0].midpoint = 0.625;
    let step = Image::gray(1, 1, |_, _| 64.0 / 255.0);
    let params = [("gradient", Value::Gradient(gradient)), ("interpolation", Value::Enum(1))];
    let output = cook("gradient_map", &[Some(step)], &params, [1, 1], false);
    assert_eq!(output.get(0, 0, false).map(f32::to_bits), [0x3ddce217, 0x3ddce217, 0x3ddce217, 0x3f800000]);
}

#[test]
fn shuffle_reads_grayscale_from_every_selected_channel() {
    let inputs = [Some(Image::filled(3, 5, Kind::Gray, [0.25; 4])), Some(Image::filled(3, 5, Kind::Gray, [0.75; 4]))];
    for channel in 0..8 {
        let params = ["red", "green", "blue", "alpha"].map(|key| (key, Value::Enum(channel)));

        let output = cook("channels_shuffle", &inputs, &params, [3, 5], true);

        assert_eq!(output.get(1, 2, false), [if channel < 4 { 0.25 } else { 0.75 }; 4], "channel {channel}");
    }
}

#[test]
fn shuffle_wraps_selectors_over_the_eight_input_channels() {
    let inputs = [Some(Image::filled(3, 5, Kind::Color, [0.125, 0.25, 0.375, 0.5])), Some(Image::filled(3, 5, Kind::Color, [0.625, 0.75, 0.875, 1.0]))];
    for channel in [0, 3, 4, 7, 8, 9, 10, 16, 255] {
        let params = ["red", "green", "blue", "alpha"].map(|key| (key, Value::Enum(channel)));

        let output = cook("channels_shuffle", &inputs, &params, [3, 5], true);

        assert_eq!(output.get(1, 2, false), [(channel % 8 + 1) as f32 / 8.0; 4], "channel {channel}");
    }
    assert_eq!(find("channels_shuffle").unwrap().param("red").unwrap().options.len(), 8);
}

#[test]
fn shuffle_resolves_out_of_range_selectors_after_input_kind() {
    for kinds in [[Kind::Gray, Kind::Gray], [Kind::Color, Kind::Gray], [Kind::Gray, Kind::Color]] {
        let inputs = [Some(Image::filled(3, 5, kinds[0], [0.125, 0.25, 0.375, 0.5])), Some(Image::filled(3, 5, kinds[1], [0.625, 0.75, 0.875, 1.0]))];
        let params = [("red", Value::Enum(8)), ("green", Value::Enum(9)), ("blue", Value::Enum(11)), ("alpha", Value::Enum(u32::MAX - 1))];

        let output = cook("channels_shuffle", &inputs, &params, [3, 5], true);

        let expected = if kinds[1] == Kind::Gray { [0.625; 4] } else { [0.125, 0.125, 1.0, 0.875] };

        assert_eq!(output.get(1, 2, false), expected, "kinds {kinds:?}");
    }
}

#[test]
fn sharpen_preserves_float_order_on_ramps() {
    let input = Image::gray(16, 16, |x, _| x as f32 / 15.0);

    let output = cook("sharpen", &[Some(input)], &[("intensity", Value::Float(1.0))], [16, 16], true);

    // Float32 ramps distinguish the Laplacian's arithmetic order
    for (x, bits) in [(1, 0x3d88_8889), (3, 0x3e4c_cccb), (7, 0x3eee_eeed), (13, 0x3f5d_ddde)] {
        assert_eq!(output.get_gray(x, 7, false).to_bits(), bits, "column {x}");
    }
}

#[test]
fn sharpen_preserves_finite_hdr_constant() {
    let constant = 2.0e38;
    let input = Image::filled(16, 16, Kind::Gray, [constant; 4]);

    let output = cook("sharpen", &[Some(input)], &[("intensity", Value::Float(1.0))], [16, 16], true);

    assert!(output.data().iter().all(|value| value.to_bits() == constant.to_bits()));
}

#[test]
fn blur_uses_fractional_box_coverage() {
    let input = Image::gray(16, 16, |x, y| f32::from(x == 8 && y == 8));

    let output = cook("blur", &[Some(input)], &[("intensity", Value::Float(10.0))], [16, 16], true);

    // A 0.625-pixel radius covers the center pixel and one eighth of each neighbor
    for (x, y, expected) in [(8, 8, 0.64), (7, 8, 0.08), (8, 7, 0.08), (7, 7, 0.01), (6, 8, 0.0)] {
        assert!((output.get_gray(x, y, false) - expected).abs() < 1e-7, "at {x},{y}");
    }
}

#[test]
fn three_texel_blurs_keep_tap_order_and_intermediate_storage() {
    let field = Image::gray(64, 64, |x, y| {
        let level = (97 * x + 43 * y + 7 * x * y) % 1024;
        (3.0 * (f64::from(level) / 1023.0) - 1.25) as f32
    });
    let input = [Some(field)];
    let controls = [("intensity", Value::Float(5.0))];
    for (format, center, off_center) in [
        (OutputFormat::U8, 0x3e48c8c9, 0x3ee4e4e5),
        (OutputFormat::U16, 0x3e4a78ca, 0x3ee558e5),
        (OutputFormat::F16, 0x3e15c000, 0x3ee54000),
        (OutputFormat::F32, 0x3e15e059, 0x3ee55957),
    ] {
        let image = cook_format("blur", &input, &controls, [64; 2], true, format);
        let stored = materialcraft_nodes::output_precision::apply(image, format);
        assert_eq!(stored.get_gray(32, 32, true).to_bits(), center);
        assert_eq!(stored.get_gray(11, 13, true).to_bits(), off_center);
    }
    let directional = cook("directional_blur", &input, &controls, [64; 2], true);
    assert_eq!(directional.get_gray(32, 32, true).to_bits(), 0xbdfdbf73);
}

#[test]
fn five_texel_blurs_filter_the_outer_pairs_before_accumulating() {
    let field = Image::generate(128, 128, Kind::Color, |x, y| {
        let value = f64::from((97 * x + 43 * y + 7 * x * y) % 1024) / 1023.0;
        [f32::from(x == 64 && y == 64), f32::from(x == 64), f32::from(y == 64), (3.0 * value - 1.25) as f32]
    });
    let input = [Some(field)];
    let params = [("intensity", Value::Float(4.0))];
    let directional = cook("directional_blur", &input, &params, [128; 2], true);
    assert_eq!(directional.get(62, 64, false).map(f32::to_bits), [0x3dff0000, 0x3dff0000, 0x3f800000, 0x3d6739cc]);
    let separable = cook("blur", &input, &params, [128; 2], true);
    assert_eq!(separable.get(63, 64, false).map(f32::to_bits), [0x3d804000, 0x3e804000, 0x3e800000, 0x3e1e66da]);
}

#[test]
fn wide_blur_reduces_aligned_runs_at_the_destination_precision() {
    let field = Image::generate(64, 128, Kind::Color, |x, y| {
        std::array::from_fn(|c| {
            let c = c as u32;
            let level = (37 * x + 71 * y + 11 * x * y + 47 * c + 3 * x * c) % 1021;
            (3.0 * (f64::from(level) / 1020.0) - 1.125) as f32
        })
    });
    let input = [Some(field)];
    let params = [("intensity", Value::Float(17.5))];
    for (format, expected) in [
        (OutputFormat::U8, [0x3ec0c0c1, 0x3edadadb, 0x3ec8c8c9, 0x3ec2c2c3]),
        (OutputFormat::U16, [0x3ec2e0c3, 0x3edb88dc, 0x3ec9f4ca, 0x3ec3f0c4]),
        (OutputFormat::F16, [0x3ebbc000, 0x3ed24000, 0x3ec30000, 0x3eb66000]),
        (OutputFormat::F32, [0x3ebbdad5, 0x3ed25b4e, 0x3ec322fc, 0x3eb6834f]),
    ] {
        let output = cook_format("blur", &input, &params, [64, 128], true, format);
        assert_eq!(materialcraft_nodes::output_precision::apply(output, format).get(11, 37, false).map(f32::to_bits), expected);
    }
    let clamped = cook("blur", &input, &params, [64, 128], false);
    assert_eq!(clamped.get(0, 1, false).map(f32::to_bits), [0xbf1b2df1, 0xbeeaad75, 0xbe9eff07, 0xbe26a135]);
}

#[test]
fn blur_preserves_finite_hdr_constant() {
    let input = Image::filled(16, 16, Kind::Gray, [1.0e38; 4]);

    let output = cook("blur", &[Some(input)], &[("intensity", Value::Float(64.0))], [16, 16], true);

    assert!(output.data().iter().all(|value| value.is_finite() && (*value - 1.0e38).abs() < 1.0e32));
}

#[test]
fn directional_blur_uses_full_radius_and_fractional_endpoints() {
    let input = Image::gray(256, 256, |x, y| f32::from(x == 128 && y == 128));

    let output = cook("directional_blur", &[Some(input)], &[("intensity", Value::Float(4.0))], [256, 256], true);

    for (offset, expected) in (-4..=4).zip([0.0625, 513.0 / 4096.0, 0.125, 0.125, 0.125, 0.125, 0.125, 0.125, 255.0 / 4096.0]) {
        assert_eq!(output.get_gray(128 + offset, 128, false), expected, "offset {offset}");
    }
    assert_eq!(output.get_gray(128, 129, false), 0.0);
}

#[test]
fn sharpen_has_center_gain_and_clamps_negative_lobes() {
    let input = Image::gray(16, 16, |x, y| f32::from(x == 8 && y == 8));

    let output = cook("sharpen", &[Some(input)], &[("intensity", Value::Float(1.0))], [16, 16], true);

    assert_eq!(output.get_gray(8, 8, false), 5.0);
    assert!(output.data().iter().all(|value| *value >= 0.0));
}

#[test]
fn empty_raster_preserves_transparent_rgb_and_rejects_mismatched_background_kind() {
    for id in ["text", "svg"] {
        let pixel = [0.25, 0.5, 0.75, 0.0];
        let color = Some(Image::filled(16, 16, Kind::Color, pixel));
        let gray = Some(Image::filled(16, 16, Kind::Gray, [0.25; 4]));

        let output = cook(id, std::slice::from_ref(&color), &[("color_mode", Value::Bool(true))], [16, 16], true);

        assert_eq!(output.get(0, 0, false), pixel, "{id} transparent color");

        let output = cook(id, &[color], &[("color_mode", Value::Bool(false))], [16, 16], true);

        assert_eq!(output.get_gray(0, 0, false), 0.0, "{id} color into gray");

        let output = cook(id, &[gray], &[("color_mode", Value::Bool(true))], [16, 16], true);

        assert_eq!(output.get(0, 0, false), [0.0; 4], "{id} gray into color");
    }
}

#[test]
fn rectangular_filters_scale_from_the_shorter_side() {
    for size in [[32, 16], [16, 32]] {
        let input = Image::gray(size[0], size[1], |x, y| f32::from(x == size[0] / 2 && y == size[1] / 2));
        for (operation, expected) in [("blur", 0.64), ("directional_blur", 0.8)] {
            let output = cook(operation, &[Some(input.clone())], &[("intensity", Value::Float(10.0))], size, true);

            assert!((output.get_gray(i64::from(size[0] / 2), i64::from(size[1] / 2), false) - expected).abs() < 1e-7, "{operation}, {size:?}");
        }
        let output = cook("sharpen", &[Some(input)], &[("intensity", Value::Float(1.0))], size, true);

        assert_eq!(output.get_gray(i64::from(size[0] / 2), i64::from(size[1] / 2), false), 4.0, "{size:?}");
    }
}
