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
fn min_blend_accumulates_the_background_weight_on_the_foreground_product() {
    let foreground = Image::gray(1, 1, |_, _| 46.0 / 255.0);
    let background = Image::gray(1, 1, |_, _| 79.0 / 255.0);
    let out = cook("blend", &[Some(foreground), Some(background)], &[("mode", Value::Enum(6)), ("opacity", Value::Float(0.56))], [1, 1], true);
    assert_eq!(out.get_gray(0, 0, false).to_bits(), 0x3e730783);
}

#[test]
fn emboss_applies_each_tint_to_a_one_sided_height_difference() {
    let color = Image::filled(32, 32, Kind::Color, [0.25, 0.5, 0.75, 0.375]);
    let height = Image::gray(32, 32, |x, y| if x == 16 && y == 16 { 1.0 } else { 0.0 });
    let image = cook(
        "emboss",
        &[Some(color), Some(height)],
        &[("intensity", Value::Float(2.0)), ("highlight", Value::Color([0.75, 0.25, 0.125, 0.5])), ("shadow", Value::Color([0.125, 0.875, 0.375, 0.25]))],
        [32, 32],
        true,
    );
    assert_eq!(image.get(16, 16, false), [0.4375, 0.5625, 0.78125, 0.375]);
    assert_eq!(image.get(17, 16, false), [0.03125, 0.46875, 0.59375, 0.375]);
    assert_eq!(image.get(15, 16, false), [0.25, 0.5, 0.75, 0.375]);
}

#[test]
fn emboss_averages_the_tints_for_a_grayscale_input() {
    let base = Image::gray(32, 32, |_, _| 0.5);
    let height = Image::gray(32, 32, |x, y| f32::from(x == 16 && y == 16));
    let image = cook(
        "emboss",
        &[Some(base), Some(height)],
        &[("intensity", Value::Float(1.0)), ("highlight", Value::Color([0.75, 0.25, 0.125, 0.5])), ("shadow", Value::Color([0.125, 0.875, 0.375, 0.25]))],
        [32, 32],
        true,
    );
    assert_eq!(image.get_gray(16, 16, false), 0.546875);
    assert_eq!(image.get_gray(17, 16, false), 0.4322917);
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
fn levels_filters_stored_integer_texels_before_remapping_the_range() {
    let field = Image::gray(4, 8, |x, y| ((7 * x + 11 * y + 19 * x * y + 9) % 2000) as f32 / 65535.0);
    let input = materialcraft_nodes::output_precision::apply(field, OutputFormat::U16);
    let image = cook("levels", &[Some(input)], &[], [8, 8], true);
    let row = std::array::from_fn(|x| image.get_gray(x as i64, 0, false).to_bits());
    assert_eq!(row, [0x396000e9, 0x393000b7, 0x396000e9, 0x39900094, 0x39a800a9, 0x39c800ce, 0x39e000e9, 0x39c800ce]);
}

#[test]
fn levels_stores_an_enlarged_input_at_half_precision_before_applying_the_curve() {
    let params = [
        ("in_low", Value::Float4([0.0625; 4])),
        ("in_mid", Value::Float4([0.625; 4])),
        ("out_low", Value::Float4([0.125; 4])),
        ("out_high", Value::Float4([0.875; 4])),
    ];
    for (width, expected) in [(4, 0x3e620000), (16, 0x3e1f4000)] {
        let field = Image::gray(width, 8, |x, y| ((341 + 3279 * x + 9759 * y + 1737 * x * y) % 65536) as f32 / 65535.0);
        let input = materialcraft_nodes::output_precision::apply(field, OutputFormat::U16);
        let image = cook_format("levels", &[Some(input)], &params, [8, 8], true, OutputFormat::F16);
        let stored = materialcraft_nodes::output_precision::apply(image, OutputFormat::F16);
        assert_eq!(stored.get_gray(5, 2, false).to_bits(), expected, "source width {width}");
    }
}

#[test]
fn grayscale_levels_averages_rgb_controls_before_bounding_integer_endpoints() {
    let input = [Some(Image::gray(1, 1, |_, _| 45308.0 / 65535.0))];
    let params = [
        ("in_low", Value::Float4([-0.5, 0.25, 1.625, -2.0])),
        ("in_mid", Value::Float4([-0.3, 0.53, 1.25, 0.1])),
        ("in_high", Value::Float4([0.601, -0.375, 1.875, 2.5])),
        ("out_low", Value::Float4([-0.125, -0.875, 1.625, -0.5])),
        ("out_high", Value::Float4([1.875, 1.125, -0.375, 2.125])),
        ("clamp_intermediate", Value::Bool(false)),
    ];
    let floating = cook("levels", &input, &params, [1, 1], true);
    assert_eq!(floating.get_gray(0, 0, false).to_bits(), 0x3f5a44c2);
    let integer = cook_format("levels", &input, &params, [1, 1], true, OutputFormat::U16);
    let stored = materialcraft_nodes::output_precision::apply(integer, OutputFormat::U16);
    assert_eq!((stored.get_gray(0, 0, false) * 65535.0).round(), 55876.0);

    let flat = [("out_low", Value::Float4([0.01341; 4])), ("out_high", Value::Float4([0.01341; 4]))];
    let image = cook("levels", &input, &flat, [1, 1], true);
    assert_eq!(image.get_gray(0, 0, false).to_bits(), 0x3c5bb59d);
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
fn zero_blur_copies_through_the_output_precision_when_resizing() {
    let field = Image::gray(32, 32, |x, y| (((17 * x + 113 * y + 59 * x * y + 19) * 7919) % 65536) as f32 / 65535.0);
    let input = [Some(materialcraft_nodes::output_precision::apply(field, OutputFormat::U16))];
    for (intensity, expected) in [(0.0, [145, 139]), (0.0009765625, [146, 140]), (3.2, [146, 140]), (-3.2, [146, 140])] {
        let params = [("intensity", Value::Float(intensity))];
        for id in ["blur", "directional_blur"] {
            let result = cook_format(id, &input, &params, [8, 8], true, OutputFormat::U8);
            let stored = materialcraft_nodes::output_precision::apply(result, OutputFormat::U8);
            assert_eq!([2, 5].map(|x| (stored.get_gray(x, 0, false) * 255.0).round() as u8), expected, "{id} intensity {intensity}");
        }
    }
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

fn impulse_and_stripes() -> Image {
    Image::generate(128, 128, Kind::Color, |x, y| {
        let value = f64::from((97 * x + 43 * y + 7 * x * y) % 1024) / 1023.0;
        [f32::from(x == 64 && y == 64), f32::from(x == 64), f32::from(y == 64), (3.0 * value - 1.25) as f32]
    })
}

#[test]
fn five_texel_blurs_filter_the_outer_pairs_before_accumulating() {
    let input = [Some(impulse_and_stripes())];
    let params = [("intensity", Value::Float(4.0))];
    let directional = cook("directional_blur", &input, &params, [128; 2], true);
    assert_eq!(directional.get(62, 64, false).map(f32::to_bits), [0x3dff0000, 0x3dff0000, 0x3f800000, 0x3d6739cc]);
    let separable = cook("blur", &input, &params, [128; 2], true);
    assert_eq!(separable.get(63, 64, false).map(f32::to_bits), [0x3d804000, 0x3e804000, 0x3e800000, 0x3e1e66da]);
}

#[test]
fn rotated_axial_blurs_project_the_outer_texel_coverage() {
    let input = [Some(impulse_and_stripes())];
    for (angle, strength, x, y, expected) in [
        (0.0, 2.0, 63, 64, [0x3e800000, 0x3e800000, 0x3f800000, 0x3e1e6799]),
        (0.25, 2.0, 64, 63, [0x3e7fffff, 0x3f800000, 0x3e7fffff, 0x3ea04810]),
        (-0.25, 4.0, 64, 64, [0x3e800001, 0x3f800000, 0x3e800001, 0x3e800001]),
        (0.75, 10.0, 64, 64, [0x3dccccce, 0x3f800001, 0x3dccccce, 0x3e80e835]),
        (0.5, 2.0, 63, 64, [0x3e7fffff, 0x3e7fffff, 0x3f800000, 0x3e1e6796]),
        (1.0, 10.0, 64, 64, [0x3dccccce, 0x3dccccce, 0x3f800001, 0x3e80e427]),
    ] {
        let params = [("intensity", Value::Float(strength)), ("angle", Value::Float(angle))];
        let image = cook("directional_blur", &input, &params, [128; 2], true);
        assert_eq!(image.get(x, y, false).map(f32::to_bits), expected, "angle {angle}, strength {strength}");
    }
}

fn signed_modular_field(width: u32, height: u32) -> Image {
    Image::generate(width, height, Kind::Color, |x, y| {
        std::array::from_fn(|c| {
            let c = c as u32;
            let level = (37 * x + 71 * y + 11 * x * y + 47 * c + 3 * x * c) % 1021;
            (3.0 * (f64::from(level) / 1020.0) - 1.125) as f32
        })
    })
}

#[test]
fn wide_diagonal_blur_filters_between_two_sheared_grids() {
    let input = [Some(impulse_and_stripes())];
    for (strength, angle, x, y, expected) in [
        (4.0, 0.125, 64, 64, [0x3e24087d, 0x3ea4087d, 0x3ea4087d, 0x3e891e8a]),
        (4.0, 0.125, 64, 65, [0x3da4087d, 0x3ea4087d, 0x3ea4087d, 0x3e532d50]),
        (10.0, 0.125, 63, 65, [0x00000000, 0x3e0b0e3c, 0x3e0b0e3c, 0x3e2adf33]),
        (4.75, -0.125, 64, 64, [0x3e0c35e8, 0x3e8c35e8, 0x3e8c35e8, 0x3e66fbdc]),
    ] {
        let params = [("intensity", Value::Float(strength)), ("angle", Value::Float(angle))];
        let image = cook("directional_blur", &input, &params, [128; 2], true);
        assert_eq!(image.get(x, y, false).map(f32::to_bits), expected, "strength {strength}, angle {angle}, ({x},{y})");
    }
}

#[test]
fn diagonal_blur_stores_the_integer_window_before_resampling() {
    let input = [Some(Image::gray(32, 32, |_, y| if y % 8 == 0 { 1.0 / 255.0 } else { 0.0 }))];
    for (format, strength, maximum, expected) in
        [(OutputFormat::U8, 19.0, 255.0, [0, 0, 1, 0, 0, 0, 1, 1]), (OutputFormat::U16, 48.0, 65535.0, [30, 30, 30, 30, 52, 30, 30, 30])]
    {
        let settings = [("intensity", Value::Float(strength)), ("angle", Value::Float(0.125))];
        let image = cook_format("directional_blur", &input, &settings, [32; 2], true, format);
        let levels: [u32; 8] = std::array::from_fn(|y| (image.get_gray(0, y as i64, false) * maximum).round() as u32);
        assert_eq!(levels, expected, "{format:?}");
    }
}

#[test]
fn oblique_blur_samples_stored_levels_across_the_tile_seam() {
    let input = [Some(impulse_and_stripes())];
    let params = [("intensity", Value::Float(10.75)), ("angle", Value::Float(0.0625))];
    let blurred = cook_format("directional_blur", &input, &params, [128; 2], true, OutputFormat::U8);
    let stored = materialcraft_nodes::output_precision::apply(blurred, OutputFormat::U8);
    for (x, y, expected) in [(0, 65, [0, 0, 57, 116]), (127, 52, [0, 0, 0, 88]), (64, 64, [17, 26, 61, 99])] {
        assert_eq!(stored.get(x, y, false).map(|v| (v * 255.0).round() as u8), expected, "({x}, {y})");
    }
}

#[test]
fn short_oblique_blur_filters_fractional_taps_and_preserves_the_image_aspect() {
    let input = [Some(impulse_and_stripes())];
    for (angle, strength, x, y, expected) in
        [(0.125, 2.0, 63, 63, [0x3e5413cc, 0x3e5413cc, 0x3e5413cc, 0x3f7efc72]), (0.0625, 2.5, 65, 64, [0x3e2e41cd, 0x3e94b308, 0x3f426ddf, 0x3de85ee0])]
    {
        let settings = [("intensity", Value::Float(strength)), ("angle", Value::Float(angle))];
        let image = cook("directional_blur", &input, &settings, [128; 2], true);
        assert_eq!(image.get(x, y, false).map(f32::to_bits), expected);
    }

    for (size, angle, tile, x, y, expected) in [
        ([128, 64], -0.0625, false, 0, 0, [0xbf87572f, 0xbf6ac022, 0xbf46d1e6, 0xbf22e3aa]),
        ([64, 128], 0.03125, true, 63, 0, [0xbeeb11b8, 0x3cb90b12, 0x3f13afa5, 0x3f90cb7a]),
    ] {
        let input = [Some(signed_modular_field(size[0], size[1]))];
        let settings = [("intensity", Value::Float(4.0)), ("angle", Value::Float(angle))];
        let image = cook("directional_blur", &input, &settings, size, tile);
        assert_eq!(image.get(x, y, false).map(f32::to_bits), expected);
    }
}

#[test]
fn wide_blur_reduces_aligned_runs_at_the_destination_precision() {
    let input = [Some(signed_modular_field(64, 128))];
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
