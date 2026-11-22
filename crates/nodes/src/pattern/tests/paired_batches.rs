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

fn pair_copy_counts(pair: u32) -> [u32; 64] {
    std::array::from_fn(|index| {
        let group = index as u32;
        match (pair, group) {
            (60, 60) => 81,
            (60, 61) => 100,
            (60, 0..48) => 40 + (26 * group + 63) % 83,
            (60, _) => 45 + (9 * group + 8) % 37,
            (_, g) if g == pair => 75,
            (_, g) if g == pair + 1 => 106,
            (_, 0..48) => 27 + (30 * group + 64) % 67,
            _ => 45 + (15 * group + 11) % 37,
        }
    })
}

fn stamp_field(group_counts: &[u32; 64], markers: &[(u32, u32, bool)]) -> Image {
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
        let extras = group_counts[group as usize];
        let quota = extras / 4 + u32::from((subtree % 4 + group) % 4 < extras % 4);
        let x = if draw % 1344 < quota { 0.0 } else { 64.5 / 128.0 };
        [x, 112.5 / 128.0, 0.25, 1.0]
    })
}

fn check_batch_markers(group_counts: &[u32; 64], seed: u32, axes: i32, markers: &[(u32, u32, bool)]) {
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
    let variables = BTreeMap::from([("$tiling".into(), Value::Int(axes))]);
    let inputs = [None, Some(stamp_field(group_counts, markers))];
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
        assert_eq!(image.get_gray(16 + 2 * j as i64, 24, false), expected, "seed {seed}, marker {j}");
    }
}

#[test]
fn full_pairs_defer_individual_copies_after_two_batches() {
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
    // With 96 extra copies in every group, the last twelve copies of group 60 draw later.
    let all_pairs = [
        (327923, 88021, true),
        (327924, 91851, true),
        (327925, 91651, true),
        (327935, 93255, true),
        (344042, 259673, true),
        (279529, 324712, false),
        (85920, 267859, false),
        (42912, 267222, false),
        (10643, 214976, false),
        (10644, 91692, false),
    ];
    let preceding_subtrees = [
        (274168, 129135, true),
        (274169, 326233, false),
        (274170, 323621, false),
        (274175, 327236, false),
        (295667, 219008, true),
        (295668, 172042, true),
        (295669, 172046, true),
        (295679, 172049, true),
        (317174, 47924, true),
        (317175, 24, true),
        (317176, 9, true),
        (317183, 34, true),
        (338676, 269607, true),
        (338677, 258061, true),
        (338678, 258081, true),
        (338687, 258057, true),
    ];
    let mut crossing = [32; 64];
    for (group, count) in [(50, 91), (54, 96), (58, 93), (62, 95)] {
        crossing[group] = count;
    }
    let second_pair_markers = [
        (279537, 138231, true),
        (279583, 94160, true),
        (87397, 15706, true),
        (90047, 978, true),
        (92773, 303597, false),
        (95423, 213678, false),
        (95461, 94418, true),
    ];
    let mut second_pair = [32; 64];
    second_pair[50] = 75;
    second_pair[51] = 107;
    let penultimate_prefix_markers = [
        (327930, 327964, false),
        (327931, 328139, false),
        (327934, 327958, false),
        (327935, 328054, false),
        (327967, 263018, true),
        (327973, 280124, true),
        (329279, 332822, false),
        (329317, 331280, false),
        (330623, 331042, false),
        (330661, 311138, true),
    ];
    let mut penultimate_prefix = [32; 64];
    penultimate_prefix[60] = 90;
    let combined_prefixes_markers = [
        (279537, 144436, true),
        (279583, 328198, false),
        (327930, 92890, true),
        (327931, 93598, true),
        (327934, 92755, true),
        (327935, 93864, true),
        (327967, 262553, true),
        (87397, 7121, true),
        (90047, 33270, true),
        (92773, 173881, false),
    ];
    let mut combined_prefixes = [32; 64];
    combined_prefixes[50] = 75;
    combined_prefixes[51] = 107;
    combined_prefixes[60] = 90;
    let fourth_pair_markers = [
        (301041, 217929, true),
        (301087, 180528, true),
        (173413, 18515, true),
        (176063, 11958, true),
        (178789, 262684, false),
        (181439, 265280, false),
        (181477, 179464, true),
    ];
    let mut fourth_pair = [32; 64];
    fourth_pair[54] = 75;
    fourth_pair[55] = 107;
    for (group_counts, seed, axes, markers) in [
        (pair_copy_counts(58), 653180, 1, pair58.as_slice()),
        (pair_copy_counts(60), 1763066, 1, pair60.as_slice()),
        (pair_copy_counts(62), 653180, 1, pair62.as_slice()),
        ([96; 64], 206407, 3, all_pairs.as_slice()),
        (crossing, 214261, 1, preceding_subtrees.as_slice()),
        (fourth_pair, 222453, 1, fourth_pair_markers.as_slice()),
        (second_pair, 220419, 1, second_pair_markers.as_slice()),
        (penultimate_prefix, 220472, 1, penultimate_prefix_markers.as_slice()),
        (combined_prefixes, 220525, 1, combined_prefixes_markers.as_slice()),
    ] {
        check_batch_markers(&group_counts, seed, axes, markers);
    }
}

#[test]
fn seven_batches_can_end_in_the_preceding_quadrant() {
    for (first, seed, markers) in [
        (8, 225641, [(80617, 251488, false), (80618, 313557, false), (80619, 2103, true), (80620, 208828, false), (80639, 321776, false), (80761, 2703, true)]),
        (
            24,
            225664,
            [(166633, 271610, false), (166634, 336460, false), (166635, 127058, true), (166636, 321100, false), (166655, 3532, true), (166777, 3002, true)],
        ),
        (
            40,
            225687,
            [(252649, 290337, false), (252650, 74284, true), (252651, 214430, true), (252652, 2247, true), (252671, 87708, true), (252793, 343869, false)],
        ),
    ] {
        let mut group_counts = [32; 64];
        group_counts[first..first + 7].fill(87);
        check_batch_markers(&group_counts, seed, 1, &markers);
    }
}

#[test]
fn deferred_copies_can_fill_the_receiving_quadrant() {
    let mut group_counts = [32; 64];
    group_counts[32] = 116;
    group_counts[54] = 90;
    group_counts[55] = 80;
    let markers = [
        (301065, 265134, true),
        (301220, 181946, true),
        (177441, 189607, false),
        (182783, 207671, false),
        (182817, 319146, false),
        (185471, 208185, false),
        (209663, 185762, true),
        (306431, 308842, false),
        (306465, 185374, true),
    ];
    check_batch_markers(&group_counts, 225301, 1, &markers);
}
