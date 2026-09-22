use std::marker::PhantomData;

use generic_a_star::cost::AStarCost;

use crate::{
    anchor::Anchor,
    anchor_generator::{
        aligner::{AnchorLimit, InexactAnchorLimitGenerator},
        offset_iter::OffsetIter,
    },
    costs::AlignmentCost,
    error::Error,
};

mod aligner;
mod offset_iter;
mod result_cell_iter;
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
///
/// This type implements the `Iterator` trait, and generating all anchors is done by exhausting the iterator (calling `next()` until it returns `None`).
pub struct InexactAlignmentAnchorGenerator<
    'context,
    Character: Eq,
    Cost: AStarCost,
    AlignmentCostImpl: AlignmentCost<Character, Cost>,
> {
    sequence_a: &'context [Character],
    sequence_b: &'context [Character],
    k: usize,
    max_mismatches: usize,
    phantom_data: PhantomData<Cost>,

    offset_iter: OffsetIter,
    limit_generator: InexactAnchorLimitGenerator<'context, Character, Cost, AlignmentCostImpl>,
}

impl<'context, Character: Eq, Cost: AStarCost, AlignmentCostImpl: AlignmentCost<Character, Cost>>
    InexactAlignmentAnchorGenerator<'context, Character, Cost, AlignmentCostImpl>
{
    /// Creates a new `InexactAlignmentAnchorGenerator`.
    ///
    /// See [`InexactAlignmentAnchorGenerator`] for more details.
    ///
    /// # Arguments
    ///
    /// * `sequence_a` - The first sequence.
    /// * `sequence_b` - The second sequence.
    /// * `costs` - The alignment costs.
    /// * `k` - The length of an anchor. At least one of the two sides of the anchor must have this length.
    /// * `max_mismatches` - The maximum number of mismatches allowed in an anchor.
    ///
    /// # Returns
    ///
    /// A new `InexactAlignmentAnchorGenerator`, or an error if `k` or `max_mismatches` are larger than 255.
    pub fn new(
        sequence_a: &'context [Character],
        sequence_b: &'context [Character],
        costs: &'context AlignmentCostImpl,
        k: usize,
        max_mismatches: usize,
    ) -> Result<Self, Error> {
        Ok(Self {
            sequence_a,
            sequence_b,
            k,
            max_mismatches,
            phantom_data: PhantomData,

            offset_iter: OffsetIter::new(),
            limit_generator: InexactAnchorLimitGenerator::new(
                0,
                0,
                sequence_a,
                sequence_b,
                costs,
                k,
                max_mismatches,
            )?,
        })
    }
}

impl<'context, Character: Eq, Cost: AStarCost, AlignmentCostImpl: AlignmentCost<Character, Cost>>
    Iterator for InexactAlignmentAnchorGenerator<'context, Character, Cost, AlignmentCostImpl>
{
    type Item = Anchor<Cost>;

    fn next(&mut self) -> Option<Self::Item> {
        let (offset_a, offset_b) = self.offset_iter.peek(
            self.sequence_a.len(),
            self.sequence_b.len(),
            self.k,
            self.max_mismatches,
        )?;

        if let Some(AnchorLimit {
            limit_a: len_a,
            limit_b: len_b,
            cost,
        }) = self.limit_generator.next()
        {
            Some(Anchor {
                offset_a,
                limit_a: offset_a + len_a,
                offset_b,
                limit_b: offset_b + len_b,
                cost,
            })
        } else {
            while self.limit_generator.peek().is_none() {
                let (offset_a, offset_b) = self.offset_iter.next(
                    self.sequence_a.len(),
                    self.sequence_b.len(),
                    self.k,
                    self.max_mismatches,
                )?;

                self.limit_generator
                    .reset(offset_a, offset_b, self.sequence_a, self.sequence_b);
            }

            // Now we must have found a limit, so we can call next() again to get it.
            self.next()
        }
    }
}
