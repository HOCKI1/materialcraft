use super::{COPIES_PER_BATCH, DEPTH_FOUR_PARENTS, DEPTH_THREE_PARENTS};

#[derive(Clone, Copy)]
pub(super) enum PairedBatches {
    ThirdFromEnd,
    Final,
}

impl PairedBatches {
    pub(super) fn ending_at(parents: usize) -> Option<Self> {
        match parents / DEPTH_THREE_PARENTS {
            60 => Some(Self::ThirdFromEnd),
            64 => Some(Self::Final),
            _ => None,
        }
    }

    pub(super) fn layout(self) -> &'static [(usize, usize)] {
        match self {
            Self::ThirdFromEnd => THIRD_PAIR_FROM_END,
            Self::Final => FINAL_PAIR,
        }
    }

    // A repeated endpoint separates a filled prefix from its later continuation.
    // The last pair can also leave enough copies to fill a batch inside group 48.
    pub(super) fn prefix_end(self, parents: usize, subtree_ends: &[u32]) -> Option<u32> {
        let end_of_group = |completed: usize| subtree_ends.get(completed * (DEPTH_THREE_PARENTS / DEPTH_FOUR_PARENTS) - 1).copied();
        let groups = parents / DEPTH_THREE_PARENTS;
        let cut = match (self, groups) {
            (Self::ThirdFromEnd, 60) | (Self::Final, 64) => end_of_group(groups - 2)?.checked_add(2 * COPIES_PER_BATCH)?,
            (Self::Final, 49) => {
                let final_pair = end_of_group(64)?.checked_sub(end_of_group(62)?)?;
                let continuation = final_pair.checked_sub(2 * COPIES_PER_BATCH)?;
                end_of_group(48)?.checked_add(COPIES_PER_BATCH.checked_sub(continuation)?)?
            }
            _ => return None,
        };
        // A prefix ending in a different depth-four subtree can change the remaining order.
        let subtree = parents / DEPTH_FOUR_PARENTS;
        let lower = *subtree_ends.get(subtree.checked_sub(2)?)?;
        let upper = *subtree_ends.get(subtree.checked_sub(1)?)?;
        (lower < cut && cut <= upper).then_some(cut)
    }
}

// Depth-seven parent endpoints and compositing ranks measured with original square overlaps.
// Inside a depth-four subtree the physical batch count determines the exact copy at the split.
const THIRD_PAIR_FROM_END: &[(usize, usize)] = &[
    (256, 15),
    (394, 34),
    (512, 41),
    (576, 40),
    (640, 45),
    (641, 38),
    (704, 46),
    (768, 42),
    (1794, 16),
    (2048, 25),
    (3846, 6),
    (4096, 17),
    (4160, 9),
    (4288, 37),
    (6023, 10),
    (6144, 30),
    (7941, 2),
    (8192, 19),
    (8256, 12),
    (8384, 31),
    (10121, 13),
    (10240, 33),
    (12033, 4),
    (12288, 27),
    (12352, 23),
    (12360, 32),
    (12416, 39),
    (12480, 51),
    (12674, 24),
    (12800, 47),
    (13058, 1),
    (13312, 8),
    (13376, 20),
    (13379, 26),
    (13440, 43),
    (13504, 53),
    (13702, 21),
    (13824, 50),
    (14082, 3),
    (14336, 11),
    (14400, 28),
    (14464, 44),
    (14528, 48),
    (14722, 29),
    (14848, 49),
    (15360, 5),
    (15360, 14),
    (15617, 7),
    (15680, 35),
    (15687, 18),
    (15808, 52),
    (15872, 36),
    (16129, 0),
    (16384, 22),
];

const FINAL_PAIR: &[(usize, usize)] = &[
    (64, 15),
    (192, 31),
    (1929, 16),
    (2048, 32),
    (3849, 6),
    (4096, 17),
    (4160, 9),
    (4288, 37),
    (6023, 10),
    (6144, 27),
    (7942, 2),
    (8192, 19),
    (8256, 12),
    (8384, 28),
    (10114, 13),
    (10240, 30),
    (12037, 4),
    (12288, 24),
    (12544, 23),
    (12544, 38),
    (12672, 29),
    (12736, 43),
    (12800, 39),
    (13058, 1),
    (13312, 8),
    (13376, 20),
    (13440, 33),
    (13504, 45),
    (13703, 21),
    (13824, 42),
    (14081, 3),
    (14336, 11),
    (14400, 25),
    (14464, 34),
    (14528, 40),
    (14725, 26),
    (14848, 41),
    (15105, 5),
    (15360, 14),
    (15618, 7),
    (15680, 35),
    (15690, 18),
    (15808, 44),
    (15872, 36),
    (16384, 0),
    (16384, 22),
];
