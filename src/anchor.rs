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
}
