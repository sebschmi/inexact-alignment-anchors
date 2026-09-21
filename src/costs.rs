pub trait AlignmentCost<Character, Cost> {
    /// Returns the cost of aligning two characters.
    fn cost(&self, a: &Character, b: &Character) -> Cost;

    /// Returns the cost of opening a gap.
    ///
    /// This is the cost paid for the first character of a gap.
    fn gap_open_cost(&self) -> Cost;

    /// Returns the cost of extending a gap.
    ///
    /// This is the cost paid for the second and all subsequent characters of a gap.
    fn gap_extend_cost(&self) -> Cost;
}

pub struct SimpleAlignmentCost {
    pub mismatch_cost: u32,
    pub gap_open_cost: u32,
    pub gap_extend_cost: u32,
}

impl<Character: Eq> AlignmentCost<Character, u32> for SimpleAlignmentCost {
    fn cost(&self, a: &Character, b: &Character) -> u32 {
        if a == b { 0 } else { self.mismatch_cost }
    }

    fn gap_open_cost(&self) -> u32 {
        self.gap_open_cost
    }

    fn gap_extend_cost(&self) -> u32 {
        self.gap_extend_cost
    }
}
