#[cfg(test)]
mod tests;

pub struct OffsetIter {
    offset_a: usize,
    offset_b: usize,
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
            if sequence_b_len < self.offset_b + k + 1 {
                self.offset_a += 1;
                self.offset_b = 0;
                return if sequence_a_len < self.offset_a + k.saturating_sub(max_mismatches) {
                    None
                } else {
                    Some((self.offset_a, self.offset_b))
                };
            }
        } else {
            // Here, a has space for at least k characters, so b can have less than k characters.
            if sequence_b_len < self.offset_b + k.saturating_sub(max_mismatches) + 1 {
                self.offset_a += 1;
                self.offset_b = 0;
                return if sequence_a_len < self.offset_a + k.saturating_sub(max_mismatches) {
                    None
                } else {
                    Some((self.offset_a, self.offset_b))
                };
            }
        }

        if sequence_a_len < self.offset_a + k.saturating_sub(max_mismatches) {
            None
        } else {
            self.offset_b += 1;
            Some((self.offset_a, self.offset_b))
        }
    }
}
