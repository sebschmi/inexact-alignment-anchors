use crate::anchor_generator::offset_iter::OffsetIter;

#[test]
fn offset_iter_3_3_2_0() {
    let sequence_a_len = 3;
    let sequence_b_len = 3;
    let k = 2;
    let max_mismatches = 0;
    let mut iter = OffsetIter::new();

    let mut actual: Vec<_> = iter
        .peek(sequence_a_len, sequence_b_len, k, max_mismatches)
        .into_iter()
        .collect();
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

    let mut actual: Vec<_> = iter
        .peek(sequence_a_len, sequence_b_len, k, max_mismatches)
        .into_iter()
        .collect();
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

    let mut actual: Vec<_> = iter
        .peek(sequence_a_len, sequence_b_len, k, max_mismatches)
        .into_iter()
        .collect();
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
