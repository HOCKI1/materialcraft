use super::{
    footprint::SampleGrid,
    pattern_stamp::{DrawRun, PatternStamp},
    raster::MIN_STAMP_SIZE,
};
use materialcraft_core::{PatternNode, PatternNodeKind};

// With draws on the last three levels, 21 consecutive stamps belong to each
// depth-six parent. Paired squares identify the order of these parent ranges
// for complete trees. Successive trees share the first submissions.
type ParentRange = (usize, usize);
const FIRST: &[ParentRange] = &[
    (1536, 1796),
    (2560, 2820),
    (512, 772),
    (3200, 3328),
    (1024, 1156),
    (3712, 3840),
    (0, 132),
    (3456, 3584),
    (2048, 2180),
    (772, 1024),
    (1408, 1416),
    (1796, 2048),
    (3328, 3336),
    (1156, 1280),
    (1472, 1536),
    (256, 328),
    (3968, 4096),
    (3072, 3200),
    (1344, 1348),
    (132, 256),
    (384, 512),
    (2304, 2312),
    (2820, 3072),
    (3584, 3592),
];
const SINGLE_TAIL: &[ParentRange] =
    &[(328, 384), (2180, 2304), (2496, 2560), (2312, 2432), (3840, 3968), (1280, 1344), (2432, 2496), (1348, 1408), (3592, 3712), (3336, 3456), (1416, 1472)];
const REPEATED: &[ParentRange] = &[
    (3336, 3456),
    (5632, 6032),
    (2180, 2304),
    (2496, 2560),
    (7552, 7624),
    (3592, 3712),
    (6656, 7056),
    (1416, 1472),
    (4608, 5072),
    (3840, 3968),
    (1280, 1344),
    (2432, 2496),
    (7936, 7940),
    (1348, 1408),
    (8064, 8192),
    (7168, 7240),
    (328, 384),
    (7296, 7424),
    (5120, 5196),
    (2312, 2432),
    (7808, 7936),
    (4096, 4160),
    (4288, 4496),
    (7624, 7680),
    (6144, 6348),
    (5196, 5248),
    (5312, 5376),
    (5568, 5632),
];
const DOUBLE_TAIL: &[ParentRange] = &[
    (4160, 4288),
    (6348, 6400),
    (6592, 6656),
    (4496, 4608),
    (6400, 6528),
    (7940, 8064),
    (5248, 5312),
    (6528, 6592),
    (7240, 7296),
    (5376, 5504),
    (7056, 7168),
    (7680, 7808),
    (6032, 6144),
    (7424, 7552),
    (5072, 5120),
    (5504, 5568),
];
const FOUR_TAIL: &[ParentRange] = &[
    (4160, 4240),
    (7240, 7296),
    (5376, 5504),
    (12160, 12236),
    (6032, 6144),
    (7424, 7552),
    (9728, 10008),
    (7056, 7168),
    (7680, 7808),
    (10752, 11032),
    (4496, 4608),
    (6400, 6528),
    (11904, 11924),
    (5072, 5120),
    (5504, 5568),
    (8704, 9112),
    (7940, 8064),
    (5248, 5312),
    (6528, 6592),
    (12032, 12040),
    (4240, 4288),
    (11392, 11520),
    (9216, 9300),
    (11924, 12032),
    (8192, 8344),
    (6348, 6400),
    (6592, 6656),
    (11648, 11776),
    (10240, 10304),
    (10432, 10644),
    (10008, 10240),
    (11520, 11548),
    (9300, 9344),
    (9408, 9472),
    (9664, 9728),
    (8448, 8536),
    (12236, 12288),
    (11264, 11392),
    (9472, 9552),
    (8344, 8448),
    (8576, 8704),
    (10304, 10332),
    (11032, 11264),
    (11776, 11804),
    (11548, 11648),
    (13824, 14244),
    (11804, 11904),
    (14848, 15268),
    (9112, 9216),
    (9600, 9664),
    (12800, 13152),
    (12040, 12160),
    (9344, 9408),
    (10368, 10432),
    (16128, 16140),
    (9552, 9600),
    (16256, 16384),
    (15360, 15444),
    (8536, 8576),
    (15488, 15616),
    (13312, 13404),
    (10332, 10368),
    (16000, 16128),
    (12288, 12416),
    (12480, 12708),
    (10644, 10752),
    (15744, 15872),
    (14336, 14400),
    (14528, 14748),
    (13404, 13440),
    (13504, 13568),
    (13760, 13824),
    (12416, 12480),
    (14748, 14848),
    (12708, 12800),
    (14400, 14464),
    (16140, 16256),
    (13440, 13504),
    (14464, 14528),
    (15444, 15488),
    (13568, 13696),
    (15268, 15360),
    (15872, 16000),
    (14244, 14336),
    (15616, 15744),
    (13152, 13312),
    (13696, 13760),
];

#[derive(Default)]
pub(super) struct TerminalQuadrants {
    starts: Vec<usize>,
    draws: usize,
    shape: Option<i32>,
}

impl TerminalQuadrants {
    pub(super) fn visit(&mut self, node: &PatternNode, depth: u32, retained: usize, at_root: bool) -> bool {
        match node.kind {
            PatternNodeKind::Iterate => at_root && node.children.first() == Some(&None),
            PatternNodeKind::Switch => depth == 0 && node.children.contains(&None),
            PatternNodeKind::Quadrant if depth == 8 => node.children.iter().all(Option::is_none),
            PatternNodeKind::Quadrant if depth < 8 && node.children.iter().flatten().count() == 4 => {
                if depth == 6 {
                    if self.starts.len() == 4 * 4096 {
                        return false;
                    }
                    // Invisible draws still occupy a place in their parent's run.
                    self.starts.push(retained);
                }
                true
            }
            _ => false,
        }
    }

    pub(super) fn draw(&mut self, stamp: &PatternStamp, weights: &[f32], grid: Option<&SampleGrid>) -> bool {
        let Some(weight) = weights.get(stamp.depth as usize) else { return false };
        if !(6..=8).contains(&stamp.depth)
            || stamp.blend != 0
            || !(2..=16).contains(&stamp.pattern)
            || *self.shape.get_or_insert(stamp.pattern) != stamp.pattern
            || !(weight * stamp.color[3]).is_normal()
            || stamp.size.iter().any(|axis| axis.abs() < MIN_STAMP_SIZE)
        {
            return false;
        }
        self.draws += 1;
        // Tiling changes these boundaries only when it actually emits a different number of copies.
        grid.is_some_and(|grid| grid.single_copy(stamp))
    }

    pub(super) fn finish(mut self, stamps: &[PatternStamp], repeats: usize) -> Option<Vec<DrawRun<'_>>> {
        let (middle, tail) = match repeats {
            1 => (&[][..], SINGLE_TAIL),
            2 => (REPEATED, DOUBLE_TAIL),
            4 => (REPEATED, FOUR_TAIL),
            _ => return None,
        };
        if self.starts.len() != repeats * 4096 || self.draws != self.starts.len() * 21 {
            return None;
        }
        self.starts.push(stamps.len());
        let mut groups = Vec::with_capacity(FIRST.len() + middle.len() + tail.len());
        for &(start, end) in FIRST.iter().chain(middle).chain(tail) {
            let stamps = stamps.get(*self.starts.get(start)?..*self.starts.get(end)?)?;
            groups.push(DrawRun::whole(stamps));
        }
        Some(groups)
    }
}
