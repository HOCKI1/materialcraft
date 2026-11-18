use craft_texture::{Image, Kind};
use materialcraft_core::{FunctionGraph, Value};
use std::collections::{BTreeMap, BTreeSet};

use super::{EvaluationContext, EvaluationState, FUNCTIONS, compile, default_value};

fn evaluate(graph: &FunctionGraph, variables: &BTreeMap<String, Value>) -> Result<Value, String> {
    let image = Image::filled(3, 5, Kind::Color, [0.25, 0.5, 0.75, 1.0]);
    let context = EvaluationContext {
        image_formats: &[],
        scope: Default::default(),
        position: [0.5; 2],
        size: [3.0, 5.0],
        images: &[Some(image)],
        tile: true,
        variables,
    };
    compile(graph, 7)?.evaluate(&context, &mut EvaluationState::default())
}

fn operation(name: &str, arguments: &[Value], setting: Option<Value>) -> Result<Value, String> {
    let mut graph = FunctionGraph::default();
    let inputs: Vec<_> = arguments.iter().map(|value| graph.add_constant(value.clone())).collect();
    graph.output = Some(graph.add(name, &inputs, setting));
    evaluate(&graph, &BTreeMap::new())
}

fn pixel_operation(op: &str, operands: &[Value], literal: bool) -> Value {
    scoped_operation(super::FunctionScope::PixelProcessor, op, operands, literal)
}

fn scoped_operation(scope: super::FunctionScope, op: &str, operands: &[Value], literal: bool) -> Value {
    let mut graph = FunctionGraph::default();
    let mut variables = BTreeMap::new();
    let mut inputs = Vec::new();
    for (i, operand) in operands.iter().enumerate() {
        let input = if literal {
            graph.add_constant(operand.clone())
        } else {
            let name = format!("input{i}");
            variables.insert(name.clone(), operand.clone());
            graph.add(&format!("get_float{}", super::value_type::dimensions(operand)), &[], Some(Value::Text(name)))
        };
        inputs.push(input);
    }
    graph.output = Some(graph.add(op, &inputs, None));
    let context = EvaluationContext { image_formats: &[], scope, position: [0.5; 2], size: [1.0; 2], images: &[], tile: true, variables: &variables };
    compile(&graph, 0).unwrap().evaluate_pixel(&context, &mut EvaluationState::default()).unwrap()
}

#[test]
fn runtime_arithmetic_flushes_subnormal_operands_and_results_with_their_signs() {
    for scope in [super::FunctionScope::Node, super::FunctionScope::ValueProcessor, super::FunctionScope::PixelProcessor, super::FunctionScope::FxMap] {
        for (op, a, b, expected) in [
            ("add", 0x00800001, 0x80800000, 0),
            ("sub", 0x00800001, 0x00800000, 0),
            ("mul", 0x807fffff, 0x40000000, 0x80000000),
            ("mul", 0x00800000, 0x3f000000, 0),
            ("div", 0x00800000, 0x40000000, 0),
            ("div", 0x00800000, 0x807fffff, f32::NEG_INFINITY.to_bits()),
        ] {
            let args = [Value::Float(f32::from_bits(a)), Value::Float(f32::from_bits(b))];
            let actual = scoped_operation(scope, op, &args, false);
            assert!(matches!(actual, Value::Float(value) if value.to_bits() == expected), "{scope:?} {op}: {actual:?}");
        }
    }
    let a = Value::Float2([f32::from_bits(0x00400000), f32::MIN_POSITIVE]);
    let b = Value::Float2([f32::from_bits(0x00400000), 0.5]);
    assert_eq!(pixel_operation("dot", &[a, b], false), Value::Float(0.0));
}

#[test]
fn uniform_logarithms_and_powers_use_their_measured_domains() {
    for (op, input, exponent, bits) in [
        ("log", -1.0, None, 0),
        ("log", f32::from_bits(1), None, 0),
        ("log", 0.5625, None, 0xbf134b11),
        ("log", 0.75, None, 0xbe934b11),
        ("pow", 0.0, Some(-2.0), 0),
        ("pow", -2.0, Some(0.5), 0),
        ("pow", -2.0, Some(-3.0), 0xbe000000),
        ("pow", 0.3, Some(2.4), 0x3d63befe),
        ("pow", 0.563_981_1, Some(2.4), 0x3e8182ad),
        ("pow", 0.95, Some(-4.125), 0x3f9e2948),
    ] {
        let mut inputs = vec![Value::Float(input)];
        inputs.extend(exponent.map(Value::Float));
        for scope in [super::FunctionScope::Node, super::FunctionScope::ValueProcessor] {
            let Value::Float(value) = scoped_operation(scope, op, &inputs, false) else { panic!("expected a scalar") };
            assert_eq!(value.to_bits(), bits, "{op}({input}, {exponent:?})");
        }
    }
}

#[test]
fn runtime_pixel_square_root_normalizes_exponents_and_rounds_halfway_down() {
    for (input, expected) in [
        (0x3f801000, 0x3f8007ff),
        (0x40300000, 0x3fd44394),
        (0x3fe20000, 0x3faa1513),
        (0x00800000, 0x20000000),
        (0x007fffff, 0),
        (0x80000000, 0),
        (0xbf800000, 0),
        (0x7fc00000, 0),
        (0x7f800000, 0x7f800000),
    ] {
        let Value::Float(value) = pixel_operation("sqrt", &[Value::Float(f32::from_bits(input))], false) else { panic!("expected a scalar") };
        assert_eq!(value.to_bits(), expected, "sqrt({input:08x})");
    }
    let x = f32::from_bits(0x3f801000);
    assert_eq!(pixel_operation("sqrt", &[Value::Float(x)], true), Value::Float(x.sqrt()));
}

#[test]
fn runtime_pixel_natural_exponential_reduces_to_a_positive_interval() {
    for (input, expected) in [
        (2.0, 0x40ec7325),
        (f32::from_bits(0xbf317219), 0x3effffff),
        (f32::from_bits(0xc2aeac4f), 0x00800026),
        (f32::from_bits(0xc2aeac50), 0),
        (f32::from_bits(0x42b17217), 0x7f7fff84),
        (f32::from_bits(0x42b17218), 0x7f800000),
    ] {
        let Value::Float(value) = pixel_operation("exp", &[Value::Float(input)], false) else { panic!("expected a scalar") };
        assert_eq!(value.to_bits(), expected, "exp({input})");
    }
    assert_eq!(pixel_operation("pow", &[Value::Float(0.7), Value::Float(4.0)], false), Value::Float(f32::from_bits(0x3e75dccb)));
}

#[test]
fn uniform_exponential_rounds_each_product_before_adding() {
    use super::FunctionScope::{FxMap, Node, ValueProcessor};
    for scope in [Node, ValueProcessor, FxMap] {
        for (input, expected) in [(0x3d07db5a, 0x3f845114), (0x3eb3866e, 0x3fb5c18a), (0x3f25a984, 0x3ff47bcc), (0xc2b00000, 0)] {
            let Value::Float(value) = scoped_operation(scope, "exp", &[Value::Float(f32::from_bits(input))], false) else { panic!("expected a scalar") };
            assert_eq!(value.to_bits(), expected, "{scope:?} exp({input:08x})");
        }
    }
    let input = Value::Float(f32::from_bits(0x3d07db5a));
    assert_eq!(pixel_operation("exp", std::slice::from_ref(&input), false), Value::Float(f32::from_bits(0x3f845115)));
    for scope in [Node, ValueProcessor, FxMap, super::FunctionScope::PixelProcessor] {
        assert_eq!(scoped_operation(scope, "exp", std::slice::from_ref(&input), true), Value::Float(f32::from_bits(0x3f845114)));
    }
}

#[test]
fn runtime_pixel_power_uses_the_natural_logarithm_and_the_sign_of_integer_powers() {
    for (base, exponent, expected) in [
        (0.7, 1.0, 0x3f333334),
        (0.7, 1.375, 0x3f1cc3e8),
        (-0.7, 3.0, 0xbeaf9db5),
        (-2.0, -1.0, 0xbf000000),
        (-2.0, 0.5, 0),
        (0.0, -1.0, 0),
        (0.0, 0.0, 0),
        (f32::MIN_POSITIVE, 1.0, 0x00800026),
    ] {
        let Value::Float(value) = pixel_operation("pow", &[Value::Float(base), Value::Float(exponent)], false) else { panic!("expected a scalar") };
        assert_eq!(value.to_bits(), expected, "{base}^{exponent}");
    }
}

#[test]
fn runtime_pixel_logarithms_and_powers_of_two() {
    for (op, input, expected) in [
        ("log2", 0.5, 0xbf7ffffe),
        ("log2", 0.7, 0xbf03bb10),
        ("log2", 1.0, 0),
        ("log2", f32::MIN_POSITIVE, 0xc2fbffff),
        ("log", 0.5, 0xbf317217),
        ("pow2", -0.5, 0x3f3504f2),
        ("pow2", 0.5, 0x3fb504f3),
        ("pow2", 1.375, 0x4025fed7),
        ("pow2", -126.0, 0x00800000),
        ("pow2", -127.0, 0),
        ("pow2", 128.0, 0x7f800000),
    ] {
        let Value::Float(value) = pixel_operation(op, &[Value::Float(input)], false) else { panic!("expected a scalar") };
        assert_eq!(value.to_bits(), expected, "{op}({input})");
    }
    for scope in [super::FunctionScope::Node, super::FunctionScope::ValueProcessor, super::FunctionScope::FxMap] {
        assert_eq!(scoped_operation(scope, "log2", &[Value::Float(0.5)], false), Value::Float(-1.0));
    }
    assert_eq!(pixel_operation("log2", &[Value::Float(0.5)], true), Value::Float(-1.0));
}

#[test]
fn runtime_pixel_sine_and_cosine_reduce_the_phase_before_interpolation() {
    use std::f32::consts::{FRAC_PI_4, TAU};

    for (angle, sine, cosine) in [
        (-0.0, 0x80000000, 0x3f800000),
        (0.625, 0x3f15c8eb, 0x3f4f9b48),
        (-FRAC_PI_4, 0xbf3504f3, 0x3f3504f2),
        (TAU, 0x80000000, 0x3f800000),
        (f32::from_bits(0xc96df2ae), 0x3e98c108, 0x3f74574d),
    ] {
        for (op, expected) in [("sin", sine), ("cos", cosine)] {
            let Value::Float(result) = pixel_operation(op, &[Value::Float(angle)], false) else { panic!("expected a scalar") };
            assert_eq!(result.to_bits(), expected, "{op}({angle})");
        }
    }
    assert_eq!(pixel_operation("sin", &[Value::Float(0.625)], true), Value::Float(f32::from_bits(0x3f15c8ef)));
    assert_eq!(pixel_operation("cos", &[Value::Float(0.625)], true), Value::Float(f32::from_bits(0x3f4f9b47)));
}

#[test]
fn runtime_pixel_cartesian_multiplies_the_radius_by_each_quarter_wave() {
    for (radius, angle, expected) in [
        (-1.25, std::f32::consts::FRAC_PI_4, [0xbf62462e, 0xbf624630]),
        (0.375, -0.0, [0x3ec00000, 0x80000000]),
        (0.375, std::f32::consts::FRAC_PI_2, [0, 0x3ec00000]),
    ] {
        let result = pixel_operation("cartesian", &[Value::Float(radius), Value::Float(angle)], false);
        let Value::Float2(xy) = result else { panic!("expected a coordinate") };
        assert_eq!(xy.map(f32::to_bits), expected);
    }
    let result = pixel_operation("cartesian", &[Value::Float(0.875), Value::Float(0.625)], true);
    let Value::Float2(xy) = result else { panic!("expected a coordinate") };
    assert_eq!(xy.map(f32::to_bits), [0x3f35a7de, 0x3f030fd1]);
}

#[test]
fn runtime_pixel_tangent_uses_the_pixel_sine_cosine_and_reciprocal() {
    for (angle, expected) in [(0x3db7cb5d, 0x3db84a0d), (0xc094d5bd, 0xc18258bb), (0x412febe1, 0x450051c9), (0x4453660e, 0x3f0de743)] {
        let Value::Float(value) = pixel_operation("tan", &[Value::Float(f32::from_bits(angle))], false) else { panic!("expected a scalar") };
        assert_eq!(value.to_bits(), expected);
    }
    for scope in [super::FunctionScope::Node, super::FunctionScope::ValueProcessor, super::FunctionScope::FxMap] {
        assert_eq!(scoped_operation(scope, "tan", &[Value::Float(0.625)], false), Value::Float(f32::from_bits(0x3f38b30b)));
    }
    let literal = Value::Float(f32::from_bits(0x3d07db5a));
    assert_eq!(pixel_operation("tan", &[literal], true), Value::Float(f32::from_bits(0x3d07e81c)));
}

#[test]
fn parameter_and_value_angles_fold_the_phase_before_the_polynomial() {
    use super::FunctionScope::{Node, ValueProcessor};
    for scope in [Node, ValueProcessor] {
        for (angle, sine, cosine) in [
            (0.625, 0x3f15c8f2, 0x3f4f9b7a),
            (-std::f32::consts::FRAC_PI_4, 0xbf350503, 0x3f350503),
            (1e-7, 0x34490fdb, 0x3f800000),
            (1e9, 0xbf800000, 0xbf800000),
        ] {
            for (op, expected) in [("sin", sine), ("cos", cosine)] {
                let Value::Float(v) = scoped_operation(scope, op, &[Value::Float(angle)], false) else { panic!("expected a scalar") };
                assert_eq!(v.to_bits(), expected, "{scope:?} {op}({angle})");
            }
        }
        assert_eq!(scoped_operation(scope, "sin", &[Value::Float(0.625)], true), Value::Float(f32::from_bits(0x3f15c8ef)));
    }
}

#[test]
fn pattern_angles_and_uniform_cartesian_use_the_same_components() {
    use super::FunctionScope::{FxMap, Node, ValueProcessor};
    for scope in [Node, ValueProcessor, FxMap] {
        let angle = Value::Float(0.625);
        assert_eq!(scoped_operation(scope, "cos", std::slice::from_ref(&angle), false), Value::Float(f32::from_bits(0x3f4f9b7a)));
        let Value::Float2(result) = scoped_operation(scope, "cartesian", &[Value::Float(0.875), angle], false) else { panic!("expected a coordinate") };
        assert_eq!(result.map(f32::to_bits), [0x3f35a80b, 0x3f030fd4]);
    }
}

#[test]
fn pixel_product_sums_round_once_unless_the_product_is_also_read() {
    let values = [0x3fe34b19, 0x400d3c6f, 0xc0732711, 0x3f9665e4].map(|bits| Value::Float(f32::from_bits(bits)));
    let variables: BTreeMap<_, _> = ["a", "b", "c", "d"].into_iter().map(str::to_owned).zip(values).collect();
    let context = EvaluationContext {
        image_formats: &[],
        scope: super::FunctionScope::PixelProcessor,
        position: [0.5; 2],
        size: [1.0; 2],
        images: &[],
        tile: true,
        variables: &variables,
    };
    let mut graph = FunctionGraph::default();
    let [a, b, c, d] = ["a", "b", "c", "d"].map(|name| graph.add("get_float1", &[], Some(Value::Text(name.into()))));
    let product = graph.add("mul", &[a, b], None);
    let other = graph.add("mul", &[c, d], None);
    let pixel = |graph: &FunctionGraph| compile(graph, 0).unwrap().evaluate_pixel(&context, &mut EvaluationState::default()).unwrap();

    for (op, inputs, bits) in [
        ("add", [product, c], 0x3df49f1b),
        ("add", [c, product], 0x3df49f1b),
        ("sub", [product, c], 0x40f6f98d),
        ("sub", [c, product], 0xc0f6f98d),
        ("add", [product, other], 0xbf0b9d31),
    ] {
        graph.output = Some(graph.add(op, &inputs, None));
        assert_eq!(pixel(&graph), Value::Float(f32::from_bits(bits)), "{op} {inputs:?}");
    }

    let sum = graph.add("add", &[product, c], None);
    graph.output = Some(graph.add("vector2", &[product, sum], None));
    assert_eq!(pixel(&graph), Value::Float2([0x407acc0a, 0x3df49f20].map(f32::from_bits)));
    let context = EvaluationContext { image_formats: &[], scope: super::FunctionScope::Node, ..context };
    graph.output = Some(sum);
    assert_eq!(compile(&graph, 0).unwrap().evaluate(&context, &mut EvaluationState::default()).unwrap(), Value::Float(f32::from_bits(0x3df49f20)));
}

#[test]
fn pixel_gray_sampler_combines_stored_color_channels_after_reading() {
    let mut graph = FunctionGraph::default();
    let position = graph.add_constant(Value::Float2([0.5; 2]));
    graph.output = Some(graph.add("samplelum", &[position], Some(Value::Int2([0, 0]))));
    let program = compile(&graph, 0).unwrap();
    let rgba = [0x40521096, 0xbfccf206, 0xbfa48635, 0x3f800000].map(f32::from_bits);
    for (kind, expected) in [(Kind::Color, 0x3e0717d6), (Kind::Gray, rgba[0].to_bits())] {
        let context = EvaluationContext {
            scope: super::FunctionScope::PixelProcessor,
            position: [0.5; 2],
            size: [1.0; 2],
            images: &[Some(Image::filled(1, 1, kind, rgba))],
            image_formats: &[],
            tile: true,
            variables: &BTreeMap::new(),
        };
        assert_eq!(program.evaluate_pixel(&context, &mut EvaluationState::default()).unwrap(), Value::Float(f32::from_bits(expected)));
    }
}

#[test]
fn pixel_dot_rounds_in_channel_order() {
    // One sampled texel distinguishes the accumulation orders in all three widths.
    let [x, y, z, w] = [0xc07089a0, 0x3fa2fba9, 0xbe30266f, 0xbcf10944].map(f32::from_bits);
    for (left, right, dynamic_bits, literal_bits) in [
        (Value::Float2([x, y]), Value::Float2([0.2126, 0.7152]), 0x3de49f4e, 0x3de49f50),
        (Value::Float3([x, y, z]), Value::Float3([0.2126, 0.7152, 0.0722]), 0x3dcb2fab, 0x3dcb2fad),
        (Value::Float4([x, y, z, w]), Value::Float4([0.2126, 0.7152, 0.0722, -0.39]), 0x3de2afee, 0x3de2aff0),
    ] {
        let operands = [left, right];
        assert_eq!(pixel_operation("dot", &operands, false), Value::Float(f32::from_bits(dynamic_bits)));
        assert_eq!(pixel_operation("dot", &operands, true), Value::Float(f32::from_bits(literal_bits)));
    }
}

#[test]
fn pixel_atan2_reconstructs_the_quadrant_from_the_runtime_ratio() {
    for (coordinate, bits) in [
        ([1.0, 1.0], 0x3f490fe1),
        ([-1.0, 1.0], 0x4016cbe3),
        ([-1.0, -1.0], 0xc016cbe3),
        ([1.0, -1.0], 0xbf490fe1),
        ([f32::from_bits(0xc07089a0), f32::from_bits(0x3fa2fba9)], 0x40342917),
        ([-1.0, -0.0], 0x40490fdb),
        ([-0.0, 0.0], 0),
    ] {
        let Value::Float(angle) = pixel_operation("atan2", &[Value::Float2(coordinate)], false) else { panic!("expected an angle") };
        assert_eq!(angle.to_bits(), bits, "{coordinate:?}");
    }
    assert_eq!(pixel_operation("atan2", &[Value::Float2([1.0, 1.0])], true), Value::Float(std::f32::consts::FRAC_PI_4));
}

#[test]
fn pixel_division_multiplies_by_the_reciprocal() {
    let operands = [Value::Float(-2.7353475), Value::Float(1.7320508)];
    assert_eq!(pixel_operation("div", &operands, false), Value::Float(f32::from_bits(0xbfca24fb)));
    assert_eq!(pixel_operation("div", &operands, true), Value::Float(f32::from_bits(0xbfca24fc)));
    // The runtime flushes both operands of this quotient, giving zero divided by zero.
    let tiny = Value::Float(f32::from_bits(1));
    let Value::Float(quotient) = pixel_operation("div", &[tiny.clone(), tiny], false) else { panic!("expected float") };
    assert!(quotient.is_nan());
}

#[test]
fn measured_pixel_reciprocals_preserve_runtime_rounding() {
    for (input, expected) in [(0x3ecccccc, 0x40200000), (0x3ecc4ccd, 0x4020643f), (0xc2ff6f11, 0xbc0048a0), (0x038537b1, 0x7b75f93f)] {
        let x = f32::from_bits(input);
        let operands = [Value::Float(1.0), Value::Float(x)];
        assert_eq!(pixel_operation("div", &operands, false), Value::Float(f32::from_bits(expected)));
        assert_eq!(pixel_operation("div", &operands, true), Value::Float(1.0 / x));
    }
}

#[test]
fn measured_pixel_random_draws_share_one_stream() {
    let mut graph = FunctionGraph::default();
    let one = graph.add_constant(Value::Float(1.0));
    let negative = graph.add_constant(Value::Float(-2.0));
    let factor = graph.add_constant(Value::Float(0.375));
    let first = graph.add("rand", &[one], None);
    let second = graph.add("rand", &[one], None);
    let third = graph.add("rand", &[negative], None);
    let fourth = graph.add("rand", &[factor], None);
    let pair1 = graph.add("vector2", &[first, second], None);
    let pair2 = graph.add("vector2", &[third, fourth], None);
    graph.output = Some(graph.add("vector4", &[pair1, pair2], None));
    let variables = BTreeMap::new();
    let context = EvaluationContext {
        scope: super::FunctionScope::PixelProcessor,
        position: [0.5; 2],
        size: [32.0; 2],
        images: &[],
        image_formats: &[],
        tile: true,
        variables: &variables,
    };
    let program = compile(&graph, 0).unwrap();
    for (mut state, bits) in [(9_091_073, [1060669180, 1057720276, 1034667712, 1054479120]), (3_342_178, [1058664790, 1044635448, 1061348922, 1062659162])] {
        let [a, b, c, d] = bits.map(f32::from_bits);
        assert_eq!(program.evaluate_with_random(&context, &mut EvaluationState::default(), &mut state).unwrap(), Value::Float4([a, b, -2.0 * c, 0.375 * d]));
    }
}

#[test]
fn pixel_lerp_fuses_the_scaled_difference() {
    let operands = [0xc04928a6, 0xbfc9a1db, 0xbefff6a8].map(|bits| Value::Float(f32::from_bits(bits)));
    assert_eq!(pixel_operation("lerp", &operands, false), Value::Float(f32::from_bits(0xc07b52ad)));
    assert_eq!(pixel_operation("lerp", &operands, true), Value::Float(f32::from_bits(0xc07b52ae)));
}

#[test]
fn system_variables_resolve_with_their_declared_types() {
    for (operation, name, expected) in [
        ("get_integer1", "$pixelratio", Value::Int(0)),
        ("get_float3", "$physicalsize", Value::Float3([0.0; 3])),
        ("get_integer2", "$uvtile", Value::Int2([0; 2])),
        ("get_integer1", "$normalformat", Value::Int(0)),
    ] {
        let mut graph = FunctionGraph::default();
        graph.output = Some(graph.add(operation, &[], Some(Value::Text(name.into()))));

        assert_eq!(evaluate(&graph, &BTreeMap::new()).unwrap(), expected, "{name}");
    }
}

#[test]
fn every_native_function_executes_with_its_declared_ports() {
    let mut covered = BTreeSet::new();
    for definition in FUNCTIONS {
        let name = definition.id;

        assert!(covered.insert(name), "duplicate {name}");

        let mut graph = FunctionGraph::default();
        let mut variables = BTreeMap::new();
        let mut setting = default_value(name);
        let mut arguments = vec![Value::Float(0.5); definition.inputs.len()];
        match name {
            "get_bool" => {
                variables.insert("variable".into(), Value::Bool(true));
            }
            "get_string" => {
                variables.insert("variable".into(), Value::Text("test".into()));
            }
            "get_float1" => {
                variables.insert("variable".into(), Value::Float(1.0));
            }
            "get_float2" => {
                variables.insert("variable".into(), Value::Float2([1.0; 2]));
            }
            "get_float3" => {
                variables.insert("variable".into(), Value::Float3([1.0; 3]));
            }
            "get_float4" => {
                variables.insert("variable".into(), Value::Float4([1.0; 4]));
            }
            "get_integer1" => {
                variables.insert("variable".into(), Value::Int(1));
            }
            "get_integer2" => {
                variables.insert("variable".into(), Value::Int2([1; 2]));
            }
            "get_integer3" => {
                variables.insert("variable".into(), Value::Int3([1; 3]));
            }
            "get_integer4" => {
                variables.insert("variable".into(), Value::Int4([1; 4]));
            }
            "and" | "or" | "not" => arguments.fill(Value::Bool(true)),
            "atan2" | "samplecol" | "samplelum" => arguments = vec![Value::Float2([0.5; 2])],
            "vector3" => arguments = vec![Value::Float2([1.0; 2]), Value::Float(2.0)],
            "vector4" => arguments = vec![Value::Float3([1.0; 3]), Value::Float(2.0)],
            "ivector2" => arguments = vec![Value::Int(1), Value::Int(2)],
            "ivector3" => arguments = vec![Value::Int2([1; 2]), Value::Int(2)],
            "ivector4" => arguments = vec![Value::Int3([1; 3]), Value::Int(2)],
            "tofloat" => arguments = vec![Value::Int(1)],
            "tofloat2" => arguments = vec![Value::Int2([1; 2])],
            "tofloat3" => arguments = vec![Value::Int3([1; 3])],
            "tofloat4" => arguments = vec![Value::Int4([1; 4])],
            "toint2" => arguments = vec![Value::Float2([1.5; 2])],
            "toint3" => arguments = vec![Value::Float3([1.5; 3])],
            "toint4" => arguments = vec![Value::Float4([1.5; 4])],
            "ifelse" => arguments = vec![Value::Bool(false), Value::Float(1.0), Value::Float(2.0)],
            "while" => arguments = vec![Value::Float(0.0), Value::Bool(true), Value::Float(1.0)],
            _ if name.starts_with("iswizzle") => arguments = vec![Value::Int4([1, 2, 3, 4])],
            _ if name.starts_with("swizzle") => arguments = vec![Value::Float4([1.0, 2.0, 3.0, 4.0])],
            _ => {}
        }
        if name.starts_with("get_") || matches!(name, "set" | "isdefined") {
            setting = Some(Value::Text("variable".into()));
        }
        let inputs: Vec<_> = arguments.into_iter().map(|value| graph.add_constant(value)).collect();
        let node = graph.add(name, &inputs, setting);
        graph.output = Some(if name == "passthrough" { graph.add("neg", &[node], None) } else { node });
        evaluate(&graph, &variables).unwrap_or_else(|error| panic!("{name}: {error}"));
    }
    assert_eq!(covered.len(), 85);
}

#[test]
fn integer_math_preserves_all_32_bits_and_rejects_overflow() {
    assert_eq!(operation("add", &[Value::Int(16_777_217), Value::Int(1)], None).unwrap(), Value::Int(16_777_218));
    assert_eq!(operation("mul", &[Value::Int3([2, 3, 4]), Value::Int3([3, 4, 5])], None).unwrap(), Value::Int3([6, 12, 20]));
    assert!(operation("add", &[Value::Int(i32::MAX), Value::Int(1)], None).is_err());
    assert!(operation("div", &[Value::Int(i32::MIN), Value::Int(-1)], None).is_err());
    assert!(operation("div", &[Value::Float(1.0), Value::Float(0.0)], None).is_err());
}

#[test]
fn vectors_concatenate_swizzle_and_cast_without_broadcasting() {
    let vector = operation("vector4", &[Value::Float2([1.0, 2.0]), Value::Float2([3.0, 4.0])], None).unwrap();

    assert_eq!(vector, Value::Float4([1.0, 2.0, 3.0, 4.0]));
    assert_eq!(operation("swizzle3", &[vector], Some(Value::Int3([3, 0, 3]))).unwrap(), Value::Float3([4.0, 1.0, 4.0]));
    assert_eq!(operation("toint2", &[Value::Float2([-1.9, 2.9])], None).unwrap(), Value::Int2([-2, 2]));
    assert_eq!(operation("swizzle1", &[Value::Float2([0.0; 2])], Some(Value::Int(2))).unwrap(), Value::Float(0.0));
    assert_eq!(operation("pow2", &[Value::Float(3.0)], None).unwrap(), Value::Float(8.0));
}

#[test]
fn branches_are_lazy_and_sequence_orders_side_effects() {
    let mut graph = FunctionGraph::default();
    let condition = graph.add_constant(Value::Bool(true));
    let expected = graph.add_constant(Value::Int(17));
    let undefined = graph.add("get_integer1", &[], Some(Value::Text("missing".into())));
    graph.output = Some(graph.add("ifelse", &[condition, expected, undefined], None));

    assert_eq!(evaluate(&graph, &BTreeMap::new()).unwrap(), Value::Int(17));

    let set = graph.add("set", &[expected], Some(Value::Text("stored".into())));
    let get = graph.add("get_integer1", &[], Some(Value::Text("stored".into())));
    graph.output = Some(graph.add("sequence", &[set, get], None));

    assert_eq!(evaluate(&graph, &BTreeMap::new()).unwrap(), Value::Int(17));
}

#[test]
fn time_and_tiling_variables_have_typed_defaults_and_host_overrides() {
    for (name, operation, default, supplied) in
        [("$time", "get_float1", Value::Float(0.0), Value::Float(2.5)), ("$tiling", "get_integer1", Value::Int(3), Value::Int(2))]
    {
        let mut graph = FunctionGraph::default();
        let get = graph.add(operation, &[], Some(Value::Text(name.into())));
        let defined = graph.add("isdefined", &[], Some(Value::Text(name.into())));
        graph.output = Some(get);

        for variables in [BTreeMap::new(), BTreeMap::from([(name.into(), supplied.clone())])] {
            let expected = variables.get(name).unwrap_or(&default);
            let context = EvaluationContext {
                image_formats: &[],
                scope: Default::default(),
                position: [0.5; 2],
                size: [1.0; 2],
                images: &[],
                tile: true,
                variables: &variables,
            };
            let program = compile(&graph, 0).unwrap();

            assert_eq!(program.evaluate(&context, &mut EvaluationState::default()).unwrap(), *expected);
            assert_eq!(program.evaluate_pixel(&context, &mut EvaluationState::default()).unwrap(), *expected);
        }

        assert!(evaluate(&graph, &BTreeMap::from([(name.into(), Value::Bool(false))])).unwrap_err().contains("wrong type"));

        graph.output = Some(defined);

        assert_eq!(evaluate(&graph, &BTreeMap::new()).unwrap(), Value::Bool(true));
    }

    let mut graph = FunctionGraph::default();
    graph.output = Some(graph.add("get_integer1", &[], Some(Value::Text("$tiling".into()))));
    let context = EvaluationContext {
        image_formats: &[],
        scope: Default::default(),
        position: [0.5; 2],
        size: [1.0; 2],
        images: &[],
        tile: false,
        variables: &BTreeMap::new(),
    };

    assert_eq!(compile(&graph, 0).unwrap().evaluate(&context, &mut EvaluationState::default()).unwrap(), Value::Int(0));
}

#[test]
fn skipped_processor_loop_runs_its_initializer_and_returns_a_typed_zero() {
    let mut graph = FunctionGraph::default();
    let yes = graph.add_constant(Value::Bool(true));
    let initialize = graph.add("set", &[yes], Some(Value::Text("started".into())));
    let body = graph.add_constant(Value::Float2([4.0, 5.0]));
    let result = graph.add("while", &[initialize, yes, body], Some(Value::Int(2)));
    let started = graph.add("get_bool", &[], Some(Value::Text("started".into())));
    let after = graph.add("sequence", &[result, started], None);
    for scope in [super::FunctionScope::Node, super::FunctionScope::PixelProcessor, super::FunctionScope::ValueProcessor, super::FunctionScope::FxMap] {
        let context =
            EvaluationContext { scope, position: [0.0; 2], size: [16.0; 2], images: &[], image_formats: &[], tile: true, variables: &BTreeMap::new() };
        for (output, expected) in [(result, Value::Float2([0.0; 2])), (after, Value::Bool(true))] {
            graph.output = Some(output);
            assert_eq!(compile(&graph, 0).unwrap().evaluate(&context, &mut EvaluationState::default()).unwrap(), expected);
        }
    }
}

#[test]
fn loops_recompute_state_and_have_a_budget() {
    let mut graph = FunctionGraph::default();
    let zero = graph.add_constant(Value::Int(0));
    let one = graph.add_constant(Value::Int(1));
    let three = graph.add_constant(Value::Int(3));
    let init = graph.add("set", &[zero], Some(Value::Text("counter".into())));
    let get = graph.add("get_integer1", &[], Some(Value::Text("counter".into())));
    let condition = graph.add("gteq", &[get, three], None);
    let add = graph.add("add", &[get, one], None);
    let body = graph.add("set", &[add], Some(Value::Text("counter".into())));
    graph.output = Some(graph.add("while", &[init, condition, body], Some(Value::Int(-1))));

    assert_eq!(evaluate(&graph, &BTreeMap::new()).unwrap(), Value::Int(3));

    let condition = graph.add_constant(Value::Bool(false));
    graph.output = Some(graph.add("while", &[zero, condition, one], Some(Value::Int(-1))));

    assert!(evaluate(&graph, &BTreeMap::new()).unwrap_err().contains("budget"));
}

#[test]
fn deep_graphs_shared_inputs_and_cycles_do_not_recurse() {
    let mut graph = FunctionGraph::constant(Value::Float(1.0));
    let mut previous = graph.output.unwrap();
    for _ in 0..3000 {
        previous = graph.add("passthrough", &[previous], None);
    }
    graph.output = Some(graph.add("abs", &[previous], None));

    assert_eq!(evaluate(&graph, &BTreeMap::new()).unwrap(), Value::Float(1.0));

    graph.nodes[0].op = "passthrough".into();
    graph.nodes[0].inputs = vec![Some(previous)];

    assert!(compile(&graph, 0).err().unwrap().contains("cycle"));

    let mut graph = FunctionGraph::constant(Value::Float(1.0));
    let mut previous = graph.output.unwrap();
    for _ in 0..30 {
        previous = graph.add("add", &[previous, previous], None);
    }
    graph.output = Some(previous);

    assert_eq!(evaluate(&graph, &BTreeMap::new()).unwrap(), Value::Float(1_073_741_824.0));
}

#[test]
fn pixel_assignments_are_local_and_match_exported_evaluation() {
    let mut graph = FunctionGraph::default();
    let current = graph.add("get_float1", &[], Some(Value::Text("counter".into())));
    let one = graph.add_constant(Value::Float(1.0));
    let next = graph.add("add", &[current, one], None);
    let assign = graph.add("set", &[next], Some(Value::Text("counter".into())));
    let get = graph.add("get_float1", &[], Some(Value::Text("counter".into())));
    graph.output = Some(graph.add("sequence", &[assign, get], None));
    let program = compile(&graph, 7).unwrap();
    let mut pixel_state = EvaluationState::default();
    for initial in [3.0, 10.0, -2.0] {
        let variables = BTreeMap::from([("counter".into(), Value::Float(initial)), ("untouched".into(), Value::Text("keep".into()))]);
        let context = EvaluationContext {
            image_formats: &[],
            scope: Default::default(),
            position: [0.5; 2],
            size: [1.0; 2],
            images: &[],
            tile: true,
            variables: &variables,
        };
        let mut exported = EvaluationState::default();
        let expected = program.evaluate(&context, &mut exported).unwrap();
        for _ in 0..2 {
            assert_eq!(program.evaluate_pixel(&context, &mut pixel_state).unwrap(), expected);
        }
        assert_eq!(expected, Value::Float(initial + 1.0));
        assert_eq!(variables["counter"], Value::Float(initial));
        assert_eq!(exported.variables()["counter"], expected);
        assert_eq!(exported.variables()["untouched"], variables["untouched"]);
    }

    let read_only = compile(&FunctionGraph::constant(Value::Float(0.5)), 7).unwrap();
    let context = EvaluationContext {
        image_formats: &[],
        scope: Default::default(),
        position: [0.5; 2],
        size: [1.0; 2],
        images: &[],
        tile: true,
        variables: &BTreeMap::new(),
    };

    assert_eq!(read_only.evaluate_pixel(&context, &mut pixel_state).unwrap(), Value::Float(0.5));
    assert!(pixel_state.variables().is_empty());
}

#[test]
fn pixel_random_seed_and_defined_variables_match_general_evaluation() {
    let mut graph = FunctionGraph::default();
    let maximum = graph.add_constant(Value::Float(1.0));
    let random = graph.add("rand", &[maximum], None);
    let defined = graph.add("isdefined", &[], Some(Value::Text("known".into())));
    let zero = graph.add_constant(Value::Float(0.0));
    graph.output = Some(graph.add("ifelse", &[defined, random, zero], None));
    let program = compile(&graph, 7).unwrap();
    let mut state = EvaluationState::default();
    for variables in [BTreeMap::from([("$randomseed".into(), Value::Int(42)), ("known".into(), Value::Bool(true))]), BTreeMap::new()] {
        let context = EvaluationContext {
            image_formats: &[],
            scope: Default::default(),
            position: [0.5; 2],
            size: [1.0; 2],
            images: &[],
            tile: true,
            variables: &variables,
        };
        let expected = program.evaluate(&context, &mut EvaluationState::default()).unwrap();

        assert_eq!(program.evaluate_pixel(&context, &mut state).unwrap(), expected);
    }
}

#[test]
fn function_calls_consume_the_callers_random_stream() {
    let mut random = FunctionGraph::default();
    let one = random.add_constant(Value::Float(1.0));
    let first = random.add("rand", &[one], None);
    random.output = Some(first);
    let source = std::sync::Arc::new(random.clone());
    let second = random.add("rand", &[one], None);
    random.output = Some(random.add("vector2", &[first, second], None));

    let mut caller = FunctionGraph::default();
    let mut calls = Vec::new();
    for _ in 0..2 {
        calls.push(caller.add("instance", &[], None));
        let node = caller.nodes.last_mut().unwrap();
        node.instance = Some(materialcraft_core::GraphInstance {
            label: String::new(),
            package: String::new(),
            graph: "random".into(),
            inputs: Vec::new(),
            outputs: Vec::new(),
            defaults: BTreeMap::new(),
        });
        node.source = Some(Ok(source.clone()));
    }
    caller.output = Some(caller.add("vector2", &calls, None));
    let expected = evaluate(&random, &BTreeMap::new()).unwrap();
    let Value::Float2(values) = expected else { panic!("expected two random values") };

    assert_ne!(values[0], values[1]);
    assert_eq!(evaluate(&caller, &BTreeMap::new()).unwrap(), expected);
}

#[test]
fn shared_conditions_keep_the_value_computed_before_a_sequence_assignment() {
    let mut graph = FunctionGraph::default();
    let get = graph.add("get_integer1", &[], Some(Value::Text("counter".into())));
    let one = graph.add_constant(Value::Int(1));
    let condition = graph.add("lr", &[get, one], None);
    let assign = graph.add("set", &[one], Some(Value::Text("counter".into())));
    let after = graph.add("sequence", &[assign, condition], None);
    graph.output = Some(graph.add("sequence", &[condition, after], None));

    assert_eq!(evaluate(&graph, &BTreeMap::from([("counter".into(), Value::Int(0))])).unwrap(), Value::Bool(true));
}

#[test]
fn shared_assignment_nodes_run_once_per_function_evaluation() {
    let mut graph = FunctionGraph::default();
    let current = graph.add("get_integer1", &[], Some(Value::Text("counter".into())));
    let one = graph.add_constant(Value::Int(1));
    let next = graph.add("add", &[current, one], None);
    let assign = graph.add("set", &[next], Some(Value::Text("counter".into())));
    graph.output = Some(graph.add("ivector2", &[assign, assign], None));

    assert_eq!(evaluate(&graph, &BTreeMap::from([("counter".into(), Value::Int(0))])).unwrap(), Value::Int2([1, 1]));
}

#[test]
fn pixel_random_samples_vary_spatially_and_repeat_for_the_same_seed() {
    let mut graph = FunctionGraph::default();
    let one = graph.add_constant(Value::Float(1.0));
    graph.output = Some(graph.add("rand", &[one], None));
    let program = compile(&graph, 7).unwrap();
    let mut state = EvaluationState::default();
    let variables = BTreeMap::new();
    let samples = |state: &mut EvaluationState| {
        (0..16)
            .map(|x| {
                let context = EvaluationContext {
                    image_formats: &[],
                    scope: super::FunctionScope::PixelProcessor,
                    position: [(x as f32 + 0.5) / 16.0, 0.5],
                    size: [16.0; 2],
                    images: &[],
                    tile: true,
                    variables: &variables,
                };

                let value = program.evaluate_pixel(&context, state).unwrap();

                assert_eq!(value, program.evaluate(&context, &mut EvaluationState::default()).unwrap());

                value
            })
            .collect::<Vec<_>>()
    };

    let first = samples(&mut state);

    assert!(first.windows(2).all(|pair| pair[0] != pair[1]));
    assert_eq!(first, samples(&mut state));
}

#[test]
fn exposed_choices_are_integers_inside_functions() {
    let mut graph = FunctionGraph::default();
    let choice = graph.add("get_integer1", &[], Some(Value::Text("choice".into())));
    let one = graph.add_constant(Value::Int(1));
    graph.output = Some(graph.add("lr", &[choice, one], None));

    assert_eq!(evaluate(&graph, &BTreeMap::from([("choice".into(), Value::Enum(0))])).unwrap(), Value::Bool(true));

    graph.output = Some(graph.add("add", &[choice, one], None));

    assert_eq!(evaluate(&graph, &BTreeMap::from([("choice".into(), Value::Enum(2))])).unwrap(), Value::Int(3));
    assert!(evaluate(&graph, &BTreeMap::from([("choice".into(), Value::Enum(u32::MAX))])).is_err());
}

#[test]
fn nested_function_assignments_escape_but_arguments_stay_local() {
    let mut inner = FunctionGraph::default();
    let argument = inner.add("get_integer1", &[], Some(Value::Text("argument".into())));
    inner.output = Some(inner.add("set", &[argument], Some(Value::Text("exported".into()))));
    let mut caller = FunctionGraph::default();
    let call = caller.add("instance", &[], None);
    let node = caller.nodes.last_mut().unwrap();
    node.instance = Some(materialcraft_core::GraphInstance {
        label: String::new(),
        package: String::new(),
        graph: "assign".into(),
        inputs: Vec::new(),
        outputs: Vec::new(),
        defaults: BTreeMap::from([("argument".into(), Value::Int(42))]),
    });
    node.source = Some(Ok(std::sync::Arc::new(inner)));
    let exported = caller.add("get_integer1", &[], Some(Value::Text("exported".into())));
    let argument = caller.add("get_integer1", &[], Some(Value::Text("argument".into())));
    let pair = caller.add("ivector2", &[exported, argument], None);
    caller.output = Some(caller.add("sequence", &[call, pair], None));

    assert_eq!(evaluate(&caller, &BTreeMap::from([("argument".into(), Value::Int(7))])).unwrap(), Value::Int2([42, 7]));
}

#[test]
fn one_random_node_reuses_its_sample_across_vector_connections() {
    let mut graph = FunctionGraph::default();
    let one = graph.add_constant(Value::Float(1.0));
    let random = graph.add("rand", &[one], None);
    let pair = graph.add("vector2", &[random, random], None);
    graph.output = Some(graph.add("vector4", &[pair, pair], None));
    let Value::Float4(channels) = evaluate(&graph, &BTreeMap::new()).unwrap() else {
        panic!("expected color");
    };

    assert!(channels.iter().all(|value| *value == channels[0]), "{channels:?}");
}

#[test]
fn loops_refresh_random_samples_but_keep_each_iterations_fanout_equal() {
    let mut samples = FunctionGraph::default();
    let one = samples.add_constant(Value::Float(1.0));
    let draws: Vec<_> = (0..3).map(|_| samples.add("rand", &[one], None)).collect();
    let pair = samples.add("vector2", &draws[..2], None);
    samples.output = Some(samples.add("vector3", &[pair, draws[2]], None));
    let Value::Float3(expected) = evaluate(&samples, &BTreeMap::new()).unwrap() else {
        panic!("expected samples");
    };

    assert_ne!(expected[0], expected[2]);

    let mut graph = FunctionGraph::default();
    let zero = graph.add_constant(Value::Int(0));
    let one_int = graph.add_constant(Value::Int(1));
    let three = graph.add_constant(Value::Int(3));
    let one = graph.add_constant(Value::Float(1.0));
    let zero_pair = graph.add_constant(Value::Float2([0.0; 2]));
    let init = graph.add("set", &[zero], Some(Value::Text("counter".into())));
    let init = graph.add("sequence", &[init, zero_pair], None);
    let counter = graph.add("get_integer1", &[], Some(Value::Text("counter".into())));
    let exit = graph.add("gteq", &[counter, three], None);
    let next = graph.add("add", &[counter, one_int], None);
    let advance = graph.add("set", &[next], Some(Value::Text("counter".into())));
    let random = graph.add("rand", &[one], None);
    let pair = graph.add("vector2", &[random, random], None);
    let body = graph.add("sequence", &[advance, pair], None);
    graph.output = Some(graph.add("while", &[init, exit, body], Some(Value::Int(3))));

    assert_eq!(evaluate(&graph, &BTreeMap::new()).unwrap(), Value::Float2([expected[2]; 2]));
}

#[test]
fn a_node_runs_once_so_readers_before_and_after_a_set_share_its_value() {
    let mut graph = FunctionGraph::default();
    let read = graph.add("get_integer1", &[], Some(Value::Text("count".into())));
    let one = graph.add_constant(Value::Int(1));
    let next = graph.add("add", &[read, one], None);
    let store = graph.add("set", &[next], Some(Value::Text("count".into())));
    graph.output = Some(graph.add("sequence", &[store, read], None));

    let variables = BTreeMap::from([("count".to_string(), Value::Int(4))]);

    assert_eq!(evaluate(&graph, &variables).unwrap(), Value::Int(4), "the Get node read 4 before the Set and keeps it");

    // A second Get node reads the variable when it runs, after the Set
    let later = graph.add("get_integer1", &[], Some(Value::Text("count".into())));
    graph.output = Some(graph.add("sequence", &[store, later], None));

    assert_eq!(evaluate(&graph, &variables).unwrap(), Value::Int(5));
}

#[test]
fn a_loop_evaluates_its_condition_and_body_again_on_every_iteration() {
    let mut graph = FunctionGraph::default();
    let zero = graph.add_constant(Value::Int(0));
    let start = graph.add("set", &[zero], Some(Value::Text("count".into())));
    let read = graph.add("get_integer1", &[], Some(Value::Text("count".into())));
    let limit = graph.add_constant(Value::Int(3));
    let reached = graph.add("gteq", &[read, limit], None);
    let one = graph.add_constant(Value::Int(1));
    let next = graph.add("add", &[read, one], None);
    let step = graph.add("set", &[next], Some(Value::Text("count".into())));
    graph.output = Some(graph.add("while", &[start, reached, step], Some(Value::Int(100))));

    assert_eq!(evaluate(&graph, &BTreeMap::new()).unwrap(), Value::Int(3));
}

#[test]
fn variables_a_called_graph_sets_reach_its_caller_but_its_inputs_do_not() {
    let mut called = FunctionGraph::default();
    let input = called.add("get_integer1", &[], Some(Value::Text("amount".into())));
    called.output = Some(called.add("set", &[input], Some(Value::Text("shared".into()))));

    let mut caller = FunctionGraph::default();
    let call = caller.add("instance", &[], None);
    let node = caller.nodes.last_mut().unwrap();
    node.instance = Some(materialcraft_core::GraphInstance {
        label: String::new(),
        package: String::new(),
        graph: "called".into(),
        inputs: vec![materialcraft_core::GraphPort { identifier: "amount".into(), type_code: 16 }],
        outputs: vec![materialcraft_core::GraphPort { identifier: "output".into(), type_code: 16 }],
        defaults: BTreeMap::from([("amount".to_string(), Value::Int(9))]),
    });
    node.inputs = vec![None];
    node.source = Some(Ok(std::sync::Arc::new(called)));
    let shared = caller.add("get_integer1", &[], Some(Value::Text("shared".into())));
    caller.output = Some(caller.add("sequence", &[call, shared], None));

    assert_eq!(evaluate(&caller, &BTreeMap::new()).unwrap(), Value::Int(9));

    let leaked = caller.add("get_integer1", &[], Some(Value::Text("amount".into())));
    caller.output = Some(caller.add("sequence", &[call, leaked], None));

    assert_eq!(evaluate(&caller, &BTreeMap::new()).unwrap(), Value::Int(0));
}

#[test]
fn a_drop_down_parameter_reads_as_the_integer_of_its_option() {
    let mut graph = FunctionGraph::default();
    let option = graph.add("get_integer1", &[], Some(Value::Text("mode".into())));
    let two = graph.add_constant(Value::Int(2));
    graph.output = Some(graph.add("gteq", &[option, two], None));

    assert_eq!(evaluate(&graph, &BTreeMap::from([("mode".to_string(), Value::Enum(3))])).unwrap(), Value::Bool(true));
}

#[test]
fn an_unconnected_input_computes_with_zero_and_a_loop_without_a_condition_runs_its_limit() {
    let mut graph = FunctionGraph::default();
    let high = graph.add_constant(Value::Float(8.0));
    let half = graph.add_constant(Value::Float(0.5));
    graph.output = Some(graph.add("lerp", &[high, high, half], None));
    graph.nodes.last_mut().unwrap().inputs = vec![None, Some(high), Some(half)];

    assert_eq!(evaluate(&graph, &BTreeMap::new()).unwrap(), Value::Float(4.0), "halfway between zero and 8");

    let mut graph = FunctionGraph::default();
    let zero = graph.add_constant(Value::Int(0));
    let start = graph.add("set", &[zero], Some(Value::Text("count".into())));
    let read = graph.add("get_integer1", &[], Some(Value::Text("count".into())));
    let one = graph.add_constant(Value::Int(1));
    let next = graph.add("add", &[read, one], None);
    let step = graph.add("set", &[next], Some(Value::Text("count".into())));
    graph.output = Some(graph.add("while", &[start, start, step], Some(Value::Int(3))));
    graph.nodes.last_mut().unwrap().inputs = vec![Some(start), None, Some(step)];

    assert_eq!(evaluate(&graph, &BTreeMap::new()).unwrap(), Value::Int(3));
}

#[test]
fn booleans_compare_in_order() {
    assert_eq!(operation("gt", &[Value::Bool(true), Value::Bool(false)], None).unwrap(), Value::Bool(true));
    assert_eq!(operation("lreq", &[Value::Bool(true), Value::Bool(false)], None).unwrap(), Value::Bool(false));
}

#[test]
fn every_pixel_of_a_processor_draws_its_own_random_numbers() {
    let mut function = FunctionGraph::default();
    let range = function.add_constant(Value::Float(1.0));
    function.output = Some(function.add("rand", &[range], None));
    let definition = crate::find("pixel_processor").unwrap();
    let parameters = crate::resolve_params(definition, &BTreeMap::from([("function".into(), Value::Function(Box::new(function)))]));
    let variables = BTreeMap::new();
    let context = crate::CookContext {
        input_formats: &[],
        format: materialcraft_core::OutputFormat::F32,
        w: 8,
        h: 8,
        seed: 3,
        tile: true,
        inputs: &[],
        params: &parameters,
        resource: &|_| None,
        vars: &variables,
    };

    let image = (definition.cook)(&context).unwrap().remove(0);

    let first = image.get(0, 0, false)[0];
    let different = (0..8).flat_map(|y| (0..8).map(move |x| (x, y))).filter(|(x, y)| (image.get(*x, *y, false)[0] - first).abs() > 1e-4).count();

    assert!(different > 56, "{different} of 64 pixels differ from the first");
}

#[test]
fn unconnected_ifelse_branches_use_the_value_type_not_the_condition() {
    for (value, zero, type_code) in
        [(Value::Float(8.0), Value::Float(0.0), 256), (Value::Int(8), Value::Int(0), 16), (Value::Float2([8.0; 2]), Value::Float2([0.0; 2]), 512)]
    {
        for declared_type in [0, type_code] {
            for missing in [1, 2] {
                for condition_value in [false, true] {
                    let mut graph = FunctionGraph::default();
                    let condition = graph.add_constant(Value::Bool(condition_value));
                    let branch = graph.add_constant(value.clone());
                    graph.output = Some(graph.add("ifelse", &[condition, branch, branch], None));
                    let node = graph.nodes.last_mut().unwrap();
                    node.inputs[missing] = None;
                    node.type_code = declared_type;
                    let selected = if condition_value { 1 } else { 2 };
                    let expected = if selected == missing { zero.clone() } else { value.clone() };

                    assert_eq!(evaluate(&graph, &BTreeMap::new()).unwrap(), expected);
                }
            }
        }
    }
}

#[test]
fn unconnected_ifelse_condition_stays_boolean_and_empty_branches_keep_declared_type() {
    let mut graph = FunctionGraph::default();
    let condition = graph.add_constant(Value::Bool(true));
    let branch = graph.add_constant(Value::Float(8.0));
    graph.output = Some(graph.add("ifelse", &[condition, branch, branch], None));
    let node = graph.nodes.last_mut().unwrap();
    node.inputs = vec![None, Some(branch), None];
    node.type_code = 0;

    assert_eq!(evaluate(&graph, &BTreeMap::new()).unwrap(), Value::Float(0.0));

    for condition_value in [false, true] {
        let mut graph = FunctionGraph::default();
        let condition = graph.add_constant(Value::Bool(condition_value));
        graph.output = Some(graph.add("ifelse", &[condition, condition, condition], None));
        let node = graph.nodes.last_mut().unwrap();
        node.inputs = vec![Some(condition), None, None];
        node.type_code = 512;

        assert_eq!(evaluate(&graph, &BTreeMap::new()).unwrap(), Value::Float2([0.0; 2]));
    }
}
