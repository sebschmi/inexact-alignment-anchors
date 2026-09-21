use std::marker::PhantomData;

use ndarray::{Array3, s};
use num_traits::{Zero, bounds::UpperBounded};

use crate::{anchor::Anchor, costs::AlignmentCost};

#[cfg(test)]
mod tests;

/// Generates all anchors between the two sequences.
///
/// An anchor is an alignment between two subsequences of the given input sequences.
/// One of the subsequences has to have length `k`, while the other subsequence can have a different length due to mismatches.
/// The number of mismatches is limited by `max_mismatches`, and any single substitution or gap character counts as one mismatch.
/// Specifically, a gap of length `n` counts as `n` mismatches.
///
/// The anchors are deduplicated, and for each possible anchor geometry only an anchor of minimum cost is generated.
pub struct InexactAlignmentAnchorGenerator<
    'sequences,
    'costs,
    Character,
    Cost,
    AlignmentCostImpl: AlignmentCost<Character, Cost>,
> {
    sequence_a: &'sequences [Character],
    sequence_b: &'sequences [Character],
    costs: &'costs AlignmentCostImpl,
    k: usize,
    max_mismatches: usize,
    phantom_data: PhantomData<Cost>,

    offset_iter: OffsetIter,
    dp_matrix: BandedDpMatrix<Cost>,
    result_cell_iter: DpMatrixResultCellIter,
}

enum DpMatrixResultCellIter {
    MaxMismatchesIsZero,
    AIsK { b_limit: usize },
    BIsKAndALowerThanK { a_limit: usize },
    BIsKAndALargerThanK { a_limit: usize },
}

struct OffsetIter {
    offset_a: usize,
    offset_b: usize,
}

struct BandedDpMatrix<Cost> {
    matrix: Array3<Cost>,
}

impl<'sequences, 'costs, Character, Cost, AlignmentCostImpl: AlignmentCost<Character, Cost>>
    InexactAlignmentAnchorGenerator<'sequences, 'costs, Character, Cost, AlignmentCostImpl>
{
    pub fn new(
        sequence_a: &'sequences [Character],
        sequence_b: &'sequences [Character],
        costs: &'costs AlignmentCostImpl,
        k: usize,
        max_mismatches: usize,
    ) -> Self
    where
        Cost: Clone + Zero,
    {
        Self {
            sequence_a,
            sequence_b,
            costs,
            k,
            max_mismatches,
            phantom_data: PhantomData,

            offset_iter: OffsetIter::new(),
            dp_matrix: BandedDpMatrix::new(k, max_mismatches),
            result_cell_iter: DpMatrixResultCellIter::new(k, max_mismatches),
        }
    }
}

impl<
    'sequences,
    'costs,
    Character,
    Cost: Clone + Zero + Ord + UpperBounded,
    AlignmentCostImpl: AlignmentCost<Character, Cost>,
> Iterator
    for InexactAlignmentAnchorGenerator<'sequences, 'costs, Character, Cost, AlignmentCostImpl>
{
    type Item = Anchor<Cost>;

    fn next(&mut self) -> Option<Self::Item> {
        let (offset_a, offset_b) = self.offset_iter.peek(
            self.sequence_a.len(),
            self.sequence_b.len(),
            self.k,
            self.max_mismatches,
        )?;
        let subsequence_a_len = self.sequence_a.len() - offset_a;
        let subsequence_b_len = self.sequence_b.len() - offset_b;

        if let Some(result_cell) = self.result_cell_iter.next(
            subsequence_a_len,
            subsequence_b_len,
            self.k,
            self.max_mismatches,
        ) {
            Some(Anchor {
                offset_a,
                limit_a: offset_a + result_cell.0,
                offset_b,
                limit_b: offset_b + result_cell.1,
                cost: self.dp_matrix.get(result_cell, self.max_mismatches),
            })
        } else {
            let (offset_a, offset_b) = self.offset_iter.next(
                self.sequence_a.len(),
                self.sequence_b.len(),
                self.k,
                self.max_mismatches,
            )?;
            let subsequence_a_len = self.sequence_a.len() - offset_a;
            let subsequence_b_len = self.sequence_b.len() - offset_b;

            self.result_cell_iter.reset(self.k, self.max_mismatches);
            self.dp_matrix.fill(
                &self.sequence_a
                    [offset_a..subsequence_a_len.min(offset_a + self.k + self.max_mismatches)],
                &self.sequence_b
                    [offset_b..subsequence_b_len.min(offset_b + self.k + self.max_mismatches)],
                self.costs,
                self.max_mismatches,
            );
            self.next()
        }
    }
}

impl DpMatrixResultCellIter {
    fn new(k: usize, max_mismatches: usize) -> Self {
        if max_mismatches == 0 {
            DpMatrixResultCellIter::MaxMismatchesIsZero
        } else {
            DpMatrixResultCellIter::AIsK {
                b_limit: k.saturating_sub(max_mismatches),
            }
        }
    }

    fn reset(&mut self, k: usize, max_mismatches: usize) {
        *self = Self::new(k, max_mismatches);
    }

    fn next(
        &mut self,
        subsequence_a_len: usize,
        subsequence_b_len: usize,
        k: usize,
        max_mismatches: usize,
    ) -> Option<(usize, usize)> {
        debug_assert!(subsequence_a_len >= k || subsequence_b_len >= k);
        debug_assert!(subsequence_a_len >= k.saturating_sub(max_mismatches));
        debug_assert!(subsequence_b_len >= k.saturating_sub(max_mismatches));

        match self {
            Self::MaxMismatchesIsZero => {
                debug_assert_eq!(max_mismatches, 0);
                *self = DpMatrixResultCellIter::BIsKAndALargerThanK { a_limit: k + 1 };
                Some((k, k))
            }
            DpMatrixResultCellIter::AIsK { b_limit } => {
                if *b_limit <= (k + max_mismatches).min(subsequence_b_len) && k <= subsequence_a_len
                {
                    let result = Some((k, *b_limit));
                    *b_limit += 1;
                    result
                } else {
                    *self = DpMatrixResultCellIter::BIsKAndALowerThanK {
                        a_limit: k.saturating_sub(max_mismatches),
                    };
                    self.next(subsequence_a_len, subsequence_b_len, k, max_mismatches)
                }
            }
            DpMatrixResultCellIter::BIsKAndALowerThanK { a_limit } => {
                if *a_limit < k && k <= subsequence_b_len {
                    let result = Some((*a_limit, k));
                    *a_limit += 1;
                    result
                } else {
                    *self = DpMatrixResultCellIter::BIsKAndALargerThanK { a_limit: k + 1 };
                    self.next(subsequence_a_len, subsequence_b_len, k, max_mismatches)
                }
            }
            DpMatrixResultCellIter::BIsKAndALargerThanK { a_limit } => {
                if *a_limit <= (k + max_mismatches).min(subsequence_a_len) && k <= subsequence_b_len
                {
                    let result = Some((*a_limit, k));
                    *a_limit += 1;
                    result
                } else {
                    None
                }
            }
        }
    }
}

impl OffsetIter {
    pub fn new() -> Self {
        Self {
            offset_a: 0,
            offset_b: 0,
        }
    }

    pub fn peek(
        &self,
        sequence_a_len: usize,
        _sequence_b_len: usize,
        k: usize,
        max_mismatches: usize,
    ) -> Option<(usize, usize)> {
        if sequence_a_len < self.offset_a + k.saturating_sub(max_mismatches) {
            None
        } else {
            Some((self.offset_a, self.offset_b))
        }
    }

    pub fn next(
        &mut self,
        sequence_a_len: usize,
        sequence_b_len: usize,
        k: usize,
        max_mismatches: usize,
    ) -> Option<(usize, usize)> {
        // One of the sequences must have space for at least k characters.
        if sequence_a_len < self.offset_a + k {
            // Here, a has space for less than k characters, so b must have space for at least k characters.
            if sequence_b_len < self.offset_b + k {
                self.offset_a += 1;
                self.offset_b = 0;
            }
        } else {
            // Here, a has space for at least k characters, so b can have less than k characters.
            if sequence_b_len < self.offset_b + k.saturating_sub(max_mismatches) {
                self.offset_a += 1;
                self.offset_b = 0;
            }
        }

        if sequence_a_len < self.offset_a + k.saturating_sub(max_mismatches) {
            None
        } else {
            let result = (self.offset_a, self.offset_b);
            self.offset_b += 1;
            Some(result)
        }
    }
}

impl<Cost> BandedDpMatrix<Cost> {
    fn new(k: usize, max_mismatches: usize) -> Self
    where
        Cost: Clone + Zero,
    {
        Self {
            matrix: Array3::zeros((k + max_mismatches + 1, 2 * max_mismatches + 1, 3)),
        }
    }

    /// Get the cost at a specific cell in the DP matrix.
    fn get(&self, cell: (usize, usize), max_mismatches: usize) -> Cost
    where
        Cost: Clone + Ord,
    {
        self.matrix
            .slice(s![cell.0, cell.1 + max_mismatches - cell.0, ..])
            .iter()
            .min()
            .cloned()
            .unwrap()
    }

    /// Fill the DP matrix based on the given subsequences.
    fn fill<Character>(
        &mut self,
        subsequence_a: &[Character],
        subsequence_b: &[Character],
        _costs: &impl AlignmentCost<Character, Cost>,
        max_mismatches: usize,
    ) where
        Cost: Clone + Zero + UpperBounded,
    {
        // Fill in order of anti-diagonals with banding according to max_mismatches.
        for a in 0..=subsequence_a.len() {
            for b in
                a.saturating_sub(max_mismatches)..=(a + max_mismatches).min(subsequence_b.len())
            {
                if a == 0 && b == 0 {
                    // Initialise empty alignment.
                    self.matrix
                        .slice_mut(s![0..=0, 0..=0, 0])
                        .fill(Cost::zero());
                    self.matrix
                        .slice_mut(s![0..=0, 0..=0, 1..=2])
                        .fill(Cost::max_value());
                } else {
                    todo!()
                }
            }
        }
    }
}
