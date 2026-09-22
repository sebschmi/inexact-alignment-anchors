use std::fmt::Display;

/// An alignment anchor.
///
/// The anchor is defined as a pair of two intervals, one for each aligned sequence.
/// The intervals are defined as [offset, limit), where the offset is inclusive and the limit is exclusive.
/// The intervals may be empty.
#[derive(Debug, Clone, Eq, PartialEq, Hash, PartialOrd, Ord)]
pub struct Anchor<Cost> {
    pub offset_a: usize,
    pub limit_a: usize,
    pub offset_b: usize,
    pub limit_b: usize,
    pub cost: Cost,
}

impl<Cost> Anchor<Cost> {
    pub fn new(
        offset_a: usize,
        limit_a: usize,
        offset_b: usize,
        limit_b: usize,
        cost: Cost,
    ) -> Self {
        Self {
            offset_a,
            limit_a,
            offset_b,
            limit_b,
            cost,
        }
    }

    pub fn shift_right(self, offset_a: usize, offset_b: usize) -> Self {
        Self {
            offset_a: self.offset_a + offset_a,
            limit_a: self.limit_a + offset_a,
            offset_b: self.offset_b + offset_b,
            limit_b: self.limit_b + offset_b,
            cost: self.cost,
        }
    }

    /// Trims leading and trailing matches from the anchor.
    ///
    /// If the anchor has at least one mismatch, then all leading and trailing matches are removed.
    /// If the first and/or last mismatch cannot be placed at a unique position, then all possible combinations are returned.
    pub fn trim<Character>(
        &self,
        sequence_a: &[Character],
        sequence_b: &[Character],
    ) -> impl Iterator<Item = Self> {
        todo!();
        None.into_iter()
    }
}

impl<Cost: Display> Display for Anchor<Cost> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "A(offset_a = {}, limit_a = {}, offset_b = {}, limit_b = {}, cost = {})",
            self.offset_a, self.limit_a, self.offset_b, self.limit_b, self.cost,
        )
    }
}
