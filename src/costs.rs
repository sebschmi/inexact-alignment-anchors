use generic_a_star::cost::U32Cost;

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
    pub mismatch_cost: U32Cost,
    pub gap_open_cost: U32Cost,
    pub gap_extend_cost: U32Cost,
}

impl<Character: Eq> AlignmentCost<Character, U32Cost> for SimpleAlignmentCost {
    fn cost(&self, a: &Character, b: &Character) -> U32Cost {
        if a == b {
            U32Cost::from(0u32)
        } else {
            self.mismatch_cost
        }
    }

    fn gap_open_cost(&self) -> U32Cost {
        self.gap_open_cost
    }

    fn gap_extend_cost(&self) -> U32Cost {
        self.gap_extend_cost
    }
}
