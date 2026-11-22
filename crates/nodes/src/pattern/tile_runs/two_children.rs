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
    early: bool,
}

fn leading(bounds: &[u32; 65], group: usize) -> Option<[Range<u32>; 2]> {
    let cut = bounds[group].checked_add(COPIES_PER_BATCH)?;
    (bounds[group + 6]..bounds[group + 8]).contains(&cut).then_some([bounds[group]..cut, cut..bounds[group + 8]])
}

fn paired(bounds: &[u32; 65], group: usize, partner: usize) -> Option<Split> {
    let partner = bounds[partner]..bounds[partner + 2];
    let remaining = COPIES_PER_BATCH.checked_sub(partner.end - partner.start)?;
    let cut = bounds[group].checked_add(remaining)?;
    // Beyond these two observed split positions the continuation order is unknown.
    if !(bounds[group + 4]..bounds[group + 6]).contains(&cut) {
        return None;
    }
    let early = cut < bounds[group + 5];
    let at = group + if early { 4 } else { 5 };
    Some(Split {
        partner,
        prefix: bounds[group]..cut,
        remainder: cut..bounds[at + 1],
        next: bounds[at + 1]..bounds[at + 2],
        extra: bounds[at + 2]..bounds[group + 7],
        last: bounds[group + 7]..bounds[group + 8],
        early,
    })
}

pub(super) fn ranges(subtree_ends: &[u32]) -> Option<Vec<Range<u32>>> {
    let ends: &[u32; 64] = subtree_ends.try_into().ok()?;
    let mut bounds = [0; 65];
    bounds[1..].copy_from_slice(ends);
    let [lead24, tail24] = leading(&bounds, 24)?;
    let [lead40, tail40] = leading(&bounds, 40)?;
    let [lead8, tail8] = leading(&bounds, 8)?;
    let pair0 = paired(&bounds, 0, 58)?;
    let pair16 = paired(&bounds, 16, 50)?;
    let pair32 = paired(&bounds, 32, 54)?;

    let mut runs = Vec::with_capacity(32);
    runs.extend([lead24, lead40, lead8]);
    let pairs = if pair16.early { [&pair16, &pair0, &pair32] } else { [&pair16, &pair32, &pair0] };
    for pair in pairs {
        runs.extend([pair.partner.clone(), pair.prefix.clone()]);
    }
    runs.extend([pair16.remainder, pair16.last]);
    // An early split leaves one extra whole subtree. Its position depends on which other
    // batches also split early, even when their stamps never overlap that subtree.
    let (after_next, after48, after56, after8) = if pair16.early {
        runs.extend([pair0.next, pair32.remainder, pair32.last, pair0.remainder, pair0.last, pair32.next]);
        let (near, far) = if pair0.early { (0..0, pair32.extra) } else { (pair32.extra, 0..0) };
        (near, pair16.extra, pair0.extra, far)
    } else if pair32.early {
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
