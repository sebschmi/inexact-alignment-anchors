use crate::anchor_generator::{DpMatrixResultCellIter, OffsetIter};

#[test]
fn dp_matrix_result_cell_iter_100_100_0_0() {
    let subsequence_a_len = 100;
    let subsequence_b_len = 100;
    let k = 0;
    let max_mismatches = 0;
    let mut iter = DpMatrixResultCellIter::new(k, max_mismatches);

    let mut actual = Vec::new();
    while let Some((a, b)) = iter.next(subsequence_a_len, subsequence_b_len, k, max_mismatches) {
        actual.push((a, b));
    }
    actual.sort_unstable();

    let mut expected = vec![(0, 0)];
    expected.sort_unstable();

    assert_eq!(actual, expected);
}

#[test]
fn dp_matrix_result_cell_iter_100_100_1_0() {
    let subsequence_a_len = 100;
    let subsequence_b_len = 100;
    let k = 1;
    let max_mismatches = 0;
    let mut iter = DpMatrixResultCellIter::new(k, max_mismatches);

    let mut actual = Vec::new();
    while let Some((a, b)) = iter.next(subsequence_a_len, subsequence_b_len, k, max_mismatches) {
        actual.push((a, b));
    }
    actual.sort_unstable();

    let mut expected = vec![(1, 1)];
    expected.sort_unstable();

    assert_eq!(actual, expected);
}

#[test]
fn dp_matrix_result_cell_iter_100_100_2_0() {
    let subsequence_a_len = 100;
    let subsequence_b_len = 100;
    let k = 2;
    let max_mismatches = 0;
    let mut iter = DpMatrixResultCellIter::new(k, max_mismatches);

    let mut actual = Vec::new();
    while let Some((a, b)) = iter.next(subsequence_a_len, subsequence_b_len, k, max_mismatches) {
        actual.push((a, b));
    }
    actual.sort_unstable();

    let mut expected = vec![(2, 2)];
    expected.sort_unstable();

    assert_eq!(actual, expected);
}

#[test]
fn dp_matrix_result_cell_iter_100_100_0_1() {
    let subsequence_a_len = 100;
    let subsequence_b_len = 100;
    let k = 0;
    let max_mismatches = 1;
    let mut iter = DpMatrixResultCellIter::new(k, max_mismatches);

    let mut actual = Vec::new();
    while let Some((a, b)) = iter.next(subsequence_a_len, subsequence_b_len, k, max_mismatches) {
        actual.push((a, b));
    }
    actual.sort_unstable();

    let mut expected = vec![(0, 0), (0, 1), (1, 0)];
    expected.sort_unstable();

    assert_eq!(actual, expected);
}

#[test]
fn dp_matrix_result_cell_iter_100_100_0_2() {
    let subsequence_a_len = 100;
    let subsequence_b_len = 100;
    let k = 0;
    let max_mismatches = 2;
    let mut iter = DpMatrixResultCellIter::new(k, max_mismatches);

    let mut actual = Vec::new();
    while let Some((a, b)) = iter.next(subsequence_a_len, subsequence_b_len, k, max_mismatches) {
        actual.push((a, b));
    }
    actual.sort_unstable();

    let mut expected = vec![(0, 0), (0, 1), (1, 0), (0, 2), (2, 0)];
    expected.sort_unstable();

    assert_eq!(actual, expected);
}

#[test]
fn dp_matrix_result_cell_iter_100_100_1_1() {
    let subsequence_a_len = 100;
    let subsequence_b_len = 100;
    let k = 1;
    let max_mismatches = 1;
    let mut iter = DpMatrixResultCellIter::new(k, max_mismatches);

    let mut actual = Vec::new();
    while let Some((a, b)) = iter.next(subsequence_a_len, subsequence_b_len, k, max_mismatches) {
        actual.push((a, b));
    }
    actual.sort_unstable();

    let mut expected = vec![(1, 1), (0, 1), (1, 0), (1, 2), (2, 1)];
    expected.sort_unstable();

    assert_eq!(actual, expected);
}

#[test]
fn dp_matrix_result_cell_iter_100_100_1_2() {
    let subsequence_a_len = 100;
    let subsequence_b_len = 100;
    let k = 1;
    let max_mismatches = 2;
    let mut iter = DpMatrixResultCellIter::new(k, max_mismatches);

    let mut actual = Vec::new();
    while let Some((a, b)) = iter.next(subsequence_a_len, subsequence_b_len, k, max_mismatches) {
        actual.push((a, b));
    }
    actual.sort_unstable();

    let mut expected = vec![(1, 1), (0, 1), (1, 0), (1, 2), (2, 1), (1, 3), (3, 1)];
    expected.sort_unstable();

    assert_eq!(actual, expected);
}

#[test]
fn dp_matrix_result_cell_iter_100_100_2_1() {
    let subsequence_a_len = 100;
    let subsequence_b_len = 100;
    let k = 2;
    let max_mismatches = 1;
    let mut iter = DpMatrixResultCellIter::new(k, max_mismatches);

    let mut actual = Vec::new();
    while let Some((a, b)) = iter.next(subsequence_a_len, subsequence_b_len, k, max_mismatches) {
        actual.push((a, b));
    }
    actual.sort_unstable();

    let mut expected = vec![(2, 2), (1, 2), (2, 1), (2, 3), (3, 2)];
    expected.sort_unstable();

    assert_eq!(actual, expected);
}

#[test]
fn dp_matrix_result_cell_iter_100_100_2_2() {
    let subsequence_a_len = 100;
    let subsequence_b_len = 100;
    let k = 2;
    let max_mismatches = 2;
    let mut iter = DpMatrixResultCellIter::new(k, max_mismatches);

    let mut actual = Vec::new();
    while let Some((a, b)) = iter.next(subsequence_a_len, subsequence_b_len, k, max_mismatches) {
        actual.push((a, b));
    }
    actual.sort_unstable();

    let mut expected = vec![
        (2, 2),
        (0, 2),
        (2, 0),
        (1, 2),
        (2, 1),
        (2, 3),
        (3, 2),
        (2, 4),
        (4, 2),
    ];
    expected.sort_unstable();

    assert_eq!(actual, expected);
}

#[test]
fn dp_matrix_result_cell_iter_0_0_0_0() {
    let subsequence_a_len = 0;
    let subsequence_b_len = 0;
    let k = 0;
    let max_mismatches = 0;
    let mut iter = DpMatrixResultCellIter::new(k, max_mismatches);

    let mut actual = Vec::new();
    while let Some((a, b)) = iter.next(subsequence_a_len, subsequence_b_len, k, max_mismatches) {
        actual.push((a, b));
    }
    actual.sort_unstable();

    let mut expected = vec![(0, 0)];
    expected.sort_unstable();

    assert_eq!(actual, expected);
}

#[test]
fn dp_matrix_result_cell_iter_1_1_1_0() {
    let subsequence_a_len = 1;
    let subsequence_b_len = 1;
    let k = 1;
    let max_mismatches = 0;
    let mut iter = DpMatrixResultCellIter::new(k, max_mismatches);

    let mut actual = Vec::new();
    while let Some((a, b)) = iter.next(subsequence_a_len, subsequence_b_len, k, max_mismatches) {
        actual.push((a, b));
    }
    actual.sort_unstable();

    let mut expected = vec![(1, 1)];
    expected.sort_unstable();

    assert_eq!(actual, expected);
}

#[test]
fn dp_matrix_result_cell_iter_1_1_1_1() {
    let subsequence_a_len = 1;
    let subsequence_b_len = 1;
    let k = 1;
    let max_mismatches = 1;
    let mut iter = DpMatrixResultCellIter::new(k, max_mismatches);

    let mut actual = Vec::new();
    while let Some((a, b)) = iter.next(subsequence_a_len, subsequence_b_len, k, max_mismatches) {
        actual.push((a, b));
    }
    actual.sort_unstable();

    let mut expected = vec![(1, 1), (1, 0), (0, 1)];
    expected.sort_unstable();

    assert_eq!(actual, expected);
}

#[test]
fn dp_matrix_result_cell_iter_1_0_1_1() {
    let subsequence_a_len = 1;
    let subsequence_b_len = 0;
    let k = 1;
    let max_mismatches = 1;
    let mut iter = DpMatrixResultCellIter::new(k, max_mismatches);

    let mut actual = Vec::new();
    while let Some((a, b)) = iter.next(subsequence_a_len, subsequence_b_len, k, max_mismatches) {
        actual.push((a, b));
    }
    actual.sort_unstable();

    let mut expected = vec![(1, 0)];
    expected.sort_unstable();

    assert_eq!(actual, expected);
}

#[test]
fn dp_matrix_result_cell_iter_0_1_1_1() {
    let subsequence_a_len = 0;
    let subsequence_b_len = 1;
    let k = 1;
    let max_mismatches = 1;
    let mut iter = DpMatrixResultCellIter::new(k, max_mismatches);

    let mut actual = Vec::new();
    while let Some((a, b)) = iter.next(subsequence_a_len, subsequence_b_len, k, max_mismatches) {
        actual.push((a, b));
    }
    actual.sort_unstable();

    let mut expected = vec![(0, 1)];
    expected.sort_unstable();

    assert_eq!(actual, expected);
}

#[test]
fn offset_iter_3_3_2_0() {
    let sequence_a_len = 3;
    let sequence_b_len = 3;
    let k = 2;
    let max_mismatches = 0;
    let mut iter = OffsetIter::new();

    let mut actual = Vec::new();
    while let Some((a, b)) = iter.next(sequence_a_len, sequence_b_len, k, max_mismatches) {
        actual.push((a, b));
    }
    actual.sort_unstable();

    let mut expected = vec![(0, 0), (0, 1), (1, 0), (1, 1)];
    expected.sort_unstable();

    assert_eq!(actual, expected);
}

#[test]
fn offset_iter_3_3_2_1() {
    let sequence_a_len = 3;
    let sequence_b_len = 3;
    let k = 2;
    let max_mismatches = 1;
    let mut iter = OffsetIter::new();

    let mut actual = Vec::new();
    while let Some((a, b)) = iter.next(sequence_a_len, sequence_b_len, k, max_mismatches) {
        actual.push((a, b));
    }
    actual.sort_unstable();

    let mut expected = vec![
        (0, 0),
        (0, 1),
        (0, 2),
        (1, 0),
        (2, 0),
        (1, 1),
        (1, 2),
        (2, 1),
    ];
    expected.sort_unstable();

    assert_eq!(actual, expected);
}

#[test]
fn offset_iter_3_3_2_2() {
    let sequence_a_len = 3;
    let sequence_b_len = 3;
    let k = 2;
    let max_mismatches = 2;
    let mut iter = OffsetIter::new();

    let mut actual = Vec::new();
    while let Some((a, b)) = iter.next(sequence_a_len, sequence_b_len, k, max_mismatches) {
        actual.push((a, b));
    }
    actual.sort_unstable();

    let mut expected = vec![
        (0, 0),
        (0, 1),
        (0, 2),
        (0, 3),
        (1, 0),
        (2, 0),
        (3, 0),
        (1, 1),
        (1, 2),
        (1, 3),
        (2, 1),
        (3, 1),
    ];
    expected.sort_unstable();

    assert_eq!(actual, expected);
}
