use materialcraft_core::{PatternNode, PatternNodeKind};

use super::{
    pattern_stamp::{DrawPosition, DrawRun, PatternStamp},
    raster::MIN_STAMP_SIZE,
    tile_runs::TileCopies,
};
use crate::CookContext;

// Each pair is the exclusive end in depth-seven parent visits and its compositing position.
// The boundaries come from overlapping original squares. A prefix can end inside a grandchild.
const TWO_CHILDREN: &[(usize, usize)] = &[
    (1308, 8),
    (1536, 15),
    (1792, 14),
    (2048, 16),
    (3868, 2),
    (4096, 25),
    (5404, 4),
    (5632, 9),
    (5888, 18),
    (6144, 10),
    (7964, 0),
    (8192, 23),
    (9500, 6),
    (9728, 12),
    (9984, 11),
    (10240, 13),
    (12060, 1),
    (12288, 21),
    (12800, 20),
    (13312, 3),
    (13824, 24),
    (14336, 5),
    (14848, 22),
    (15360, 7),
    (15872, 17),
    (16384, 19),
];
// Wrapping both children of every sixteenth parent moves the split into the preceding
// subtree. Paired interior squares also exchange order between whole subtrees.
const TWO_CHILDREN_WRAPPED: &[(usize, usize)] = &[
    (1235, 6),
    (1280, 14),
    (1536, 11),
    (1792, 24),
    (2048, 15),
    (3795, 2),
    (4096, 27),
    (5331, 4),
    (5376, 9),
    (5632, 18),
    (5888, 21),
    (6144, 10),
    (7891, 0),
    (8192, 25),
    (9427, 8),
    (9472, 12),
    (9728, 16),
    (9984, 28),
    (10240, 13),
    (11987, 1),
    (12288, 22),
    (12800, 20),
    (13312, 3),
    (13824, 26),
    (14336, 7),
    (14848, 23),
    (15360, 5),
    (15872, 17),
    (16384, 19),
];
const FOUR_CHILDREN: &[(usize, usize)] = &[
    (580, 6),
    (1024, 19),
    (1416, 15),
    (1536, 24),
    (2048, 20),
    (3140, 2),
    (4096, 9),
    (4676, 4),
    (5120, 13),
    (5376, 29),
    (5444, 18),
    (5632, 31),
    (5768, 10),
    (5888, 34),
    (6144, 14),
    (7236, 0),
    (8192, 11),
    (8772, 8),
    (9216, 25),
    (9352, 21),
    (9728, 27),
    (9984, 30),
    (10240, 26),
    (11332, 1),
    (12288, 22),
    (12800, 17),
    (13312, 3),
    (13448, 12),
    (13824, 33),
    (14336, 7),
    (14472, 23),
    (14848, 32),
    (15360, 5),
    (15872, 28),
    (16384, 16),
];

#[derive(Default)]
pub(super) struct MixedDepthRuns {
    layout: &'static [(usize, usize)],
    starts: Vec<DrawPosition>,
    next_boundary: usize,
    parents: usize,
    children_per_parent: usize,
    draws: usize,
    shape: Option<i32>,
    copies: Option<TileCopies>,
}

impl MixedDepthRuns {
    pub(super) fn new(context: &CookContext) -> Option<Self> {
        if context.b("cull_outside") {
            return None;
        }
        let copies = match context.tiling_axes() {
            [false, false] => None,
            [true, true] => Some(TileCopies::new(context)?),
            _ => return None,
        };
        Some(Self { copies, ..Self::default() })
    }

    pub(super) fn visit(&mut self, node: &PatternNode, depth: u32, retained: usize) -> bool {
        if node.kind != PatternNodeKind::Quadrant {
            return false;
        }
        if depth != 7 {
            return match depth {
                0..=6 => node.children.iter().flatten().count() == 4,
                8 => node.children.iter().all(Option::is_none),
                _ => false,
            };
        }
        let children = node.children.iter().flatten().count();
        if self.layout.is_empty() {
            self.layout = match children {
                2 => TWO_CHILDREN,
                4 => FOUR_CHILDREN,
                _ => return false,
            };
            self.children_per_parent = children;
            self.starts.reserve_exact(self.layout.len() + 1);
        }
        if children != self.children_per_parent {
            return false;
        }
        if self.parents == self.next_boundary {
            let Some(&(end, _)) = self.layout.get(self.starts.len()) else { return false };
            self.starts.push(DrawPosition { stamp: retained, copy: 0 });
            self.next_boundary = end;
        }
        self.parents += 1;
        true
    }

    pub(super) fn draw(&mut self, stamp: &PatternStamp, opacities: &[f32], retained: bool) -> bool {
        let Some(weight) = opacities.get(stamp.depth as usize) else { return false };
        // Turning off unrelated draws changes the order even where those draws never overlap the readout.
        if !(7..=8).contains(&stamp.depth)
            || stamp.blend != 0
            || !(2..=16).contains(&stamp.pattern)
            || *self.shape.get_or_insert(stamp.pattern) != stamp.pattern
            || !(stamp.color[3] * weight).is_normal()
            || stamp.size.iter().any(|axis| axis.abs() < MIN_STAMP_SIZE)
        {
            return false;
        }
        self.draws += 1;
        self.copies.as_mut().is_none_or(|copies| copies.record(stamp, retained, self.draws, 1 + self.children_per_parent).is_some())
    }

    pub(super) fn finish(mut self, stamps: &[PatternStamp]) -> Option<Vec<DrawRun<'_>>> {
        if self.parents != 16384 || self.draws != self.parents * (1 + self.children_per_parent) || self.starts.len() != self.layout.len() {
            return None;
        }
        self.starts.push(DrawPosition { stamp: stamps.len(), copy: 0 });
        if let Some(copies) = self.copies {
            self.starts = if let Some(starts) = copies.boundaries(self.layout, 0) {
                starts
            } else if self.children_per_parent == 2 {
                self.layout = TWO_CHILDREN_WRAPPED;
                // The leading three prefixes can finish in either of the last two subtrees
                // before their continuations end. The remaining cuts must stay in their subtree.
                copies.boundaries(self.layout, 3)?
            } else {
                return None;
            };
        }
        let mut groups = vec![DrawRun::whole(&[]); self.layout.len()];
        for ((_, rank), bounds) in self.layout.iter().zip(self.starts.windows(2)) {
            *groups.get_mut(*rank)? = DrawRun::between(stamps, bounds[0], bounds[1])?;
        }
        Some(groups)
    }
}
