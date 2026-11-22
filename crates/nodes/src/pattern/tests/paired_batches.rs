use super::*;
use craft_texture::{Image, Kind};
use materialcraft_core::OutputFormat;

// Three disjoint portions of a procedural input address the active quadrant levels.
fn field_property(depth: u32, offset: [f32; 2], frame: bool) -> Value {
    let mut graph = FunctionGraph::default();
    let pos = graph.add("get_float2", &[], Some(Value::Text("$pos".into())));
    let scale = graph.add_constant(Value::Float2([(1 << depth) as f32 / 1024.0; 2]));
    let scaled = graph.add("mul", &[pos, scale], None);
    let origin = graph.add_constant(Value::Float2(offset));
    let uv = graph.add("add", &[scaled, origin], None);
    let sample = graph.add("samplecol", &[uv], Some(Value::Int2([0, 0])));
    let result = if frame {
        let center = graph.add("swizzle2", &[sample], Some(Value::Int2([0, 1])));
        graph.add("sub", &[center, pos], None)
    } else {
        graph.add("swizzle1", &[sample], Some(Value::Int(2)))
    };
    graph.output = Some(result);
    Value::Function(Box::new(graph))
}

fn stamp_field(pair: u32, markers: &[(u32, u32, bool)]) -> Image {
    let tiny = f32::EPSILON * 0.25;
    let mut selected = BTreeMap::new();
    for (j, &(negative, positive, _)) in markers.iter().enumerate() {
        let x = (16.5 + 2.0 * j as f32) / 128.0;
        selected.insert(negative, [x, 24.5 / 128.0, -1.0, 1.0]);
        selected.insert(positive, [x, 24.5 / 128.0, tiny, 1.0]);
    }
    Image::generate(1024, 1024, Kind::Color, |x, y| {
        let (depth, x, y) = match (x, y) {
            (0..512, 0..512) => (9, x, y),
            (512..768, 0..256) => (8, x - 512, y),
            (0..128, 512..640) => (7, x, y - 512),
            _ => return [0.0; 4],
        };
        let mut visit = 0;
        for bit in 0..depth {
            visit |= (((x >> bit) & 1) | (((y >> bit) & 1) << 1)) << (2 * bit);
        }
        // A parent is followed by four children, each with four grandchildren.
        let draw = match depth {
            7 => visit * 21,
            8 => (visit / 4) * 21 + 1 + (visit % 4) * 5,
            _ => (visit / 16) * 21 + 2 + ((visit / 4) % 4) * 5 + visit % 4,
        };
        if let Some(marker) = selected.get(&draw) {
            return *marker;
        }
        let subtree = draw / 1344;
        let group = subtree / 4;
        let extras = match (pair, group) {
            (60, 60) => 81,
            (60, 61) => 100,
            (60, 0..48) => 40 + (26 * group + 63) % 83,
            (60, _) => 45 + (9 * group + 8) % 37,
            (_, g) if g == pair => 75,
            (_, g) if g == pair + 1 => 106,
            (_, 0..48) => 27 + (30 * group + 64) % 67,
            _ => 45 + (15 * group + 11) % 37,
        };
        let quota = extras / 4 + u32::from((subtree % 4 + group) % 4 < extras % 4);
        let x = if draw % 1344 < quota { 0.0 } else { 64.5 / 128.0 };
        [x, 112.5 / 128.0, 0.25, 1.0]
    })
}

#[test]
fn a_full_pair_defers_individual_copies_after_two_batches() {
    let mut graph = PatternGraph::with_root(PatternNodeKind::Quadrant);
    let mut parent = graph.root.unwrap();
    for depth in 1..=9 {
        let child = graph.add(PatternNodeKind::Quadrant).unwrap();
        graph.nodes[(parent - 1) as usize].children.fill(Some(child));
        if depth >= 7 {
            let offset = match depth {
                7 => [0.0, 0.5],
                8 => [0.5, 0.0],
                _ => [0.0; 2],
            };
            let mut params = additive_square(0.25, [0.0; 2], [0.0; 2]);
            params.insert("frameoffset".into(), field_property(depth, offset, true));
            params.insert("opacity".into(), field_property(depth, offset, false));
            graph.nodes[(child - 1) as usize].parameters = params;
        }
        parent = child;
    }

    let definition = find("fx_map").unwrap();
    let params = resolve_params(
        definition,
        &BTreeMap::from([
            ("pattern".into(), Value::PatternGraph(Box::new(graph))),
            ("roughness".into(), Value::Float(0.0)),
            ("color_mode".into(), Value::Bool(false)),
            ("background".into(), Value::Color([1.0; 4])),
        ]),
    );
    let variables = BTreeMap::from([("$tiling".into(), Value::Int(1))]);
    // Each pair of numbered stamps overlaps at one pixel. The final bit is set when the negative stamp draws first.
    let pair58 = [(322546, 46225, true), (322547, 1168, true), (322548, 2154, true), (322559, 818, true), (5359, 322551, false), (10731, 11628, false)];
    let pair60 = [
        (90032, 332682, false),
        (90033, 281378, false),
        (90034, 302451, false),
        (90047, 303240, false),
        (90046, 303080, false),
        (90045, 303435, false),
        (90044, 302501, false),
        (333311, 212967, true),
        (333298, 89032, true),
    ];
    let pair62 = [
        (344050, 272241, true),
        (344051, 258965, true),
        (344052, 262618, true),
        (344063, 262962, true),
        (263423, 332391, false),
        (263418, 253025, true),
        (263407, 257852, true),
        (268779, 333188, false),
    ];
    for (group, seed, markers) in [(58, 653180, pair58.as_slice()), (60, 1763066, pair60.as_slice()), (62, 653180, pair62.as_slice())] {
        let inputs = [None, Some(stamp_field(group, markers))];
        let context = CookContext {
            w: 128,
            h: 128,
            seed,
            format: OutputFormat::F32,
            tile: true,
            inputs: &inputs,
            input_formats: &[OutputFormat::F32; 2],
            params: &params,
            resource: &|_| None,
            vars: &variables,
        };
        let image = (definition.cook)(&context).unwrap().remove(0);
        for (j, &(_, _, negative_first)) in markers.iter().enumerate() {
            let expected = if negative_first { f32::EPSILON * 0.25 } else { 0.0 };
            assert_eq!(image.get_gray(16 + 2 * j as i64, 24, false), expected, "group {group}, marker {j}");
        }
    }
}
