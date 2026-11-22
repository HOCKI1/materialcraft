use super::{
    footprint::Footprint,
    pattern_stamp::{DrawPosition, PatternStamp},
    region::Region,
};
use crate::CookContext;
use std::ops::Range;

const UNIT_WINDOW: [f32; 4] = [0.0, 1.0, 0.0, 1.0];
const PARENTS_PER_SUBTREE: usize = 256;
// Paired interior squares change order when the preceding draws fill 5,460 tile copies.
const COPIES_PER_BATCH: u32 = 5460;

pub(super) struct TileCopies {
    window: Region,
    total: u32,
    subtree_ends: Vec<u32>,
    retained: Vec<Range<u32>>,
}

impl TileCopies {
    pub(super) fn new(context: &CookContext) -> Option<Self> {
        if context.v4("render_region") != UNIT_WINDOW || context.v4("tiling_region") != UNIT_WINDOW {
            return None;
        }
        Some(Self { window: Region::new(UNIT_WINDOW).ok()?, total: 0, subtree_ends: Vec::new(), retained: Vec::new() })
    }

    pub(super) fn record(&mut self, stamp: &PatternStamp, retained: bool, draws: usize, draws_per_parent: usize) -> Option<()> {
        let footprint = Footprint::new(stamp).ok()?;
        let center = self.window.copy_origin(stamp.center, [true; 2]);
        let [x, y] = self.window.repeats(&self.window, center, footprint.radius, [true; 2]).ok()?;
        if !(1..=2).contains(&x.count) || !(1..=2).contains(&y.count) {
            return None;
        }
        let end = self.total.checked_add((x.count * y.count) as u32)?;
        if retained {
            self.retained.push(self.total..end);
        }
        self.total = end;
        if draws.is_multiple_of(draws_per_parent * PARENTS_PER_SUBTREE) {
            self.subtree_ends.push(end);
        }
        Some(())
    }

    pub(super) fn boundaries(&self, layout: &[(usize, usize)], leading_paired_subtrees: usize) -> Option<Vec<DrawPosition>> {
        let mut cuts = vec![None; layout.len() + 1];
        let mut order = vec![0; layout.len()];
        cuts[0] = Some(0_u32);
        for (run, &(parents, rank)) in layout.iter().enumerate() {
            *order.get_mut(rank)? = run;
            if parents % PARENTS_PER_SUBTREE == 0 {
                cuts[run + 1] = self.subtree_ends.get(parents / PARENTS_PER_SUBTREE - 1).copied();
            }
        }

        let mut drawn = 0_u32;
        for run in order {
            let start = cuts[run]?;
            let end = match cuts[run + 1] {
                Some(end) => end,
                None => {
                    let end = start.checked_add(COPIES_PER_BATCH - drawn % COPIES_PER_BATCH)?;
                    // Crossing the measured subtree interval can change the order of the later runs.
                    let (parents, rank) = layout[run];
                    let first = parents / PARENTS_PER_SUBTREE;
                    let last = first + usize::from(rank < leading_paired_subtrees);
                    if !(first..=last).contains(&self.subtree_ends.partition_point(|boundary| *boundary <= end)) {
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
        cuts.into_iter()
            .map(|cut| {
                let cut = cut?;
                let stamp = self.retained.partition_point(|range| range.end <= cut);
                // Across discarded draws the next retained stamp starts at its first copy.
                let copy = self.retained.get(stamp).map_or(0, |range| cut.saturating_sub(range.start));
                Some(DrawPosition { stamp, copy })
            })
            .collect()
    }
}
