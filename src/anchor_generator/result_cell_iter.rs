pub enum DpMatrixResultCellIter {
    MaxMismatchesIsZero,
    AIsK { b_limit: usize },
    BIsKAndALowerThanK { a_limit: usize },
    BIsKAndALargerThanK { a_limit: usize },
}

impl DpMatrixResultCellIter {
    pub fn new(k: usize, max_mismatches: usize) -> Self {
        if max_mismatches == 0 {
            DpMatrixResultCellIter::MaxMismatchesIsZero
        } else {
            DpMatrixResultCellIter::AIsK {
                b_limit: k.saturating_sub(max_mismatches),
            }
        }
    }

    pub fn next(
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
