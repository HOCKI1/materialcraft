use super::COPIES_PER_BATCH;
use std::ops::Range;

// A batch counts individual tile copies, including copies outside the readout. Overlapping
// squares distinguish both the split inside a subtree and the order of the following subtrees.
struct Split {
    partner: Range<u32>,
    prefix: Range<u32>,
    remainder: Range<u32>,
    next: Range<u32>,
    extra: Range<u32>,
    last: Range<u32>,
    // Whole groups after the split and before the separately submitted last group.
    whole_groups: usize,
}

fn leading(bounds: &[u32; 65], group: usize) -> Option<[Range<u32>; 2]> {
    let cut = bounds[group].checked_add(COPIES_PER_BATCH)?;
    // A short third leading block is submitted whole in its later position.
    if group == 8 && cut > bounds[group + 8] {
        return Some([0..0, bounds[group]..bounds[group + 8]]);
    }
    if group == 8 && cut == bounds[group + 8] {
        return Some([bounds[group]..cut, 0..0]);
    }
    (bounds[group + 6]..bounds[group + 8]).contains(&cut).then_some([bounds[group]..cut, cut..bounds[group + 8]])
}

fn paired(bounds: &[u32; 65], group: usize, partner: usize) -> Option<Split> {
    let partner = bounds[partner]..bounds[partner + 2];
    let remaining = COPIES_PER_BATCH.checked_sub(partner.end - partner.start)?;
    let cut = bounds[group].checked_add(remaining)?;
    // Beyond these three observed split positions the continuation order is unknown.
    if !(bounds[group + 4]..bounds[group + 7]).contains(&cut) {
        return None;
    }
    let whole_groups = if cut < bounds[group + 5] {
        2
    } else if cut < bounds[group + 6] {
        1
    } else {
        0
    };
    let at = group + 6 - whole_groups;
    // The last group is submitted separately even when the preceding group contains the cut.
    let next_end = (at + 2).min(group + 7);
    Some(Split {
        partner,
        prefix: bounds[group]..cut,
        remainder: cut..bounds[at + 1],
        next: bounds[at + 1]..bounds[next_end],
        extra: bounds[next_end]..bounds[group + 7],
        last: bounds[group + 7]..bounds[group + 8],
        whole_groups,
    })
}

pub(super) fn ranges(subtree_ends: &[u32]) -> Option<Vec<Range<u32>>> {
    let ends: &[u32; 64] = subtree_ends.try_into().ok()?;
    let mut bounds = [0; 65];
    bounds[1..].copy_from_slice(ends);
    let [lead24, tail24] = leading(&bounds, 24)?;
    let [lead40, tail40] = leading(&bounds, 40)?;
    let [lead8, mut tail8] = leading(&bounds, 8)?;
    let pair0 = paired(&bounds, 0, 58)?;
    let pair16 = paired(&bounds, 16, 50)?;
    let mut pair32 = paired(&bounds, 32, 54)?;

    let mut runs = Vec::with_capacity(32);
    runs.extend([lead24, lead40, lead8]);
    let pairs = if pair16.whole_groups == 2 { [&pair16, &pair0, &pair32] } else { [&pair16, &pair32, &pair0] };
    for pair in pairs {
        runs.extend([pair.partner.clone(), pair.prefix.clone()]);
    }
    let missing = COPIES_PER_BATCH.saturating_sub(bounds[16] - bounds[8]);
    if missing != 0 && pair16.whole_groups == 2 && pair0.whole_groups == 2 && !pair32.extra.is_empty() {
        // This extra group normally follows the last leading block. Its first copies
        // can fill that block after the three paired prefixes have been submitted.
        let cut = pair32.extra.start.checked_add(missing)?;
        if cut > pair32.extra.end {
            return None;
        }
        runs.extend([tail8, pair32.extra.start..cut]);
        tail8 = 0..0;
        pair32.extra.start = cut;
    }
    runs.extend([pair16.remainder, pair16.last]);
    // An early split leaves one extra whole subtree. Its position depends on which other
    // batches also split early, even when their stamps never overlap that subtree.
    let (after_next, after48, after56, after8) = if pair16.whole_groups == 2 {
        if pair0.whole_groups == 0 {
            runs.extend([pair32.next, pair32.remainder, pair32.last, pair0.remainder, pair0.last]);
        } else {
            runs.extend([pair0.next, pair32.remainder, pair32.last, pair0.remainder, pair0.last, pair32.next]);
        }
        let (near, far) = if pair0.whole_groups == 2 { (0..0, pair32.extra) } else { (pair32.extra, 0..0) };
        (near, pair16.extra, pair0.extra, far)
    } else if pair32.whole_groups != 1 {
        runs.extend([pair32.next, pair0.next, pair32.remainder, pair32.last, pair0.extra, pair0.remainder, pair0.last]);
        (0..0, pair32.extra, 0..0, 0..0)
    } else {
        runs.extend([pair32.next, pair32.remainder, pair32.last, pair0.next, pair0.remainder, pair0.last]);
        (0..0, pair0.extra, 0..0, 0..0)
    };
    runs.extend([
        bounds[60]..bounds[62],
        pair16.next,
        after_next,
        bounds[62]..bounds[64],
        bounds[48]..bounds[50],
        after48,
        tail40,
        bounds[56]..bounds[58],
        after56,
        tail24,
        bounds[52]..bounds[54],
        tail8,
        after8,
    ]);
    runs.retain(|r| !r.is_empty());
    Some(runs)
}

#[cfg(test)]
mod tests {
    use super::ranges;

    #[test]
    fn sparse_quadrants_submit_whole_groups_around_late_splits() {
        // Each square pair shares one pixel in an independent signed F32 readout.
        for ([left, center, right, short], markers) in [
            (
                [145, 95, 66, 96],
                &[(4611, 28380, false), (5522, 28416, false), (4616, 28985, true), (5532, 28385, false), (5553, 3920, false), (6446, 6489, true)][..],
            ),
            (
                [66, 145, 95, 85],
                &[(27796, 3636, false), (28507, 3672, false), (27801, 27075, false), (27806, 28517, true), (4256, 27260, false), (4231, 28559, false)][..],
            ),
            (
                [145, 145, 145, 85],
                &[
                    (31524, 16510, false),
                    (31539, 16740, false),
                    (32435, 16546, false),
                    (31529, 19273, true),
                    (31749, 20189, false),
                    (31994, 43209, false),
                    (8046, 16552, false),
                    (12483, 43935, false),
                    (31754, 7369, false),
                ][..],
            ),
        ] {
            let mut copies = 0_u32;
            let ends: [u32; 64] = std::array::from_fn(|group| {
                let quota = match group {
                    0..8 => left,
                    8..16 => short,
                    16..24 => center,
                    32..40 => right,
                    _ => 96,
                };
                let parent_slot = if group < 48 { 0 } else { 2 };
                copies += 4 * (85 + quota) + 1 + u32::from(group % 4 == parent_slot);
                copies
            });
            let order: Vec<_> = ranges(&ends).unwrap().into_iter().flatten().collect();
            let mut ranks = vec![usize::MAX; copies as usize];
            for (rank, copy) in order.into_iter().enumerate() {
                assert_eq!(ranks[copy as usize], usize::MAX);
                ranks[copy as usize] = rank;
            }
            assert!(!ranks.contains(&usize::MAX));
            for &(first, second, before) in markers {
                assert_eq!(ranks[first] < ranks[second], before, "copies {first} and {second}, quotas {left}, {center}, {right}, {short}");
            }
        }
    }
}
