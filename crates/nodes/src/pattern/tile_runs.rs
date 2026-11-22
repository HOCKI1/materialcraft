use super::{footprint::Footprint, pattern_stamp::PatternStamp, region::Region};
use crate::CookContext;

const UNIT_WINDOW: [f32; 4] = [0.0, 1.0, 0.0, 1.0];
const DRAWS_PER_PARENT: usize = 5;
const PARENTS_PER_SUBTREE: usize = 256;
// Paired interior squares change order when the preceding draws fill 5,460 tile copies.
const COPIES_PER_BATCH: u32 = 5460;

pub(super) struct TileCopies {
    window: Region,
    ends: Vec<u32>,
    retained_ends: Vec<u32>,
}

impl TileCopies {
    pub(super) fn new(context: &CookContext) -> Option<Self> {
        if context.v4("render_region") != UNIT_WINDOW || context.v4("tiling_region") != UNIT_WINDOW {
            return None;
        }
        Some(Self { window: Region::new(UNIT_WINDOW).ok()?, ends: Vec::new(), retained_ends: Vec::new() })
    }

    pub(super) fn record(&mut self, stamp: &PatternStamp, retained: bool) -> Option<()> {
        let footprint = Footprint::new(stamp).ok()?;
        let center = self.window.copy_origin(stamp.center, [true; 2]);
        let [x, y] = self.window.repeats(&self.window, center, footprint.radius, [true; 2]).ok()?;
        if !(1..=2).contains(&x.count) || !(1..=2).contains(&y.count) {
            return None;
        }
        let end = self.ends.last().copied().unwrap_or(0).checked_add((x.count * y.count) as u32)?;
        self.ends.push(end);
        if retained {
            self.retained_ends.push(end);
        }
        Some(())
    }

    pub(super) fn boundaries(&self, layout: &[(usize, usize)]) -> Option<Vec<usize>> {
        let mut cuts = vec![None; layout.len() + 1];
        let mut order = vec![0; layout.len()];
        cuts[0] = Some(0_u32);
        for (run, &(parents, rank)) in layout.iter().enumerate() {
            *order.get_mut(rank)? = run;
            if parents % PARENTS_PER_SUBTREE == 0 {
                cuts[run + 1] = self.ends.get(parents * DRAWS_PER_PARENT - 1).copied();
            }
        }

        let mut drawn = 0_u32;
        for run in order {
            let start = cuts[run]?;
            let end = match cuts[run + 1] {
                Some(end) => end,
                None => {
                    let end = start.checked_add(COPIES_PER_BATCH - drawn % COPIES_PER_BATCH)?;
                    // A split inside a repeated stamp would need separate copy ranges at render time.
                    let draws = self.ends.binary_search(&end).ok()? + 1;
                    // Crossing a depth-three subtree can change the order of the later runs.
                    if draws / (DRAWS_PER_PARENT * PARENTS_PER_SUBTREE) != layout[run].0 / PARENTS_PER_SUBTREE {
                        return None;
                    }
                    cuts[run + 1] = Some(end);
                    end
                }
            };
            drawn = drawn.checked_add(end.checked_sub(start)?)?;
        }
        if self.ends.last().copied() != Some(drawn) {
            return None;
        }
        cuts.into_iter().map(|end| end.map(|cut| self.retained_ends.partition_point(|value| *value <= cut))).collect()
    }
}
