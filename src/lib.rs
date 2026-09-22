use std::ops::Range;

use generic_a_star::cost::AStarCost;

use crate::{
    anchor::Anchor, anchor_generator::InexactAlignmentAnchorGenerator, costs::AlignmentCost,
};

pub mod anchor;
pub mod anchor_generator;
pub mod costs;
pub mod error;

// Convenience functions.

/// Generate the inexact alignment anchors between two sequences.
///
/// See [`InexactAlignmentAnchorGenerator`] for more details.
pub fn generate_inexact_alignment_anchors<
    'context,
    Character: Eq,
    Cost: AStarCost,
    AlignmentCostImpl: AlignmentCost<Character, Cost>,
>(
    sequence_a: &'context [Character],
    sequence_b: &'context [Character],
    costs: &'context AlignmentCostImpl,
    k: usize,
    max_mismatches: usize,
) -> Result<impl Iterator<Item = Anchor<Cost>>, error::Error> {
    InexactAlignmentAnchorGenerator::new(sequence_a, sequence_b, costs, k, max_mismatches)
}

/// Generate the inexact alignment anchors between subsequences of two sequences.
///
/// This will return the anchors in the coordinates of the complete sequences,
/// so the first anchor of a subsequence that starts at offset `i` on `sequence_a` will also have offset `i` on `sequence_a`
/// (or more, if there is no anchor starting at offset `i` on `sequence_a`).
///
/// See [`InexactAlignmentAnchorGenerator`] for more details about anchor generation in general.
pub fn generate_inexact_alignment_anchors_of_subsequences<
    'context,
    Character: Eq,
    Cost: AStarCost,
    AlignmentCostImpl: AlignmentCost<Character, Cost>,
>(
    sequence_a: &'context [Character],
    sequence_b: &'context [Character],
    range_a: Range<usize>,
    range_b: Range<usize>,
    costs: &'context AlignmentCostImpl,
    k: usize,
    max_mismatches: usize,
) -> Result<impl Iterator<Item = Anchor<Cost>>, error::Error> {
    InexactAlignmentAnchorGenerator::new(
        &sequence_a[range_a.clone()],
        &sequence_b[range_b.clone()],
        costs,
        k,
        max_mismatches,
    )
    .map(move |iter| iter.map(move |anchor| anchor.shift_right(range_a.start, range_b.start)))
}
