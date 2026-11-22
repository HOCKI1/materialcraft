use super::paired_batches::field_property;
use super::*;
use craft_texture::{Image, Kind};
use materialcraft_core::OutputFormat;

// Each depth-six parent draws before four children, each followed by four leaves.
// Address the three levels in separate columns and successive iterations in separate rows.
fn location(draw: u32) -> (u32, u32) {
    let iteration = draw / 86_016;
    let parent = draw % 86_016 / 21;
    let within = draw % 21;
    let (depth, visit, column) = match within {
        0 => (6, parent, 384),
        n if (n - 1) % 5 == 0 => (7, parent * 4 + (n - 1) / 5, 256),
        n => (8, parent * 16 + (n - 1) / 5 * 4 + (n - 2) % 5, 0),
    };
    let mut x = 0;
    let mut y = 0;
    for bit in 0..depth {
        x |= ((visit >> (2 * bit)) & 1) << bit;
        y |= ((visit >> (2 * bit + 1)) & 1) << bit;
    }
    (column + x, iteration * 256 + y)
}

#[test]
fn three_terminal_levels_interleave_repeated_quadrant_trees() {
    // In a white pixel a tiny positive value survives only when the negative square draws first.
    let tiny = f32::EPSILON * 0.25;
    for (count, markers) in [
        (1, [(28193, 32245, false), (52923, 80165, false), (13176, 44146, true), (9090, 37313, false), (17308, 46423, true), (6792, 15213, false)]),
        (2, [(148776, 167500, false), (108044, 154830, false), (2356, 50934, true), (80910, 98000, false), (24344, 108669, true), (29011, 120962, false)]),
        (4, [(81374, 106292, false), (261307, 262710, false), (43010, 163056, true), (224189, 266224, false), (176006, 303409, true), (52157, 146973, false)]),
    ] {
        let root = if count == 1 { PatternNodeKind::Quadrant } else { PatternNodeKind::Iterate };
        let mut graph = PatternGraph::with_root(root);
        let mut parent = graph.root.unwrap();
        if count > 1 {
            parent = graph.add(PatternNodeKind::Quadrant).unwrap();
            graph.nodes[0].parameters.insert("numberadded".into(), Value::Int(count));
            graph.nodes[0].children[1] = Some(parent);
        }
        for depth in 1..=8 {
            let child = graph.add(PatternNodeKind::Quadrant).unwrap();
            graph.nodes[(parent - 1) as usize].children.fill(Some(child));
            if depth >= 6 {
                let column = match depth {
                    6 => 384,
                    7 => 256,
                    _ => 0,
                };
                let offset = [column as f32 / 1024.0, 0.0];
                let mut properties = additive_square(0.25, [0.0; 2], [0.0; 2]);
                properties.insert("frameoffset".into(), field_property(depth, offset, true, count > 1));
                properties.insert("opacity".into(), field_property(depth, offset, false, count > 1));
                graph.nodes[(child - 1) as usize].parameters = properties;
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
        let variables = BTreeMap::from([("$tiling".into(), Value::Int(0))]);
        let mut field = BTreeMap::new();
        for (j, &(negative, positive, _)) in markers.iter().enumerate() {
            let x = (8.5 + j as f32) / 128.0;
            field.insert(location(negative), [x, 12.5 / 128.0, -1.0, 1.0]);
            field.insert(location(positive), [x, 12.5 / 128.0, tiny, 1.0]);
        }
        let input = Image::generate(1024, 1024, Kind::Color, |x, y| field.get(&(x, y)).copied().unwrap_or([0.875, 0.90625, 0.25, 1.0]));
        let inputs = [None, Some(input)];
        let context = CookContext {
            w: 128,
            h: 128,
            seed: 257831 + 19 * count as u32,
            format: OutputFormat::F32,
            tile: false,
            inputs: &inputs,
            input_formats: &[OutputFormat::F32; 2],
            params: &params,
            resource: &|_| None,
            vars: &variables,
        };
        let image = (definition.cook)(&context).unwrap().remove(0);
        for (j, &(_, _, negative_first)) in markers.iter().enumerate() {
            assert_eq!(image.get_gray(8 + j as i64, 12, false), if negative_first { tiny } else { 0.0 }, "{count} trees, marker {j}");
        }
    }
}
