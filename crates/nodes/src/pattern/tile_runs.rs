use super::{
    footprint::Footprint,
    pattern_stamp::{DrawPosition, DrawRun, PatternStamp},
    region::Region,
};
use crate::CookContext;
use std::ops::Range;

mod two_children;

const UNIT_WINDOW: [f32; 4] = [0.0, 1.0, 0.0, 1.0];
const DEPTH_THREE_PARENTS: usize = 256;
const DEPTH_FOUR_PARENTS: usize = 64;
const PAIRED_SUBTREE_PARENTS: usize = 2 * DEPTH_THREE_PARENTS;
const FIRST_PAIR_END: usize = 48 * DEPTH_THREE_PARENTS + PAIRED_SUBTREE_PARENTS;
// Paired interior squares change order when the preceding draws fill 5,460 tile copies.
const COPIES_PER_BATCH: u32 = 5460;

pub(super) struct TileCopies {
    window: Region,
    axes: [bool; 2],
    parents_per_subtree: usize,
    total: u32,
    subtree_ends: Vec<u32>,
    retained: Vec<Range<u32>>,
}

impl TileCopies {
    pub(super) fn new(context: &CookContext, deepest: u32) -> Option<Self> {
        if context.v4("render_region") != UNIT_WINDOW || context.v4("tiling_region") != UNIT_WINDOW {
            return None;
        }
        let parents_per_subtree = if deepest == 9 { DEPTH_FOUR_PARENTS } else { DEPTH_THREE_PARENTS };
        Some(Self {
            window: Region::new(UNIT_WINDOW).ok()?,
            axes: context.tiling_axes(),
            parents_per_subtree,
            total: 0,
            subtree_ends: Vec::new(),
            retained: Vec::new(),
        })
    }

    pub(super) fn record(&mut self, stamp: &PatternStamp, retained: bool, draws: usize, draws_per_parent: usize) -> Option<()> {
        let footprint = Footprint::new(stamp).ok()?;
        let center = self.window.copy_origin(stamp.center, self.axes);
        let [x, y] = self.window.repeats(&self.window, center, footprint.radius, self.axes).ok()?;
        if !(1..=2).contains(&x.count) || !(1..=2).contains(&y.count) {
            return None;
        }
        let end = self.total.checked_add((x.count * y.count) as u32)?;
        if retained {
            self.retained.push(self.total..end);
        }
        self.total = end;
        if draws.is_multiple_of(draws_per_parent * self.parents_per_subtree) {
            let parents = draws / draws_per_parent;
            // The first pair of the final quadrant submits its parts separately.
            // In later pairs, filling two batches can reorder other subtrees.
            if self.parents_per_subtree == DEPTH_FOUR_PARENTS && parents > FIRST_PAIR_END && parents.is_multiple_of(PAIRED_SUBTREE_PARENTS) {
                let subtrees = PAIRED_SUBTREE_PARENTS / DEPTH_FOUR_PARENTS;
                let start = self.subtree_ends.iter().rev().nth(subtrees - 1)?;
                if end - start >= 2 * COPIES_PER_BATCH {
                    return None;
                }
            }
            self.subtree_ends.push(end);
        }
        Some(())
    }

    fn draw_position(&self, cut: u32) -> DrawPosition {
        let stamp = self.retained.partition_point(|range| range.end <= cut);
        // Across discarded draws the next retained stamp starts at its first copy.
        let copy = self.retained.get(stamp).map_or(0, |range| cut.saturating_sub(range.start));
        DrawPosition { stamp, copy }
    }

    pub(super) fn two_child_runs<'a>(&self, stamps: &'a [PatternStamp]) -> Option<Vec<DrawRun<'a>>> {
        let ranges = two_children::ranges(&self.subtree_ends)?;
        ranges.into_iter().map(|r| DrawRun::between(stamps, self.draw_position(r.start), self.draw_position(r.end))).collect()
    }

    pub(super) fn boundaries(&self, layout: &[(usize, usize)], draws_per_parent: usize) -> Option<Vec<DrawPosition>> {
        let mut cuts = vec![None; layout.len() + 1];
        let mut order = vec![0; layout.len()];
        cuts[0] = Some(0_u32);
        for (run, &(parents, rank)) in layout.iter().enumerate() {
            *order.get_mut(rank)? = run;
            if parents % self.parents_per_subtree == 0 {
                cuts[run + 1] = self.subtree_ends.get(parents / self.parents_per_subtree - 1).copied();
            }
        }

        let mut drawn = 0_u32;
        for run in order {
            let start = cuts[run]?;
            let end = match cuts[run + 1] {
                Some(end) => end,
                None => {
                    let (parents, _) = layout[run];
                    let first = run.checked_sub(1).map_or(0, |previous| layout[previous].0);
                    // Consecutive batches can form one run. The first can already
                    // contain copies from the previous run.
                    let length = u32::try_from((parents - first) * draws_per_parent).ok()?;
                    let capacity = length.div_ceil(COPIES_PER_BATCH).checked_mul(COPIES_PER_BATCH)?;
                    let end = start.checked_add(capacity - drawn % COPIES_PER_BATCH)?;
                    // Crossing the measured subtree interval can change the order of the later runs.
                    let subtree = self.subtree_ends.partition_point(|boundary| *boundary <= end);
                    if subtree != parents / self.parents_per_subtree {
                        return None;
                    }
                    if self.parents_per_subtree == DEPTH_FOUR_PARENTS && self.subtree_ends.get(subtree.checked_sub(1)?) == Some(&end) {
                        return None;
                    }
                    cuts[run + 1] = Some(end);
                    end
                }
            };
            drawn = drawn.checked_add(end.checked_sub(start)?)?;
        }
        if self.total != drawn {
            return None;
        }
        cuts.into_iter().map(|cut| cut.map(|cut| self.draw_position(cut))).collect()
    }
}
