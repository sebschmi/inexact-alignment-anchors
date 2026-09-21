/// An alignment anchor.
///
/// The anchor is defined as a pair of two intervals, one for each aligned sequence.
/// The intervals are defined as [offset, limit), where the offset is inclusive and the limit is exclusive.
/// The intervals may be empty.
pub struct Anchor<Cost> {
    pub offset_a: usize,
    pub limit_a: usize,
    pub offset_b: usize,
    pub limit_b: usize,
    pub cost: Cost,
}
