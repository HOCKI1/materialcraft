use super::{
    footprint::Footprint,
    pattern_stamp::{DrawPosition, DrawRun, PatternStamp},
    region::Region,
};
use crate::CookContext;
use std::{borrow::Cow, ops::Range};

mod paired_batches;
mod sparse_quadrants;
mod two_children;

pub(super) use sparse_quadrants::SparseQuadrantRuns;

use paired_batches::PairedBatches;

const UNIT_WINDOW: [f32; 4] = [0.0, 1.0, 0.0, 1.0];
const DEPTH_THREE_PARENTS: usize = 256;
const DEPTH_FOUR_PARENTS: usize = 64;
const PAIRED_SUBTREE_PARENTS: usize = 2 * DEPTH_THREE_PARENTS;
const FIRST_PAIR_END: usize = 48 * DEPTH_THREE_PARENTS + PAIRED_SUBTREE_PARENTS;
// Paired interior squares change order when the preceding draws fill 5,460 tile copies.
pub(super) const COPIES_PER_BATCH: u32 = 5460;

pub(super) struct TileCopies {
    window: Region,
    axes: [bool; 2],
    parents_per_subtree: usize,
    total: u32,
    subtree_ends: Vec<u32>,
    retained: Vec<Range<u32>>,
    full_pairs: u8,
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
            full_pairs: 0,
        })
    }

    fn count_stamp(&mut self, stamp: Option<&PatternStamp>, retained: bool) -> Option<Range<u32>> {
        let start = self.total;
        if let Some(stamp) = stamp {
            let footprint = Footprint::new(stamp).ok()?;
            let center = self.window.copy_origin(stamp.center, self.axes);
            let [x, y] = self.window.repeats(&self.window, center, footprint.radius, self.axes).ok()?;
            if !(1..=2).contains(&x.count) || !(1..=2).contains(&y.count) {
                return None;
            }
            self.total = self.total.checked_add((x.count * y.count) as u32)?;
        }
        let range = start..self.total;
        if retained {
            self.retained.push(range.clone());
        }
        Some(range)
    }

    pub(super) fn record(&mut self, stamp: &PatternStamp, retained: bool, draws: usize, draws_per_parent: usize) -> Option<()> {
        let end = self.count_stamp(Some(stamp), retained)?.end;
        if draws.is_multiple_of(draws_per_parent * self.parents_per_subtree) {
            let parents = draws / draws_per_parent;
            // Count the eight pairs in the final quadrant. Their combined occupancy determines
            // whether to continue a single overflowing pair or all eight in submission order.
            if self.parents_per_subtree == DEPTH_FOUR_PARENTS && parents >= FIRST_PAIR_END && parents.is_multiple_of(PAIRED_SUBTREE_PARENTS) {
                let subtrees = PAIRED_SUBTREE_PARENTS / DEPTH_FOUR_PARENTS;
                let start = self.subtree_ends.iter().rev().nth(subtrees - 1)?;
                if end - start >= 2 * COPIES_PER_BATCH {
                    let pair = (parents - FIRST_PAIR_END) / PAIRED_SUBTREE_PARENTS;
                    self.full_pairs |= 1_u8.checked_shl(u32::try_from(pair).ok()?)?;
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

    fn subtree_copies(&self, subtrees: Range<usize>) -> Option<u32> {
        let start = if subtrees.start == 0 { 0 } else { *self.subtree_ends.get(subtrees.start - 1)? };
        self.subtree_ends.get(subtrees.end.checked_sub(1)?)?.checked_sub(start)
    }

    fn group_copies(&self, groups: Range<usize>) -> Option<u32> {
        let subtrees = DEPTH_THREE_PARENTS / self.parents_per_subtree;
        self.subtree_copies(groups.start * subtrees..groups.end * subtrees)
    }

    fn horizontal_layout(&self, ordinary: &'static [(usize, usize)], pair: Option<(usize, usize)>) -> Option<Cow<'static, [(usize, usize)]>> {
        let prefix = self.group_copies(60..61)?;
        let full_prefix = prefix >= COPIES_PER_BATCH;
        let fallback = || pair.is_none().then_some(Cow::Borrowed(ordinary));
        if !full_prefix && pair.is_none() {
            return fallback();
        }
        if let Some((group, head)) = pair {
            // The paired prefix defers everything after its second batch. Filling another
            // batch in the receiving subtree changes the order of the remaining draws.
            let deferred = self.group_copies(group..group + 2)?.checked_sub(2 * COPIES_PER_BATCH)?;
            if self.group_copies(head..head + 1)?.checked_add(deferred)? >= COPIES_PER_BATCH {
                if group == 50 && !full_prefix {
                    return self.full_second_quadrant();
                }
                if group == 54 && !full_prefix {
                    return Some(Cow::Borrowed(paired_batches::FULL_RECEIVING_QUADRANT));
                }
                return fallback();
            }
        }
        if full_prefix && self.group_copies(16..17)?.checked_add(prefix - COPIES_PER_BATCH)? >= COPIES_PER_BATCH {
            // The later copy from group 60 can then move across another partially filled batch.
            return fallback();
        }

        Some(Self::shift_horizontal_boundaries(ordinary, pair, full_prefix))
    }

    fn shift_horizontal_boundaries(ordinary: &'static [(usize, usize)], pair: Option<(usize, usize)>, full_prefix: bool) -> Cow<'static, [(usize, usize)]> {
        if pair.is_none() && !full_prefix {
            return Cow::Borrowed(ordinary);
        }
        let mut layout = ordinary.to_vec();
        for (end, _) in &mut layout {
            if let Some((group, head)) = pair {
                let start = head * DEPTH_THREE_PARENTS;
                if *end == start + DEPTH_FOUR_PARENTS || *end == start + 3 * DEPTH_FOUR_PARENTS {
                    *end += DEPTH_THREE_PARENTS;
                } else if ((group + 1) * DEPTH_THREE_PARENTS..(group + 2) * DEPTH_THREE_PARENTS).contains(end) {
                    // Repeated endpoints split the final subtree at the physical copy count.
                    *end = (group + 2) * DEPTH_THREE_PARENTS;
                }
            }
            let last = 61 * DEPTH_THREE_PARENTS;
            if full_prefix && last < *end && *end <= last + 3 * DEPTH_FOUR_PARENTS {
                *end = (*end - DEPTH_FOUR_PARENTS).max(last);
            }
        }
        Cow::Owned(layout)
    }

    fn full_second_quadrant(&self) -> Option<Cow<'static, [(usize, usize)]>> {
        let mut layout = Cow::Borrowed(paired_batches::FULL_SECOND_QUADRANT);
        let prefix = self.group_copies(62..63)?;
        if prefix != COPIES_PER_BATCH {
            // The first submission ends after one batch. It can fall on either side
            // of group 63, which supplies the remaining draws later in the sequence.
            let (end, _) = layout.to_mut().iter_mut().find(|(_, rank)| *rank == 0)?;
            let boundary = 63 * DEPTH_THREE_PARENTS;
            *end = if prefix < COPIES_PER_BATCH { boundary + 1 } else { boundary - 1 };
        }
        Some(layout)
    }

    fn penultimate_batch_fills(&self, prefix: u32) -> Option<bool> {
        let subtrees = DEPTH_THREE_PARENTS / DEPTH_FOUR_PARENTS;
        let receiver = 16 * subtrees;
        let following = 61 * subtrees;
        let first = if prefix >= COPIES_PER_BATCH { following + 2 } else { following };
        // After the first batch, the first remaining part of group 61 and its
        // final subtree join the two middle subtrees of group 16. Only a full
        // batch at that point separates the later runs.
        let mut copies = prefix;
        for range in [first..first + 1, following + 3..following + 4, receiver + 1..receiver + 3] {
            copies = copies.checked_add(self.subtree_copies(range)?)?;
        }
        Some(copies >= 2 * COPIES_PER_BATCH)
    }

    fn penultimate_layout(&self, ordinary: &'static [(usize, usize)]) -> Option<Cow<'static, [(usize, usize)]>> {
        let prefix = self.group_copies(60..61)?;
        let full_prefix = prefix >= COPIES_PER_BATCH;
        let layout = if self.penultimate_batch_fills(prefix)? { PairedBatches::Penultimate.layout() } else { ordinary };
        Some(Self::shift_horizontal_boundaries(layout, None, full_prefix))
    }

    fn three_final_pairs_layout(&self) -> Option<Cow<'static, [(usize, usize)]>> {
        let penultimate = self.group_copies(60..61)?;
        if self.group_copies(58..59)? < COPIES_PER_BATCH || penultimate < COPIES_PER_BATCH || self.group_copies(62..63)? < COPIES_PER_BATCH {
            return None;
        }
        // The first overflowing pair's tail fills group 0. The final pair's
        // tail leaves space in group 48, where its remaining draws continue.
        let first_tail = self.group_copies(58..60)?.checked_sub(2 * COPIES_PER_BATCH)?;
        let last_tail = self.group_copies(62..64)?.checked_sub(2 * COPIES_PER_BATCH)?;
        if self.group_copies(0..1)?.checked_add(first_tail)? < COPIES_PER_BATCH || self.group_copies(48..49)?.checked_add(last_tail)? >= COPIES_PER_BATCH {
            return None;
        }
        Some(Cow::Owned(paired_batches::three_final_pairs(self.penultimate_batch_fills(penultimate)?)))
    }

    fn final_pair_layout(&self) -> Option<Cow<'static, [(usize, usize)]>> {
        let ordinary = PairedBatches::Final.layout();
        let deferred = self.group_copies(62..64)?.checked_sub(2 * COPIES_PER_BATCH)?;
        if self.group_copies(48..49)?.checked_add(deferred)? >= COPIES_PER_BATCH {
            return Some(Cow::Borrowed(ordinary));
        }
        let full_prefix = self.group_copies(60..61)? >= COPIES_PER_BATCH;
        let mut layout = Self::shift_horizontal_boundaries(ordinary, None, full_prefix);
        // The last pair's deferred copies leave room after group 48. Its first run
        // fills the batch inside the next subtree and its second run finishes that subtree.
        let group_end = 49 * DEPTH_THREE_PARENTS;
        let mut ends = layout.to_mut().iter_mut().filter(|(parents, _)| *parents == group_end);
        ends.next()?.0 += 1;
        ends.next()?.0 += DEPTH_FOUR_PARENTS;
        Some(layout)
    }

    pub(super) fn layout(&self, ordinary: &'static [(usize, usize)]) -> Option<Cow<'static, [(usize, usize)]>> {
        if self.full_pairs == u8::MAX && self.axes[0] {
            return Some(Cow::Borrowed(paired_batches::ALL_FULL_PAIRS));
        }
        // The first pair also submits its parts separately in the ordinary layout.
        let later = self.full_pairs & !1;
        if self.parents_per_subtree == DEPTH_FOUR_PARENTS && self.axes == [true, false] {
            match later {
                // Groups 52–53 and 56–57 submit their subtrees separately,
                // including when both pairs contain two full batches.
                0 | 0b100 | 0b10000 | 0b10100 => return self.horizontal_layout(ordinary, None),
                0b1110_0000 if later == self.full_pairs => return self.three_final_pairs_layout(),
                0b1000000 => return self.penultimate_layout(ordinary),
                0b10000000 => return self.final_pair_layout(),
                0b10 => return self.horizontal_layout(ordinary, Some((50, 16))),
                0b1000 => return self.horizontal_layout(ordinary, Some((54, 32))),
                _ => {}
            }
        }
        if later == 0 {
            return Some(Cow::Borrowed(ordinary));
        }
        if self.axes != [true, false] || !later.is_power_of_two() {
            return None;
        }
        let end = FIRST_PAIR_END + later.trailing_zeros() as usize * PAIRED_SUBTREE_PARENTS;
        Some(Cow::Borrowed(PairedBatches::ending_at(end)?.layout()))
    }

    fn split_fits_subtree(&self, parents: usize, cut: u32) -> bool {
        let subtree = parents / self.parents_per_subtree;
        if parents.is_multiple_of(self.parents_per_subtree) {
            // The first part of a repeated endpoint may fill its final subtree exactly.
            return self.subtree_ends.partition_point(|&end| end < cut) + 1 == subtree;
        }
        let group = parents / DEPTH_THREE_PARENTS;
        if self.parents_per_subtree == DEPTH_FOUR_PARENTS && self.axes == [true, false] && parents % DEPTH_THREE_PARENTS < DEPTH_FOUR_PARENTS {
            let previous = match group {
                // When the seventh batch ends early, the remaining copies join the
                // following quadrant. Filling that batch changes the later draw order.
                15 | 31 | 47 => {
                    let group_end = subtree + DEPTH_THREE_PARENTS / DEPTH_FOUR_PARENTS - 1;
                    self.subtree_ends.get(group_end).and_then(|end| end.checked_sub(cut)).is_some_and(|remaining| remaining < COPIES_PER_BATCH)
                }
                // These four short pairs can also end one subtree early.
                51 | 55 | 59 | 63 => self.full_pairs & (1 << ((group - 49) / 2)) == 0,
                _ => false,
            };
            if previous {
                return self.subtree_ends.get(subtree - 2).zip(self.subtree_ends.get(subtree)).is_some_and(|(&lower, &upper)| lower < cut && cut < upper);
            }
        }
        self.subtree_ends.partition_point(|&end| end <= cut) == subtree
            && (self.parents_per_subtree != DEPTH_FOUR_PARENTS
                || subtree.checked_sub(1).and_then(|i| self.subtree_ends.get(i)).is_some_and(|&lower| cut > lower))
    }

    pub(super) fn boundaries(&self, layout: &[(usize, usize)], draws_per_parent: usize) -> Option<Vec<DrawPosition>> {
        let mut cuts = vec![None; layout.len() + 1];
        let mut order = vec![0; layout.len()];
        cuts[0] = Some(0_u32);
        for (run, &(parents, rank)) in layout.iter().enumerate() {
            *order.get_mut(rank)? = run;
            // A repeated endpoint leaves the first part for the active batch to size.
            let split = layout.get(run + 1).is_some_and(|&(next, _)| next == parents);
            if !split && parents.is_multiple_of(self.parents_per_subtree) {
                cuts[run + 1] = self.subtree_ends.get((parents / self.parents_per_subtree).checked_sub(1)?).copied();
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
                    if !self.split_fits_subtree(parents, end) {
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
